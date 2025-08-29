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

    if url.contains("USDJPY=FX") {
        let name_selector = Selector::parse("h2[class*=\"PriceBoard__name\"]").expect("Failed to parse name selector");
        let time_selector = Selector::parse("time").expect("Failed to parse time selector");
        let fx_price_selector = Selector::parse("div > span").expect("Failed to parse fx price selector");
        let bid_term_selector = Selector::parse("dt[class*=\"_FxPriceBoard__term\"]").expect("Failed to parse bid term selector");
        let dd_selector = Selector::parse("dd[class*=\"_FxPriceBoard__description\"]").expect("Failed to parse dd selector");
        let price_span_selector = Selector::parse("span[class*=\"_FxPriceBoard__price\"]").expect("Failed to parse price span selector");
        let dd_plain_selector = Selector::parse("dd").expect("Failed to parse dd plain selector");
        let span_selector = Selector::parse("span").expect("Failed to parse span selector");
        let get_text = |el: Option<scraper::ElementRef>| el.map(|e| e.text().collect::<String>());

        let mut data = FinancialData {
            name: get_text(document.select(&name_selector).next()),
            code: Some("USDJPY=FX".to_string()),
            update_time: get_text(document.select(&time_selector).next()),
            current_value: None,
            bid_value: None,
            previous_day_change: None,
            change_rate: None,
        };

        data.current_value = get_text(document.select(&fx_price_selector).next());
        for dt in document.select(&bid_term_selector) {
            if dt.text().collect::<String>().trim() == "Bid（売値）" {
                if let Some(dl) = dt.parent().and_then(scraper::ElementRef::wrap) {
                    data.bid_value = get_text(dl.select(&dd_selector).next().and_then(|dd| dd.select(&price_span_selector).next()));
                    break;
                }
            }
        }
        for dt in document.select(&bid_term_selector) {
            if dt.text().collect::<String>().trim() == "Change（始値比）" {
                if let Some(dl) = dt.parent().and_then(scraper::ElementRef::wrap) {
                    data.change_rate = get_text(dl.select(&dd_plain_selector).next().and_then(|dd| dd.select(&span_selector).next()));
                    break;
                }
            }
        }
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
                text.replace(|c: char| c.is_whitespace(), "").trim().to_string()
            });

        let update_time_selector = Selector::parse("time").expect("Failed to parse update time selector");
        data.update_time = document.select(&update_time_selector).next().map(|e| e.text().collect::<String>());

        data
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

    #[test]
    fn test_parse_stock_data() {
        let sample_html = r#"
            <!DOCTYPE html>
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
        let sample_html = r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2 class="PriceBoard__name__166W">米ドル/円</h2>
                <div>
                    <span data-testid="text-for-price">152.50</span>
                </div>
                <time>12:00</time>
            </body>
            </html>
        "#;
        let url = "https://finance.yahoo.co.jp/quote/USDJPY=FX".to_string();
        let expected_data = FinancialData {
            name: Some("米ドル/円".to_string()),
            code: Some("USDJPY=FX".to_string()),
            update_time: Some("12:00".to_string()),
            current_value: Some("152.50".to_string()),
            bid_value: None,
            previous_day_change: None,
            change_rate: None,
        };

        let result_data = parse_financial_data(sample_html, url);
        assert_eq!(result_data, expected_data);
    }
}