mod handlers;
mod healing;
mod models;
mod parsers;
mod utils;
pub mod libhtml;

use worker::{event, Context, Env, Request, Response, Result};

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Use the router from the handlers module
    let router = worker::Router::new();

    router
        .get_async("/api/quote", handlers::handle_quote)
        .post_async("/api/verify-selectors", |mut req, _ctx| async move {
            handlers::handle_verify_selectors_post(&mut req).await
        })
        .get_async("/api/verify-selectors", |req, ctx| async move {
            handlers::handle_verify_selectors_get(req, ctx.env).await
        })
        .post_async("/api/update-selectors", |mut req, ctx| async move {
            handlers::handle_update_selectors(&mut req, ctx.env).await
        })
        .post_async("/api/test-parser", |mut req, _ctx| async move {
            handlers::handle_test_parser(req).await
        })
        .get_async("/api/dji", |_req, _ctx| async move {
            let result = handlers::get_dji_data().await;
            match result {
                Ok(data) => handlers::json_response(models::ApiResponse {
                    status: "success".to_string(),
                    data,
                }),
                Err(e) => handlers::json_response(models::ApiResponse {
                    status: "error".to_string(),
                    data: e.to_string(),
                }),
            }
        })
        // Routes for managing selectors
        .get_async("/api/selectors/:code_type", handlers::handle_get_selectors)
        .post_async("/api/selectors/:code_type", handlers::handle_post_selectors)
        .delete_async("/api/selectors/:code_type", handlers::handle_delete_selectors)
        .get_async("/api/manual-check", handlers::handle_manual_check)
        .get_async("/api/debug-preloaded-state", handlers::handle_debug_preloaded_state)

        .options("/api/quote", handlers::cors_preflight)
        .options("/api/verify-selectors", handlers::cors_preflight)
        .options("/api/update-selectors", handlers::cors_preflight)
        .options("/api/test-parser", handlers::cors_preflight)
        .options("/api/dji", handlers::cors_preflight)
        .options("/api/selectors/:code_type", handlers::cors_preflight)
        .options("/api/manual-check", handlers::cors_preflight)
        .run(req, env).await
}