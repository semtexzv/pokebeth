use serde::{Serialize, Deserialize};
use anyhow::*;
use tide::Request;

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonFlavorEntry {
    #[serde(rename = "flavor_text")]
    text: String
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
    let mut data = pokeapi_get(req.param("name")?).await?;
    let flavor = data.flavor.remove(0);
    let text = shakespearify(&flavor.text).await?;

    panic!("Flavor {:?}", text);
    Ok(tide::Response::new(tide::http::StatusCode::Ok))
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