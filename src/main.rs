mod pokeapi;

use serde::{Serialize, Deserialize};
use anyhow::*;
use tide::Request;
use rand::prelude::SliceRandom;


#[derive(Debug, Deserialize, Serialize)]
pub struct ShakespeareContents {
    translated: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShakespeareReply {
    contents: ShakespeareContents
}

async fn shakespearify(txt: &str) -> Result<String> {
    let url = format!("https://api.funtranslations.com/translate/shakespeare.json");
    let cl = reqwest::Client::new();
    let res = cl.post(&url)
        .form(&[("text", txt)][..])
        .send().await?;

    Ok(res.json::<ShakespeareReply>().await?.contents.translated)
}

async fn handle_get(mut req: Request<()>) -> tide::Result {
    let name = req.param("name")?;

    let description = pokeapi::describe(&name).await?;
    
    // Perform the translation
    let description = shakespearify(&description).await?;

    Ok(tide::Response::builder(tide::http::StatusCode::Ok)
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