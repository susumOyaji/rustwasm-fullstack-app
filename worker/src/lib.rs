use worker::*;
use serde_json::json;
use scraper::{Html, Selector};
use web_sys::console;
use wasm_bindgen::JsValue;

// HTMLから株価データを抽出する関数
fn extract_stock_data(html_body: &str) -> serde_json::Value {
    let document = Html::parse_document(html_body);

    let mut company_code = "N/A".to_string();
    let mut company_name = "N/A".to_string();
    let mut current_price = "N/A".to_string();
    let mut change_amount = "N/A".to_string();
    let mut change_percentage = "N/A".to_string();
    let mut update_time = "N/A".to_string();

    // 企業名
    let name_selector = Selector::parse("h2.PriceBoardMain__name__6uDh").unwrap();
    if let Some(element) = document.select(&name_selector).next() {
        company_name = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }

    // 企業コード
    let code_selector = Selector::parse("span.PriceBoardMain__code__2wso").unwrap();
    if let Some(element) = document.select(&code_selector).next() {
        company_code = element.text().collect::<Vec<_>>().join("").trim().to_string();
        company_code = company_code.trim_matches('(').trim_matches(')').replace(".T", "").to_string();
    }

    // 現在の株価
    let current_price_selector = Selector::parse("div.PriceBoardMain__priceArea__2Mh7 span.StyledNumber__value__3rXW").unwrap();
    if let Some(element) = document.select(&current_price_selector).next() {
        current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }

    // 前日比 (金額とパーセント)
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

    // 更新日時
    //let update_time_selector = Selector::parse("ul.PriceBoardMain__times__1A li:nth-child(2)").unwrap();
    //if let Some(element) = document.select(&update_time_selector).next() {
    //    update_time = element.text().collect::<Vec<_>>().join("").trim().to_string();
    //}

    let time_selector = Selector::parse("#root > main > div > section > div.PriceBoardMain__1nb3 > div.PriceBoardMain__priceInformation__3YfB > div.PriceBoardMain__supplementBottom__380e > ul > li:nth-child(2)").unwrap();

if let Some(element) = document.select(&time_selector).next() {

update_time = element.text().collect::<Vec<_>>().join("").trim().to_string();
}



    json!({
        "company_code": company_code,
        "company_name": company_name,
        "current_price": current_price,
        "change_amount": change_amount,
        "change_percentage": change_percentage,
        "update_time": update_time,
        "source": "Yahoo! Finance Japan"
    })
}

// fetch イベントハンドラ
#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console::log_1(&JsValue::from_str("Worker received request."));

    // OPTIONSリクエスト（CORSプリフライト）
    if req.method() == Method::Options {
        let mut response = Response::empty()?; // 修正済み
        response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
        response.headers_mut().set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
        response.headers_mut().set("Access-Control-Allow-Headers", "Content-Type")?;
        console::log_1(&JsValue::from_str("Handling OPTIONS request for CORS."));
        return Ok(response);
    }

    let url = req.url()?;
    let query_params = url.query_pairs();

    let mut stock_codes: Vec<String> = Vec::new();

    for (key, value) in query_params {
        if key == "codes" {
            stock_codes = value.split(',').map(|s| s.trim().to_string()).collect();
            break;
        }
    }

    if stock_codes.is_empty() {
        stock_codes.push("6758.T".to_string());
        console::log_1(&JsValue::from_str("No 'codes' query parameter provided. Using default: 6758.T"));
    } else {
        console::log_1(&JsValue::from_str(&format!("Received codes: {:?}", stock_codes)));
    }

    let mut results = Vec::new();

    for code in stock_codes {
        let yahoo_finance_url = format!("https://finance.yahoo.co.jp/quote/{}", code);

        console::log_1(&JsValue::from_str(&format!("Fetching data for: {}", code)));

        let fetch_result = worker::Fetch::Url(yahoo_finance_url.parse().unwrap())
            .send()
            .await?
            .text()
            .await;

        match fetch_result {
            Ok(html_body) => {
                console::log_1(&JsValue::from_str(&format!("Fetched HTML for {}", code)));
                let stock_data = extract_stock_data(&html_body);
                results.push(stock_data);
            }
            Err(e) => {
                console::log_1(&JsValue::from_str(&format!("Failed to fetch HTML for {}: {:?}", code, e)));
                results.push(json!({
                    "company_code": code,
                    "status": "error",
                    "message": format!("Fetch error: {:?}", e)
                }));
            }
        }
    }

    let response_data = json!({
        "status": "success",
        "data": results
    });

    console::log_1(&JsValue::from_str(&format!("Response JSON: {}", response_data.to_string())));

    let mut response = Response::ok(response_data.to_string())?;
    response.headers_mut().set("Content-Type", "application/json; charset=utf-8")?;
    response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
    response.headers_mut().set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    response.headers_mut().set("Access-Control-Allow-Headers", "Content-Type")?;

    Ok(response)
}

#[event(start)]
fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
