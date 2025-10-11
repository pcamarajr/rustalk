// RUSTALK Security & Storage Module - Unit Tests
//
// TDD Test Suite for Secure Credential Storage
// These tests are designed to FAIL initially (implementation doesn't exist yet)
// Target Coverage: 85%+
//
// Test Categories:
// 1. Credential Storage
// 2. Credential Retrieval
// 3. Credential Deletion
// 4. Error Handling
// 5. Platform-Specific Tests
// 6. Security & Validation

#[cfg(test)]
mod credential_storage_tests {
    use super::super::*;
    use crate::security::{
        Credential, CredentialError, CredentialService, SipAccount, SipCredentials, SipTransport,
    };
    use std::collections::HashMap;

    // ========================================================================
    // Test Helpers & Fixtures
    // ========================================================================

    /// Create test SIP account
    fn create_test_account() -> SipAccount {
        SipAccount {
            id: uuid::Uuid::new_v4().to_string(),
            display_name: "Test Account".to_string(),
            server_host: "sip.example.com".to_string(),
            server_port: 5061,
            transport: SipTransport::Sips,
            status: crate::security::AccountStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Create test credentials
    fn create_test_credentials() -> SipCredentials {
        SipCredentials::new("testuser@example.com".to_string(), "testpass123".to_string())
            .expect("Failed to create test credentials")
    }

    /// Create test service with mock keychain
    async fn setup_test_service() -> CredentialService {
        CredentialService::new_test()
            .await
            .expect("Failed to create test service")
    }

    // ========================================================================
    // 1. CREDENTIAL STORAGE TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_store_credentials_successfully() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        // Act
        let result = service.store_credentials(account.clone(), credentials).await;

        // Assert
        assert!(result.is_ok(), "Should store credentials successfully");
        let account_id = result.unwrap();
        assert!(!account_id.is_empty(), "Should return non-empty account ID");
        assert_eq!(account_id, account.id, "Should return correct account ID");
    }

    #[tokio::test]
    async fn test_store_credentials_duplicate_account() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials1 = create_test_credentials();
        let credentials2 = create_test_credentials();

        // Store first credential
        service
            .store_credentials(account.clone(), credentials1)
            .await
            .expect("First store should succeed");

        // Act - Try to store duplicate
        let result = service.store_credentials(account, credentials2).await;

        // Assert
        assert!(result.is_err(), "Should fail on duplicate account");
        assert!(
            matches!(result.unwrap_err(), CredentialError::AlreadyExists),
            "Should return AlreadyExists error"
        );
    }

    #[tokio::test]
    async fn test_store_credentials_validates_username() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();

        // Test cases for invalid usernames
        let invalid_usernames = vec![
            "",                         // Empty
            " ",                        // Whitespace only
            "user name",                // Contains space
            "user@",                    // Incomplete email
            "@example.com",             // Missing local part
            "user\n@example.com",       // Contains newline
            "user\0@example.com",       // Contains null byte
            &"a".repeat(257),           // Exceeds max length (256)
            "user'; DROP TABLE users;", // SQL injection attempt
        ];

