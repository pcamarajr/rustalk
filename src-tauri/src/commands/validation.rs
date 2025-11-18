// Validation functions for Tauri command inputs
// All validators return Result<(), CommandError> for use with the ? operator

use crate::domain::errors::CommandError;

/// Validates that a string is non-empty
///
/// # Arguments
///
/// * `field` - The name of the field being validated (for error messages)
/// * `value` - The string value to validate
///
/// # Returns
///
/// * `Ok(())` if the string is non-empty
/// * `Err(CommandError::ValidationError)` if the string is empty
///
/// # Example
///
/// ```rust
/// use rustalk_lib::commands::validate_non_empty_string;
/// # fn main() -> Result<(), rustalk_lib::domain::CommandError> {
/// let name = "Alice";
/// validate_non_empty_string("name", name)?;
/// # Ok(())
/// # }
/// ```
pub fn validate_non_empty_string(field: &str, value: &str) -> Result<(), CommandError> {
    if value.trim().is_empty() {
        return Err(CommandError::ValidationError {
            field: field.to_string(),
            message: format!("{} cannot be empty", field),
        });
    }
    Ok(())
}

/// Validates that a string has a length within the specified range
///
/// # Arguments
///
/// * `field` - The name of the field being validated (for error messages)
/// * `value` - The string value to validate
/// * `min` - Minimum allowed length (inclusive)
/// * `max` - Maximum allowed length (inclusive)
///
/// # Returns
///
/// * `Ok(())` if the string length is within the range
/// * `Err(CommandError::ValidationError)` if the length is outside the range
///
/// # Example
///
/// ```rust
/// use rustalk_lib::commands::validate_string_length;
/// # fn main() -> Result<(), rustalk_lib::domain::CommandError> {
/// let username = "testuser";
/// validate_string_length("username", username, 3, 20)?;
/// # Ok(())
/// # }
/// ```
pub fn validate_string_length(
    field: &str,
    value: &str,
    min: usize,
    max: usize,
) -> Result<(), CommandError> {
    let len = value.len();
    if len < min {
        return Err(CommandError::ValidationError {
            field: field.to_string(),
            message: format!("{} must be at least {} characters long", field, min),
        });
    }
    if len > max {
        return Err(CommandError::ValidationError {
            field: field.to_string(),
            message: format!("{} must be at most {} characters long", field, max),
        });
    }
    Ok(())
}

/// Validates a phone number format (placeholder for future implementation)
///
/// # Arguments
///
/// * `number` - The phone number string to validate
///
/// # Returns
///
/// * `Ok(())` if the phone number format is valid
/// * `Err(CommandError::ValidationError)` if the format is invalid
///
/// # Note
///
/// This is a placeholder implementation. Future versions will include
/// proper phone number format validation.
pub fn validate_phone_number(number: &str) -> Result<(), CommandError> {
    // Placeholder: just check non-empty for now
    validate_non_empty_string("phone_number", number)?;
    // TODO: Add proper phone number format validation
    Ok(())
}

/// Validates that a port number is within the valid range (1-65535)
///
/// # Arguments
///
/// * `port` - The port number to validate
///
/// # Returns
///
/// * `Ok(())` if the port is valid
/// * `Err(CommandError::InvalidArgument)` if the port is outside the valid range
///
/// # Example
///
/// ```rust
/// use rustalk_lib::commands::validate_port;
/// # fn main() -> Result<(), rustalk_lib::domain::CommandError> {
/// validate_port(8080)?;
/// # Ok(())
/// # }
/// ```
pub fn validate_port(port: u16) -> Result<(), CommandError> {
    if port == 0 {
        return Err(CommandError::InvalidArgument {
            argument: "port".to_string(),
            reason: "Port cannot be 0".to_string(),
        });
    }
    // u16 is already constrained to 0-65535, so we only need to check for 0
    Ok(())
}

