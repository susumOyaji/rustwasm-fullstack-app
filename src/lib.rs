use worker::*;
use serde_json::json;
use scraper::{Html, Selector,ElementRef}; // ★追加！HTMLパースに必要
use web_sys::console; // これを追加
use wasm_bindgen::JsValue; // これも追加



#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let yahoo_finance_url = "https://finance.yahoo.co.jp/quote/6758.T";

    // 1. Yahoo!ファイナンスのHTMLコンテンツを取得
    let yahoo_html_body = worker::Fetch::Url(yahoo_finance_url.parse().unwrap())
        .send()
        .await?
        .text() // レスポンスボディをテキストとして取得
        .await?;

    // 抽出するデータ用の変数
    let mut company_code = "N/A".to_string();
    let mut company_name = "N/A".to_string();
    let mut current_price = "N/A".to_string();
    let mut change_amount = "N/A".to_string();
    let mut change_percentage = "N/A".to_string();
    let mut update_time = "N/A".to_string();

    // 2. HTMLをパース
    let document = Html::parse_document(&yahoo_html_body);

    // 3. 各情報に対応するCSSセレクタを定義し、情報を抽出
    // これらのセレクタはYahoo!ファイナンスのHTML構造に依存します。
    // ブラウザの開発者ツールで要素を検証し、正確なセレクタを見つけることが重要です。

    // 企業コード (例: 6758.T) と企業名 (例: ソニーグループ)
    // <h1 class="sc-6s03j6-0 gCgsFq">ソニーグループ (6758.T)</h1> のような構造を想定
    let name_selector = Selector::parse("h2.PriceBoardMain__name__6uDh").unwrap();
    if let Some(element) = document.select(&name_selector).next() {
        company_name = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }

    let code_selector = Selector::parse("span.PriceBoardMain__code__2wso").unwrap();
    if let Some(element) = document.select(&code_selector).next() {
        company_code = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }
    
   

    // 現在の株価 (例: 14,000.5)
    // <dd class="sc-6s03j6-0 fHlKcj" data-test-id="currentPrice">14,000.5</dd> のような構造を想定
    let price_selector = Selector::parse("span.StyledNumber__value__3rXW").unwrap();
    if let Some(element) = document.select(&price_selector).next() {
        current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }






 let selector = Selector::parse("span.StyledNumber__value__3rXW").unwrap();

    let mut values = document
        .select(&selector)
        .filter_map(|el| el.text().next())
        .collect::<Vec<_>>();
    
    for (i, value) in values.iter().enumerate() {
    console::log_1(&JsValue::from_str(&format!("値 {}: {}", i, value)));
    }


   

    if values.len() >= 3 {
    //current_price = values[0].trim().to_string();
    change_amount = values[1].trim().to_string();
    change_percentage = values[2].trim().to_string();

    console::log_1(&JsValue::from_str(&format!("現在価格: {}", current_price)));
    console::log_1(&JsValue::from_str(&format!("前日比: {}", change_amount)));
    console::log_1(&JsValue::from_str(&format!("変動率: {}%", change_percentage)));
} else {
    console::log_1(&JsValue::from_str("値が不足しています。"));
}



 
 


    // 更新日時 (例: 2025/06/17 10:00)
    // <time class="sc-6s03j6-0 fHlKcj" data-test-id="stockTime">2025/06/17 10:00</time> のような構造を想定
    let time_selector = Selector::parse("#root > main > div > section > div.PriceBoardMain__1nb3 > div.PriceBoardMain__priceInformation__3YfB > div.PriceBoardMain__supplementBottom__380e > ul > li:nth-child(2)").unwrap();
    if let Some(element) = document.select(&time_selector).next() {
        update_time = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }


    // 4. 抽出したデータをJSON形式で構築
    let response_data = json!({
        "status": "success",
        "company_code": company_code,
        "company_name": company_name,
        "current_price": current_price,
        "change_amount": change_amount,
        "change_percentage": change_percentage,
        "update_time": update_time,
        "source": "Yahoo! Finance Japan"
    });

    // 5. JSONデータを文字列に変換
    let json_string = response_data.to_string();

    // 6. レスポンスを作成し、Content-Typeヘッダをapplication/jsonに設定
    let mut response = Response::ok(json_string)?;
    response.headers_mut().set("Content-Type", "application/json; charset=utf-8")?;

    // 7. CORSヘッダを追加
    response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
    response.headers_mut().set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    response.headers_mut().set("Access-Control-Allow-Headers", "Content-Type")?;

    Ok(response)
}