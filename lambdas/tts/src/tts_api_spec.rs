#[cfg(test)]
mod tests {
    use hamcrest::assert_that;
    use super::*;

    use mockall::*;
    use mockall::predicate::*;
    use hamcrest::prelude::*;
    use httpmock::prelude::*;
    use serde_json::json;


    use crate::tts_api::{AudioConfig, Input, TtsApi, TtsRequest, Voice};

    #[tokio::test]
    async fn it_works() {
        // Start a lightweight mock server.
        let server = MockServer::start();

        // Create a mock on the server.
        let gcp_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/text:synthesize")
                .header("authorization", "Bearer token")
                .json_body(json!({
                  "input":{
                    "text":"eudes"
                  },
                  "voice":{
                    "languageCode":"es-ES",
                    "name":"es-ES-Standard-A",
                    "ssmlGender":"FEMALE"
                  },
                  "audioConfig":{
                    "audioEncoding":"MP3"
                  }
                }));
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(r#"{
                    "audioContent": "//NExAASCCIIAAhEAGAAEMW4kAYPnwwIKw/BBTpwTvB+IAxIfghUfW.."
                }"#);
        });

        let tested = TtsApi::new(server.base_url(), String::from("token"));

        // Send an HTTP request to the mock server. This simulates your code.
        let response = tested.tts(TtsRequest {
            input: Input {
                text: String::from("eudes"),
            },
            voice: Voice {
                language_code: String::from("es-ES"),
                name: String::from("es-ES-Standard-A"),
                ssml_gender: String::from("FEMALE"),
            },
            audio_config: AudioConfig {
                audio_encoding: String::from("MP3"),
            },
        }).await;

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        gcp_mock.assert();

        // Ensure the mock server did respond as specified.
        assert_that!(response.is_ok(), is(true));
        assert_eq!(response.unwrap().status(), 200);
    }
}
