use anyhow::*;
use serde::{Serialize, Deserialize};
use rand::prelude::SliceRandom;

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageSpec {
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonFlavorEntry {
    pub language: LanguageSpec,
    #[serde(rename = "flavor_text")]
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonInfo {
    #[serde(rename = "flavor_text_entries")]
    pub flavor: Vec<PokemonFlavorEntry>
}

pub async fn describe(name: &str) -> Result<String> {
    let res = reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon-species/{}/", name)).await?;
    let data: PokemonInfo = res.json().await?;

    let entries = data.flavor
        .into_iter()
        .filter(|e| e.language.name == "en")
        .collect::<Vec<_>>();

    let flavor = entries.choose(&mut rand::thread_rng())
        .ok_or_else(|| Error::msg("Missing flavor text entries"))?;

    // Don't know why, but the text returned by api has embedded escaped newlines, fix that issue
    let flavor = flavor.text.replace("\\n", "\n")
        .replace("\\t", "\t");

    Ok(flavor)
}
