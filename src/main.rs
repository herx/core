use axum::{
    routing::post,
    Router,
    Json,
    http::StatusCode,
};

use aegis_runtime::proxy;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct InferenceRequest {
    proof_token: String,
    prompt: String,
}

#[derive(Serialize)]
struct InferenceResponse {
    status: String,
    output: Option<String>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing and configuration
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/v1/chat/completions", post(handle_proxy_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("AegisMesh Secure Proxy listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_proxy_request(
    Json(payload): Json<InferenceRequest>,
) -> Result<Json<InferenceResponse>, StatusCode> {
    // Step 1: Verify Zero-Knowledge Proof of Personhood
    if !verify_zk_proof(&payload.proof_token).await {
        return Ok(Json(InferenceResponse {
            status: "REJECTED_INVALID_IDENTITY".into(),
            output: None,
        }));
    }

    // Step 2: Execute Prompt inside Ephemeral Wasm Sandbox
    let sandbox_result = proxy::intercept_prompt(payload.prompt.clone()).await;
    
    if sandbox_result.starts_with("SUCCESS") {
        // Step 3: Forward to local LLM backend (e.g., Ollama)
        let result = forward_to_llm(&payload.prompt).await;
        Ok(Json(InferenceResponse {
            status: "SUCCESS".into(),
            output: Some(result),
        }))
    } else {
        Ok(Json(InferenceResponse {
            status: "REJECTED_MALICIOUS_PAYLOAD".into(),
            output: Some(sandbox_result),
        }))
    }
}

async fn verify_zk_proof(token: &str) -> bool {
    // Reuse the Zero‑Knowledge proof verification logic
    use aegis_runtime::identity::verify_zkp_header;
    let result = verify_zkp_header(Some(token));
    result.is_verified
}


async fn forward_to_llm(prompt: &str) -> String {
    // Placeholder for local model inference call
    format!("Processed safely: {}", prompt)
}