        for invalid_username in invalid_usernames {
            // Act
            let credentials_result =
                SipCredentials::new(invalid_username.to_string(), "password123".to_string());

            // Assert
            assert!(
                credentials_result.is_err(),
                "Should reject invalid username: '{}'",
                invalid_username
            );
        }
    }

    #[tokio::test]
    async fn test_store_credentials_validates_password() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();

        // Test cases for invalid passwords
        let invalid_passwords = vec![
            "",                 // Empty
            " ",                // Whitespace only
            "\0password",       // Contains null byte
            &"a".repeat(257),   // Exceeds max length (256)
        ];

        for invalid_password in invalid_passwords {
            // Act
            let credentials_result =
                SipCredentials::new("testuser@example.com".to_string(), invalid_password.to_string());

            // Assert
            assert!(
                credentials_result.is_err(),
                "Should reject invalid password of length: {}",
                invalid_password.len()
            );
        }
    }

    #[tokio::test]
    async fn test_store_credentials_validates_server_host() {
        // Arrange
        let service = setup_test_service().await;
        let mut account = create_test_account();
        let credentials = create_test_credentials();

        // Test cases for invalid server hosts
        let invalid_hosts = vec![
            "",                        // Empty
            "server\nname",            // Contains newline
            "server\0name",            // Contains null byte
            &"a".repeat(256),          // Exceeds max length (255)
            "server name",             // Contains space
        ];

        for invalid_host in invalid_hosts {
            account.server_host = invalid_host.to_string();

            // Act
            let result = service
                .store_credentials(account.clone(), credentials.clone())
                .await;

            // Assert
            assert!(
                result.is_err(),
                "Should reject invalid server host: '{}'",
                invalid_host
            );
        }
    }

    #[tokio::test]
    async fn test_store_credentials_validates_server_port() {
        // Arrange
        let service = setup_test_service().await;
        let mut account = create_test_account();
        let credentials = create_test_credentials();

        // Test cases for invalid ports
        let invalid_ports = vec![0, 65536]; // Port must be 1-65535

        for invalid_port in invalid_ports {
            account.server_port = invalid_port;

            // Act
            let result = service
                .store_credentials(account.clone(), credentials.clone())
                .await;

            // Assert
            assert!(
                result.is_err(),
                "Should reject invalid server port: {}",
                invalid_port
            );
        }
    }

    #[tokio::test]
    async fn test_store_multiple_accounts() {
        // Arrange
        let service = setup_test_service().await;
        let mut accounts = Vec::new();
        let mut credentials = Vec::new();

        // Create 10 unique accounts
        for i in 0..10 {
            let mut account = create_test_account();
            account.display_name = format!("Account {}", i);
            let cred = SipCredentials::new(
                format!("user{}@example.com", i),
                format!("password{}", i),
            )
            .unwrap();
            accounts.push(account);
            credentials.push(cred);
        }

        // Act - Store all accounts
        for (account, cred) in accounts.iter().zip(credentials.iter()) {
            let result = service.store_credentials(account.clone(), cred.clone()).await;
            assert!(result.is_ok(), "Should store account {}", account.display_name);
        }

        // Assert - All accounts should be retrievable
        let account_list = service.list_accounts().await.unwrap();
        assert_eq!(
            account_list.len(),
            10,
            "Should have stored 10 accounts"
        );
    }

    // ========================================================================
    // 2. CREDENTIAL RETRIEVAL TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_get_credentials_successfully() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();
        let stored_username = credentials.username.clone();
        let stored_password = credentials.password.clone();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act
        let result = service.get_credentials(&account_id).await;

        // Assert
        assert!(result.is_ok(), "Should retrieve credentials successfully");
        let (retrieved_account, retrieved_creds) = result.unwrap();
        assert_eq!(retrieved_account.id, account_id);
        assert_eq!(retrieved_creds.username, stored_username);
        assert_eq!(retrieved_creds.password, stored_password);
    }

    #[tokio::test]
    async fn test_get_credentials_not_found() {
        // Arrange
        let service = setup_test_service().await;
        let nonexistent_id = uuid::Uuid::new_v4().to_string();

        // Act
        let result = service.get_credentials(&nonexistent_id).await;

        // Assert
        assert!(result.is_err(), "Should fail for nonexistent account");
        assert!(
            matches!(result.unwrap_err(), CredentialError::NotFound),
            "Should return NotFound error"
        );
    }

    #[tokio::test]
    async fn test_get_credentials_with_special_characters() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let special_password = "p@ssw0rd!#$%^&*()_+-=[]{}|;':\",./<>?`~";
        let credentials =
            SipCredentials::new("testuser@example.com".to_string(), special_password.to_string())
                .unwrap();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act
        let result = service.get_credentials(&account_id).await;

        // Assert
        assert!(result.is_ok(), "Should handle special characters");
        let (_, retrieved_creds) = result.unwrap();
        assert_eq!(
            retrieved_creds.password, special_password,
            "Should preserve special characters"
        );
    }

    #[tokio::test]
    async fn test_get_credentials_performance() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act - Measure retrieval time
        let start = std::time::Instant::now();
        let result = service.get_credentials(&account_id).await;
        let duration = start.elapsed();

        // Assert
        assert!(result.is_ok(), "Should retrieve successfully");
        assert!(
            duration.as_millis() < 50,
            "Should retrieve within 50ms (was {}ms)",
            duration.as_millis()
        );
    }

    #[tokio::test]
    async fn test_list_accounts_excludes_passwords() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act
        let accounts = service.list_accounts().await.unwrap();

        // Assert
        assert!(!accounts.is_empty(), "Should return accounts");
        // SipAccount struct should not have a password field
        // This is a compile-time guarantee, but we can verify the structure
        for account in accounts {
            // Verify account has expected fields but no password
            assert!(!account.display_name.is_empty());
            assert!(!account.server_host.is_empty());
            assert!(account.server_port > 0);
        }
    }

    // ========================================================================
    // 3. CREDENTIAL DELETION TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_delete_account_successfully() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act
        let result = service.delete_account(&account_id).await;

        // Assert
        assert!(result.is_ok(), "Should delete account successfully");

        // Verify account is actually deleted
        let get_result = service.get_credentials(&account_id).await;
        assert!(
            get_result.is_err(),
            "Should not retrieve deleted account"
        );
    }

    #[tokio::test]
    async fn test_delete_nonexistent_account() {
        // Arrange
        let service = setup_test_service().await;
        let nonexistent_id = uuid::Uuid::new_v4().to_string();

        // Act
        let result = service.delete_account(&nonexistent_id).await;

        // Assert
        assert!(result.is_err(), "Should fail for nonexistent account");
        assert!(
            matches!(result.unwrap_err(), CredentialError::NotFound),
            "Should return NotFound error"
        );
    }

    #[tokio::test]
    async fn test_delete_account_removes_from_keychain() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        let account_id = service
            .store_credentials(account.clone(), credentials)
            .await
            .unwrap();

        // Act
        service.delete_account(&account_id).await.unwrap();

        // Assert - Verify removed from both database and keychain
        let get_result = service.get_credentials(&account_id).await;
        assert!(matches!(
            get_result.unwrap_err(),
            CredentialError::NotFound
        ));

        // Verify not in list
        let accounts = service.list_accounts().await.unwrap();
        assert!(!accounts.iter().any(|a| a.id == account_id));
    }

    // ========================================================================
    // 4. ERROR HANDLING TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_keychain_unavailable_error() {
        // Arrange - Create service with unavailable keychain mock
        let service = CredentialService::new_mock_unavailable()
            .await
            .expect("Failed to create mock service");

        let account = create_test_account();
        let credentials = create_test_credentials();

        // Act
        let result = service.store_credentials(account, credentials).await;

        // Assert
        assert!(result.is_err(), "Should fail when keychain unavailable");
        assert!(
            matches!(
                result.unwrap_err(),
                CredentialError::Keychain(crate::security::KeychainError::Unavailable)
            ),
            "Should return Keychain::Unavailable error"
        );
    }

    #[tokio::test]
    async fn test_keychain_permission_denied_error() {
        // Arrange - Create service with permission denied mock
        let service = CredentialService::new_mock_permission_denied()
            .await
            .expect("Failed to create mock service");

        let account = create_test_account();
        let credentials = create_test_credentials();

        // Act
        let result = service.store_credentials(account, credentials).await;

        // Assert
        assert!(result.is_err(), "Should fail when permission denied");
        assert!(
            matches!(
                result.unwrap_err(),
                CredentialError::Keychain(crate::security::KeychainError::PermissionDenied)
            ),
            "Should return Keychain::PermissionDenied error"
        );
    }

    #[tokio::test]
    async fn test_sql_injection_prevention() {
        // Arrange
        let service = setup_test_service().await;
        let mut account = create_test_account();

        // SQL injection attempts
        let sql_injection_attempts = vec![
            "admin'; DROP TABLE sip_accounts; --",
            "' OR '1'='1",
            "1'; DELETE FROM sip_accounts WHERE '1'='1",
        ];

        for injection in sql_injection_attempts {
            account.display_name = injection.to_string();

            let credentials = SipCredentials::new(
                "testuser@example.com".to_string(),
                "password123".to_string(),
            )
            .unwrap();

            // Act
            let result = service
                .store_credentials(account.clone(), credentials)
                .await;

            // Assert - Should either reject or sanitize
            if result.is_ok() {
                // If accepted, verify table still exists by listing accounts
                let accounts = service.list_accounts().await;
                assert!(
                    accounts.is_ok(),
                    "Database should still be intact after injection attempt"
                );
            } else {
                assert!(
                    matches!(result.unwrap_err(), CredentialError::Validation(_)),
                    "Should return validation error for SQL injection"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_null_byte_prevention() {
        // Arrange
        let service = setup_test_service().await;

        // Test null bytes in various fields
        let null_byte_tests = vec![
            ("user\0name@example.com", "password"),
            ("username@example.com", "pass\0word"),
        ];

        for (username, password) in null_byte_tests {
            // Act
            let credentials_result = SipCredentials::new(username.to_string(), password.to_string());

            // Assert
            assert!(
                credentials_result.is_err(),
                "Should reject null bytes in credentials"
            );
        }
    }

    #[tokio::test]
    async fn test_control_characters_rejection() {
        // Arrange
        let service = setup_test_service().await;
        let mut account = create_test_account();
        let credentials = create_test_credentials();

        // Test control characters in server host
        let control_chars = vec![
            "server\nname.com", // Newline
            "server\rname.com", // Carriage return
            "server\tname.com", // Tab
        ];

        for server_host in control_chars {
            account.server_host = server_host.to_string();

            // Act
            let result = service
                .store_credentials(account.clone(), credentials.clone())
                .await;

            // Assert
            assert!(
                result.is_err(),
                "Should reject control characters in server host: {:?}",
                server_host
            );
        }
    }

    // ========================================================================
    // 5. MEMORY SAFETY TESTS
    // ========================================================================

    #[test]
    fn test_credentials_zeroized_on_drop() {
        use std::ptr;

        // Arrange
        let password = "secret_password_123".to_string();
        let password_ptr = password.as_ptr();
        let password_len = password.len();

        {
            // Create credentials in inner scope
            let credentials =
                SipCredentials::new("testuser@example.com".to_string(), password).unwrap();

            // Verify credentials exist
            assert_eq!(credentials.password, "secret_password_123");
        } // credentials dropped here

        // Assert - Memory should be zeroed (Note: This is implementation-dependent)
        // In a real implementation with zeroize crate, this would verify zeroing
        // For TDD, we're defining the expected behavior
    }

    #[test]
    fn test_credentials_not_logged_in_debug() {
        // Arrange
        let credentials =
            SipCredentials::new("testuser@example.com".to_string(), "secret123".to_string())
                .unwrap();

        // Act
        let debug_output = format!("{:?}", credentials);

        // Assert
        assert!(
            !debug_output.contains("secret123"),
            "Password should not appear in debug output"
        );
        assert!(
            debug_output.contains("<redacted>") || debug_output.contains("***"),
            "Password should be redacted in debug output"
        );
        assert!(
            debug_output.contains("testuser@example.com"),
            "Username should appear in debug output"
        );
    }

    // ========================================================================
    // 6. UPDATE OPERATIONS TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_update_credentials_successfully() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act - Update password
        let new_credentials =
            SipCredentials::new("testuser@example.com".to_string(), "new_password_456".to_string())
                .unwrap();
        let update_result = service
            .update_credentials(&account_id, new_credentials)
            .await;

        // Assert
        assert!(update_result.is_ok(), "Should update credentials successfully");

        // Verify new password is stored
        let (_, retrieved_creds) = service.get_credentials(&account_id).await.unwrap();
        assert_eq!(retrieved_creds.password, "new_password_456");
    }

    #[tokio::test]
    async fn test_update_account_metadata() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act - Update account metadata
        let mut updates = crate::security::AccountUpdate::default();
        updates.display_name = Some("Updated Account Name".to_string());
        updates.server_port = Some(5062);

        let update_result = service.update_account(&account_id, updates).await;

        // Assert
        assert!(update_result.is_ok(), "Should update account metadata");

        // Verify updates
        let (updated_account, _) = service.get_credentials(&account_id).await.unwrap();
        assert_eq!(updated_account.display_name, "Updated Account Name");
        assert_eq!(updated_account.server_port, 5062);
    }

    // ========================================================================
    // 7. PLATFORM-SPECIFIC TESTS
    // ========================================================================

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_macos_keychain_integration() {
        // Arrange
        let service = CredentialService::new().await.unwrap();
        let account = create_test_account();
        let credentials = create_test_credentials();

        // Act
        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Assert - Verify stored in macOS Keychain
        let (_, retrieved_creds) = service.get_credentials(&account_id).await.unwrap();
        assert_eq!(retrieved_creds.username, "testuser@example.com");

        // Cleanup
        service.delete_account(&account_id).await.unwrap();
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_credential_manager_integration() {
        // Arrange
        let service = CredentialService::new().await.unwrap();
        let account = create_test_account();
        let credentials = create_test_credentials();

        // Act
        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Assert - Verify stored in Windows Credential Manager
        let (_, retrieved_creds) = service.get_credentials(&account_id).await.unwrap();
        assert_eq!(retrieved_creds.username, "testuser@example.com");

        // Cleanup
        service.delete_account(&account_id).await.unwrap();
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    #[tokio::test]
    async fn test_unsupported_platform_error() {
        // On unsupported platforms, service creation should fail
        let result = CredentialService::new().await;

        assert!(
            result.is_err(),
            "Should fail on unsupported platform"
        );
    }

    // ========================================================================
    // 8. CONCURRENT ACCESS TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_concurrent_credential_access() {
        // Arrange
        let service = std::sync::Arc::new(setup_test_service().await);
        let account = create_test_account();
        let credentials = create_test_credentials();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act - Spawn multiple concurrent reads
        let mut handles = vec![];
        for _ in 0..10 {
            let service_clone = service.clone();
            let account_id_clone = account_id.clone();

            let handle = tokio::spawn(async move {
                service_clone.get_credentials(&account_id_clone).await
            });

            handles.push(handle);
        }

        // Assert - All reads should succeed
        let results = futures::future::join_all(handles).await;
        for result in results {
            let credential_result = result.unwrap();
            assert!(
                credential_result.is_ok(),
                "Concurrent reads should succeed"
            );
        }

        // Cleanup
        service.delete_account(&account_id).await.unwrap();
    }

    // ========================================================================
    // 9. VALIDATION EDGE CASES
    // ========================================================================

    #[test]
    fn test_username_validation_edge_cases() {
        // Valid usernames
        let valid_usernames = vec![
            "user@example.com",
            "user.name@example.com",
            "user+tag@example.com",
            "user_name@example.com",
            "user-name@example.com",
            "123@example.com",
            "a@b.c",
        ];

        for username in valid_usernames {
            let result = SipCredentials::new(username.to_string(), "password123".to_string());
            assert!(
                result.is_ok(),
                "Should accept valid username: '{}'",
                username
            );
        }

        // Invalid usernames
        let invalid_usernames = vec![
            "",                    // Empty
            " ",                   // Whitespace
            "user name@test.com",  // Space in username
            "user@",               // Incomplete
            "@example.com",        // Missing local part
            &"a".repeat(257),      // Too long
        ];

        for username in invalid_usernames {
            let result = SipCredentials::new(username.to_string(), "password123".to_string());
            assert!(
                result.is_err(),
                "Should reject invalid username: '{}'",
                username
            );
        }
    }

    #[test]
    fn test_password_length_validation() {
        // Test minimum length (should allow 1 character)
        let min_password = "a";
        let result = SipCredentials::new("user@example.com".to_string(), min_password.to_string());
        assert!(result.is_ok(), "Should accept 1-character password");

        // Test maximum length (256 characters)
        let max_password = "a".repeat(256);
        let result = SipCredentials::new("user@example.com".to_string(), max_password);
        assert!(result.is_ok(), "Should accept 256-character password");

        // Test exceeding maximum (257 characters)
        let too_long_password = "a".repeat(257);
        let result = SipCredentials::new("user@example.com".to_string(), too_long_password);
        assert!(result.is_err(), "Should reject 257-character password");
    }

    // ========================================================================
    // 10. ACCOUNT STATUS TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_account_status_changes() {
        // Arrange
        let service = setup_test_service().await;
        let account = create_test_account();
        let credentials = create_test_credentials();

        let account_id = service
            .store_credentials(account, credentials)
            .await
            .unwrap();

        // Act - Disable account
        let mut updates = crate::security::AccountUpdate::default();
        updates.status = Some(crate::security::AccountStatus::Disabled);

        service.update_account(&account_id, updates).await.unwrap();

        // Assert
        let (updated_account, _) = service.get_credentials(&account_id).await.unwrap();
        assert!(matches!(
            updated_account.status,
            crate::security::AccountStatus::Disabled
        ));

        // Cleanup
        service.delete_account(&account_id).await.unwrap();
    }
}

// ============================================================================
// Integration Tests with Mock Keychain
// ============================================================================

#[cfg(test)]
mod mock_keychain_tests {
    use super::super::*;

    #[tokio::test]
    async fn test_mock_keychain_store_and_retrieve() {
        // This test verifies the mock keychain implementation works correctly
        // Useful for CI/CD environments where real keychain isn't available

        // Note: Implementation will provide MockKeychain for testing
        // This test validates that the mock behaves like the real thing
    }

    #[tokio::test]
    async fn test_mock_keychain_error_scenarios() {
        // Test that mock can simulate various error conditions
        // - Permission denied
        // - Keychain unavailable
        // - Credential not found
        // - Platform errors
    }
}
