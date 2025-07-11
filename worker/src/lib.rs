// src/lib.rs

use worker::{event, console_log, Env, Request, Response, Result, Method, Context};
use scraper::{Html, Selector};
use serde_json::json;
use worker::wasm_bindgen::JsValue;

// HTMLから個別株価データを抽出する関数 (変更なし)
fn extract_stock_data(html_body: &str, code: &str) -> serde_json::Value {
    let document = Html::parse_document(html_body);

    let mut company_code = code.to_string();
    let mut company_name = "N/A".to_string();
    let mut current_price = "N/A".to_string();
    let mut change_amount = "N/A".to_string();
    let mut change_percentage = "N/A".to_string();
    let mut update_time = "N/A".to_string();

    let name_selector = Selector::parse("h2.PriceBoardMain__name__6uDh").unwrap();
    if let Some(element) = document.select(&name_selector).next() {
        company_name = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }

    let code_selector = Selector::parse("span.PriceBoardMain__code__2wso").unwrap();
    if let Some(element) = document.select(&code_selector).next() {
        let extracted_code = element.text().collect::<Vec<_>>().join("").trim().to_string();
        if !extracted_code.is_empty() {
             company_code = extracted_code.trim_matches('(').trim_matches(')').replace(".T", "").to_string();
        }
    }

    let current_price_selector = Selector::parse("span.StyledNumber__value__3rXW").unwrap();
    if let Some(element) = document.select(&current_price_selector).next() {
        current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }

    let price_change_dd_selector = Selector::parse("dd.PriceChangeLabel__description__a5Lp").unwrap();
    if let Some(dd_element) = document.select(&price_change_dd_selector).next() {
        let value_spans_selector = Selector::parse("span.StyledNumber__value__3rXW").unwrap();
        let values: Vec<String> = dd_element
            .select(&value_spans_selector)
            .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if values.len() >= 2 {
            change_amount = values[0].clone();
            change_percentage = values[1].clone();
        }
    }

    // 更新日時。動的なクラス名に依存しないように、より一般的なセレクタに変更。
    let time_selector = Selector::parse("div[class*='supplement'] span[class*='time']").unwrap();
    if let Some(element) = document.select(&time_selector).next() {
        update_time = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }

    json!({
        "type": "stock",
        "company_code": company_code,
        "company_name": company_name,
        "current_price": current_price,
        "change_amount": change_amount,
        "change_percentage": change_percentage,
        "update_time": update_time,
        "source": "Yahoo! Finance Japan"
    })
}

// ダウ平均株価をスクレイピングする関数 (変更なし)
async fn scrape_dow_average() -> Result<serde_json::Value> {
    let target_url = "https://finance.yahoo.co.jp/quote/%5EDJI";
    console_log!("Scraping Dow Average from: {}", target_url);

    let fetch_result = worker::Fetch::Url(target_url.parse().unwrap())
        .send()
        .await?
        .text()
        .await;

    match fetch_result {
        Ok(html_body) => {
            let document = Html::parse_document(&html_body);
            
            let mut current_price = "N/A".to_string();
            let mut change_amount = "N/A".to_string();
            let mut change_percentage = "N/A".to_string();
            let mut update_time = "N/A".to_string();

            let price_selector = Selector::parse("span._StyledNumber__value_x0ii7_10").unwrap();
            if let Some(element) = document.select(&price_selector).next() {
                current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
            }

            // 前日比（絶対額）
            // セレクタはYahoo Financeのページ構造に依存します
            let change_amount_selector = Selector::parse("span._StyledNumber__item_x0ii7_7._PriceChangeLabel__primary_l4zfe_55 > span._StyledNumber__value_x0ii7_10").unwrap();
            if let Some(amount_el) = document.select(&change_amount_selector).next() {
                change_amount = amount_el.text().collect::<Vec<_>>().join("").trim().to_string();
            }

            // 騰落率（パーセント）
            // セレクタはYahoo Financeのページ構造に依存します
            let change_percentage_selector = Selector::parse("span._StyledNumber__item_x0ii7_7._StyledNumber__item--secondary_x0ii7_27._PriceChangeLabel__secondary_l4zfe_61 > span._StyledNumber__value_x0ii7_10").unwrap();
            if let Some(percent_el) = document.select(&change_percentage_selector).next() {
                change_percentage = percent_el.text().collect::<Vec<_>>().join("").trim().to_string();
            }
           
            
            // 更新日時。動的なクラス名に依存しないように、より一般的なセレクタに変更。
            let time_selector = Selector::parse("div[class*='supplement'] li").unwrap();
             if let Some(element) = document.select(&time_selector).next() {
                 update_time = element.text().collect::<Vec<_>>().join("").trim().to_string();
            }

            Ok(json!({
                "type": "index",
                "index_name": "Dow Jones Industrial Average",
                "symbol": "^DJI",
                "current_price": current_price,
                "change_amount": change_amount,
                "change_percentage": change_percentage,
                "update_time": update_time,
                "source": "Yahoo! Finance (US)"
            }))
        },
        Err(e) => {
            console_log!("Failed to scrape Dow Average: {:?}", e);
            Ok(json!({
                "type": "index",
                "index_name": "Dow Jones Industrial Average",
                "symbol": "^DJI",
                "status": "error",
                "message": format!("Scraping error: {:?}", e)
            }))
        }
    }
}

