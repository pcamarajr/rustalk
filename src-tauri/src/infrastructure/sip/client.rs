// SIP client implementation
// Provides a high-level interface for sending and receiving SIP messages
// using the transport layer abstraction

use crate::domain::errors::SipError;
use crate::infrastructure::sip::parser::parse_message;
use crate::infrastructure::sip::transport::{
    SipTransport, TcpTransport, TlsTransport, UdpTransport,
};
use rsip::SipMessage;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Transport type enumeration
pub enum TransportType {
    Udp,
    Tcp,
    Tls,
}

/// SIP client that manages transport and message handling
pub struct SipClient {
    transport: Arc<Mutex<Box<dyn SipTransport>>>,
    local_address: SocketAddr,
    connected: bool,
}

impl SipClient {
    /// Create a new SIP client with UDP transport
    pub async fn new_udp(bind_addr: SocketAddr) -> Result<Self, SipError> {
        let transport = UdpTransport::bind(bind_addr).await?;
        let local_address = transport.local_address()?;

        Ok(Self {
            transport: Arc::new(Mutex::new(Box::new(transport))),
            local_address,
            connected: false, // UDP is connectionless
        })
    }

    /// Create a new SIP client with UDP transport on any available port
    pub async fn new_udp_any() -> Result<Self, SipError> {
        let transport = UdpTransport::bind_any().await?;
        let local_address = transport.local_address()?;

        Ok(Self {
            transport: Arc::new(Mutex::new(Box::new(transport))),
            local_address,
            connected: false,
        })
    }

    /// Create a new SIP client with TCP transport
    pub fn new_tcp() -> Self {
        let transport = TcpTransport::new();
        // TCP transport is not connected initially
        Self {
            transport: Arc::new(Mutex::new(Box::new(transport))),
            local_address: "0.0.0.0:0".parse().unwrap(), // Will be set on connect
            connected: false,
        }
    }

    /// Create a new SIP client with TLS transport
    pub fn new_tls() -> Self {
        let transport = TlsTransport::new();
        // TLS transport is not connected initially
        Self {
            transport: Arc::new(Mutex::new(Box::new(transport))),
            local_address: "0.0.0.0:0".parse().unwrap(), // Will be set on connect
            connected: false,
        }
    }

    /// Send a SIP message
    /// Converts the SipMessage to bytes and sends via transport
    pub async fn send_message(
        &self,
        message: &SipMessage,
        destination: &SocketAddr,
    ) -> Result<(), SipError> {
        // Convert SipMessage to bytes
        let message_bytes: Vec<u8> = message.clone().into();

        let transport = self.transport.lock().await;
        transport.send(&message_bytes, destination).await
    }

    /// Send raw SIP message bytes
    pub async fn send_bytes(
        &self,
        message_bytes: &[u8],
        destination: &SocketAddr,
    ) -> Result<(), SipError> {
        let transport = self.transport.lock().await;
        transport.send(message_bytes, destination).await
    }

    /// Receive a SIP message
    /// Receives bytes from transport and parses into SipMessage
    pub async fn receive_message(&mut self) -> Result<(SipMessage, SocketAddr), SipError> {
        let mut transport = self.transport.lock().await;
        let (bytes, source_addr) = transport.receive().await?;

        // Parse the received bytes
        let message = parse_message(&bytes)?;

        Ok((message, source_addr))
    }

    /// Establish a connection (for TCP/TLS transports)
    pub async fn connect(&mut self, address: &SocketAddr) -> Result<(), SipError> {
        let mut transport = self.transport.lock().await;
        transport.connect(address).await?;

        // Update local address and connection state
        self.local_address = transport.local_address()?;
        self.connected = true;

        Ok(())
    }

    /// Disconnect from the remote server (for TCP/TLS transports)
    /// Note: For UDP, this is a no-op
    pub async fn disconnect(&mut self) -> Result<(), SipError> {
        // For connection-oriented transports, we would close the connection here
        // However, since we're using the transport trait, we can't directly close
        // The connection will be closed when the transport is dropped
        self.connected = false;
        Ok(())
    }

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Get the local address this client is bound to
    pub fn local_address(&self) -> SocketAddr {
        self.local_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_udp_client_creation() {
        let client = SipClient::new_udp_any().await;
        assert!(client.is_ok(), "Should create UDP client");

        let client = client.unwrap();
        assert!(!client.is_connected(), "UDP client should not be connected");
    }

    #[tokio::test]
    async fn test_tcp_client_creation() {
        let client = SipClient::new_tcp();
        assert!(
            !client.is_connected(),
            "TCP client should not be connected initially"
        );
    }

    #[tokio::test]
    async fn test_tls_client_creation() {
        let client = SipClient::new_tls();
        assert!(
            !client.is_connected(),
            "TLS client should not be connected initially"
        );
    }
}
