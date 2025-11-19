// End-to-end test for registration and audio selection flow
// Tests the complete flow: Login → Register → Settings → Select audio
//
// These tests require a test SIP server to be available.
// Configure via environment variables:
//   - SIP_SERVER_HOST: Hostname or IP of the SIP server (default: localhost)
//   - SIP_SERVER_PORT: SIP server port (default: 5060)
//   - SIP_TEST_USER: SIP username for testing (default: testuser)
//   - SIP_TEST_PASSWORD: SIP password for testing (default: testpass)
//   - SKIP_SIP_INTEGRATION_TESTS: Set to skip tests if no server available
//
// To run these tests:
//   cargo test --test registration_flow_test -- --ignored
//
// To skip E2E tests (if no server available):
//   SKIP_SIP_INTEGRATION_TESTS=true cargo test --test registration_flow_test

use rustalk_lib::domain::entities::credentials::{Credentials, TransportProtocol};
use rustalk_lib::domain::entities::registration::RegistrationState;
use rustalk_lib::domain::errors::CredentialStoreError;
use rustalk_lib::domain::traits::{AudioEngine, CredentialStore};
use rustalk_lib::infrastructure::sip::client::SipClient;
use rustalk_lib::services::AudioService;
use rustalk_lib::state::AppState;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

/// Check if E2E tests should be skipped
fn should_skip_e2e_tests() -> bool {
    std::env::var("SKIP_SIP_INTEGRATION_TESTS").is_ok()
}

/// Get SIP server hostname from environment or use default
fn get_sip_server_host() -> String {
    std::env::var("SIP_SERVER_HOST").unwrap_or_else(|_| "localhost".to_string())
}

