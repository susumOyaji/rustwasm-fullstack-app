use futures::future::join_all;
use scraper::{Html, Selector};
use serde::Serialize;
use worker::{event, Env, Method, Request, Response, Result, Context, Headers, RequestInit};

// レスポンスのJSON形式を定義
#[derive(Serialize)]
struct ApiResponse {
    status: String,
    data: Vec<FinancialData>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct FinancialData {
    name: Option<String>,
    code: Option<String>,
    update_time: Option<String>,
    current_value: Option<String>,
    bid_value: Option<String>,
    previous_day_change: Option<String>,
    change_rate: Option<String>,
}

pub fn parse_financial_data(html_content: &str, url: String) -> FinancialData {
    let document = Html::parse_document(html_content);

    let clean_change_rate = |s: Option<String>| {
        s.map(|text| {
            text.replace(|c: char| c.is_whitespace(), "")
                .replace("(", "")
                .replace(")", "")
                .trim()
                .to_string()
        })
    };

    if url.contains("USDJPY=FX") {
        let get_text = |el: Option<scraper::ElementRef>| el.map(|e| e.text().collect::<String>().trim().to_string());

        let name_selector = Selector::parse("h2[class*=\"_BasePriceBoard__name\"]").expect("Failed to parse name selector");
        
        let mut data = FinancialData {
            name: get_text(document.select(&name_selector).next()),
            code: Some("USDJPY=FX".to_string()),
            update_time: None, // The time is not easily scrapable anymore
            current_value: None, // Ask
            bid_value: None,
            previous_day_change: None, // Not available on FX page
            change_rate: None,
        };

        let item_selector = Selector::parse("div[class*=\"_FxPriceBoard__item\"] > dl").expect("Failed to parse item selector");
        let term_selector = Selector::parse("dt").expect("Failed to parse term selector");
        let price_selector = Selector::parse("dd span[class*=\"_FxPriceBoard__price\"]").expect("Failed to parse price selector");

        for item in document.select(&item_selector) {
            let term = match get_text(item.select(&term_selector).next()) {
                Some(text) => text,
                None => continue,
            };
            
            let value = get_text(item.select(&price_selector).next());

            match term.as_str() {
                "Bid（売値）" => data.bid_value = value,
                "Ask（買値）" => data.current_value = value, // Using current_value for Ask
                "Change（始値比）" => data.change_rate = clean_change_rate(value),
                _ => (),
            }
        }
        data
    } else if url.contains("998407.O") {
        let get_text = |el: Option<scraper::ElementRef>| el.map(|e| e.text().collect::<String>().trim().to_string());

        let name_selector = Selector::parse("h2[class*=\"PriceBoard__name\"]").expect("Failed to parse name selector");
        
        let mut data = FinancialData {
            name: get_text(document.select(&name_selector).next()),
            code: Some("998407.O".to_string()),
            update_time: None,
            current_value: None,
            bid_value: None, // N/A for indices
            previous_day_change: None,
            change_rate: None,
        };

        let current_value_selector = Selector::parse(".PriceBoard__price__1V0k .StyledNumber__value__3rXW").expect("Failed to parse current value selector");
        let previous_day_change_selector = Selector::parse(".PriceChangeLabel__primary__Y_ut .StyledNumber__value__3rXW").expect("Failed to parse previous day change selector");
        let change_rate_selector = Selector::parse(".PriceChangeLabel__secondary__3BXI").expect("Failed to parse change rate selector");
        let update_time_selector = Selector::parse("time").expect("Failed to parse update time selector");

        data.current_value = get_text(document.select(&current_value_selector).next());
        data.previous_day_change = get_text(document.select(&previous_day_change_selector).next());
        data.change_rate = clean_change_rate(get_text(document.select(&change_rate_selector).next()));
        data.update_time = get_text(document.select(&update_time_selector).next());
        
        data
    } else if url.contains("^DJI") {
        let get_text = |el: Option<scraper::ElementRef>| el.map(|e| e.text().collect::<String>().trim().to_string());

        let code = url.split('/').last().map(|s| s.to_string());
        let name_selector = Selector::parse("h2[class*=\"_BasePriceBoard__name\"]").expect("Failed to parse name selector");
        
        let mut data = FinancialData {
            name: get_text(document.select(&name_selector).next()),
            code,
            update_time: None,
            current_value: None,
            bid_value: None, // N/A for indices
            previous_day_change: None,
            change_rate: None,
        };

        let current_value_selector = Selector::parse("span[class*=\"_CommonPriceBoard__price\"] span[class*=\"_StyledNumber__value\"]").expect("Failed to parse current value selector");
        let change_selector = Selector::parse("span[class*=\"__primary\"] span[class*=\"_StyledNumber__value\"]").expect("Failed to parse change selector");
        let change_rate_selector = Selector::parse("span[class*=\"__secondary\"]").expect("Failed to parse change rate selector");
        let time_selector = Selector::parse("li[class*=\"_CommonPriceBoard__time\"] > time").expect("Failed to parse time selector");

        data.current_value = get_text(document.select(&current_value_selector).next());
        data.previous_day_change = get_text(document.select(&change_selector).next());
        data.change_rate = clean_change_rate(get_text(document.select(&change_rate_selector).next()));
        data.update_time = get_text(document.select(&time_selector).next());
        
        data
    } else {
        let title_selector = Selector::parse("title").expect("Failed to parse title selector");
        let title_text = document.select(&title_selector).next().map(|e| e.text().collect::<String>()).unwrap_or_default();
        
        let name = title_text.split('【').next().map(|s| s.trim().to_string());
        let code = title_text.split('【').nth(1).and_then(|s| s.split('】').next()).map(|s| s.to_string());

        let mut data = FinancialData {
            name,
            code,
            update_time: None,
            current_value: None,
            bid_value: None, // Not available on stock page
            previous_day_change: None,
            change_rate: None,
        };

        let current_value_selector = Selector::parse(".PriceBoard__price__1V0k .StyledNumber__value__3rXW").expect("Failed to parse current value selector");
        let previous_day_change_selector = Selector::parse(".PriceChangeLabel__primary__Y_ut .StyledNumber__value__3rXW").expect("Failed to parse previous day change selector");
        let change_rate_selector = Selector::parse(".PriceChangeLabel__secondary__3BXI").expect("Failed to parse change rate selector");

        data.current_value = document.select(&current_value_selector).next().map(|e| e.text().collect::<String>());
        data.previous_day_change = document.select(&previous_day_change_selector).next().map(|e| e.text().collect::<String>());
        data.change_rate = document.select(&change_rate_selector)
            .next()
            .map(|e| {
                let text = e.text().collect::<String>();
                text.replace(|c: char| c.is_whitespace(), "")
                    .replace("(", "") // Remove opening parenthesis
                    .replace(")", "") // Remove closing parenthesis
                    .trim()
                    .to_string()
            });

        let update_time_selector = Selector::parse("time").expect("Failed to parse update time selector");
        data.update_time = document.select(&update_time_selector).next().map(|e| e.text().collect::<String>());

        data
    }
}

pub fn get_selectors_for_code(code: &str) -> Vec<String> {
    if code.contains("USDJPY=FX") {
        vec![
            "h2[class*=\"_BasePriceBoard__name\"]".to_string(),
            "div[class*=\"_FxPriceBoard__item\"] > dl".to_string(),
            "dd span[class*=\"_FxPriceBoard__price\"]".to_string(), // for bid/ask
            "dt".to_string(), // for bid/ask terms
        ]
    } else if code.contains("998407.O") {
        vec![
            "h2[class*=\"PriceBoard__name\"]".to_string(),
            ".PriceBoard__price__1V0k .StyledNumber__value__3rXW".to_string(),
            ".PriceChangeLabel__primary__Y_ut .StyledNumber__value__3rXW".to_string(),
            ".PriceChangeLabel__secondary__3BXI".to_string(),
            "time".to_string(),
        ]
    } else if code.contains("^DJI") {
        vec![
            "h2[class*=\"_BasePriceBoard__name\"]".to_string(),
            "span[class*=\"_CommonPriceBoard__price\"] span[class*=\"_StyledNumber__value\"]".to_string(),
            "span[class*=\"__primary\"] span[class*=\"_StyledNumber__value\"]".to_string(),
            "span[class*=\"__secondary\"]".to_string(),
            "li[class*=\"_CommonPriceBoard__time\"] > time".to_string(),
        ]
    } else { // General stock
        vec![
            "title".to_string(), // Used for name and code extraction
            ".PriceBoard__price__1V0k .StyledNumber__value__3rXW".to_string(),
            ".PriceChangeLabel__primary__Y_ut .StyledNumber__value__3rXW".to_string(),
            ".PriceChangeLabel__secondary__3BXI".to_string(),
            "time".to_string(),
        ]
    }
}

pub async fn fetch_financial_data(url: String) -> Result<FinancialData> {
    if let Err(e) = url.parse::<worker::Url>() {
        return Err(worker::Error::from(format!("URL parse error: {}", e)));
    };

    let mut headers = Headers::new();
    headers.set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")?;
    headers.set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")?;
    headers.set("Accept-Language", "en-US,en;q=0.9,ja;q=0.8")?;
    headers.set("Accept-Encoding", "gzip, deflate, br")?;
    headers.set("Connection", "keep-alive")?;
    headers.set("Upgrade-Insecure-Requests", "1")?;

    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Get).with_headers(headers);

