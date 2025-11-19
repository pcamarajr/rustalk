// Integration tests for initiate_call command
// Note: Full integration testing requires a SIP server and registered account.
// These tests focus on validation, command structure, and error conversion.

use rustalk_lib::commands::validation::validate_phone_number;
use rustalk_lib::domain::CommandError;

#[test]
fn test_initiate_call_validation_empty_number() {
    let result = validate_phone_number("");
    assert!(result.is_err());
    if let Err(CommandError::ValidationError { field, .. }) = result {
        assert_eq!(field, "phone_number");
    } else {
        panic!("Expected ValidationError for empty phone number");
    }
}

#[test]
fn test_initiate_call_validation_valid_number() {
    // Test various valid phone number formats
    assert!(validate_phone_number("1234567890").is_ok());
    assert!(validate_phone_number("+1-555-123-4567").is_ok());
    assert!(validate_phone_number("5551234567").is_ok());
    assert!(validate_phone_number("sip:user@example.com").is_ok());
}

#[test]
fn test_initiate_call_validation_whitespace_only() {
    let result = validate_phone_number("   ");
    assert!(result.is_err());
}

// Note: Full integration testing requires:
// 1. A registered SIP account (via register_account)
// 2. A test SIP server or mock server
// 3. Proper network setup
// 4. Async test environment with tokio runtime
//
// Example of what a full integration test would look like:
// #[tokio::test]
// async fn test_initiate_call_success() {
//     // Setup: Create AppState with registered account
//     // Execute: Call initiate_call command with valid number
//     // Verify: Check that CallId is returned
// }
//
// #[tokio::test]
// async fn test_initiate_call_not_registered() {
//     // Setup: Create AppState with unregistered account
//     // Execute: Call initiate_call command
//     // Verify: Check that error is returned indicating not registered
// }
//
// #[tokio::test]
// async fn test_initiate_call_invalid_number() {
//     // Setup: Create AppState with registered account
//     // Execute: Call initiate_call with empty/invalid number
//     // Verify: Check that validation error is returned
// }

