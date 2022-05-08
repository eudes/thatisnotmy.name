mkdir output

cross build --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/debug/tts-lambda output/ && cp Dockerfile output/  && cp start.sh output/ && chmod +x output/start.sh && gcloud run deploy tts --source output --project lexical-aileron-349514  --allow-unauthenticated

# GCP seems to like these
cross build --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/debug/tts-lambda output/ && cp Dockerfile output/  && cp start.sh output/ && chmod +x output/start.sh && gcloud run deploy tts --source output --project lexical-aileron-349514  --allow-unauthenticated

cargo build
cp target/debug/tts-lambda output/ && cp Dockerfile output/  && cp start.sh output/ && chmod +x output/start.sh
gcloud run deploy tts --source output --project lexical-aileron-349514  --allow-unauthenticated
