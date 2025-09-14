use crate::healing::{auto_update_invalid_selectors, check_selectors_against_url, heal_fx_selectors};
use crate::models::*;
use crate::parsers::parse_with_selectors;
use crate::utils::{fetch_html, get_code_type, get_default_selectors};
use futures::future::join_all;
use serde::Serialize;
use std::collections::HashMap;
use worker::{Env, Headers, Request, Response, Result, RouteContext};

// --- Helper for JSON Responses ---
pub fn json_response<T: Serialize>(data: T) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Content-Type", "application/json; charset=utf-8")?;
    Ok(Response::from_json(&data)?.with_headers(headers))
}

// --- Route Handlers ---

pub fn cors_preflight(_req: Request, _ctx: worker::RouteContext<()>) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, X-Admin-Key")?;
    Ok(Response::empty()?.with_headers(headers))
}

pub async fn handle_manual_check(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(key) = ctx.env.secret("ADMIN_KEY").ok() {
        if req.headers().get("X-Admin-Key")? != Some(key.to_string()) {
            return Response::error("Unauthorized", 403);
        }
    } else {
        return Response::error("ADMIN_KEY not set", 500);
    }

    worker::console_log!("[Manual Check] Starting comprehensive health check...");

    let mut results = HashMap::new();

    let css_selector_codes = vec![CodeType::Fx, CodeType::Dji];
    worker::console_log!("[Manual Check] Checking CSS Selectors...");
    for code_type in css_selector_codes {
        let url = match code_type {
            CodeType::Fx => "https://finance.yahoo.co.jp/quote/USDJPY=FX",
            CodeType::Dji => "https://finance.yahoo.co.jp/quote/^DJI",
            _ => continue,
        };
        let check_result = check_selectors_against_url(url, code_type).await;
        let is_ok = check_result.is_ok() && check_result.unwrap().values().all(|&v| v);
        results.insert(format!("{}_css_selectors_valid", code_type.as_str()), is_ok);

        if code_type == CodeType::Fx && !is_ok {
            let healing_succeeded = heal_fx_selectors(&ctx).await?;
            results.insert("fx_healing_attempted".to_string(), true);
            results.insert("fx_healing_succeeded".to_string(), healing_succeeded);
        }
    }

    let json_parsing_targets = vec![("stock", "6758.T"), ("nikkei", "998407.O")];
    worker::console_log!("[Manual Check] Checking JSON Parsers...");
    let kv = ctx.env.kv("FIN_SELECTORS")?;
    for (name, code) in json_parsing_targets {
        let yahoo_url = format!("https://finance.yahoo.co.jp/quote/{}", code);
        let is_ok = match fetch_html(&yahoo_url).await {
            Ok(html_content) => crate::libhtml::parse_from_preloaded_state(&html_content, code, &kv).await.is_ok(),
            Err(_) => false,
        };
        results.insert(format!("{}_json_parser_valid", name), is_ok);
    }

    json_response(results)
}

pub async fn handle_test_parser(mut req: Request) -> Result<Response> {
    let test_data: TestParseRequest = req.json().await?;
    let result = parse_with_selectors(&test_data.html_content, &test_data.code, &test_data.selectors);

    match result {
        Ok(data) => json_response(ApiResponse { status: "success".to_string(), data }),
        Err(e) => json_response(ApiResponse { status: "error".to_string(), data: e.to_string() }),
    }
}

pub async fn handle_quote(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = req.url()?;
    let query_params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let codes_str = match query_params.get("codes") {
        Some(c) => c,
        None => return Response::error("Query parameter 'codes' is required.", 400),
    };

    let codes: Vec<&str> = codes_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    if codes.is_empty() {
        return Response::error("Query parameter 'codes' cannot be empty.", 400);
    }
    let kv = ctx.env.kv("FIN_SELECTORS")?;

    let futures: Vec<_> = codes.iter().map(|&code| {
        let kv_clone = kv.clone();
        async move {
            let code_type = get_code_type(code);
            let yahoo_url = format!("https://finance.yahoo.co.jp/quote/{}", code);

            let html_content_result = fetch_html(&yahoo_url).await;
            if let Err(e) = html_content_result {
                return Err(worker::Error::from(format!("Failed to fetch HTML for {}: {}", code, e)));
            }
            let html_content = html_content_result.unwrap();

            

            match code_type {
                CodeType::Fx | CodeType::Dji => {
                    let selectors: SelectorConfig = kv_clone.get(code_type.as_str()).json().await
                        .map_err(|e| worker::Error::from(format!("KV store error: {}", e)))?
                        .unwrap_or_else(|| get_default_selectors(code_type));
                    parse_with_selectors(&html_content, code, &selectors)
                        .map_err(|e| worker::Error::from(format!("Selector parsing error: {}", e)))
                },
                _ => {
                    crate::libhtml::parse_from_preloaded_state(&html_content, code, &kv_clone).await
                        .map_err(|e| worker::Error::RustError(e))
                }
            }
        }
    }).collect();

    let results: Vec<Result<FinancialData>> = join_all(futures).await;

    let mut success_data = Vec::new();
    let mut failed_data = Vec::new();

    for (i, result) in results.into_iter().enumerate() {
        if let Some(&code) = codes.get(i) {
            match result {
                Ok(data) => success_data.push(data),
                Err(e) => failed_data.push(QuoteError { code: code.to_string(), error: e.to_string() }),
            }
        }
    }

    let status = if failed_data.is_empty() { "success" } else if success_data.is_empty() { "failure" } else { "partial_success" };

    json_response(ApiResponse {
        status: status.to_string(),
        data: QuoteResponse { success: success_data, failed: failed_data },
    })
}

