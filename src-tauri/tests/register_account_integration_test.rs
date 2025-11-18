// Integration tests for register_account command
// Note: Full integration testing requires a SIP server. These tests focus on
// validation and command structure. Actual registration testing should be done
// with a test SIP server or mock server.

use rustalk_lib::commands::validation::{
    validate_contact_uri, validate_hostname, validate_non_empty_string, validate_port_range,
    validate_protocol,
};
use rustalk_lib::domain::CommandError;

#[test]
fn test_register_account_validation_empty_server() {
    let result = validate_non_empty_string("server", "");
    assert!(result.is_err());
    if let Err(CommandError::ValidationError { field, .. }) = result {
        assert_eq!(field, "server");
    } else {
        panic!("Expected ValidationError for empty server");
    }
}

#[test]
fn test_register_account_validation_invalid_port() {
    let result = validate_port_range(0);
    assert!(result.is_err());
    if let Err(CommandError::InvalidArgument { argument, .. }) = result {
        assert_eq!(argument, "port");
    } else {
        panic!("Expected InvalidArgument for port 0");
    }
}

#[test]
fn test_register_account_validation_valid_port() {
    assert!(validate_port_range(5060).is_ok());
    assert!(validate_port_range(1).is_ok());
    assert!(validate_port_range(65535).is_ok());
}

#[test]
fn test_register_account_validation_invalid_protocol() {
    let result = validate_protocol("http");
    assert!(result.is_err());
    if let Err(CommandError::ValidationError { field, .. }) = result {
        assert_eq!(field, "protocol");
    } else {
        panic!("Expected ValidationError for invalid protocol");
    }
}

#[test]
fn test_register_account_validation_valid_protocols() {
    assert!(validate_protocol("udp").is_ok());
    assert!(validate_protocol("tcp").is_ok());
    assert!(validate_protocol("tls").is_ok());
    assert!(validate_protocol("UDP").is_ok()); // Case insensitive
}

#[test]
fn test_register_account_validation_empty_username() {
    let result = validate_non_empty_string("username", "");
    assert!(result.is_err());
    if let Err(CommandError::ValidationError { field, .. }) = result {
        assert_eq!(field, "username");
    } else {
        panic!("Expected ValidationError for empty username");
    }
}

#[test]
fn test_register_account_validation_empty_password() {
    let result = validate_non_empty_string("password", "");
    assert!(result.is_err());
    if let Err(CommandError::ValidationError { field, .. }) = result {
        assert_eq!(field, "password");
    } else {
        panic!("Expected ValidationError for empty password");
    }
}

#[test]
fn test_register_account_validation_invalid_hostname() {
    let result = validate_hostname("");
    assert!(result.is_err());
}

#[test]
fn test_register_account_validation_valid_hostname() {
    assert!(validate_hostname("sip.example.com").is_ok());
    assert!(validate_hostname("192.168.1.100").is_ok());
    assert!(validate_hostname("localhost").is_ok());
}

#[test]
fn test_register_account_validation_invalid_contact_uri() {
    // Missing sip: prefix
    let result = validate_contact_uri(Some("user@example.com"));
    assert!(result.is_err());

    // Missing @ symbol
    let result = validate_contact_uri(Some("sip:userexample.com"));
    assert!(result.is_err());

    // Empty string
    let result = validate_contact_uri(Some(""));
    assert!(result.is_err());
}

#[test]
fn test_register_account_validation_valid_contact_uri() {
    assert!(validate_contact_uri(Some("sip:user@example.com")).is_ok());
    assert!(validate_contact_uri(Some("sips:user@example.com")).is_ok());
    assert!(validate_contact_uri(Some("sip:user@192.168.1.100:5060")).is_ok());
    assert!(validate_contact_uri(None).is_ok()); // Optional field
}

#[test]
fn test_register_account_validation_complete_valid_input() {
    // Test that all validations pass for a complete valid input set
    assert!(validate_non_empty_string("server", "sip.example.com").is_ok());
    assert!(validate_hostname("sip.example.com").is_ok());
    assert!(validate_port_range(5060).is_ok());
    assert!(validate_protocol("udp").is_ok());
    assert!(validate_non_empty_string("username", "user1").is_ok());
    assert!(validate_non_empty_string("password", "password123").is_ok());
    assert!(validate_contact_uri(Some("sip:user1@192.168.1.100:5060")).is_ok());
}

// Note: Actual registration flow testing requires:
// 1. A test SIP server (e.g., Asterisk, FreeSWITCH, or a mock server)
// 2. Proper network setup
// 3. Async test environment with tokio runtime
//
// Example of what a full integration test would look like:
// #[tokio::test]
// async fn test_register_account_success() {
//     // Setup: Create AppState with SIP client
//     // Execute: Call register_account command
//     // Verify: Check registration status is "registered"
// }
//
// #[tokio::test]
// async fn test_register_account_invalid_credentials() {
//     // Setup: Create AppState with SIP client
//     // Execute: Call register_account with invalid credentials
//     // Verify: Check registration status is "failed" with appropriate error
// }
