// Input validation for security operations

use crate::security::errors::{SecurityError, SecurityResult};
use once_cell::sync::Lazy;
use regex::Regex;

/// Maximum lengths for security fields
const MAX_USERNAME_LENGTH: usize = 256;
const MAX_PASSWORD_LENGTH: usize = 256;
const MAX_SERVICE_LENGTH: usize = 256;

/// Valid username pattern (email or alphanumeric with common special chars)
/// Allows: a-z, A-Z, 0-9, dot, underscore, percent, plus, at, hyphen
static USERNAME_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+@-]+$").unwrap());

/// Valid service name pattern (alphanumeric, dot, hyphen, underscore)
static SERVICE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap());

/// Validator for credential inputs
pub struct Validator;

impl Validator {
    /// Validate username
    ///
    /// # Rules
    /// - Cannot be empty
    /// - Maximum 256 characters
    /// - Must match USERNAME_PATTERN
    /// - No control characters
    pub fn validate_username(username: &str) -> SecurityResult<()> {
        if username.is_empty() {
            return Err(SecurityError::InvalidInput {
                field: "username".to_string(),
                reason: "Username cannot be empty".to_string(),
            });
        }

        if username.len() > MAX_USERNAME_LENGTH {
            return Err(SecurityError::InvalidInput {
                field: "username".to_string(),
                reason: format!("Username exceeds {} characters", MAX_USERNAME_LENGTH),
            });
        }

        if !USERNAME_PATTERN.is_match(username) {
            return Err(SecurityError::InvalidInput {
                field: "username".to_string(),
                reason: "Username contains invalid characters".to_string(),
            });
        }

        // Check for control characters
        if username.chars().any(|c| c.is_control()) {
            return Err(SecurityError::InvalidInput {
                field: "username".to_string(),
                reason: "Username contains control characters".to_string(),
            });
        }

        Ok(())
    }

    /// Validate password
    ///
    /// # Rules
    /// - Cannot be empty
    /// - Maximum 256 characters
    /// - No null bytes (can cause issues in FFI)
    pub fn validate_password(password: &str) -> SecurityResult<()> {
        if password.is_empty() {
            return Err(SecurityError::InvalidInput {
                field: "password".to_string(),
                reason: "Password cannot be empty".to_string(),
            });
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(SecurityError::InvalidInput {
                field: "password".to_string(),
                reason: format!("Password exceeds {} characters", MAX_PASSWORD_LENGTH),
            });
        }

        // Check for null bytes (can cause issues in C FFI)
        if password.contains('\0') {
            return Err(SecurityError::InvalidInput {
                field: "password".to_string(),
                reason: "Password contains invalid null bytes".to_string(),
            });
        }

        Ok(())
    }

    /// Validate service name
    ///
    /// # Rules
    /// - Cannot be empty
    /// - Maximum 256 characters
    /// - Must match SERVICE_PATTERN
    pub fn validate_service(service: &str) -> SecurityResult<()> {
        if service.is_empty() {
            return Err(SecurityError::InvalidInput {
                field: "service".to_string(),
                reason: "Service name cannot be empty".to_string(),
            });
        }

        if service.len() > MAX_SERVICE_LENGTH {
            return Err(SecurityError::InvalidInput {
                field: "service".to_string(),
                reason: format!("Service name exceeds {} characters", MAX_SERVICE_LENGTH),
            });
        }

        if !SERVICE_PATTERN.is_match(service) {
            return Err(SecurityError::InvalidInput {
                field: "service".to_string(),
                reason: "Service name contains invalid characters".to_string(),
            });
        }

        Ok(())
    }

    /// Validate server host
    ///
    /// # Rules
    /// - Cannot be empty
    /// - Maximum 255 characters
    /// - No control characters
    /// - No null bytes
    pub fn validate_server_host(host: &str) -> SecurityResult<()> {
        if host.is_empty() {
            return Err(SecurityError::InvalidInput {
                field: "server_host".to_string(),
                reason: "Server host cannot be empty".to_string(),
            });
        }

        if host.len() > 255 {
            return Err(SecurityError::InvalidInput {
                field: "server_host".to_string(),
                reason: "Server host exceeds 255 characters".to_string(),
            });
        }

        // Check for control characters
        if host.chars().any(|c| c.is_control()) {
            return Err(SecurityError::InvalidInput {
                field: "server_host".to_string(),
                reason: "Server host contains control characters".to_string(),
            });
        }

        // Check for null bytes
        if host.contains('\0') {
            return Err(SecurityError::InvalidInput {
                field: "server_host".to_string(),
                reason: "Server host contains null bytes".to_string(),
            });
        }

        // Check for spaces
        if host.contains(' ') {
            return Err(SecurityError::InvalidInput {
                field: "server_host".to_string(),
                reason: "Server host contains spaces".to_string(),
            });
        }

        Ok(())
    }

    /// Validate server port
    ///
    /// # Rules
    /// - Must be in range 1-65535
    pub fn validate_port(port: u16) -> SecurityResult<()> {
        if port == 0 || port == 65536 {
            return Err(SecurityError::InvalidInput {
                field: "server_port".to_string(),
                reason: "Server port must be between 1 and 65535".to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username_valid() {
        assert!(Validator::validate_username("user@example.com").is_ok());
        assert!(Validator::validate_username("john_doe").is_ok());
        assert!(Validator::validate_username("user123").is_ok());
        assert!(Validator::validate_username("user.name+tag@example.com").is_ok());
    }

    #[test]
    fn test_validate_username_empty() {
        let result = Validator::validate_username("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_username_too_long() {
        let long_username = "a".repeat(257);
        let result = Validator::validate_username(&long_username);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds"));
    }

    #[test]
    fn test_validate_username_invalid_chars() {
        assert!(Validator::validate_username("user name").is_err()); // space
        assert!(Validator::validate_username("user/name").is_err()); // slash
        assert!(Validator::validate_username("user;name").is_err()); // semicolon
    }

    #[test]
    fn test_validate_username_control_chars() {
        assert!(Validator::validate_username("user\nname").is_err());
        assert!(Validator::validate_username("user\rname").is_err());
    }

    #[test]
    fn test_validate_password_valid() {
        assert!(Validator::validate_password("password123").is_ok());
        assert!(Validator::validate_password("P@ssw0rd!").is_ok());
        assert!(Validator::validate_password("very-long-password-with-special-chars!@#$%^&*()").is_ok());
    }

    #[test]
    fn test_validate_password_empty() {
        let result = Validator::validate_password("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_password_too_long() {
        let long_password = "a".repeat(257);
        let result = Validator::validate_password(&long_password);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds"));
    }

    #[test]
    fn test_validate_password_null_byte() {
        let result = Validator::validate_password("pass\0word");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("null bytes"));
    }

    #[test]
    fn test_validate_service_valid() {
        assert!(Validator::validate_service("com.rustalk.sip").is_ok());
        assert!(Validator::validate_service("my-service_123").is_ok());
        assert!(Validator::validate_service("service.name").is_ok());
    }

    #[test]
    fn test_validate_service_empty() {
        let result = Validator::validate_service("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_service_invalid_chars() {
        assert!(Validator::validate_service("has spaces").is_err());
        assert!(Validator::validate_service("has/slash").is_err());
        assert!(Validator::validate_service("has@at").is_err());
    }
}
