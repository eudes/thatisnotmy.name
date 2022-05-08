use std::collections::HashMap;
use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use gcp_auth::AuthenticationManager;
use warp::Filter;
use warp::http::{Response, StatusCode};
use warp::hyper::body::HttpBody;
use crate::tts_api::{AudioConfig, Input, TtsApi, TtsRequest as TtsApiRequest, Voice};
use anyhow::{Ok, anyhow, Context, Error, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TtsRequest {
    language: String,
    country_code: String,
    word: String,
}

async fn tts_do(event: TtsRequest) -> Result<String> {
    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;
    let api = TtsApi::new(String::from("https://texttospeech.googleapis.com/v1"), String::from(token.as_str()));
    let response = api.tts(TtsApiRequest {
        input: Input {
            text: event.word,
        },
        voice: Voice {
            language_code: format!("{}-{}", event.language, event.country_code),
            name: format!("{}-{}-Wavenet-A", event.language, event.country_code),
            ssml_gender: String::from("FEMALE"),
        },
        audio_config: AudioConfig {
            audio_encoding: String::from("MP3"),
        },
    }).await?;
    response.text().await
        .map(|text| text)
        .map_err(|e| anyhow!("Error code: {}", e.status().unwrap()))
}

pub async fn tts(event: TtsRequest) -> Response<String> {
    match tts_do(event).await {
        Result::Ok(response) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(response)
                .unwrap()
        }
        Result::Err(error) => {
            tracing::error!("Error from request: {}", error);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string())
                .unwrap()
        }
    }
}
