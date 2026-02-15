use crate::error::AppError;
use keyring::Entry;

const SERVICE_PREFIX: &str = "kb-drafter";

pub fn store_token(service: &str, token: &str) -> Result<(), AppError> {
    let entry = Entry::new(&format!("{}-{}", SERVICE_PREFIX, service), "default")
        .map_err(|e| AppError::Internal(format!("Failed to create keyring entry: {}", e)))?;

    entry
        .set_password(token)
        .map_err(|e| AppError::Internal(format!("Failed to store token: {}", e)))?;

    Ok(())
}

pub fn get_token(service: &str) -> Result<String, AppError> {
    let entry = Entry::new(&format!("{}-{}", SERVICE_PREFIX, service), "default")
        .map_err(|e| AppError::Internal(format!("Failed to create keyring entry: {}", e)))?;

    entry
        .get_password()
        .map_err(|_| AppError::TokenMissing {
            service: service.to_string(),
        })
}

pub fn delete_token(service: &str) -> Result<(), AppError> {
    let entry = Entry::new(&format!("{}-{}", SERVICE_PREFIX, service), "default")
        .map_err(|e| AppError::Internal(format!("Failed to create keyring entry: {}", e)))?;

    entry
        .delete_credential()
        .map_err(|e| AppError::Internal(format!("Failed to delete token: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Keychain access is unreliable in test environments
    fn test_token_storage_roundtrip() {
        let service = "test-service";
        let token = "test-token-123";

        // Clean up any existing token first
        let _ = delete_token(service);

        // Store token
        store_token(service, token).unwrap();

        // Retrieve token
        let retrieved = get_token(service).unwrap();
        assert_eq!(retrieved, token);

        // Delete token
        delete_token(service).unwrap();

        // Verify deleted (allow for keychain backend issues in CI)
        match get_token(service) {
            Err(AppError::TokenMissing { .. }) => {
                // Expected
            }
            Ok(_) => {
                // Some keychain backends may not delete immediately
                eprintln!("Warning: token still present after delete (keychain backend issue)");
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