// 日経平均株価をスクレイピングする関数 (変更なし)
async fn scrape_nikkei_average() -> Result<serde_json::Value> {
    let target_url = "https://finance.yahoo.co.jp/quote/998407.O";
    console_log!("Scraping Nikkei Average from: {}", target_url);

    let fetch_result = worker::Fetch::Url(target_url.parse().unwrap())
        .send()
        .await?
        .text()
        .await;

    match fetch_result {
        Ok(html_body) => {
            let document = Html::parse_document(&html_body);
            
            let mut current_price = "N/A".to_string();
            let mut change_amount = "N/A".to_string();
            let mut change_percentage = "N/A".to_string();
            let mut update_time = "N/A".to_string();

            let price_selector = Selector::parse("span.number__3wVT").unwrap();
            if let Some(element) = document.select(&price_selector).next() {
                current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
            }

            let change_amount_selector = Selector::parse("p.changePriceLabel__2rHy span.changePrice__3dJY span span.number__3wVT").unwrap();
            if let Some(amount_el) = document.select(&change_amount_selector).next() {
                change_amount = amount_el.text().collect::<Vec<_>>().join("").trim().to_string();
            }

            let change_percentage_selector = Selector::parse("p.changePriceLabel__2rHy span.changePriceRate__3pJv span span.number__3wVT").unwrap();
            if let Some(percent_el) = document.select(&change_percentage_selector).next() {
                change_percentage = percent_el.text().collect::<Vec<_>>().join("").trim().to_string();
            }

            // 更新日時。動的なクラス名に依存しないように、より一般的なセレクタに変更。
            let time_selector = Selector::parse("div[class*='supplement'] li").unwrap();
             if let Some(element) = document.select(&time_selector).next() {
                 update_time = element.text().collect::<Vec<_>>().join("").trim().to_string();
            }
            
            Ok(json!({
                "type": "index",
                "index_name": "Nikkei 225 Futures",
                "symbol": "998407.O",
                "current_price": current_price,
                "change_amount": change_amount,
                "change_percentage": change_percentage,
                "update_time": update_time,
                "source": "Yahoo! Finance Japan (Futures)"
            }))
        },
        Err(e) => {
            console_log!("Failed to scrape Nikkei Average from 998407.O: {:?}", e);
            Ok(json!({
                "type": "index",
                "index_name": "Nikkei 225 Futures",
                "symbol": "998407.O",
                "status": "error",
                "message": format!("Scraping error: {:?}", e)
            }))
        }
    }
}


// src/lib.rs

// ... (既存のuseステートメントやextract_stock_data, scrape_dow_average, scrape_nikkei_average 関数) ...

