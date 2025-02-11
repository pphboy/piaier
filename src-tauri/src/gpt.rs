use std::{
    env,
    error::Error,
    ffi::IntoStringError,
    fmt::{self, Debug},
    fs,
};

use crate::message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GptMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GptModel {
    pub model: String,
    pub url: String,
    pub api_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GptConfig {
    pub models: Vec<GptModel>,
}

impl fmt::Display for GptConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "models: {}",
            self.models
                .iter()
                .map(|m| m.model.clone())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub async fn call_gpt(gc: GptModel, messages: &Vec<GptMessage>) -> Result<String, String> {
    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "stream": false,
        "model": gc.model.clone(),
        "messages": messages.iter().map(|msg| {
            serde_json::json!({
                "role": msg.role.to_lowercase(),
                "content": msg.content
            })
        }).collect::<Vec<_>>()
    });

    let response = client
        .post(gc.url.clone())
        .header("Authorization", format!("Bearer {}", gc.api_token.clone()))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    let gpt_response = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response")
        .to_string();

    Ok(gpt_response)
}

#[derive(Debug)]
struct StringError {
    message: String,
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for StringError {}

pub fn get_config() -> Result<GptConfig, Box<dyn Error>> {
    let path = format!("{}/.config/piaier/gpt.toml", env::var("HOME").unwrap());

    let config = fs::read_to_string(path.clone()).map_err(|e| -> Box<dyn Error> {
        Box::new(StringError {
            message: format!("{} {}", path.clone(), e.to_string()),
        })
    })?;

    toml::from_str(&config).map_err(|e| -> Box<dyn Error> {
        let msg = format!("{} {}", path.clone(), e.message());
        Box::new(StringError { message: msg })
    })
}