/// Validates a hostname format (basic validation)
///
/// # Arguments
///
/// * `hostname` - The hostname string to validate
///
/// # Returns
///
/// * `Ok(())` if the hostname format is valid
/// * `Err(CommandError::ValidationError)` if the format is invalid
///
/// # Validation Rules
///
/// - Must be non-empty
/// - Must not contain spaces
/// - Must not start or end with a dot or hyphen
/// - Must contain only valid hostname characters (alphanumeric, dots, hyphens)
///
/// # Example
///
/// ```rust
/// use rustalk_lib::commands::validate_hostname;
/// # fn main() -> Result<(), rustalk_lib::domain::CommandError> {
/// validate_hostname("example.com")?;
/// # Ok(())
/// # }
/// ```
pub fn validate_hostname(hostname: &str) -> Result<(), CommandError> {
    validate_non_empty_string("hostname", hostname)?;

    // Check for spaces
    if hostname.contains(' ') {
        return Err(CommandError::ValidationError {
            field: "hostname".to_string(),
            message: "Hostname cannot contain spaces".to_string(),
        });
    }

    // Check that it doesn't start or end with dot or hyphen
    if hostname.starts_with('.') || hostname.ends_with('.') {
        return Err(CommandError::ValidationError {
            field: "hostname".to_string(),
            message: "Hostname cannot start or end with a dot".to_string(),
        });
    }

    if hostname.starts_with('-') || hostname.ends_with('-') {
        return Err(CommandError::ValidationError {
            field: "hostname".to_string(),
            message: "Hostname cannot start or end with a hyphen".to_string(),
        });
    }

    // Basic character validation: alphanumeric, dots, hyphens
    if !hostname
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
    {
        return Err(CommandError::ValidationError {
            field: "hostname".to_string(),
            message: "Hostname contains invalid characters".to_string(),
        });
    }

    Ok(())
}

/// Validates that a port number is within the valid range (1-65535)
///
/// # Arguments
///
/// * `port` - The port number to validate
///
/// # Returns
///
/// * `Ok(())` if the port is valid
/// * `Err(CommandError::InvalidArgument)` if the port is outside the valid range
///
/// # Example
///
/// ```rust
/// use rustalk_lib::commands::validate_port_range;
/// # fn main() -> Result<(), rustalk_lib::domain::CommandError> {
/// validate_port_range(8080)?;
/// # Ok(())
/// # }
/// ```
pub fn validate_port_range(port: u16) -> Result<(), CommandError> {
    if port == 0 {
        return Err(CommandError::InvalidArgument {
            argument: "port".to_string(),
            reason: "Port must be between 1 and 65535".to_string(),
        });
    }
    // u16 is already constrained to 0-65535, so we only need to check for 0
    Ok(())
}

/// Validates a protocol string (udp, tcp, or tls)
///
/// # Arguments
///
/// * `protocol` - The protocol string to validate
///
/// # Returns
///
/// * `Ok(())` if the protocol is valid
/// * `Err(CommandError::ValidationError)` if the protocol is invalid
///
/// # Example
///
/// ```rust
/// use rustalk_lib::commands::validate_protocol;
/// # fn main() -> Result<(), rustalk_lib::domain::CommandError> {
/// validate_protocol("udp")?;
/// # Ok(())
/// # }
/// ```
pub fn validate_protocol(protocol: &str) -> Result<(), CommandError> {
    let protocol_lower = protocol.to_lowercase();
    match protocol_lower.as_str() {
        "udp" | "tcp" | "tls" => Ok(()),
        _ => Err(CommandError::ValidationError {
            field: "protocol".to_string(),
            message: format!(
                "Protocol must be 'udp', 'tcp', or 'tls', got '{}'",
                protocol
            ),
        }),
    }
}

