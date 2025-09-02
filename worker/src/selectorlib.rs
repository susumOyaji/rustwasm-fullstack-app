async fn handle_verify_selectors_get(req: Request, env: Env) -> Result<Response> {
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
    let selectors: SelectorConfig = kv.get(code_type).json().await?.unwrap_or_else(|| get_default_selectors(code_type));

    let yahoo_url = format!("https://finance.yahoo.co.jp/quote/{}", code);
    let html_content = fetch_html(&yahoo_url).await?;
    let data = parse_with_selectors(&html_content, code, &selectors)?;

    let api_response = ApiResponse {
        status: "success".to_string(),
        data,
    };

    let mut response = Response::from_json(&api_response)?;
    response.headers_mut().set("Access-Control-Allow-Origin", "*")?;
    Ok(response)
}


#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = worker::Router::new();
    router
        .get_async("/api/quote", |req, ctx| async move {
            handle_quote(req, ctx.env).await
        })
        .post_async("/api/verify-selectors", |mut req, _ctx| async move {
            handle_verify_selectors_post(&mut req).await
        })
        .get_async("/api/verify-selectors", |req, ctx| async move {
            handle_verify_selectors_get(req, ctx.env).await
        })
        .post_async("/api/update-selectors", |mut req, ctx| async move {
            handle_update_selectors(&mut req, ctx.env).await
        })
        .options("/api/quote", cors_preflight)
        .options("/api/verify-selectors", cors_preflight)
        .options("/api/update-selectors", cors_preflight)
        .run(req, env).await
}

//curl "https://your-worker-domain.workers.dev/api/verify-selectors?code_type=fx&code=USDJPY=FX"