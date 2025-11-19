// Call commands for initiating outbound calls

use crate::commands::validation::validate_phone_number;
use crate::domain::entities::registration::RegistrationState;
use crate::domain::errors::CommandError;
use crate::state::AppState;
use rand::Rng;
use tauri::State;

/// Generate an even RTP port number in the range 49152-65534
///
/// RTP ports must be even numbers. This function generates a random
/// even port in the standard RTP dynamic port range.
///
/// # Returns
/// An even port number between 49152 and 65534 (inclusive)
fn generate_rtp_port() -> u16 {
    let mut rng = rand::thread_rng();
    // Generate a random number in range 49152-65534, then make it even
    let port = rng.gen_range(49152..=65534);
    // Ensure it's even (if odd, subtract 1)
    if port % 2 == 0 {
        port
    } else {
        port - 1
    }
}

/// Initiate an outbound call
///
/// This command:
/// 1. Validates the phone number
/// 2. Checks that registration state is Registered
/// 3. Gets registration info (credentials, server address, contact URI, local address)
/// 4. Generates an RTP port for audio
/// 5. Calls CallService to initiate the outbound call
///
/// # Arguments
/// * `number` - Phone number or SIP URI to call
///
/// # Returns
/// * `Ok(String)` - Call ID if call was initiated successfully
/// * `Err(CommandError)` - Validation or service error
#[tauri::command]
pub async fn initiate_call(
    number: String,
    state: State<'_, AppState>,
) -> Result<String, CommandError> {
    eprintln!("DEBUG:[INITIATE_CALL] Initiating call to: {}", number);

    // Validate phone number
    validate_phone_number(&number)?;

    // Get registration state from auth_service
    let registration_state = {
        let auth_service = state.auth_service.lock().await;
        auth_service.get_registration_state().await
    };

    // Check that we're registered
    if !matches!(registration_state, RegistrationState::Registered) {
        eprintln!(
            "DEBUG:[INITIATE_CALL] Registration not registered, state: {:?}",
            registration_state
        );
        return Err(CommandError::ServiceError {
            message: format!(
                "Cannot initiate call: registration state is {:?}",
                registration_state
            ),
        });
    }

    eprintln!("DEBUG:[INITIATE_CALL] Registration validated, getting registration info");

    // Get registration info from auth_service
    let (credentials, server_addr, contact_uri, local_address) = {
        let auth_service = state.auth_service.lock().await;

        // Get credentials
        let creds =
            auth_service
                .get_credentials()
                .await
                .ok_or_else(|| CommandError::ServiceError {
                    message: "No credentials available (not registered)".to_string(),
                })?;

        // Get server address
        let server =
            auth_service
                .get_server_address()
                .ok_or_else(|| CommandError::ServiceError {
                    message: "No server address available (not registered)".to_string(),
                })?;

        // Get contact URI
        let contact = auth_service
            .get_contact_uri()
            .ok_or_else(|| CommandError::ServiceError {
                message: "No contact URI available (not registered)".to_string(),
            })?;

        // Get local address from SIP client
        let local_addr =
            auth_service
                .get_local_address()
                .await
                .map_err(|e| CommandError::ServiceError {
                    message: format!("Failed to get local address: {}", e),
                })?;

        (creds, server, contact, local_addr)
    };

    eprintln!(
        "DEBUG:[INITIATE_CALL] Registration info retrieved: server={}, contact={}, local={}",
        server_addr, contact_uri, local_address
    );

    // Construct local URI from credentials (sip:username@server)
    let local_uri = format!("sip:{}@{}", credentials.username, credentials.server);

    // Generate RTP port (even number in 49152-65534 range)
    let rtp_port = generate_rtp_port();
    eprintln!("DEBUG:[INITIATE_CALL] Generated RTP port: {}", rtp_port);

    // Call CallService to initiate the outbound call
    let call_id = {
        let call_service = state.call_service.lock().await;
        call_service
            .initiate_outbound_call(
                number.clone(),
                local_address,
                contact_uri,
                local_uri,
                rtp_port,
                credentials.username.clone(),
            )
            .await
            .map_err(|e| {
                eprintln!("DEBUG:[INITIATE_CALL] CallService error: {}", e);
                CommandError::ServiceError {
                    message: format!("Failed to initiate call: {}", e),
                }
            })?
    };

    eprintln!(
        "DEBUG:[INITIATE_CALL] Call initiated successfully, CallId: {}",
        call_id.as_str()
    );

    // Return CallId as string
    Ok(call_id.as_str().to_string())
}
