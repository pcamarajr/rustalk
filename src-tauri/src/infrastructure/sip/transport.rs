// SIP transport layer abstraction
// Provides UDP, TCP, and TLS (SIPS) transport implementations using Tokio

use crate::domain::errors::SipError;
use crate::infrastructure::sip::tls::create_tls_config;
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio_util::codec::{Decoder, Encoder, Framed};

/// Trait for SIP transport implementations
/// All transport types must implement this trait to provide a unified interface
#[async_trait]
pub trait SipTransport: Send + Sync {
    /// Send a SIP message to the destination address
    async fn send(&self, message: &[u8], destination: &SocketAddr) -> Result<(), SipError>;

    /// Receive a SIP message from the network
    /// Returns the message bytes and the source address
    async fn receive(&mut self) -> Result<(Vec<u8>, SocketAddr), SipError>;

    /// Establish a connection to the destination (for connection-oriented transports)
    /// For UDP, this may be a no-op or just set the destination
    async fn connect(&mut self, address: &SocketAddr) -> Result<(), SipError>;

    /// Get the local address this transport is bound to
    fn local_address(&self) -> Result<SocketAddr, SipError>;
}

/// UDP transport implementation
/// Connectionless transport for SIP messages
pub struct UdpTransport {
    socket: UdpSocket,
    local_addr: SocketAddr,
}

impl UdpTransport {
    /// Create a new UDP transport bound to the specified address
    pub async fn bind(addr: SocketAddr) -> Result<Self, SipError> {
        let socket = UdpSocket::bind(addr)
            .await
            .map_err(|e| SipError::ConnectionError {
                message: format!("Failed to bind UDP socket to {}: {}", addr, e),
            })?;

        let local_addr = socket.local_addr().map_err(|e| SipError::TransportError {
            message: format!("Failed to get local address: {}", e),
        })?;

        Ok(Self { socket, local_addr })
    }

    /// Create a new UDP transport bound to any available port
    pub async fn bind_any() -> Result<Self, SipError> {
        Self::bind("0.0.0.0:0".parse().expect("0.0.0.0:0 is a valid SocketAddr")).await
    }
}

#[async_trait]
impl SipTransport for UdpTransport {
    async fn send(&self, message: &[u8], destination: &SocketAddr) -> Result<(), SipError> {
        self.socket
            .send_to(message, destination)
            .await
            .map_err(|e| SipError::TransportError {
                message: format!("Failed to send UDP message to {}: {}", destination, e),
            })?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<(Vec<u8>, SocketAddr), SipError> {
        let mut buf = vec![0u8; 65535]; // Maximum UDP packet size
        let (size, addr) =
            self.socket
                .recv_from(&mut buf)
                .await
                .map_err(|e| SipError::TransportError {
                    message: format!("Failed to receive UDP message: {}", e),
                })?;

        buf.truncate(size);
        Ok((buf, addr))
    }

    async fn connect(&mut self, _address: &SocketAddr) -> Result<(), SipError> {
        // UDP is connectionless, so connect is essentially a no-op
        // However, we can use connect() to set a default destination for send()
        self.socket
            .connect(_address)
            .await
            .map_err(|e| SipError::ConnectionError {
                message: format!("Failed to connect UDP socket to {}: {}", _address, e),
            })?;
        Ok(())
    }

    fn local_address(&self) -> Result<SocketAddr, SipError> {
        Ok(self.local_addr)
    }
}

/// Codec for framing SIP messages over TCP/TLS
/// SIP messages are delimited by `\r\n\r\n`
pub struct SipCodec {
    // Buffer for incomplete messages
    buffer: BytesMut,
}

/// Maximum buffer size to prevent DoS attacks from unbounded buffer growth
const MAX_BUFFER_SIZE: usize = 65535; // Maximum UDP packet size

impl SipCodec {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
        }
    }
}