// 米ドル/円の外国為替レートをスクレイピングする関数
// Yahoo!ファイナンス日本版のUSDJPY=XページのHTML構造に依存します。
// セレクタはウェブサイトの変更により機能しなくなる可能性があります。
async fn scrape_usdjpy() -> Result<serde_json::Value> {
    let target_url = "https://finance.yahoo.co.jp/quote/USDJPY=X";
    console_log!("Scraping USDJPY from: {}", target_url);

    let fetch_result = worker::Fetch::Url(target_url.parse().unwrap())
        .send()
        .await?
        .text()
        .await;

    match fetch_result {
        Ok(html_body) => {
            let document = Html::parse_document(&html_body);
            
            let mut current_price = "N/A".to_string();
            let mut change_amount = "N/A".to_string();
            let mut change_percentage = "N/A".to_string();
            let mut update_time = "N/A".to_string(); // 為替では「更新日時」ではなく「取引時間」などになることもあります

            // 現在価格
            //#contents > div > div.board__1-Hj > div.contents__103w > div.nameAndPrice__2AQd > p.price__1c9r > span
            let price_selector = Selector::parse("#contents > div > div.board__1-Hj > div.contents__103w > div.nameAndPrice__2AQd > p.price__1c9r > span").unwrap();
            if let Some(element) = document.select(&price_selector).next() {
                current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
            }

           
            // 更新日時。動的なクラス名に依存しないように、より一般的なセレクタに変更。
            let time_selector = Selector::parse("div[class*='supplement'] p").unwrap();
            if let Some(element) = document.select(&time_selector).next() {
                update_time = element.text().collect::<Vec<_>>().join("").trim().to_string();
            }
            
            Ok(json!({
                "type": "forex", // 新しいタイプを追加
                "currency_pair": "米ドル/円",
                "symbol": "USDJPY=X",
                "current_price": current_price,
                //"change_amount": change_amount,
                //"change_percentage": change_percentage,
                "update_time": update_time,
                "source": "Yahoo! Finance Japan"
            }))
        },
        Err(e) => {
            console_log!("Failed to scrape USDJPY: {:?}", e);
            Ok(json!({
                "type": "forex",
                "currency_pair": "米ドル/円",
                "symbol": "USDJPY=X",
                "status": "error",
                "message": format!("Scraping error: {:?}", e)
            }))
        }
    }
}


// fetch イベントハンドラ
#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Worker received request.");

    if req.method() == Method::Options {
        let mut response = Response::empty()?;
        response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
        response.headers_mut().set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
        response.headers_mut().set("Access-Control-Allow-Headers", "Content-Type")?;
        console_log!("Handling OPTIONS request for CORS.");
        return Ok(response);
    }

    let url = req.url()?;
    let path = url.path();
    let query_params = url.query_pairs();

    if path == "/" || path == "/api/stocks" {
        let mut default_codes = vec![
            "^DJI".to_string(),
            "^N225".to_string(),
           // "6758.T".to_string(),
            "USDJPY=X".to_string(), // ★★★ ここを追加 ★★★
        ];

        let mut received_codes_from_param: Vec<String> = Vec::new();
        for (key, value) in query_params {
            if key == "codes" {
                received_codes_from_param = value.split(',').map(|s| s.trim().to_string()).collect();
                break;
            }
        }

        let mut final_codes_to_fetch = Vec::new();

        if received_codes_from_param.is_empty() {
            final_codes_to_fetch.extend(default_codes);
            console_log!("No 'codes' query parameter provided. Using default: {:?}", final_codes_to_fetch);
        } else {
            final_codes_to_fetch.extend(default_codes);
            final_codes_to_fetch.extend(received_codes_from_param.clone());
            console_log!("Received codes: {:?}. Appending to defaults. Final list: {:?}", received_codes_from_param, final_codes_to_fetch);
        }

        let mut results = Vec::new();

        for code in final_codes_to_fetch {
            let processed_code = code.to_uppercase(); 

            let result_json = match processed_code.as_str() {
                "^DJI" => scrape_dow_average().await?,
                "^N225" => scrape_nikkei_average().await?, 
                "USDJPY=X" => scrape_usdjpy().await?, // ★★★ ここを追加 ★★★
                _ => {
                    let yahoo_finance_url = format!("https://finance.yahoo.co.jp/quote/{}", code);
                    console_log!("Fetching data for: {}", code);
                    
                    let fetch_result = worker::Fetch::Url(yahoo_finance_url.parse().unwrap())
                        .send()
                        .await?
                        .text()
                        .await;

                    match fetch_result {
                        Ok(html_body) => {
                            console_log!("Fetched HTML for {}", code);
                            let stock_data = extract_stock_data(&html_body, &code);
                            stock_data
                        }
                        Err(e) => {
                            console_log!("Failed to fetch HTML for {}: {:?}", code, e);
                            json!({
                                "type": "stock",
                                "company_code": code,
                                "status": "error",
                                "message": format!("Fetch error: {:?}", e)
                            })
                        }
                    }
                }
            };
            results.push(result_json);
        }

        let response_data = json!({
            "status": "success",
            "data": results
        });

        console_log!("Response JSON: {}", response_data.to_string());

        let mut response = Response::ok(response_data.to_string())?;
        response.headers_mut().set("Content-Type", "application/json; charset=utf-8")?;
        response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
        response.headers_mut().set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
        response.headers_mut().set("Access-Control-Allow-Headers", "Content-Type")?;

        Ok(response)
    } else {
        Response::error("Not Found", 404)
    }
}


// ... (start関数はそのまま) ...
#[event(start)]
fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}