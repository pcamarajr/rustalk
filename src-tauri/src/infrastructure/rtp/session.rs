// RTP session management
// Handles RTP packet transmission and reception for audio streaming

use crate::domain::errors::RtpError;
use crate::infrastructure::rtp::codec::{Codec, G711Codec, G711Type};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

/// RTP session configuration
#[derive(Debug, Clone)]
pub struct RtpSessionConfig {
    /// Local RTP port (must be even)
    pub local_port: u16,
    /// Remote address and port for RTP
    pub remote_addr: SocketAddr,
    /// Codec type (PCMU or PCMA)
    pub codec_type: G711Type,
    /// SSRC (Synchronization Source) - randomly generated if None
    pub ssrc: Option<u32>,
}

/// RTP session for bidirectional audio streaming
pub struct RtpSession {
    /// Configuration
    config: RtpSessionConfig,
    /// UDP socket for RTP
    socket: Option<Arc<UdpSocket>>,
    /// Codec for encoding/decoding
    codec: G711Codec,
    /// SSRC for this session
    ssrc: u32,
    /// Current sequence number
    sequence_number: u16,
    /// Current timestamp (in samples, 8000 samples per second)
    timestamp: u32,
    /// Send task handle
    send_handle: Option<JoinHandle<Result<(), RtpError>>>,
    /// Receive task handle
    receive_handle: Option<JoinHandle<Result<(), RtpError>>>,
    /// Channel for sending audio to RTP encoder
    audio_tx: Option<mpsc::Sender<Vec<i16>>>,
    /// Channel for receiving decoded audio from RTP
    audio_rx: Option<mpsc::Receiver<Vec<i16>>>,
    /// Stop signal (broadcast channel so both tasks can receive it)
    stop_tx: Option<broadcast::Sender<()>>,
}

impl RtpSession {
    /// Create a new RTP session
    pub fn new(config: RtpSessionConfig) -> Self {
        // Generate SSRC if not provided
        let ssrc = config.ssrc.unwrap_or_else(|| {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .hash(&mut hasher);
            hasher.finish() as u32
        });

        let codec = match config.codec_type {
            G711Type::Pcmu => G711Codec::pcmu(),
            G711Type::Pcma => G711Codec::pcma(),
        };

        Self {
            config,
            socket: None,
            codec,
            ssrc,
            sequence_number: 0,
            timestamp: 0,
            send_handle: None,
            receive_handle: None,
            audio_tx: None,
            audio_rx: None,
            stop_tx: None,
        }
    }

