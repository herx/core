use crate::sandbox::inspect_prompt_in_wasm;

pub async fn intercept_prompt(body: String) -> String {
    match inspect_prompt_in_wasm(&body) {
        Ok(result) if result.is_safe => {
            format!("SUCCESS: {}", result.reason)
        }
        Ok(result) => {
            format!("BLOCKED: {}", result.reason)
        }
        Err(e) => {
            format!("SANDBOX_ERROR: Failed to execute isolation environment: {}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_intercept_prompt_safe() {
        let result = intercept_prompt("safe payload".to_string()).await;
        assert_eq!(result, "SUCCESS: Payload successfully cleared ephemeral Wasm sandbox inspection.");
    }

    #[tokio::test]
    async fn test_intercept_prompt_blocked() {
        let result = intercept_prompt("a".repeat(10001)).await;
        assert_eq!(result, "BLOCKED: Payload exceeds maximum allowable token window.");
    }
}
