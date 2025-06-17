use worker::*;
use serde_json::json;
use scraper::{Html, Selector}; // ★追加！HTMLパースに必要

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
    let title_selector = Selector::parse("h2.PriceBoardMain__name__6uDh").unwrap();
    let code_selector = Selector::parse("span.PriceBoardMain__code__2wso").unwrap();
    if let Some(element) = document.select(&code_selector).next() {
        let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
        // "ソニーグループ (6758.T)" から抽出
        if let Some(start) = text.find('(') {
            if let Some(end) = text.find(')') {
                company_code = text[..start].trim().to_string();
                company_name = text[start + 1..end].trim().to_string();
            }
        } else {
            // カッコがない場合、全体を企業名とみなす
            company_name = text;
        }
    }


    // 現在の株価 (例: 14,000.5)
    // <dd class="sc-6s03j6-0 fHlKcj" data-test-id="currentPrice">14,000.5</dd> のような構造を想定
    let price_selector = Selector::parse("[data-test-id='currentPrice']").unwrap();
    if let Some(element) = document.select(&price_selector).next() {
        current_price = element.text().collect::<Vec<_>>().join("").trim().to_string();
    }

    // 前日比の金額とパーセンテージ
    // <dd class="sc-6s03j6-0 bLwA-Dg" data-test-id="priceDiff">
    //   <span class="sc-1g400y-0 fKqTjt">+</span>150.0
    //   <span class="sc-1g400y-0 fKqTjt">(</span><span class="sc-1g400y-0 fKqTjt">+</span>1.08<span class="sc-1g400y-0 fKqTjt">%)</span>
    // </dd> のような構造を想定
    let diff_selector = Selector::parse("[data-test-id='priceDiff']").unwrap();
    if let Some(element) = document.select(&diff_selector).next() {
        let full_text = element.text().collect::<Vec<_>>().join("").trim().to_string();
        // 例: "+150.0(+1.08%)" の形式から抽出
        if let Some(start_percent) = full_text.find('(') {
            change_amount = full_text[..start_percent].trim().to_string();
            if let Some(end_percent) = full_text.find(')') {
                change_percentage = full_text[start_percent + 1..end_percent].trim().to_string();
            }
        } else {
            // %表記がない場合、金額のみ取得
            change_amount = full_text;
        }
    }

    // 更新日時 (例: 2025/06/17 10:00)
    // <time class="sc-6s03j6-0 fHlKcj" data-test-id="stockTime">2025/06/17 10:00</time> のような構造を想定
    let time_selector = Selector::parse("[data-test-id='stockTime']").unwrap();
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