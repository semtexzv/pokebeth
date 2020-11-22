use anyhow::*;
use tide::{Request, Response};

mod pokeapi;
mod shakespeare;

async fn handle_get(req: Request<()>) -> tide::Result {
    let name = req.param("name")?;

    // Get original description
    let description = pokeapi::describe(&name).await?;

    // Perform the translation
    let description = shakespeare::translate(&description).await?;

    Ok(Response::builder(tide::http::StatusCode::Ok)
        .body(json::json!({
            "name": name,
            "description": description
        }))
        .build()
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.at("/pokemon/:name").get(handle_get);
    app.listen("127.0.0.1:5000").await?;
    Ok(())
}