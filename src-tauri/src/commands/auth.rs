// Authentication commands for SIP account registration

use crate::commands::validation::{
    validate_contact_uri, validate_hostname, validate_non_empty_string, validate_port_range,
    validate_protocol,
};
use crate::domain::entities::credentials::{Credentials, TransportProtocol};
use crate::domain::entities::registration::RegistrationState;
use crate::domain::errors::CommandError;
use crate::state::AppState;
use std::net::ToSocketAddrs;
use tauri::State;

/// Convert protocol string to TransportProtocol enum
fn parse_protocol(protocol: &str) -> Result<TransportProtocol, CommandError> {
    match protocol.to_lowercase().as_str() {
        "udp" => Ok(TransportProtocol::Udp),
        "tcp" => Ok(TransportProtocol::Tcp),
        "tls" => Ok(TransportProtocol::Tls),
        _ => Err(CommandError::ValidationError {
            field: "protocol".to_string(),
            message: format!(
                "Invalid protocol: {}. Must be 'udp', 'tcp', or 'tls'",
                protocol
            ),
        }),
    }
}

/// Resolve server address to SocketAddr
fn resolve_server_address(server: &str, port: u16) -> Result<std::net::SocketAddr, CommandError> {
    let addr_string = format!("{}:{}", server, port);
    addr_string
        .to_socket_addrs()
        .map_err(|e| CommandError::ServiceError {
            message: format!("Failed to resolve server address '{}': {}", addr_string, e),
        })?
        .next()
        .ok_or_else(|| CommandError::ServiceError {
            message: format!("No address found for server '{}'", server),
        })
}

/// Register a SIP account
///
/// # Arguments
/// * `server` - SIP server hostname or IP address
/// * `port` - SIP server port (1-65535)
/// * `protocol` - Transport protocol ("udp", "tcp", or "tls")
/// * `username` - SIP username
/// * `password` - SIP password
/// * `contact_uri` - Optional contact URI (e.g., "sip:user@192.168.1.100:5060")
/// * `expires` - Optional registration expiration time in seconds (default: 3600)
///
/// # Returns
/// * `Ok(String)` - Success message with registration status
/// * `Err(CommandError)` - Validation or service error
#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn register_account(
    server: String,
    port: u16,
    protocol: String,
    username: String,
    password: String,
    contact_uri: Option<String>,
    expires: Option<u32>,
    state: State<'_, AppState>,
) -> Result<String, CommandError> {
    // Validate all inputs
    validate_non_empty_string("server", &server)?;
    validate_hostname(&server)?;
    validate_port_range(port)?;
    validate_protocol(&protocol)?;
    validate_non_empty_string("username", &username)?;
    validate_non_empty_string("password", &password)?;
    validate_contact_uri(contact_uri.as_deref())?;

    // Convert protocol string to enum
    let transport_protocol = parse_protocol(&protocol)?;

    // Build credentials
    let credentials =
        Credentials::new(server.clone(), port, transport_protocol, username, password);

    // Resolve server address
    let server_addr = resolve_server_address(&server, port)?;

    // Generate contact URI if not provided
    let contact = contact_uri.unwrap_or_else(|| {
        // Default: use local address from SIP client
        // For now, use a simple format - in production, get actual local IP
        format!("sip:{}@{}:{}", credentials.username, server, port)
    });

    // Default expires to 3600 seconds if not provided
    let expires_seconds = expires.unwrap_or(3600);

    // Call auth service
    let mut auth_service = state.auth_service.lock().await;
    auth_service
        .register(credentials, server_addr, contact, expires_seconds)
        .await
        .map_err(CommandError::from)?;

    Ok("Registration initiated successfully".to_string())
}

/// Get current registration status
///
/// # Returns
/// * `Ok(String)` - Current registration state as string
/// * `Err(CommandError)` - Service error
#[tauri::command]
pub async fn get_registration_status(state: State<'_, AppState>) -> Result<String, CommandError> {
    let auth_service = state.auth_service.lock().await;
    let status = auth_service.get_registration_state().await;

    let status_string = match status {
        RegistrationState::Unregistered => "unregistered".to_string(),
        RegistrationState::Registering => "registering".to_string(),
        RegistrationState::Registered => "registered".to_string(),
        RegistrationState::Failed(error) => format!("failed: {}", error),
        RegistrationState::Expired => "expired".to_string(),
    };

    Ok(status_string)
}

/// Unregister a SIP account
///
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(CommandError)` - Service error
#[tauri::command]
pub async fn unregister_account(state: State<'_, AppState>) -> Result<String, CommandError> {
    let auth_service = state.auth_service.lock().await;
    auth_service
        .unregister()
        .await
        .map_err(CommandError::from)?;

    Ok("Account unregistered successfully".to_string())
}