    let request = Request::new_with_init(&url, &request_init)?;
    let html_content = worker::Fetch::Request(request).send().await?.text().await?;
    
    let data = parse_financial_data(&html_content, url);
    Ok(data)
}

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.method() == Method::Options {
        let mut headers = Headers::new();
        headers.set("Access-Control-Allow-Origin", "*")?;
        headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
        headers.set("Access-Control-Allow-Headers", "Content-Type")?;
        return Ok(Response::empty()?.with_headers(headers));
    }

    let url = req.url()?;
    if url.path() == "/api/quote" {
        let query_params: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        
        if let Some(codes_str) = query_params.get("codes") {
            let codes: Vec<&str> = codes_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
            
            let futures: Vec<_> = codes.into_iter().map(|code| {
                let yahoo_url = format!("https://finance.yahoo.co.jp/quote/{}", code);
                fetch_financial_data(yahoo_url)
            }).collect();
            
            let results: Vec<FinancialData> = join_all(futures).await
                .into_iter()
                .filter_map(Result::ok)
                .collect();

            let api_response = ApiResponse {
                status: "success".to_string(),
                data: results,
            };

            let mut response = Response::from_json(&api_response)?;
            response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
            Ok(response)

        } else {
            Response::error("Query parameter 'codes' is required.", 400)
        }
    } else {
        Response::error("Not Found", 404)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    fn test_parse_stock_data() {
        let sample_html = r#"<!DOCTYPE html>
            <html>
            <head>
                <title>テスト株式会社【9999.T】：株価・株式情報 - Yahoo!ファイナンス</title>
            </head>
            <body>
                <div class="PriceBoard__priceInformation__78Tl">
                    <div class="PriceBoard__priceBlock__1PmX">
                        <span class="StyledNumber__1fof StyledNumber--vertical__2aoh PriceBoard__price__1V0k">
                            <span class="StyledNumber__item__1-yu">
                                <span class="StyledNumber__value__3rXW">1,234</span>
                            </span>
                        </span>
                        <div class="PriceChangeLabel__2Kf0 PriceChangeLabel--red__2zs-">
                            <dl class="PriceChangeLabel__definition__3Jdj">
                                <dt class="PriceChangeLabel__term__3H4k">前日比</dt>
                                <dd class="PriceChangeLabel__description__a5Lp">
                                    <span class="StyledNumber__1fof StyledNumber--horizontal__HwH8 PriceChangeLabel__prices__30Ey">
                                        <span class="StyledNumber__item__1-yu PriceChangeLabel__primary__Y_ut">
                                            <span class="StyledNumber__value__3rXW">+56</span>
                                        </span>
                                        <span class="StyledNumber__item__1-yu StyledNumber__item--secondary__RTJc StyledNumber__item--small__2hJE PriceChangeLabel__secondary__3BXI">
                                            <span class="StyledNumber__punctuation__3pWV">(</span>
                                            <span class="StyledNumber__value__3rXW">+4.75</span>
                                            <span class="StyledNumber__suffix__2SD5">%</span>
                                            <span class="StyledNumber__punctuation__3pWV">)</span>
                                        </span>
                                    </span>
                                </dd>
                            </dl>
                        </div>
                    </div>
                </div>
                <div>
                    <span>リアルタイム株価</span>
                    <time>15:00</time>
                </div>
            </body>
            </html>
        "#;
        let url = "https://finance.yahoo.co.jp/quote/9999.T".to_string();
        let expected_data = FinancialData {
            name: Some("テスト株式会社".to_string()),
            code: Some("9999.T".to_string()),
            update_time: Some("15:00".to_string()),
            current_value: Some("1,234".to_string()),
            bid_value: None,
            previous_day_change: Some("+56".to_string()),
            change_rate: Some("(+4.75%)".to_string()),
        };

        let result_data = parse_financial_data(sample_html, url);
        assert_eq!(result_data, expected_data);
    }

    #[test]
    fn test_parse_usdjpy_data() {
        let sample_html = r#"<!DOCTYPE html>
            <html>
            <body>
                <h2 class="_BasePriceBoard__name_1tkwp_66">米ドル/円</h2>
                <div class="_FxPriceBoard__items_p3rf8_6">
                    <div class="_FxPriceBoard__item_p3rf8_6">
                        <dl>
                            <dt>Bid（売値）</dt>
                            <dd><span class="_FxPriceBoard__price_p3rf8_54">147.045</span></dd>
                        </dl>
                    </div>
                    <div class="_FxPriceBoard__item_p3rf8_6">
                        <dl>
                            <dt>Ask（買値）</dt>
                            <dd><span class="_FxPriceBoard__price_p3rf8_54">147.047</span></dd>
                        </dl>
                    </div>
                    <div class="_FxPriceBoard__item_p3rf8_6">
                        <dl>
                            <dt>Change（始値比）</dt>
                            <dd><span class="_FxPriceBoard__price_p3rf8_54">0.184</span></dd>
                        </dl>
                    </div>
                </div>
            </body>
            </html>
        "#;
        let url = "https://finance.yahoo.co.jp/quote/USDJPY=FX".to_string();
        let expected_data = FinancialData {
            name: Some("米ドル/円".to_string()),
            code: Some("USDJPY=FX".to_string()),
            update_time: None,
            current_value: Some("147.047".to_string()), // Ask
            bid_value: Some("147.045".to_string()),
            previous_day_change: None,
            change_rate: Some("0.184".to_string()),
        };

        let result_data = parse_financial_data(sample_html, url);
        assert_eq!(result_data, expected_data);
    }

    #[wasm_bindgen_test]
    #[ignore]
    async fn verify_selectors_live() {
        let targets = vec![
            (
                "https://finance.yahoo.co.jp/quote/7203.T", // Toyota (general stock)
                vec![
                    "title",
                    ".PriceBoard__price__1V0k .StyledNumber__value__3rXW",
                    ".PriceChangeLabel__primary__Y_ut .StyledNumber__value__3rXW",
                    ".PriceChangeLabel__secondary__3BXI",
                    "time",
                ],
            ),
            (
                "https://finance.yahoo.co.jp/quote/998407.O", // Nikkei 225
                vec![ // Uses same selectors as general stock
                    "title",
                    ".PriceBoard__price__1V0k .StyledNumber__value__3rXW",
                    ".PriceChangeLabel__primary__Y_ut .StyledNumber__value__3rXW",
                    ".PriceChangeLabel__secondary__3BXI",
                    "time",
                ],
            ),
            (
                "https://finance.yahoo.co.jp/quote/USDJPY=FX", // FX
                vec![
                    "h2[class*=\"_BasePriceBoard__name\"]",
                    "div[class*=\"_FxPriceBoard__item\"] > dl",
                ],
            ),
            (
                "https://finance.yahoo.co.jp/quote/^DJI", // Dow Jones
                vec![
                    "h2[class*=\"_BasePriceBoard__name\"]",
                    "span[class*=\"_CommonPriceBoard__price\"] span[class*=\"_StyledNumber__value\"]",
                    "span[class*=\"__primary\"] span[class*=\"_StyledNumber__value\"]",
                    "span[class*=\"__secondary\"]",
                    "li[class*=\"_CommonPriceBoard__time\"] > time",
                ],
            ),
        ];

        for (url, selectors) in targets {
            println!("Checking URL: {}", url);
            
            let mut headers = worker::Headers::new();
            headers.set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36").unwrap();
            let mut req_init = worker::RequestInit::new();
            req_init.with_headers(headers);
            let req = worker::Request::new_with_init(url, &req_init).unwrap();

            let mut res = worker::Fetch::Request(req).send().await.unwrap();
            let html_content = res.text().await.unwrap();
            let document = Html::parse_document(&html_content);

            for selector_str in selectors {
                let selector = Selector::parse(selector_str).expect("Failed to parse selector");
                let found = document.select(&selector).next().is_some();
                
                println!("  - Selector '{}': {}", selector_str, if found { "OK" } else { "FAILED" });
                assert!(found, "Selector check failed for URL: {}. Selector: {}", url, selector_str);
            }
        }
    }
}