/// Validates a SIP contact URI format (basic validation)
///
/// # Arguments
///
/// * `contact_uri` - The contact URI string to validate (optional)
///
/// # Returns
///
/// * `Ok(())` if the contact URI is valid or None
/// * `Err(CommandError::ValidationError)` if the format is invalid
///
/// # Validation Rules
///
/// - Must start with "sip:" or "sips:"
/// - Must contain "@" (user@host format)
/// - Must not be empty if provided
///
/// # Example
///
/// ```rust
/// use rustalk_lib::commands::validate_contact_uri;
/// # fn main() -> Result<(), rustalk_lib::domain::CommandError> {
/// validate_contact_uri(Some("sip:user@192.168.1.100:5060"))?;
/// # Ok(())
/// # }
/// ```
pub fn validate_contact_uri(contact_uri: Option<&str>) -> Result<(), CommandError> {
    if let Some(uri) = contact_uri {
        if uri.trim().is_empty() {
            return Err(CommandError::ValidationError {
                field: "contact_uri".to_string(),
                message: "Contact URI cannot be empty if provided".to_string(),
            });
        }

        // Basic SIP URI format check: must start with sip: or sips:
        if !uri.starts_with("sip:") && !uri.starts_with("sips:") {
            return Err(CommandError::ValidationError {
                field: "contact_uri".to_string(),
                message: "Contact URI must start with 'sip:' or 'sips:'".to_string(),
            });
        }

        // Must contain @ for user@host format
        if !uri.contains('@') {
            return Err(CommandError::ValidationError {
                field: "contact_uri".to_string(),
                message: "Contact URI must contain '@' (user@host format)".to_string(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for validate_non_empty_string
    #[test]
    fn test_validate_non_empty_string_valid() {
        assert!(validate_non_empty_string("name", "test").is_ok());
        assert!(validate_non_empty_string("name", "  test  ").is_ok());
    }

    #[test]
    fn test_validate_non_empty_string_empty() {
        let result = validate_non_empty_string("name", "");
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "name");
            assert!(message.contains("cannot be empty"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_non_empty_string_whitespace_only() {
        let result = validate_non_empty_string("name", "   ");
        assert!(result.is_err());
    }

    // Tests for validate_string_length
    #[test]
    fn test_validate_string_length_valid() {
        assert!(validate_string_length("username", "test", 3, 20).is_ok());
        assert!(validate_string_length("username", "abc", 3, 20).is_ok()); // min boundary
        assert!(validate_string_length("username", "abcdefghijklmnopqrst", 3, 20).is_ok());
        // max boundary
    }

    #[test]
    fn test_validate_string_length_too_short() {
        let result = validate_string_length("username", "ab", 3, 20);
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "username");
            assert!(message.contains("at least 3 characters"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_string_length_too_long() {
        let result = validate_string_length("username", "this_is_too_long_for_validation", 3, 20);
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "username");
            assert!(message.contains("at most 20 characters"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    // Tests for validate_phone_number
    #[test]
    fn test_validate_phone_number_valid() {
        assert!(validate_phone_number("1234567890").is_ok());
        assert!(validate_phone_number("+1-555-123-4567").is_ok());
    }

    #[test]
    fn test_validate_phone_number_empty() {
        let result = validate_phone_number("");
        assert!(result.is_err());
    }

    // Tests for validate_port
    #[test]
    fn test_validate_port_valid() {
        assert!(validate_port(1).is_ok());
        assert!(validate_port(8080).is_ok());
        assert!(validate_port(65535).is_ok());
    }

    #[test]
    fn test_validate_port_zero() {
        let result = validate_port(0);
        assert!(result.is_err());
        if let Err(CommandError::InvalidArgument { argument, reason }) = result {
            assert_eq!(argument, "port");
            assert!(reason.contains("cannot be 0"));
        } else {
            panic!("Expected InvalidArgument");
        }
    }

    // Tests for validate_hostname
    #[test]
    fn test_validate_hostname_valid() {
        assert!(validate_hostname("example.com").is_ok());
        assert!(validate_hostname("subdomain.example.com").is_ok());
        assert!(validate_hostname("example-host.com").is_ok());
        assert!(validate_hostname("localhost").is_ok());
    }

    #[test]
    fn test_validate_hostname_empty() {
        let result = validate_hostname("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_hostname_with_spaces() {
        let result = validate_hostname("example .com");
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "hostname");
            assert!(message.contains("spaces"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_hostname_starts_with_dot() {
        let result = validate_hostname(".example.com");
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "hostname");
            assert!(message.contains("start or end with a dot"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_hostname_ends_with_dot() {
        let result = validate_hostname("example.com.");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_hostname_starts_with_hyphen() {
        let result = validate_hostname("-example.com");
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "hostname");
            assert!(message.contains("start or end with a hyphen"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_hostname_ends_with_hyphen() {
        let result = validate_hostname("example.com-");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_hostname_invalid_characters() {
        let result = validate_hostname("example@com");
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "hostname");
            assert!(message.contains("invalid characters"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    // Edge case tests
    #[test]
    fn test_validate_string_length_empty_string() {
        let result = validate_string_length("field", "", 1, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_string_length_min_equals_max() {
        assert!(validate_string_length("field", "abc", 3, 3).is_ok());
        assert!(validate_string_length("field", "ab", 3, 3).is_err());
        assert!(validate_string_length("field", "abcd", 3, 3).is_err());
    }

    // Tests for validate_port_range
    #[test]
    fn test_validate_port_range_valid() {
        assert!(validate_port_range(1).is_ok());
        assert!(validate_port_range(8080).is_ok());
        assert!(validate_port_range(65535).is_ok());
    }

    #[test]
    fn test_validate_port_range_zero() {
        let result = validate_port_range(0);
        assert!(result.is_err());
        if let Err(CommandError::InvalidArgument { argument, reason }) = result {
            assert_eq!(argument, "port");
            assert!(reason.contains("between 1 and 65535"));
        } else {
            panic!("Expected InvalidArgument");
        }
    }

    // Tests for validate_protocol
    #[test]
    fn test_validate_protocol_valid() {
        assert!(validate_protocol("udp").is_ok());
        assert!(validate_protocol("tcp").is_ok());
        assert!(validate_protocol("tls").is_ok());
        assert!(validate_protocol("UDP").is_ok()); // Case insensitive
        assert!(validate_protocol("TCP").is_ok());
        assert!(validate_protocol("TLS").is_ok());
    }

    #[test]
    fn test_validate_protocol_invalid() {
        let result = validate_protocol("http");
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "protocol");
            assert!(message.contains("udp") || message.contains("tcp") || message.contains("tls"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_protocol_empty() {
        let result = validate_protocol("");
        assert!(result.is_err());
    }

    // Tests for validate_contact_uri
    #[test]
    fn test_validate_contact_uri_valid() {
        assert!(validate_contact_uri(Some("sip:user@example.com")).is_ok());
        assert!(validate_contact_uri(Some("sips:user@example.com")).is_ok());
        assert!(validate_contact_uri(Some("sip:user@192.168.1.100:5060")).is_ok());
        assert!(validate_contact_uri(None).is_ok()); // Optional field
    }

    #[test]
    fn test_validate_contact_uri_empty() {
        let result = validate_contact_uri(Some(""));
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "contact_uri");
            assert!(message.contains("cannot be empty"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_contact_uri_no_sip_prefix() {
        let result = validate_contact_uri(Some("user@example.com"));
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "contact_uri");
            assert!(message.contains("sip:") || message.contains("sips:"));
        } else {
            panic!("Expected ValidationError");
        }
    }

    #[test]
    fn test_validate_contact_uri_no_at_symbol() {
        let result = validate_contact_uri(Some("sip:userexample.com"));
        assert!(result.is_err());
        if let Err(CommandError::ValidationError { field, message }) = result {
            assert_eq!(field, "contact_uri");
            assert!(message.contains("@"));
        } else {
            panic!("Expected ValidationError");
        }
    }
}
