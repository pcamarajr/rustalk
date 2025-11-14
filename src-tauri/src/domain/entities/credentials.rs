// Credentials value object for SIP account credentials

use serde::{Deserialize, Serialize};

/// SIP account credentials
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credentials {
    /// SIP server hostname or IP address
    pub server: String,
    /// SIP server port
    pub port: u16,
    /// Transport protocol (UDP, TCP, or TLS)
    pub protocol: TransportProtocol,
    /// SIP username
    pub username: String,
    /// SIP password (sensitive, should be stored securely)
    pub password: String,
}

/// Transport protocol for SIP communication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportProtocol {
    /// UDP transport
    Udp,
    /// TCP transport
    Tcp,
    /// TLS transport (SIPS)
    Tls,
}

impl Credentials {
    /// Create new credentials
    pub fn new(
        server: String,
        port: u16,
        protocol: TransportProtocol,
        username: String,
        password: String,
    ) -> Self {
        Self {
            server,
            port,
            protocol,
            username,
            password,
        }
    }

    /// Get the full server address (server:port)
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server, self.port)
    }

    /// Validate credentials fields
    ///
    /// Returns `Ok(())` if all fields are valid, or `Err(String)` with a validation error message.
    ///
    /// Validates:
    /// - Server: not empty and contains valid characters (hostname or IP address format)
    /// - Port: valid range (1-65535)
    /// - Username: not empty
    /// - Password: not empty
    pub fn validate(&self) -> Result<(), String> {
        // Validate server
        if self.server.is_empty() {
            return Err("Server cannot be empty".to_string());
        }
        // Basic format check: server should not contain spaces or invalid characters
        if self.server.trim() != self.server {
            return Err("Server cannot have leading or trailing whitespace".to_string());
        }
        if self.server.contains(' ') {
            return Err("Server cannot contain spaces".to_string());
        }

        // Validate port
        if self.port == 0 {
            return Err("Port must be between 1 and 65535".to_string());
        }

        // Validate username
        if self.username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        // Validate password
        if self.password.is_empty() {
            return Err("Password cannot be empty".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_new() {
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        assert_eq!(creds.server, "sip.example.com");
        assert_eq!(creds.port, 5060);
        assert_eq!(creds.protocol, TransportProtocol::Udp);
        assert_eq!(creds.username, "user1");
        assert_eq!(creds.password, "password123");
    }

    #[test]
    fn test_server_address() {
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Tls,
            "user1".to_string(),
            "password123".to_string(),
        );

        assert_eq!(creds.server_address(), "sip.example.com:5060");
    }

    #[test]
    fn test_credentials_serialization() {
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        let json = serde_json::to_string(&creds).unwrap();
        let deserialized: Credentials = serde_json::from_str(&json).unwrap();

        assert_eq!(creds, deserialized);
    }

    #[test]
    fn test_validate_success() {
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        assert!(creds.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_server() {
        let creds = Credentials::new(
            "".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        let result = creds.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Server cannot be empty"));
    }

    #[test]
    fn test_validate_server_with_whitespace() {
        let creds = Credentials::new(
            " sip.example.com ".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        let result = creds.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("leading or trailing whitespace"));
    }

    #[test]
    fn test_validate_server_with_spaces() {
        let creds = Credentials::new(
            "sip.example.com server".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        let result = creds.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot contain spaces"));
    }

    #[test]
    fn test_validate_invalid_port() {
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            0,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );

        let result = creds.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Port must be between 1 and 65535"));
    }

    #[test]
    fn test_validate_empty_username() {
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "".to_string(),
            "password123".to_string(),
        );

        let result = creds.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Username cannot be empty"));
    }

    #[test]
    fn test_validate_empty_password() {
        let creds = Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "".to_string(),
        );

        let result = creds.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Password cannot be empty"));
    }

    #[test]
    fn test_validate_valid_port_range() {
        let creds_min = Credentials::new(
            "sip.example.com".to_string(),
            1,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );
        assert!(creds_min.validate().is_ok());

        let creds_max = Credentials::new(
            "sip.example.com".to_string(),
            65535,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        );
        assert!(creds_max.validate().is_ok());
    }
}