impl Default for SipCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for SipCodec {
    type Item = Bytes;
    type Error = SipError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Append new data to buffer
        self.buffer.extend_from_slice(src);

        // Check buffer size to prevent DoS attacks from unbounded growth
        if self.buffer.len() > MAX_BUFFER_SIZE {
            return Err(SipError::TransportError {
                message: format!(
                    "SIP message exceeds maximum size of {} bytes",
                    MAX_BUFFER_SIZE
                ),
            });
        }

        // Look for SIP message delimiter: \r\n\r\n
        let delimiter = b"\r\n\r\n";
        if let Some(pos) = self
            .buffer
            .windows(delimiter.len())
            .position(|w| w == delimiter)
        {
            // Found complete message
            let message_end = pos + delimiter.len();
            let message = self.buffer.split_to(message_end).freeze();
            src.clear(); // Clear the source buffer since we've consumed it
            Ok(Some(message))
        } else {
            // Incomplete message, need more data
            Ok(None)
        }
    }
}

impl Encoder<Bytes> for SipCodec {
    type Error = SipError;

    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item);
        Ok(())
    }
}

/// TCP transport implementation
/// Connection-oriented transport for SIP messages
pub struct TcpTransport {
    framed: Option<Arc<Mutex<Framed<TcpStream, SipCodec>>>>,
    local_addr: Option<SocketAddr>,
    remote_addr: Option<SocketAddr>,
}

impl TcpTransport {
    /// Create a new TCP transport (not yet connected)
    pub fn new() -> Self {
        Self {
            framed: None,
            local_addr: None,
            remote_addr: None,
        }
    }
}

impl Default for TcpTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SipTransport for TcpTransport {
    async fn send(&self, message: &[u8], _destination: &SocketAddr) -> Result<(), SipError> {
        let framed = self
            .framed
            .as_ref()
            .ok_or_else(|| SipError::ConnectionError {
                message: "TCP transport not connected".to_string(),
            })?;

        // Ensure message ends with \r\n\r\n
        let mut message_bytes = message.to_vec();
        if !message_bytes.ends_with(b"\r\n\r\n") {
            message_bytes.extend_from_slice(b"\r\n\r\n");
        }

        let mut framed_guard = framed.lock().await;
        framed_guard
            .send(Bytes::from(message_bytes))
            .await
            .map_err(|e| SipError::TransportError {
                message: format!("Failed to send TCP message: {}", e),
            })?;

        Ok(())
    }

    async fn receive(&mut self) -> Result<(Vec<u8>, SocketAddr), SipError> {
        let framed = self
            .framed
            .as_ref()
            .ok_or_else(|| SipError::ConnectionError {
                message: "TCP transport not connected".to_string(),
            })?;

        let mut framed_guard = framed.lock().await;
        let message = framed_guard
            .next()
            .await
            .ok_or_else(|| SipError::TransportError {
                message: "TCP stream closed".to_string(),
            })?
            .map_err(|e| SipError::TransportError {
                message: format!("Failed to receive TCP message: {}", e),
            })?;

        let remote_addr = self.remote_addr.ok_or_else(|| SipError::TransportError {
            message: "Remote address not available".to_string(),
        })?;

        Ok((message.to_vec(), remote_addr))
    }

    async fn connect(&mut self, address: &SocketAddr) -> Result<(), SipError> {
        let stream = TcpStream::connect(address)
            .await
            .map_err(|e| SipError::ConnectionError {
                message: format!("Failed to connect TCP to {}: {}", address, e),
            })?;

        let local_addr = stream.local_addr().map_err(|e| SipError::TransportError {
            message: format!("Failed to get local TCP address: {}", e),
        })?;

        let remote_addr = stream.peer_addr().map_err(|e| SipError::TransportError {
            message: format!("Failed to get remote TCP address: {}", e),
        })?;

        let codec = SipCodec::new();
        let framed = Framed::new(stream, codec);

        self.framed = Some(Arc::new(Mutex::new(framed)));
        self.local_addr = Some(local_addr);
        self.remote_addr = Some(remote_addr);

        Ok(())
    }