/// Get SIP server port from environment or use default
fn get_sip_server_port() -> u16 {
    std::env::var("SIP_SERVER_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5060)
}

/// Resolve server address from hostname and port
fn resolve_server_address(host: &str, port: u16) -> Result<SocketAddr, String> {
    let server_ip = if host == "localhost" {
        "127.0.0.1"
    } else {
        host
    };
    format!("{}:{}", server_ip, port)
        .parse()
        .map_err(|e| format!("Failed to parse server address: {:?}", e))
}

/// Mock audio engine for testing
struct MockAudioEngine {
    input_devices: Arc<Mutex<Vec<rustalk_lib::domain::traits::audio_engine::AudioDevice>>>,
    output_devices: Arc<Mutex<Vec<rustalk_lib::domain::traits::audio_engine::AudioDevice>>>,
    current_input: Arc<Mutex<Option<String>>>,
    current_output: Arc<Mutex<Option<String>>>,
}

impl MockAudioEngine {
    fn new() -> Self {
        let input_devices = vec![
            rustalk_lib::domain::traits::audio_engine::AudioDevice::new(
                "input-1".to_string(),
                "Built-in Microphone".to_string(),
                true,
            ),
            rustalk_lib::domain::traits::audio_engine::AudioDevice::new(
                "input-2".to_string(),
                "USB Microphone".to_string(),
                true,
            ),
        ];
        let output_devices = vec![
            rustalk_lib::domain::traits::audio_engine::AudioDevice::new(
                "output-1".to_string(),
                "Built-in Speakers".to_string(),
                false,
            ),
            rustalk_lib::domain::traits::audio_engine::AudioDevice::new(
                "output-2".to_string(),
                "USB Headphones".to_string(),
                false,
            ),
        ];

        Self {
            input_devices: Arc::new(Mutex::new(input_devices)),
            output_devices: Arc::new(Mutex::new(output_devices)),
            current_input: Arc::new(Mutex::new(None)),
            current_output: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl AudioEngine for MockAudioEngine {
    async fn enumerate_input_devices(
        &self,
    ) -> Result<
        Vec<rustalk_lib::domain::traits::audio_engine::AudioDevice>,
        rustalk_lib::domain::errors::AudioEngineError,
    > {
        let devices = self.input_devices.lock().await;
        Ok(devices.clone())
    }

    async fn enumerate_output_devices(
        &self,
    ) -> Result<
        Vec<rustalk_lib::domain::traits::audio_engine::AudioDevice>,
        rustalk_lib::domain::errors::AudioEngineError,
    > {
        let devices = self.output_devices.lock().await;
        Ok(devices.clone())
    }

    async fn get_input_device(
        &self,
    ) -> Result<
        Option<rustalk_lib::domain::traits::audio_engine::AudioDevice>,
        rustalk_lib::domain::errors::AudioEngineError,
    > {
        let current_id = self.current_input.lock().await.clone();
        if let Some(id) = current_id {
            let devices = self.input_devices.lock().await;
            Ok(devices.iter().find(|d| d.id == id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn get_output_device(
        &self,
    ) -> Result<
        Option<rustalk_lib::domain::traits::audio_engine::AudioDevice>,
        rustalk_lib::domain::errors::AudioEngineError,
    > {
        let current_id = self.current_output.lock().await.clone();
        if let Some(id) = current_id {
            let devices = self.output_devices.lock().await;
            Ok(devices.iter().find(|d| d.id == id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn set_input_device(
        &self,
        device_id: &str,
    ) -> Result<(), rustalk_lib::domain::errors::AudioEngineError> {
        let devices = self.input_devices.lock().await;
        if !devices.iter().any(|d| d.id == device_id) {
            return Err(
                rustalk_lib::domain::errors::AudioEngineError::DeviceNotFound {
                    device_id: device_id.to_string(),
                },
            );
        }
        drop(devices);
        let mut current = self.current_input.lock().await;
        *current = Some(device_id.to_string());
        Ok(())
    }

    async fn set_output_device(
        &self,
        device_id: &str,
    ) -> Result<(), rustalk_lib::domain::errors::AudioEngineError> {
        let devices = self.output_devices.lock().await;
        if !devices.iter().any(|d| d.id == device_id) {
            return Err(
                rustalk_lib::domain::errors::AudioEngineError::DeviceNotFound {
                    device_id: device_id.to_string(),
                },
            );
        }
        drop(devices);
        let mut current = self.current_output.lock().await;
        *current = Some(device_id.to_string());
        Ok(())
    }

    async fn start_input_stream(
        &self,
    ) -> Result<String, rustalk_lib::domain::errors::AudioEngineError> {
        Ok("stream-1".to_string())
    }

    async fn start_output_stream(
        &self,
    ) -> Result<String, rustalk_lib::domain::errors::AudioEngineError> {
        Ok("stream-1".to_string())
    }

    async fn stop_stream(
        &self,
        _handle: &String,
    ) -> Result<(), rustalk_lib::domain::errors::AudioEngineError> {
        Ok(())
    }

    async fn mute_input(&self) -> Result<(), rustalk_lib::domain::errors::AudioEngineError> {
        Ok(())
    }

    async fn unmute_input(&self) -> Result<(), rustalk_lib::domain::errors::AudioEngineError> {
        Ok(())
    }

    async fn is_input_muted(&self) -> Result<bool, rustalk_lib::domain::errors::AudioEngineError> {
        Ok(false)
    }

    async fn get_input_level(&self) -> Result<f32, rustalk_lib::domain::errors::AudioEngineError> {
        Ok(0.0)
    }

    async fn get_output_level(&self) -> Result<f32, rustalk_lib::domain::errors::AudioEngineError> {
        Ok(0.0)
    }
}

/// Mock credential store for testing
struct MockCredentialStore {
    storage: Arc<Mutex<HashMap<String, Credentials>>>,
}

impl MockCredentialStore {
    fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl CredentialStore for MockCredentialStore {
    async fn save(&self, key: &str, credentials: &Credentials) -> Result<(), CredentialStoreError> {
        let mut storage = self.storage.lock().await;
        storage.insert(key.to_string(), credentials.clone());
        Ok(())
    }

    async fn load(&self, key: &str) -> Result<Option<Credentials>, CredentialStoreError> {
        let storage = self.storage.lock().await;
        Ok(storage.get(key).cloned())
    }

    async fn delete(&self, key: &str) -> Result<(), CredentialStoreError> {
        let mut storage = self.storage.lock().await;
        storage.remove(key);
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, CredentialStoreError> {
        let storage = self.storage.lock().await;
        Ok(storage.contains_key(key))
    }
}

/// Create test AppState with real SipClient and mock AudioEngine and CredentialStore
async fn create_test_app_state() -> AppState {
    // Create real UDP SIP client for AuthService
    let client = SipClient::new_udp_any()
        .await
        .expect("Should create UDP client");

    // Create real UDP SIP client for CallService
    let call_client = SipClient::new_udp_any()
        .await
        .expect("Should create UDP client for calls");

    // Create mock audio engine
    let mock_audio_engine: Arc<dyn AudioEngine> = Arc::new(MockAudioEngine::new());
    let audio_service = AudioService::new(mock_audio_engine);

    // Create mock credential store
    let credential_store: Arc<dyn CredentialStore> = Arc::new(MockCredentialStore::new());

    // Create a mock event emitter for tests using tauri::test::mock_app()
    use rustalk_lib::commands::events::EventEmitter;
    let app = tauri::test::mock_app();
    let event_emitter = EventEmitter::new(app.handle().clone());
    AppState::new(client, call_client, audio_service, credential_store, event_emitter)
}

#[tokio::test]
#[ignore] // Ignore by default - run with: cargo test --test registration_flow_test -- --ignored
async fn test_registration_and_audio_flow() {
    if should_skip_e2e_tests() {
        println!("DEBUG:[E2E] Skipping E2E test (SKIP_SIP_INTEGRATION_TESTS set)");
        return;
    }

    let server_host = get_sip_server_host();
    let server_port = get_sip_server_port();
    let username = std::env::var("SIP_TEST_USER").unwrap_or_else(|_| "testuser".to_string());
    let password = std::env::var("SIP_TEST_PASSWORD").unwrap_or_else(|_| "testpass".to_string());

    println!(
        "DEBUG:[E2E] Starting E2E test with server={}:{}, username={}",
        server_host, server_port, username
    );

    // Step 1: Initial State Verification
    println!("DEBUG:[E2E] Step 1: Verifying initial state");
    let app_state = create_test_app_state().await;

    // Verify registration status is "unregistered"
    let auth_service = app_state.auth_service.lock().await;
    let initial_state = auth_service.get_registration_state().await;
    assert!(
        matches!(initial_state, RegistrationState::Unregistered),
        "Expected initial status to be Unregistered, got: {:?}",
        initial_state
    );
    println!(
        "DEBUG:[E2E] Initial registration status: {:?}",
        initial_state
    );
    drop(auth_service);

    // Verify no saved credentials exist
    let saved_creds = app_state
        .credential_store
        .load("default_account")
        .await
        .unwrap();
    assert!(
        saved_creds.is_none(),
        "Expected no saved credentials initially, got: {:?}",
        saved_creds
    );
    println!("DEBUG:[E2E] No saved credentials (as expected)");

    // Verify no audio devices are selected
    let audio_service = app_state.audio_service.lock().await;
    let input_device = audio_service.get_input_device().await.unwrap();
    assert!(
        input_device.is_none(),
        "Expected no input device selected initially"
    );
    let output_device = audio_service.get_output_device().await.unwrap();
    assert!(
        output_device.is_none(),
        "Expected no output device selected initially"
    );
    println!("DEBUG:[E2E] No audio devices selected (as expected)");
    drop(audio_service);

    // Step 2: Register Account
    println!("DEBUG:[E2E] Step 2: Registering account");
    let server_addr =
        resolve_server_address(&server_host, server_port).expect("Should parse server address");
    let contact_uri = format!("sip:{}@{}:{}", username, server_host, server_port);

    let credentials = Credentials::new(
        server_host.clone(),
        server_port,
        TransportProtocol::Udp,
        username.clone(),
        password.clone(),
    );

    let mut auth_service = app_state.auth_service.lock().await;
    let register_result = timeout(
        Duration::from_secs(10),
        auth_service.register(credentials.clone(), server_addr, contact_uri, 3600),
    )
    .await;

    match register_result {
        Ok(Ok(_)) => {
            println!("DEBUG:[E2E] Registration initiated successfully");
        }
        Ok(Err(e)) => {
            panic!("Registration failed: {:?}", e);
        }
        Err(_) => {
            panic!("Timeout initiating registration");
        }
    }
    drop(auth_service);

    // Wait for registration to complete
    println!("DEBUG:[E2E] Waiting for registration to complete...");
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let poll_interval = Duration::from_millis(500);

    loop {
        if tokio::time::Instant::now() >= deadline {
            let auth_service = app_state.auth_service.lock().await;
            let current_state = auth_service.get_registration_state().await;
            panic!(
                "Timeout waiting for registered status. Current state: {:?}",
                current_state
            );
        }

        let auth_service = app_state.auth_service.lock().await;
        let state = auth_service.get_registration_state().await;
        drop(auth_service);

        if matches!(state, RegistrationState::Registered) {
            println!("DEBUG:[E2E] Registration completed successfully");
            break;
        } else if matches!(state, RegistrationState::Failed(_)) {
            panic!("Registration failed with state: {:?}", state);
        }

        tokio::time::sleep(poll_interval).await;
    }

    // Step 3: Verify Credential Persistence
    println!("DEBUG:[E2E] Step 3: Verifying credential persistence");
    // Poll for credentials with timeout (credentials are saved via tokio::spawn which is fire-and-forget)
    let credential_key = format!("{}@{}", username, server_host);
    let credential_store = &app_state.credential_store;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    let poll_interval = Duration::from_millis(100);

    loop {
        if tokio::time::Instant::now() >= deadline {
            panic!("Timeout waiting for credentials to be saved");
        }

        // Check default_account pointer first
        let default_account = credential_store.load("default_account").await.unwrap();
        if default_account.is_some() {
            // Load actual credentials
            let loaded_creds = credential_store.load(&credential_key).await.unwrap();
            if loaded_creds.is_some() {
                break;
            }
        }

        tokio::time::sleep(poll_interval).await;
    }

    // Verify credentials were saved
    let default_account = credential_store.load("default_account").await.unwrap();
    assert!(
        default_account.is_some(),
        "Expected default_account pointer to exist"
    );

    let loaded_creds = credential_store.load(&credential_key).await.unwrap();
    assert!(
        loaded_creds.is_some(),
        "Expected saved credentials after registration"
    );

    let saved_creds = loaded_creds.unwrap();
    assert_eq!(
        saved_creds.username, username,
        "Saved username should match"
    );
    assert_eq!(saved_creds.server, server_host, "Saved server should match");
    assert_eq!(saved_creds.port, server_port, "Saved port should match");
    println!("DEBUG:[E2E] Credentials persisted correctly");

    // Verify credentials were saved to credential store (check mock store directly)
    let exists = credential_store.exists(&credential_key).await.unwrap();
    assert!(exists, "Credentials should exist in credential store");
    println!("DEBUG:[E2E] Credentials verified in credential store");

    // Step 4: List Audio Devices
    println!("DEBUG:[E2E] Step 4: Listing audio devices");
    let audio_service = app_state.audio_service.lock().await;
    let input_devices =
        match timeout(Duration::from_secs(3), audio_service.list_input_devices()).await {
            Ok(Ok(devices)) => devices,
            Ok(Err(e)) => panic!("Failed to list input devices: {:?}", e),
            Err(_) => panic!("Timeout waiting for input device list"),
        };
    assert!(
        !input_devices.is_empty(),
        "Expected at least one input device"
    );
    println!("DEBUG:[E2E] Found {} input device(s)", input_devices.len());

    let output_devices =
        match timeout(Duration::from_secs(3), audio_service.list_output_devices()).await {
            Ok(Ok(devices)) => devices,
            Ok(Err(e)) => panic!("Failed to list output devices: {:?}", e),
            Err(_) => panic!("Timeout waiting for output device list"),
        };
    assert!(
        !output_devices.is_empty(),
        "Expected at least one output device"
    );
    println!(
        "DEBUG:[E2E] Found {} output device(s)",
        output_devices.len()
    );

    // Step 5: Select Audio Devices
    println!("DEBUG:[E2E] Step 5: Selecting audio devices");
    let first_input_id = input_devices[0].id.clone();
    let first_output_id = output_devices[0].id.clone();

    let set_input_result = match timeout(
        Duration::from_secs(3),
        audio_service.set_input_device(&first_input_id),
    )
    .await
    {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => panic!("Timeout waiting for input device to be set"),
    };
    assert!(set_input_result.is_ok(), "Should set input device");
    println!("DEBUG:[E2E] Input device set successfully");

    let set_output_result = match timeout(
        Duration::from_secs(3),
        audio_service.set_output_device(&first_output_id),
    )
    .await
    {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => panic!("Timeout waiting for output device to be set"),
    };
    assert!(set_output_result.is_ok(), "Should set output device");
    println!("DEBUG:[E2E] Output device set successfully");

    // Step 6: Verify Audio Device Persistence
    println!("DEBUG:[E2E] Step 6: Verifying audio device persistence");
    let selected_input =
        match timeout(Duration::from_secs(3), audio_service.get_input_device()).await {
            Ok(Ok(device)) => device,
            Ok(Err(e)) => panic!("Failed to get input device: {:?}", e),
            Err(_) => panic!("Timeout waiting for input device"),
        };
    assert!(
        selected_input.is_some(),
        "Expected input device to be selected"
    );
    assert_eq!(
        selected_input.unwrap().id,
        first_input_id,
        "Selected input device should match"
    );
    println!("DEBUG:[E2E] Input device persistence verified");

    let selected_output =
        match timeout(Duration::from_secs(3), audio_service.get_output_device()).await {
            Ok(Ok(device)) => device,
            Ok(Err(e)) => panic!("Failed to get output device: {:?}", e),
            Err(_) => panic!("Timeout waiting for output device"),
        };
    assert!(
        selected_output.is_some(),
        "Expected output device to be selected"
    );
    assert_eq!(
        selected_output.unwrap().id,
        first_output_id,
        "Selected output device should match"
    );
    println!("DEBUG:[E2E] Output device persistence verified");
    drop(audio_service);

    println!("DEBUG:[E2E] E2E test completed successfully!");
}
