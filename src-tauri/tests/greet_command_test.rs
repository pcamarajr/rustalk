// Integration tests for the greet command validation

use rustalk_lib::domain::CommandError;
use rustalk_lib::greet;

#[test]
fn test_greet_command_valid_input() {
    let result = greet("Alice");
    assert!(result.is_ok());
    let message = result.unwrap();
    assert!(message.contains("Hello, Alice!"));
    assert!(message.contains("greeted from Rust"));
}

#[test]
fn test_greet_command_empty_string() {
    let result = greet("");
    assert!(result.is_err());
    if let Err(CommandError::ValidationError { field, message }) = result {
        assert_eq!(field, "name");
        assert!(message.contains("cannot be empty"));
    } else {
        panic!("Expected ValidationError for empty string");
    }
}

#[test]
fn test_greet_command_whitespace_only() {
    let result = greet("   ");
    assert!(result.is_err());
    if let Err(CommandError::ValidationError { field, .. }) = result {
        assert_eq!(field, "name");
    } else {
        panic!("Expected ValidationError for whitespace-only string");
    }
}

#[test]
fn test_greet_command_valid_with_whitespace() {
    // Should trim and accept strings with leading/trailing whitespace
    let result = greet("  Bob  ");
    assert!(result.is_ok());
    let message = result.unwrap();
    assert!(message.contains("Bob"));
}

#[test]
fn test_greet_command_special_characters() {
    // Should accept names with special characters
    let result = greet("O'Brien");
    assert!(result.is_ok());
    let message = result.unwrap();
    assert!(message.contains("O'Brien"));
}

#[test]
fn test_greet_command_unicode() {
    // Should accept unicode characters
    let result = greet("José");
    assert!(result.is_ok());
    let message = result.unwrap();
    assert!(message.contains("José"));
}

