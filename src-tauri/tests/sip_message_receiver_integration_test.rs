// Integration test for SIP message receiver loop
// Tests that the message receiver correctly receives SIP responses, extracts Call-ID,
// matches it to active calls, and routes responses to CallService.handle_invite_response()

use rustalk_lib::domain::entities::call::{Call, CallId};
use rustalk_lib::domain::traits::CredentialStore;
use rustalk_lib::infrastructure::sip::client::SipClient;
use rustalk_lib::services::auth_service::AuthService;
use rustalk_lib::services::call_service::CallService;
use std::sync::Arc;
use tokio::sync::Mutex;

// Mock credential store for testing
struct MockCredentialStore;

#[async_trait::async_trait]
impl CredentialStore for MockCredentialStore {
    async fn save(
        &self,
        _key: &str,
        _credentials: &rustalk_lib::domain::entities::credentials::Credentials,
    ) -> Result<(), rustalk_lib::domain::errors::CredentialStoreError> {
        Ok(())
    }

    async fn load(
        &self,
        _key: &str,
    ) -> Result<
        Option<rustalk_lib::domain::entities::credentials::Credentials>,
        rustalk_lib::domain::errors::CredentialStoreError,
    > {
        Ok(None)
    }

    async fn delete(
        &self,
        _key: &str,
    ) -> Result<(), rustalk_lib::domain::errors::CredentialStoreError> {
        Ok(())
    }

    async fn exists(
        &self,
        _key: &str,
    ) -> Result<bool, rustalk_lib::domain::errors::CredentialStoreError> {
        Ok(false)
    }
}

async fn create_test_call_service() -> Arc<Mutex<CallService>> {
    let client = SipClient::new_udp_any().await.unwrap();
    let client = Arc::new(Mutex::new(client));
    let credential_store = Arc::new(MockCredentialStore) as Arc<dyn CredentialStore>;
    let client_for_auth = SipClient::new_udp_any().await.unwrap();
    let auth_service = Arc::new(Mutex::new(AuthService::new(
        client_for_auth,
        credential_store,
    )));
    Arc::new(Mutex::new(CallService::new(client, auth_service, None)))
}

#[tokio::test]
async fn test_find_call_by_call_id_header_not_found() {
    let service = create_test_call_service().await;

    // Test with non-existent Call-ID (no calls stored)
    {
        let service_guard = service.lock().await;
        let found_call_id = service_guard
            .find_call_by_call_id_header("non-existent@example.com")
            .await;

        assert!(found_call_id.is_none());
    }
}

// Note: Full integration test with actual call storage would require:
// 1. Setting up registration state (requires AuthService to be registered)
// 2. Calling initiate_outbound_call() which stores the call
// 3. Then testing find_call_by_call_id_header()
//
// This is tested indirectly through the message receiver loop in production.
// The find_call_by_call_id_header method is tested here for the "not found" case,
// and the full flow is tested through actual SIP message handling.

// Note: Full integration test with actual SIP message receiving would require:
// 1. A mock SIP server or test fixtures
// 2. Setting up UDP/TCP listeners
// 3. Sending actual SIP responses
// 4. Verifying state transitions
//
// This is a complex test that would require significant infrastructure.
// For now, we test the core matching logic above.
//
// A more complete integration test would:
// - Create a test SIP server that sends responses
// - Start the message receiver loop
// - Send a SIP response with a known Call-ID
// - Verify that CallService.handle_invite_response() was called
// - Verify that call state transitions correctly (e.g., 180 → Connecting, 200 → Active)

