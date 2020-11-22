use anyhow::*;
use tide::Request;

#[derive(Deserialize, Serialize)]
pub struct PokemonFlavorEntry {
    #[serde(rename = "flavor_text")]
    text: String
}

#[derive(Deserialize, Serialize)]
pub struct PokemonInfo {
    #[serde(rename = "flavor_text_entries")]
    flavor: Vec<PokemonFlavorEntry>
}

async fn pokeapi_get(name: &str) -> Result<json::Value> {
    let res = reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon-species/{}/", name)).await?;
    Ok(res.json().await?)
}

async fn handle_get(mut req: Request<()>) -> tide::Result {
    let data = pokeapi_get(req.param("name")?).await?;
    Ok(tide::Response::new(tide::http::StatusCode::Ok))
}


#[tokio::main]
async fn main() -> Result<()> {
    let mut app = tide::new();
    app.at("/pokemon/:name").get(handle_get);
    app.listen("127.0.0.1:5000").await?;
    Ok(())
}