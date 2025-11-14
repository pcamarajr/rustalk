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
}
