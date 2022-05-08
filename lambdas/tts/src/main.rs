use warp::Filter;

mod request_handler;
mod tts_api;
mod tts_api_spec;

use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::{Response};
use crate::tts_api::TtsRequest;


#[tokio::main]
async fn main() {
    // The runtime logging can be enabled here by initializing `tracing` with `tracing-subscriber`
    // While `tracing` is used internally, `log` can be used as well if preferred.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let api = warp::path!("tts")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1000))
        .and(warp::body::json())
        .then(request_handler::tts);

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api
        .with(warp::log("api"))
        .with(warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
            .allow_headers(vec!["Accept", "Content-Type"])
        );
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
