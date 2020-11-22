use anyhow::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ShakespeareContents {
    translated: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShakespeareReply {
    contents: ShakespeareContents
}

pub async fn translate(txt: &str) -> Result<String> {
    let url = format!("https://api.funtranslations.com/translate/shakespeare.json");
    let cl = reqwest::Client::new();
    let res = cl.post(&url)
        .form(&[("text", txt)][..])
        .send().await?;

    Ok(res.json::<ShakespeareReply>().await?.contents.translated)
}
