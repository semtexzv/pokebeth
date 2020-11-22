use serde::{Serialize, Deserialize};
use anyhow::*;
use tide::Request;
use rand::prelude::SliceRandom;

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageSpec {
    name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonFlavorEntry {
    language: LanguageSpec,
    #[serde(rename = "flavor_text")]
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonInfo {
    #[serde(rename = "flavor_text_entries")]
    flavor: Vec<PokemonFlavorEntry>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShakespeareContents {
    translated: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShakespeareReply {
    contents: ShakespeareContents
}

async fn pokeapi_get(name: &str) -> Result<PokemonInfo> {
    let res = reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon-species/{}/", name)).await?;
    Ok(res.json().await?)
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
    let data = pokeapi_get(&name).await?;
    let entries = data.flavor
        .into_iter()
        .filter(|e| e.language.name == "en")
        .collect::<Vec<_>>();

    let flavor = entries.choose(&mut rand::thread_rng())
        .ok_or_else(|| Error::msg("Missing flavor text entries"))?;

    let text = flavor.text.replace("\\n", "\n");
    let description = shakespearify(&text).await?;

    panic!("Flavor {:?}", description);
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