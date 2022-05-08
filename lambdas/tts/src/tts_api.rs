use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string;
use reqwest::{Client, Response};
use anyhow::{Ok, anyhow, Context, Error, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TtsRequest {
    pub input: Input,
    pub voice: Voice,
    pub audio_config: AudioConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Voice {
    pub language_code: String,
    pub name: String,
    pub ssml_gender: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioConfig {
    pub audio_encoding: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TtsResponse {
    pub audio_content: String,
}

pub struct TtsApi {
    base_url: String,
    token: String,
}

impl TtsApi {
    pub fn new(base_url: String, token: String) -> TtsApi {
        TtsApi {
            base_url,
            // base_url: String::from("https://texttospeech.googleapis.com/v1"),
            token,
        }
    }

    pub async fn tts(&self, request: TtsRequest) -> Result<Response> {
        let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
        //let token: Token = self.authentication_manager.get_token(scopes).await?;

        let request_url = format!("{}/text:synthesize", self.base_url);
        let json_request = to_string(&request);

        let response = Client::new()
            .post(&request_url)
            .bearer_auth(&self.token)
            .body(json_request.unwrap())
            .send().await;

        println!("Created {:?}", response);

        response
            .map_err(|e| anyhow!("Error code: {}", e.status().unwrap()))
    }
}
