use aegis_runtime::proxy::intercept_prompt;

#[tokio::test]
async fn test_proxy_integration_safe_prompt() {
    let prompt = "How do I bake a cake?".to_string();
    let result = intercept_prompt(prompt).await;
    assert!(result.starts_with("SUCCESS"));
}

#[tokio::test]
async fn test_proxy_integration_malicious_length_prompt() {
    let prompt = "a".repeat(15000);
    let result = intercept_prompt(prompt).await;
    assert!(result.starts_with("BLOCKED"));
}
