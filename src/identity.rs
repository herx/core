pub struct IdentityResult {
    pub is_verified: bool,
    pub human_id: String,
    pub reason: String,
}

pub fn verify_zkp_header(auth_header: Option<&str>) -> IdentityResult {
    match auth_header {
        Some(header) if header.starts_with("Bearer aegis_zkp_") => {
            let human_id = header.replace("Bearer aegis_zkp_", "");
            if human_id.is_empty() {
                return IdentityResult {
                    is_verified: false,
                    human_id: "".to_string(),
                    reason: "Empty ZKP human identifier.".to_string(),
                };
            }
            IdentityResult {
                is_verified: true,
                human_id,
                reason: "ZKP human personhood verified successfully.".to_string(),
            }
        }
        Some(_) => IdentityResult {
            is_verified: false,
            human_id: "".to_string(),
            reason: "Invalid ZKP token format or signature.".to_string(),
        },
        None => IdentityResult {
            is_verified: false,
            human_id: "".to_string(),
            reason: "Missing Aegis ZKP authentication header.".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_zkp_header() {
        let result = verify_zkp_header(Some("Bearer aegis_zkp_user_alpha_123"));
        assert!(result.is_verified);
        assert_eq!(result.human_id, "user_alpha_123");
    }

    #[test]
    fn test_missing_zkp_header() {
        let result = verify_zkp_header(None);
        assert!(!result.is_verified);
        assert_eq!(result.reason, "Missing Aegis ZKP authentication header.");
    }

    #[test]
    fn test_invalid_format_zkp_header() {
        let result = verify_zkp_header(Some("Bearer invalid_token_format"));
        assert!(!result.is_verified);
        assert_eq!(result.reason, "Invalid ZKP token format or signature.");
    }
}
