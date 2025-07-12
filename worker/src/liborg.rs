use worker::*;
use serde_json::json;
use scraper::{Html, Selector, ElementRef, Node}; // Node を追加
use web_sys::console;
use wasm_bindgen::JsValue;

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let yahoo_finance_url = "https://finance.yahoo.co.jp/quote/6758.T";

    console::log_1(&JsValue::from_str("Fetching Yahoo! Finance HTML..."));

    // 1. Yahoo!ファイナンスのHTMLコンテンツを取得
    let yahoo_html_body = worker::Fetch::Url(yahoo_finance_url.parse().unwrap())
        .send()
        .await?
        .text()
        .await?;

    console::log_1(&JsValue::from_str("HTML fetched. Parsing..."));

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

    // 企業名 (例: ソニーグループ(株))
    // Yahooの構造では、h2.PriceBoard__name__166W は `ソニーグループ(株)` です
    let name_selector = Selector::parse("h2.PriceBoard__name__166W").unwrap();
    if let Some(element) = document.select(&name_selector).next() {
        company_name = element.text().collect::<Vec<_>>().join("").trim().to_string();
        console::log_1(&JsValue::from_str(&format!("企業名: {}", company_name)));
    }

    // 企業コード (例: 6758.T)
    // span.PriceBoardMain__code__2wso は `(6758.T)` のような形です
    let code_selector = Selector::parse("span.PriceBoardMain__code__2wso").unwrap();
    if let Some(element) = document.select(&code_selector).next() {
        company_code = element.text().collect::<Vec<_>>().join("").trim().to_string();
        // 例: "(6758.T)" から "6758" を抽出する場合の処理
        company_code = company_code.trim_matches('(').trim_matches(')').replace(".T", "").to_string();
        console::log_1(&JsValue::from_str(&format!("企業コード: {}", company_code)));
    }
    
    // 現在の株価
    //#root > main > div > section > div.PriceBoardMain__1nb3 > div.PriceBoardMain__priceInformation__3YfB > div.PriceBoardMain__headerPrice__gbs7 > span > span > span
    // div.PriceBoardMain__priceArea__2Mh7 の中の span.StyledNumber__value__3rXW をターゲット
    // これは株価ボードのメイン価格なので、これで良いはず
    let current_price_selector = Selector::parse("span.StyledNumber__value__3rXW").unwrap();
    if let Some(element) = document.select(&current_price_selector).next() {
        current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
        console::log_1(&JsValue::from_str(&format!("現在の株価: {}", current_price)));
    }

    // 前日比 (金額) と 前日比 (パーセント)
    // 前日比全体を囲むdd.PriceChangeLabel__description__a5Lpを起点に探す
    let price_change_dd_selector = Selector::parse("dd.PriceChangeLabel__description__a5Lp").unwrap();
    if let Some(dd_element) = document.select(&price_change_dd_selector).next() {
        // dd_element の中にある全てのStyledNumber__value__3rXWを探す
        // 1つ目が金額、2つ目がパーセント
        let value_spans_selector = Selector::parse("span.StyledNumber__value__3rXW").unwrap();
        let values: Vec<String> = dd_element
            .select(&value_spans_selector)
            .filter_map(|el| Some(el.text().collect::<Vec<_>>().join("").trim().to_string()))
            .collect();

        console::log_1(&JsValue::from_str(&format!("前日比のStyledNumber__value__3rXW値: {:?}", values)));

        if values.len() >= 2 {
            change_amount = values[0].clone(); // 金額
            change_percentage = values[1].clone(); // パーセント

            console::log_1(&JsValue::from_str(&format!("前日比金額: {}", change_amount)));
            console::log_1(&JsValue::from_str(&format!("前日比パーセント: {}", change_percentage)));
        } else {
            console::log_1(&JsValue::from_str("前日比の値が不足しています。"));
        }
    } else {
        console::log_1(&JsValue::from_str("前日比のdd要素が見つかりませんでした。"));
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

    console::log_1(&JsValue::from_str(&format!("Final JSON response: {}", response_data.to_string())));

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

#[event(start)]
fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}