    fn local_address(&self) -> Result<SocketAddr, SipError> {
        self.local_addr.ok_or_else(|| SipError::TransportError {
            message: "TCP transport not connected".to_string(),
        })
    }
}

/// Type alias for TLS framed stream to reduce complexity
type TlsFramed = Framed<tokio_rustls::client::TlsStream<TcpStream>, SipCodec>;

/// TLS transport implementation
/// Secure connection-oriented transport for SIP messages (SIPS)
pub struct TlsTransport {
    framed: Option<Arc<Mutex<TlsFramed>>>,
    local_addr: Option<SocketAddr>,
    remote_addr: Option<SocketAddr>,
}

impl TlsTransport {
    /// Create a new TLS transport (not yet connected)
    pub fn new() -> Self {
        Self {
            framed: None,
            local_addr: None,
            remote_addr: None,
        }
    }

    /// Establish a TLS connection using a hostname for certificate validation
    ///
    /// This method should be preferred over `connect()` when a hostname is available,
    /// as it enables proper TLS certificate validation against the hostname.
    ///
    /// # Arguments
    /// * `address` - The socket address (IP and port) to connect to
    /// * `hostname` - The hostname to use for TLS certificate validation
    ///
    /// # Errors
    /// Returns `SipError` if:
    /// - TCP connection fails
    /// - TLS handshake fails
    /// - Certificate validation fails
    /// - Hostname is invalid
    pub async fn connect_with_hostname(
        &mut self,
        address: &SocketAddr,
        hostname: &str,
    ) -> Result<(), SipError> {
        // Create TLS connector with proper certificate validation
        let config = create_tls_config();
        let connector = tokio_rustls::TlsConnector::from(Arc::new(config));

        // Connect TCP first
        let stream = TcpStream::connect(address)
            .await
            .map_err(|e| SipError::ConnectionError {
                message: format!("Failed to connect TCP to {}: {}", address, e),
            })?;

        let local_addr = stream.local_addr().map_err(|e| SipError::TransportError {
            message: format!("Failed to get local TCP address: {}", e),
        })?;

        // Perform TLS handshake with hostname for certificate validation
        let server_name = rustls::ServerName::try_from(hostname).map_err(|_| {
            SipError::TlsError {
                message: format!("Invalid server name for TLS: {}", hostname),
            }
        })?;

        let tls_stream = connector
            .connect(server_name, stream)
            .await
            .map_err(|e| SipError::TlsError {
                message: format!(
                    "TLS handshake failed for hostname '{}': {}",
                    hostname, e
                ),
            })?;

        let remote_addr = tls_stream
            .get_ref()
            .0
            .peer_addr()
            .map_err(|e| SipError::TransportError {
                message: format!("Failed to get remote TLS address: {}", e),
            })?;

        let codec = SipCodec::new();
        let framed = Framed::new(tls_stream, codec);

        self.framed = Some(Arc::new(Mutex::new(framed)));
        self.local_addr = Some(local_addr);
        self.remote_addr = Some(remote_addr);

        Ok(())
    }
}

impl Default for TlsTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SipTransport for TlsTransport {
    async fn send(&self, message: &[u8], _destination: &SocketAddr) -> Result<(), SipError> {
        let framed = self
            .framed
            .as_ref()
            .ok_or_else(|| SipError::ConnectionError {
                message: "TLS transport not connected".to_string(),
            })?;

        // Ensure message ends with \r\n\r\n
        let mut message_bytes = message.to_vec();
        if !message_bytes.ends_with(b"\r\n\r\n") {
            message_bytes.extend_from_slice(b"\r\n\r\n");
        }

        let mut framed_guard = framed.lock().await;
        framed_guard
            .send(Bytes::from(message_bytes))
            .await
            .map_err(|e| SipError::TransportError {
                message: format!("Failed to send TLS message: {}", e),
            })?;

        Ok(())
    }