    /// Start the RTP session
    /// Creates UDP socket and starts send/receive tasks
    pub async fn start(
        &mut self,
    ) -> Result<(mpsc::Sender<Vec<i16>>, mpsc::Receiver<Vec<i16>>), RtpError> {
        if self.socket.is_some() {
            return Err(RtpError::SessionAlreadyStarted);
        }

        eprintln!(
            "DEBUG:[RTP/START] Starting RTP session on port {}",
            self.config.local_port
        );

        // Validate RTP port is even
        if !self.config.local_port.is_multiple_of(2) {
            return Err(RtpError::InvalidConfiguration {
                message: format!("RTP port must be even, got {}", self.config.local_port),
            });
        }

        // Bind UDP socket
        let bind_addr = format!("0.0.0.0:{}", self.config.local_port);
        let socket = UdpSocket::bind(&bind_addr)
            .await
            .map_err(|e| RtpError::SocketBindFailed {
                message: format!("Failed to bind to {}: {}", bind_addr, e),
            })?;

        eprintln!("DEBUG:[RTP/START] Socket bound to {}", bind_addr);

        let socket = Arc::new(socket);
        self.socket = Some(socket.clone());

        // Create channels for audio data
        let (audio_tx, mut audio_rx_in) = mpsc::channel::<Vec<i16>>(100);
        let (audio_tx_out, audio_rx) = mpsc::channel::<Vec<i16>>(100);
        let (stop_tx, _) = broadcast::channel::<()>(1);

        self.audio_tx = Some(audio_tx.clone());
        self.audio_rx = Some(audio_rx);
        self.stop_tx = Some(stop_tx.clone());

        // Start send task
        let socket_send = socket.clone();
        let codec_send = self.codec.clone();
        let remote_addr = self.config.remote_addr;
        let mut sequence = self.sequence_number;
        let mut timestamp = self.timestamp;
        let ssrc = self.ssrc;
        let mut stop_rx_send = stop_tx.subscribe();

        let send_handle = tokio::spawn(async move {
            // 20ms interval is standard for G.711 (PCMU/PCMA) codecs
            // This provides 50 packets/sec, which balances:
            // - Low latency (20ms is acceptable for real-time audio)
            // - Reasonable overhead (RTP header is 12 bytes, payload is 160 bytes for 20ms of G.711)
            // - Compatibility with most SIP endpoints
            // For future codecs (e.g., Opus), this may need to be configurable
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(20)); // 20ms = 50 packets/sec
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = stop_rx_send.recv() => {
                        eprintln!("DEBUG:[RTP/SEND] Stop signal received");
                        break;
                    }
                    _ = interval.tick() => {
                        // Try to get audio data (non-blocking)
                        match audio_rx_in.try_recv() {
                            Ok(samples) => {
                            // Encode audio
                            let encoded = codec_send.encode(&samples).map_err(|e| RtpError::CodecError {
                                message: format!("Encode failed: {}", e),
                            })?;

                            // Build RTP packet
                            let packet = build_rtp_packet(
                                codec_send.payload_type(),
                                sequence,
                                timestamp,
                                ssrc,
                                &encoded,
                            );

                            // Send packet
                            if let Err(e) = socket_send.send_to(&packet, &remote_addr).await {
                                eprintln!("DEBUG:[RTP/SEND] Send error: {}", e);
                                return Err(RtpError::SendFailed {
                                    message: format!("Failed to send RTP packet: {}", e),
                                });
                            }

                                // Update sequence and timestamp
                                sequence = sequence.wrapping_add(1);
                                timestamp = timestamp.wrapping_add(samples.len() as u32);
                            }
                            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                                // No data available, continue
                            }
                            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                                eprintln!("DEBUG:[RTP/SEND] Audio channel disconnected");
                                break;
                            }
                        }
                    }
                }
            }

            Ok(())
        });

        // Start receive task
        let socket_recv = socket.clone();
        let codec_recv = self.codec.clone();
        let mut stop_rx_recv = stop_tx.subscribe();

        let receive_handle = tokio::spawn(async move {
            let mut buf = [0u8; 1500]; // Standard MTU size

            loop {
                tokio::select! {
                    _ = stop_rx_recv.recv() => {
                        eprintln!("DEBUG:[RTP/RECEIVE] Stop signal received");
                        break;
                    }
                    result = socket_recv.recv_from(&mut buf) => {
                        match result {
                            Ok((len, _addr)) => {
                                // Parse RTP packet
                                if let Ok(payload) = parse_rtp_packet(&buf[..len]) {
                                    // Decode audio
                                    match codec_recv.decode(&payload) {
                                        Ok(samples) => {
                                            // Send to audio output
                                            if audio_tx_out.send(samples).await.is_err() {
                                                eprintln!("DEBUG:[RTP/RECEIVE] Audio channel closed");
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("DEBUG:[RTP/RECEIVE] Decode error: {}", e);
                                        }
                                    }
                                } else {
                                    eprintln!("DEBUG:[RTP/RECEIVE] Invalid RTP packet");
                                }
                            }
                            Err(e) => {
                                eprintln!("DEBUG:[RTP/RECEIVE] Receive error: {}", e);
                                return Err(RtpError::ReceiveFailed {
                                    message: format!("Failed to receive RTP packet: {}", e),
                                });
                            }
                        }
                    }
                }
            }

            Ok(())
        });

        self.send_handle = Some(send_handle);
        self.receive_handle = Some(receive_handle);

        eprintln!("DEBUG:[RTP/START] RTP session started successfully");

        Ok((audio_tx, self.audio_rx.take().unwrap()))
    }

    /// Stop the RTP session
    pub async fn stop(&mut self) -> Result<(), RtpError> {
        if self.socket.is_none() {
            return Err(RtpError::SessionNotStarted);
        }

        eprintln!("DEBUG:[RTP/STOP] Stopping RTP session");

        // Send stop signal
        if let Some(stop_tx) = self.stop_tx.take() {
            if let Err(e) = stop_tx.send(()) {
                eprintln!("DEBUG:[RTP/STOP] Warning: Failed to send stop signal (no receivers): {}", e);
            }
        }

        // Wait for tasks to complete
        if let Some(handle) = self.send_handle.take() {
            match handle.await {
                Ok(Ok(())) => {
                    eprintln!("DEBUG:[RTP/STOP] Send task completed successfully");
                }
                Ok(Err(e)) => {
                    eprintln!("DEBUG:[RTP/STOP] Send task returned error: {}", e);
                }
                Err(e) => {
                    eprintln!("DEBUG:[RTP/STOP] Send task join error: {}", e);
                }
            }
        }

        if let Some(handle) = self.receive_handle.take() {
            match handle.await {
                Ok(Ok(())) => {
                    eprintln!("DEBUG:[RTP/STOP] Receive task completed successfully");
                }
                Ok(Err(e)) => {
                    eprintln!("DEBUG:[RTP/STOP] Receive task returned error: {}", e);
                }
                Err(e) => {
                    eprintln!("DEBUG:[RTP/STOP] Receive task join error: {}", e);
                }
            }
        }

        // Close socket
        self.socket = None;
        self.audio_tx = None;
        self.audio_rx = None;

        eprintln!("DEBUG:[RTP/STOP] RTP session stopped");

        Ok(())
    }

    /// Get the SSRC for this session
    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }
}

