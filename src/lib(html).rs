use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use worker::*;

// JSONで出力するためのデータ構造を定義
#[derive(Debug, Serialize, Deserialize)]
struct Link {
    text: String,
    href: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // URLのクエリパラメータ(?url=...)を取得
    let router = Router::new();
    router
        .get_async("/", |req, _ctx| async move {
            // リクエストURLから 'url' という名前のクエリパラメータを取得
            let Some(target_url) = req.url()?.query_pairs().find_map(|(k, v)| if k == "url" { Some(v) } else { None }) else {
                return Response::error("Query parameter 'url' is required.", 400);
            };

            // --------------------------------------------------
            // スクレイピング処理
            // --------------------------------------------------
            // 指定されたURLにリクエストを送信し、HTMLを取得
            let mut response = Fetch::Url(target_url.parse()?).send().await?;
            if !response.status_code() == 200 {
                return Response::error(format!("Failed to fetch URL. Status: {}", response.status_code()), 500);
            }
            let body = response.text().await?;

            // HTMLドキュメントをパース
            let document = Html::parse_document(&body);

            // aタグのhref属性を抜き出すためのセレクタを定義
            let selector = Selector::parse("a[href]").unwrap();

            // 収集したリンクを格納するためのVec
            let mut links: Vec<Link> = Vec::new();

            // セレクタに一致するすべての要素をループ処理
            for element in document.select(&selector) {
                // 要素のテキストを取得し、前後の空白を削除
                let text = element.text().collect::<String>().trim().to_string();
                
                // href属性の値を取得
                if let Some(href) = element.value().attr("href") {
                    // テキストが空でなく、URLが相対パスでない場合にリストに追加
                    if !text.is_empty() && (href.starts_with("http://") || href.starts_with("https://")) {
                        links.push(Link {
                            text,
                            href: href.to_string(),
                        });
                    }
                }
            }

            // --------------------------------------------------
            // 結果をJSONで返す
            // --------------------------------------------------
            Response::from_json(&links)
        })
        .run(req, env)
        .await
}