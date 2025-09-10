'''use worker::{event, Env, Request, Response, Result, Router, RouteContext, kv::KvStore, Method, Headers, RequestInit};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use scraper::{Html, Selector, ElementRef};

// --- Data Structures ---

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SelectorInfo {
    selector: String,
    keyword: String,
}

type SelectorMap = HashMap<String, SelectorInfo>;

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

// --- KV Store Utilities ---

async fn get_selectors_from_kv(kv: &KvStore, ticker: &str) -> Result<Option<SelectorMap>> {
    kv.get(ticker).json().await
}

async fn update_selector_in_kv(kv: &KvStore, ticker: &str, field: &str, info: SelectorInfo) -> Result<()> {
    let mut db = get_selectors_from_kv(kv, ticker).await?.unwrap_or_default();
    db.insert(field.to_string(), info);
    kv.put(ticker, db)?.execute().await
}

// --- HTML Fetching and Parsing Logic ---

async fn fetch_html(url: &str) -> Result<String> {
    let mut headers = Headers::new();
    headers.set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")?;
    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Get).with_headers(headers);
    let request = Request::new_with_init(url, &request_init)?;
    worker::Fetch::Request(request).send().await?.text().await
}

// --- Healing and Signature Logic (adapted for scraper) ---
//新しいセレクター（タグ＋クラス名）を抽出するコード
fn extract_signature(elem: &ElementRef) -> String {
    let name = elem.value().name();
    let classes = elem.value().classes().collect::<Vec<_>>();
    format!("{}{}", name, if !classes.is_empty() {
        format!(".{}", classes.join("."))
    } else {
        "".to_string()
    })
}

// --- Core API Logic ---

async fn get_value(
    html_document: &Html,
    kv: &KvStore,
    ticker: &str,
    field: &str,
    keyword: &str,
) -> Result<String> {
    // 1. Try to use the stored selector from KV
    if let Some(selectors) = get_selectors_from_kv(kv, ticker).await? {
        if let Some(info) = selectors.get(field) {
            if let Ok(selector) = Selector::parse(&info.selector) {
                if let Some(element) = html_document.select(&selector).next() {
                    let text = element.text().collect::<String>();
                    if !text.trim().is_empty() {
                        return Ok(text);
                    }
                }
            }
        }
    }

    // 2. Self-healing if the stored selector fails or doesn't exist
    let candidate_selector = Selector::parse("span, div, fin-streamer").unwrap();
    let candidates = html_document.select(&candidate_selector);

    let mut best_score = 0.0;
    let mut best_value = None;
    let mut best_sig = None;

    for cand_ref in candidates {
        let text = cand_ref.text().collect::<String>();
        if text.contains(keyword) { // Pre-filter for candidates containing the keyword
            let current_sig = extract_signature(&cand_ref);
            let s = if text.contains(keyword) { 1.0 } else { 0.0 };

            if s > best_score {
                best_score = s;
                best_value = Some(text.clone());
                best_sig = Some(current_sig);
            }
        }
    }

    // 3. If healing was successful, update the KV store and return the value
    if let (Some(value), Some(selector_str)) = (best_value, best_sig) {
        let new_info = SelectorInfo {
            selector: selector_str,
            keyword: keyword.to_string(),
        };
        update_selector_in_kv(kv, ticker, field, new_info).await?;
        return Ok(value);
    }

    Err(worker::Error::from("Value not found and self-healing failed."))
}

// --- CORS Preflight Handler ---

fn cors_preflight(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    Ok(Response::empty()?.with_headers(headers))
}

// --- Worker Entrypoint and Routing ---

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .options("/api/test/:ticker/:field", cors_preflight) // Add OPTIONS route for CORS
        .get_async("/api/test/:ticker/:field", |req, ctx| async move {
            let ticker = ctx.param("ticker").unwrap();
            let field = ctx.param("field").unwrap();
            
            let url_params: HashMap<String, String> = req.url()?.query_pairs().into_owned().collect();
            let keyword = url_params.get("keyword").map_or(field, |k| k);
            let site_url = url_params.get("url").map_or_else(
                || format!("https://finance.yahoo.com/quote/{}", ticker),
                |u| u.to_string()
            );

            let kv = ctx.env.kv("TEST_SELECTORS")?;

            let html_content = match fetch_html(&site_url).await {
                Ok(content) => content,
                Err(e) => {
                    let mut response = Response::error(format!("Failed to fetch URL: {}", e), 500)?;
                    response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
                    return Ok(response);
                }
            };

            let document = Html::parse_document(&html_content);

            let mut response = match get_value(&document, &kv, ticker, field, keyword).await {
                Ok(value) => {
                    let response_data = serde_json::json!({ "ticker": ticker, "field": field, "value": value });
                    Response::from_json(&ApiResponse { status: "success".to_string(), data: response_data })?
                },
                Err(e) => Response::error(e.to_string(), 500)?,
            };

            // Add CORS header to the final response
            response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
            Ok(response)
        })
        .run(req, env).await
}
''
