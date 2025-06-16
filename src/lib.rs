use worker::*;

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = "https://www.yahoo.co.jp";
    let body = worker::Fetch::Url(url.parse().unwrap())
        .send()
        .await?
        .text()
        .await?;

    // 最初の500文字だけ返す
    Response::ok(&body[..500.min(body.len())])
}