/// Build an RTP packet
/// RTP header format (RFC 3550):
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |V=2|P|X|  CC   |M|     PT      |       sequence number         |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                           timestamp                             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |           synchronization source (SSRC) identifier              |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
fn build_rtp_packet(
    payload_type: u8,
    sequence: u16,
    timestamp: u32,
    ssrc: u32,
    payload: &[u8],
) -> Vec<u8> {
    let mut packet = Vec::with_capacity(12 + payload.len());

    // Version (2 bits) = 2, Padding (1 bit) = 0, Extension (1 bit) = 0, CC (4 bits) = 0
    let vpxcc = 0x80u8; // V=2, P=0, X=0, CC=0
    packet.push(vpxcc);

    // Marker (1 bit) = 0, Payload Type (7 bits)
    let mpt = payload_type & 0x7f;
    packet.push(mpt);

    // Sequence number (16 bits, network byte order)
    packet.extend_from_slice(&sequence.to_be_bytes());

    // Timestamp (32 bits, network byte order)
    packet.extend_from_slice(&timestamp.to_be_bytes());

    // SSRC (32 bits, network byte order)
    packet.extend_from_slice(&ssrc.to_be_bytes());

    // Payload
    packet.extend_from_slice(payload);

    packet
}

/// Parse an RTP packet and extract payload
fn parse_rtp_packet(data: &[u8]) -> Result<Vec<u8>, RtpError> {
    if data.len() < 12 {
        return Err(RtpError::InvalidPacket {
            message: "RTP packet too short".to_string(),
        });
    }

    // Check version (must be 2)
    let vpxcc = data[0];
    let version = (vpxcc >> 6) & 0x03;
    if version != 2 {
        return Err(RtpError::InvalidPacket {
            message: format!("Invalid RTP version: {}", version),
        });
    }

    // Extract payload type
    let _payload_type = data[1] & 0x7f;

    // Extract sequence number (for logging/debugging)
    let _sequence = u16::from_be_bytes([data[2], data[3]]);

    // Extract timestamp (for logging/debugging)
    let _timestamp = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);

    // Extract SSRC (for logging/debugging)
    let _ssrc = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);

    // Payload starts at offset 12
    let payload = data[12..].to_vec();

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtp_packet_build_parse() {
        let payload = vec![0x01, 0x02, 0x03, 0x04];
        let packet = build_rtp_packet(0, 1234, 5678, 9012, &payload);

        assert_eq!(packet.len(), 12 + payload.len());
        assert_eq!(packet[0] & 0xc0, 0x80); // Version 2

        let parsed = parse_rtp_packet(&packet).unwrap();
        assert_eq!(parsed, payload);
    }

    #[test]
    fn test_rtp_packet_too_short() {
        let data = vec![0u8; 10];
        assert!(parse_rtp_packet(&data).is_err());
    }
}