    async fn receive(&mut self) -> Result<(Vec<u8>, SocketAddr), SipError> {
        let framed = self
            .framed
            .as_ref()
            .ok_or_else(|| SipError::ConnectionError {
                message: "TLS transport not connected".to_string(),
            })?;

        let mut framed_guard = framed.lock().await;
        let message = framed_guard
            .next()
            .await
            .ok_or_else(|| SipError::TransportError {
                message: "TLS stream closed".to_string(),
            })?
            .map_err(|e| SipError::TransportError {
                message: format!("Failed to receive TLS message: {}", e),
            })?;

        let remote_addr = self.remote_addr.ok_or_else(|| SipError::TransportError {
            message: "Remote address not available".to_string(),
        })?;

        Ok((message.to_vec(), remote_addr))
    }

    async fn connect(&mut self, address: &SocketAddr) -> Result<(), SipError> {
        // Use IP address as fallback (for backward compatibility)
        // Note: This may cause certificate validation issues.
        // Prefer using connect_with_hostname() when a hostname is available.
        let hostname = address.ip().to_string();
        self.connect_with_hostname(address, &hostname).await
    }

    fn local_address(&self) -> Result<SocketAddr, SipError> {
        self.local_addr.ok_or_else(|| SipError::TransportError {
            message: "TLS transport not connected".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_udp_transport_bind() {
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let transport = UdpTransport::bind(addr).await;
        assert!(transport.is_ok(), "Should bind UDP socket");

        let transport = transport.unwrap();
        let local_addr = transport.local_address();
        assert!(local_addr.is_ok(), "Should get local address");
    }

    #[tokio::test]
    async fn test_udp_transport_send_receive() {
        // Create two UDP transports for client and server simulation
        let server_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let server = UdpTransport::bind(server_addr).await.unwrap();
        let server_addr = server.local_address().unwrap();

        let client_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let client = UdpTransport::bind(client_addr).await.unwrap();

        // Send a test message
        let test_message = b"REGISTER sip:example.com SIP/2.0\r\nContent-Length: 0\r\n\r\n";
        client.send(test_message, &server_addr).await.unwrap();

        // Receive on server
        let mut server_mut = server;
        let (received, source) = server_mut.receive().await.unwrap();

        assert_eq!(received, test_message);
        assert_eq!(source, client.local_address().unwrap());
    }

    #[tokio::test]
    async fn test_tcp_transport_creation() {
        let transport = TcpTransport::new();
        assert!(
            transport.local_address().is_err(),
            "TCP should not be connected initially"
        );
    }

    #[tokio::test]
    async fn test_tls_transport_creation() {
        let transport = TlsTransport::new();
        assert!(
            transport.local_address().is_err(),
            "TLS should not be connected initially"
        );
    }

    #[tokio::test]
    async fn test_sip_codec_decode() {
        let mut codec = SipCodec::new();
        let mut buffer =
            BytesMut::from("REGISTER sip:example.com SIP/2.0\r\nContent-Length: 0\r\n\r\n");

        let result = codec.decode(&mut buffer);
        assert!(result.is_ok(), "Should decode complete message");
        assert!(
            result.unwrap().is_some(),
            "Should return Some for complete message"
        );
    }

    #[tokio::test]
    async fn test_sip_codec_decode_incomplete() {
        let mut codec = SipCodec::new();
        let mut buffer =
            BytesMut::from("REGISTER sip:example.com SIP/2.0\r\nContent-Length: 0\r\n");

        let result = codec.decode(&mut buffer);
        assert!(result.is_ok(), "Should handle incomplete message");
        assert!(
            result.unwrap().is_none(),
            "Should return None for incomplete message"
        );
    }
}