pub async fn handle_get_selectors(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let code_type_str = match ctx.param("code_type") {
        Some(ct) => ct,
        None => return Response::error("Parameter 'code_type' is required.", 400),
    };
    let code_type: CodeType = code_type_str.parse()?;
    let kv = ctx.env.kv("FIN_SELECTORS")?;
    let selectors: Option<SelectorConfig> = kv.get(code_type.as_str()).json().await?;

    json_response(ApiResponse { status: "success".to_string(), data: selectors })
}

pub async fn handle_post_selectors(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(key) = ctx.env.secret("ADMIN_KEY").ok() {
        if req.headers().get("X-Admin-Key")? != Some(key.to_string()) {
            return Response::error("Unauthorized", 403);
        }
    } else {
        return Response::error("ADMIN_KEY not set", 500);
    }

    let req_url = req.url()?;
    let query_params: HashMap<_, _> = req_url.query_pairs().into_owned().collect();
    let code_type_str = match ctx.param("code_type") {
        Some(ct) => ct,
        None => return Response::error("Parameter 'code_type' is required.", 400),
    };
    let code_type: CodeType = code_type_str.parse()?;
    let kv = ctx.env.kv("FIN_SELECTORS")?;

    if query_params.get("autoupdate").map_or(false, |v| v == "true") {
        let url_to_check = match query_params.get("url") {
            Some(u) => u,
            None => return Response::error("Query parameter 'url' is required for autoupdate.", 400),
        };
        let new_selectors: SelectorConfig = req.json().await?;
        let update_result = auto_update_invalid_selectors(url_to_check, code_type, &new_selectors, &kv).await?;
        json_response(ApiResponse { status: "success".to_string(), data: update_result })
    } else {
        let selectors: SelectorConfig = req.json().await?;
        kv.put(code_type.as_str(), selectors)?.execute().await?;
        json_response(ApiResponse { status: "success".to_string(), data: format!("Selectors for '{}' updated.", code_type.as_str()) })
    }
}

pub async fn handle_delete_selectors(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(key) = ctx.env.secret("ADMIN_KEY").ok() {
        if req.headers().get("X-Admin-Key")? != Some(key.to_string()) {
            return Response::error("Unauthorized", 403);
        }
    } else {
        return Response::error("ADMIN_KEY not set", 500);
    }

    let code_type_str = match ctx.param("code_type") {
        Some(ct) => ct,
        None => return Response::error("Parameter 'code_type' is required.", 400),
    };
    let code_type: CodeType = code_type_str.parse()?;
    let kv = ctx.env.kv("FIN_SELECTORS")?;
    kv.delete(code_type.as_str()).await?;

    json_response(ApiResponse { status: "success".to_string(), data: format!("Selectors for '{}' deleted.", code_type.as_str()) })
}

pub async fn handle_verify_selectors_get(req: Request, env: Env) -> Result<Response> {
    let url = req.url()?;
    let query_params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let code_type = match query_params.get("code_type") {
        Some(c) => c,
        None => return Response::error("Query parameter 'code_type' is required.", 400),
    };
    let code = match query_params.get("code") {
        Some(c) => c,
        None => return Response::error("Query parameter 'code' is required.", 400),
    };

    let kv = env.kv("FIN_SELECTORS")?;
    let selectors: SelectorConfig = kv.get(code_type).json().await?.unwrap_or_else(|| get_default_selectors(code_type.parse().unwrap()));
    let yahoo_url = format!("https://finance.yahoo.co.jp/quote/{}", code);
    let html_content = fetch_html(&yahoo_url).await?;
    let data = parse_with_selectors(&html_content, code, &selectors)?;

    json_response(ApiResponse { status: "success".to_string(), data })
}

pub async fn handle_verify_selectors_post(_req: &mut Request) -> Result<Response> {
    Response::error("Not Implemented", 501)
}

pub async fn handle_update_selectors(_req: &mut Request, _env: Env) -> Result<Response> {
    Response::error("Not Implemented", 501)
}

pub async fn get_dji_data() -> Result<FinancialData> {
    let code = "^DJI";
    let yahoo_url = format!("https://finance.yahoo.co.jp/quote/{}", code);
    let html_content = fetch_html(&yahoo_url).await?;
    let code_type = get_code_type(code);
    let selectors = get_default_selectors(code_type);
    parse_with_selectors(&html_content, code, &selectors)
}

pub async fn handle_debug_preloaded_state(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let yahoo_url = "https://finance.yahoo.co.jp/quote/998407.O";
    let html_content = fetch_html(&yahoo_url).await?;

    const SCRIPT_START: &str = "window.__PRELOADED_STATE__ = ";
    let script_line = html_content
        .lines()
        .find(|line| line.trim().starts_with(SCRIPT_START));

    if let Some(line) = script_line {
        let json_str = line.trim()
            .strip_prefix(SCRIPT_START)
            .and_then(|s| s.strip_suffix(';'))
            .unwrap_or_else(|| &line.trim()[SCRIPT_START.len()..])
            .to_string();
        
        Response::ok(json_str)
    } else {
        Response::error("Could not find __PRELOADED_STATE__ in HTML content", 404)
    }
}