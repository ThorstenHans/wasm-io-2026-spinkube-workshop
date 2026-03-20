use anyhow::Result;
use spin_sdk::http::{IntoResponse, Request, Response, Router};
use spin_sdk::http_component;

mod constants;
mod models;

fn bad_request() -> Result<Response> {
    Ok(Response::new(400, "Bad Request"))
}

#[http_component]
fn handle_http_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    // Register routes
    // GET /tasks
    // POST /tasks
    // PUT /tasks
    Ok(router.handle(req))
}
