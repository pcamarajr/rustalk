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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Udp,
    Tcp,
    Tls,
}

/// Internal enum to wrap transports for type-safe downcasting
enum TransportWrapper {
    Udp(Box<UdpTransport>),
    Tcp(Box<TcpTransport>),
    Tls(Box<TlsTransport>),
}

impl TransportWrapper {
    fn as_transport(&mut self) -> &mut dyn SipTransport {
        match self {
            TransportWrapper::Udp(t) => t.as_mut(),
            TransportWrapper::Tcp(t) => t.as_mut(),
            TransportWrapper::Tls(t) => t.as_mut(),
        }
    }

    fn as_tls(&mut self) -> Option<&mut TlsTransport> {
        match self {
            TransportWrapper::Tls(t) => Some(t.as_mut()),
            _ => None,
        }
    }
}

/// SIP client that manages transport and message handling
pub struct SipClient {
    transport: Arc<Mutex<TransportWrapper>>,
    transport_type: TransportType,
    local_address: SocketAddr,
    connected: bool,
}

impl SipClient {
    /// Create a new SIP client with UDP transport
    pub async fn new_udp(bind_addr: SocketAddr) -> Result<Self, SipError> {
        let transport = UdpTransport::bind(bind_addr).await?;
        let local_address = transport.local_address()?;

        Ok(Self {
            transport: Arc::new(Mutex::new(TransportWrapper::Udp(Box::new(transport)))),
            transport_type: TransportType::Udp,
            local_address,
            connected: false, // UDP is connectionless
        })
    }

    /// Create a new SIP client with UDP transport on any available port
    pub async fn new_udp_any() -> Result<Self, SipError> {
        let transport = UdpTransport::bind_any().await?;
        let local_address = transport.local_address()?;

        Ok(Self {
            transport: Arc::new(Mutex::new(TransportWrapper::Udp(Box::new(transport)))),
            transport_type: TransportType::Udp,
            local_address,
            connected: false,
        })
    }

    /// Create a new SIP client with TCP transport
    pub fn new_tcp() -> Self {
        let transport = TcpTransport::new();
        // TCP transport is not connected initially
        Self {
            transport: Arc::new(Mutex::new(TransportWrapper::Tcp(Box::new(transport)))),
            transport_type: TransportType::Tcp,
            local_address: "0.0.0.0:0"
                .parse()
                .expect("0.0.0.0:0 is a valid SocketAddr"),
            connected: false,
        }
    }

    /// Create a new SIP client with TLS transport
    pub fn new_tls() -> Self {
        let transport = TlsTransport::new();
        // TLS transport is not connected initially
        Self {
            transport: Arc::new(Mutex::new(TransportWrapper::Tls(Box::new(transport)))),
            transport_type: TransportType::Tls,
            local_address: "0.0.0.0:0"
                .parse()
                .expect("0.0.0.0:0 is a valid SocketAddr"),
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

        let mut transport = self.transport.lock().await;
        transport
            .as_transport()
            .send(&message_bytes, destination)
            .await
    }

    /// Send raw SIP message bytes
    pub async fn send_bytes(
        &self,
        message_bytes: &[u8],
        destination: &SocketAddr,
    ) -> Result<(), SipError> {
        let mut transport = self.transport.lock().await;
        transport
            .as_transport()
            .send(message_bytes, destination)
            .await
    }

    /// Receive a SIP message
    /// Receives bytes from transport and parses into SipMessage
    pub async fn receive_message(&mut self) -> Result<(SipMessage, SocketAddr), SipError> {
        let mut transport = self.transport.lock().await;
        let (bytes, source_addr) = transport.as_transport().receive().await?;

        // Parse the received bytes
        let message = parse_message(&bytes)?;

        Ok((message, source_addr))
    }

    /// Establish a connection (for TCP/TLS transports)
    pub async fn connect(&mut self, address: &SocketAddr) -> Result<(), SipError> {
        let mut transport = self.transport.lock().await;
        transport.as_transport().connect(address).await?;

        // Update local address and connection state
        self.local_address = transport.as_transport().local_address()?;
        self.connected = true;

        Ok(())
    }

    /// Establish a TLS connection with hostname for certificate validation
    ///
    /// This method should be used for TLS connections when a hostname is available.
    /// It enables proper TLS certificate validation against the hostname.
    ///
    /// # Arguments
    /// * `address` - The socket address (IP and port) to connect to
    /// * `hostname` - The hostname to use for TLS certificate validation
    ///
    /// # Errors
    /// Returns `SipError` if the connection fails or if the transport is not TLS.
    pub async fn connect_tls_with_hostname(
        &mut self,
        address: &SocketAddr,
        hostname: &str,
    ) -> Result<(), SipError> {
        if self.transport_type != TransportType::Tls {
            return Err(SipError::InvalidMessage {
                reason: "connect_tls_with_hostname can only be used with TLS transport".to_string(),
            });
        }

        let mut transport = self.transport.lock().await;
        if let Some(tls_transport) = transport.as_tls() {
            tls_transport
                .connect_with_hostname(address, hostname)
                .await?;

            // Update local address and connection state
            self.local_address = tls_transport.local_address()?;
            self.connected = true;

            Ok(())
        } else {
            Err(SipError::InvalidMessage {
                reason: "Transport is not TLS".to_string(),
            })
        }
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
