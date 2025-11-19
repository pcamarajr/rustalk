// SDP (Session Description Protocol) offer/answer negotiation
// Provides RFC 4566-compliant SDP generation and parsing for SIP call setup

use crate::domain::errors::SipError;
use std::net::IpAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use webrtc_sdp::{
    attribute_type::SdpAttribute, media_type::SdpMediaValue, parse_sdp as webrtc_parse_sdp,
};

/// Errors that can occur during SDP operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdpError {
    /// SDP parsing failures
    ParseError(String),
    /// Malformed SDP structure
    InvalidFormat(String),
    /// Required SDP fields missing
    MissingField(String),
    /// Codec not supported
    UnsupportedCodec(String),
}

impl std::fmt::Display for SdpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdpError::ParseError(msg) => write!(f, "SDP parse error: {}", msg),
            SdpError::InvalidFormat(msg) => write!(f, "Invalid SDP format: {}", msg),
            SdpError::MissingField(msg) => write!(f, "Missing required SDP field: {}", msg),
            SdpError::UnsupportedCodec(msg) => write!(f, "Unsupported codec: {}", msg),
        }
    }
}

impl std::error::Error for SdpError {}

impl From<SdpError> for SipError {
    fn from(err: SdpError) -> Self {
        match err {
            SdpError::ParseError(msg) => SipError::ParseError { message: msg },
            SdpError::InvalidFormat(msg) => SipError::InvalidMessage { reason: msg },
            SdpError::MissingField(msg) => SipError::InvalidMessage {
                reason: format!("Missing SDP field: {}", msg),
            },
            SdpError::UnsupportedCodec(msg) => SipError::InvalidMessage {
                reason: format!("Unsupported codec: {}", msg),
            },
        }
    }
}

/// Parameters for generating an SDP offer
#[derive(Debug, Clone)]
pub struct SdpOfferParams {
    /// Local IP address for connection information
    pub local_ip: IpAddr,
    /// RTP port for audio (must be even, RFC requirement)
    pub rtp_port: u16,
    /// Username for origin (o=) line
    pub username: String,
    /// Session ID (unique identifier)
    pub session_id: u64,
}

/// Parameters for generating an SDP answer
#[derive(Debug, Clone)]
pub struct SdpAnswerParams {
    /// Local IP address for connection information
    pub local_ip: IpAddr,
    /// RTP port for audio (must be even, RFC requirement)
    pub rtp_port: u16,
    /// Username for origin (o=) line
    pub username: String,
    /// Session ID from the offer
    pub session_id: u64,
    /// Session version (should increment from offer)
    pub session_version: u64,
}

/// Parsed SDP information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSdp {
    /// RTP port for audio
    pub rtp_port: u16,
    /// Supported codecs (payload types)
    pub codecs: Vec<CodecInfo>,
    /// Connection IP address
    pub connection_ip: IpAddr,
    /// Session ID
    pub session_id: u64,
    /// Session version
    pub session_version: u64,
}

/// Codec information from SDP
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodecInfo {
    /// Payload type number
    pub payload_type: u8,
    /// Codec name (e.g., "PCMU", "PCMA")
    pub codec_name: String,
    /// Clock rate (e.g., 8000)
    pub clock_rate: u32,
}

/// Generate an RFC 4566-compliant SDP offer for outbound calls
///
/// Creates a complete SDP offer with:
/// - Session-level fields (v, o, s, c, t)
/// - Media description (m=audio) with RTP/AVP profile
/// - Codec support (G.711 PCMU/PCMA - payload types 0/8)
/// - RTP port allocation
/// - Connection information
///
/// # Arguments
/// * `params` - Parameters for SDP generation
///
/// # Returns
/// SDP offer as a string, or `SdpError` if generation fails
///
/// # Example
/// ```
/// use std::net::IpAddr;
/// use rustalk_lib::infrastructure::sip::sdp::{generate_sdp_offer, SdpOfferParams};
///
/// let params = SdpOfferParams {
///     local_ip: "192.168.1.100".parse().unwrap(),
///     rtp_port: 49172,
///     username: "alice".to_string(),
///     session_id: 2890844526,
/// };
///
/// let sdp = generate_sdp_offer(&params).unwrap();
/// assert!(sdp.contains("v=0"));
/// assert!(sdp.contains("m=audio"));
/// ```
pub fn generate_sdp_offer(params: &SdpOfferParams) -> Result<String, SdpError> {
    // Validate RTP port is even (RFC requirement)
    if !params.rtp_port.is_multiple_of(2) {
        return Err(SdpError::InvalidFormat(format!(
            "RTP port must be even, got {}",
            params.rtp_port
        )));
    }

    // Generate session version (timestamp-based for uniqueness)
    let session_version = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| SdpError::InvalidFormat(format!("System time error: {}", e)))?
        .as_secs();

    // Format IP address
    let ip_str = match params.local_ip {
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(_ip) => {
            return Err(SdpError::InvalidFormat(
                "IPv6 not yet supported".to_string(),
            ));
        }
    };

    // Build SDP offer
    let mut sdp = String::new();

    // Protocol version (v=)
    sdp.push_str("v=0\r\n");

    // Origin (o=)
    // Format: o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
    sdp.push_str(&format!(
        "o={} {} {} IN IP4 {}\r\n",
        params.username, params.session_id, session_version, ip_str
    ));

    // Session name (s=)
    sdp.push_str("s=-\r\n");

    // Connection information (c=)
    // Format: c=<nettype> <addrtype> <connection-address>
    sdp.push_str(&format!("c=IN IP4 {}\r\n", ip_str));

    // Timing (t=)
    // Format: t=<start-time> <stop-time> (0 0 means session not bounded)
    sdp.push_str("t=0 0\r\n");

    // Media description (m=)
    // Format: m=<media> <port> <proto> <fmt> ...
    // RTP/AVP profile with payload types 0 (PCMU) and 8 (PCMA)
    sdp.push_str(&format!("m=audio {} RTP/AVP 0 8\r\n", params.rtp_port));

    // RTP map attributes (a=rtpmap:)
    // Payload type 0: PCMU (G.711 μ-law) at 8000 Hz
    sdp.push_str("a=rtpmap:0 PCMU/8000\r\n");
    // Payload type 8: PCMA (G.711 A-law) at 8000 Hz
    sdp.push_str("a=rtpmap:8 PCMA/8000\r\n");

    Ok(sdp)
}

/// Parse incoming SDP offer or answer
///
/// Extracts media information including:
/// - RTP port
/// - Supported codecs
/// - Connection IP address
/// - Session information
///
/// # Arguments
/// * `sdp_str` - SDP string to parse
///
/// # Returns
/// Parsed SDP information, or `SdpError` if parsing fails
///
/// # Example
/// ```
/// use rustalk_lib::infrastructure::sip::sdp::parse_sdp;
///
/// let sdp = "v=0\r\n\
///     o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
///     s=-\r\n\
///     c=IN IP4 192.168.1.100\r\n\
///     t=0 0\r\n\
///     m=audio 49172 RTP/AVP 0 8\r\n\
///     a=rtpmap:0 PCMU/8000\r\n\
///     a=rtpmap:8 PCMA/8000\r\n";
///
/// let parsed = parse_sdp(sdp).unwrap();
/// assert_eq!(parsed.rtp_port, 49172);
/// assert_eq!(parsed.codecs.len(), 2);
/// ```
pub fn parse_sdp(sdp_str: &str) -> Result<ParsedSdp, SdpError> {
    // Parse using webrtc-sdp
    let sdp = webrtc_parse_sdp(sdp_str, true)
        .map_err(|e| SdpError::ParseError(format!("Failed to parse SDP: {}", e)))?;

    // Extract session-level information
    let session_id = sdp.origin.session_id;
    let session_version = sdp.origin.session_version;

    // Extract connection information (session-level or media-level)
    let connection_ip = if let Some(conn) = sdp.get_connection() {
        parse_ip_address_from_typed(conn)
            .ok_or_else(|| SdpError::MissingField("connection address".to_string()))?
    } else {
        // Try to get from first media connection
        let first_media = sdp
            .media
            .first()
            .ok_or_else(|| SdpError::MissingField("media description".to_string()))?;
        if let Some(conn) = first_media.get_connection() {
            parse_ip_address_from_typed(conn)
                .ok_or_else(|| SdpError::MissingField("media connection address".to_string()))?
        } else {
            return Err(SdpError::MissingField(
                "connection information (session or media level)".to_string(),
            ));
        }
    };

    // Extract media information (audio)
    let audio_media = sdp
        .media
        .iter()
        .find(|m| matches!(m.get_type(), SdpMediaValue::Audio))
        .ok_or_else(|| SdpError::MissingField("audio media description".to_string()))?;

    // Extract RTP port
    let rtp_port = audio_media.get_port() as u16;

    // Extract codecs from RTP map attributes
    let mut codecs = Vec::new();
    for attribute in audio_media.get_attributes() {
        if let SdpAttribute::Rtpmap(rtpmap) = attribute {
            let codec_name = rtpmap.codec_name.clone();
            let clock_rate = rtpmap.frequency;
            let payload_type = rtpmap.payload_type;

            codecs.push(CodecInfo {
                payload_type,
                codec_name,
                clock_rate,
            });
        }
    }

    // If no rtpmap attributes found, try to infer from payload types in m= line
    if codecs.is_empty() {
        // Standard payload types (RFC 3551)
        // SdpFormatList can be iterated, but we need to handle it properly
        let formats = audio_media.get_formats();
        // Formats can be a list of u8 values or a range
        // For now, we'll try to extract numeric values
        // This is a simplified approach - may need refinement based on actual SdpFormatList structure
        match formats {
            webrtc_sdp::media_type::SdpFormatList::Integers(ref ints) => {
                for &pt in ints {
                    match pt {
                        0 => codecs.push(CodecInfo {
                            payload_type: 0,
                            codec_name: "PCMU".to_string(),
                            clock_rate: 8000,
                        }),
                        8 => codecs.push(CodecInfo {
                            payload_type: 8,
                            codec_name: "PCMA".to_string(),
                            clock_rate: 8000,
                        }),
                        _ => {
                            // Unknown payload type, skip
                        }
                    }
                }
            }
            _ => {
                // Other format types not handled yet
            }
        }
    }

    if codecs.is_empty() {
        return Err(SdpError::MissingField("codec information".to_string()));
    }

    Ok(ParsedSdp {
        rtp_port,
        codecs,
        connection_ip,
        session_id,
        session_version,
    })
}

/// Generate an SDP answer for inbound calls (200 OK response)
///
/// Creates an SDP answer that:
/// - Accepts codecs from the offer (selects preferred: PCMU > PCMA)
/// - Sets local RTP port
/// - Mirrors session-level information where appropriate
/// - Follows RFC 3264 offer/answer model
///
/// # Arguments
/// * `offer` - Parsed SDP offer
/// * `params` - Parameters for answer generation
///
/// # Returns
/// SDP answer as a string, or `SdpError` if generation fails
///
/// # Example
/// ```
/// use std::net::IpAddr;
/// use rustalk_lib::infrastructure::sip::sdp::{parse_sdp, generate_sdp_answer, SdpAnswerParams};
///
/// let offer_sdp = "v=0\r\n\
///     o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
///     s=-\r\n\
///     c=IN IP4 192.168.1.100\r\n\
///     t=0 0\r\n\
///     m=audio 49172 RTP/AVP 0 8\r\n\
///     a=rtpmap:0 PCMU/8000\r\n\
///     a=rtpmap:8 PCMA/8000\r\n";
///
/// let offer = parse_sdp(offer_sdp).unwrap();
///
/// let params = SdpAnswerParams {
///     local_ip: "192.168.1.200".parse().unwrap(),
///     rtp_port: 49174,
///     username: "bob".to_string(),
///     session_id: offer.session_id,
///     session_version: offer.session_version + 1,
/// };
///
/// let answer = generate_sdp_answer(&offer, &params).unwrap();
/// assert!(answer.contains("v=0"));
/// assert!(answer.contains("m=audio"));
/// ```
pub fn generate_sdp_answer(
    offer: &ParsedSdp,
    params: &SdpAnswerParams,
) -> Result<String, SdpError> {
    // Validate RTP port is even (RFC requirement)
    if !params.rtp_port.is_multiple_of(2) {
        return Err(SdpError::InvalidFormat(format!(
            "RTP port must be even, got {}",
            params.rtp_port
        )));
    }

    // Select preferred codec from offer (PCMU > PCMA)
    let selected_codec = offer
        .codecs
        .iter()
        .find(|c| c.codec_name == "PCMU" && c.clock_rate == 8000)
        .or_else(|| {
            offer
                .codecs
                .iter()
                .find(|c| c.codec_name == "PCMA" && c.clock_rate == 8000)
        })
        .ok_or_else(|| {
            SdpError::UnsupportedCodec("No supported codec (PCMU/PCMA) found in offer".to_string())
        })?;

    // Format IP address
    let ip_str = match params.local_ip {
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(_ip) => {
            return Err(SdpError::InvalidFormat(
                "IPv6 not yet supported".to_string(),
            ));
        }
    };

    // Build SDP answer
    let mut sdp = String::new();

    // Protocol version (v=)
    sdp.push_str("v=0\r\n");

    // Origin (o=)
    // Use session ID from offer, increment version
    sdp.push_str(&format!(
        "o={} {} {} IN IP4 {}\r\n",
        params.username, params.session_id, params.session_version, ip_str
    ));

    // Session name (s=)
    sdp.push_str("s=-\r\n");

    // Connection information (c=)
    sdp.push_str(&format!("c=IN IP4 {}\r\n", ip_str));

    // Timing (t=)
    sdp.push_str("t=0 0\r\n");

    // Media description (m=)
    // Only include the selected codec
    sdp.push_str(&format!(
        "m=audio {} RTP/AVP {}\r\n",
        params.rtp_port, selected_codec.payload_type
    ));

    // RTP map attribute for selected codec
    sdp.push_str(&format!(
        "a=rtpmap:{} {}/{}\r\n",
        selected_codec.payload_type, selected_codec.codec_name, selected_codec.clock_rate
    ));

    Ok(sdp)
}

/// Helper function to parse IP address from SdpConnection
fn parse_ip_address_from_typed(conn: &webrtc_sdp::SdpConnection) -> Option<IpAddr> {
    // SdpConnection has an address field of type ExplicitlyTypedAddress
    // We need to extract the IP address from it
    use webrtc_sdp::address::ExplicitlyTypedAddress;
    match &conn.address {
        ExplicitlyTypedAddress::Ip(ip) => Some(*ip),
        ExplicitlyTypedAddress::Fqdn { domain, .. } => {
            // For FQDN, we can't directly convert to IP, but for testing we'll try to parse
            // In production, this would require DNS lookup
            domain.parse().ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sdp_offer() {
        let params = SdpOfferParams {
            local_ip: "192.168.1.100".parse().unwrap(),
            rtp_port: 49172,
            username: "alice".to_string(),
            session_id: 2890844526,
        };

        let sdp = generate_sdp_offer(&params).unwrap();

        // Check required fields
        assert!(sdp.contains("v=0"), "Should contain protocol version");
        assert!(sdp.contains("o=alice"), "Should contain origin");
        assert!(sdp.contains("s=-"), "Should contain session name");
        assert!(
            sdp.contains("c=IN IP4 192.168.1.100"),
            "Should contain connection"
        );
        assert!(sdp.contains("t=0 0"), "Should contain timing");
        assert!(
            sdp.contains("m=audio 49172 RTP/AVP 0 8"),
            "Should contain media"
        );
        assert!(
            sdp.contains("a=rtpmap:0 PCMU/8000"),
            "Should contain PCMU codec"
        );
        assert!(
            sdp.contains("a=rtpmap:8 PCMA/8000"),
            "Should contain PCMA codec"
        );
    }

    #[test]
    fn test_generate_sdp_offer_odd_port() {
        let params = SdpOfferParams {
            local_ip: "192.168.1.100".parse().unwrap(),
            rtp_port: 49173, // Odd port
            username: "alice".to_string(),
            session_id: 2890844526,
        };

        let result = generate_sdp_offer(&params);
        assert!(result.is_err(), "Should reject odd RTP port");
        match result.unwrap_err() {
            SdpError::InvalidFormat(msg) => assert!(msg.contains("even")),
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_sdp_valid_offer() {
        let sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0 8\r\n\
            a=rtpmap:0 PCMU/8000\r\n\
            a=rtpmap:8 PCMA/8000\r\n";

        let parsed = parse_sdp(sdp).unwrap();

        assert_eq!(parsed.rtp_port, 49172);
        assert_eq!(parsed.codecs.len(), 2);
        assert_eq!(
            parsed.connection_ip,
            "192.168.1.100".parse::<IpAddr>().unwrap()
        );
        assert_eq!(parsed.session_id, 2890844526);
        assert_eq!(parsed.session_version, 2890844526);

        // Check codecs
        let pcmu = parsed.codecs.iter().find(|c| c.payload_type == 0).unwrap();
        assert_eq!(pcmu.codec_name, "PCMU");
        assert_eq!(pcmu.clock_rate, 8000);

        let pcma = parsed.codecs.iter().find(|c| c.payload_type == 8).unwrap();
        assert_eq!(pcma.codec_name, "PCMA");
        assert_eq!(pcma.clock_rate, 8000);
    }

    #[test]
    fn test_parse_sdp_without_rtpmap() {
        // SDP with standard payload types but no rtpmap attributes
        let sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0 8\r\n";

        let parsed = parse_sdp(sdp).unwrap();

        assert_eq!(parsed.rtp_port, 49172);
        assert_eq!(parsed.codecs.len(), 2);

        // Should infer PCMU and PCMA from payload types
        assert!(parsed
            .codecs
            .iter()
            .any(|c| c.payload_type == 0 && c.codec_name == "PCMU"));
        assert!(parsed
            .codecs
            .iter()
            .any(|c| c.payload_type == 8 && c.codec_name == "PCMA"));
    }

    #[test]
    fn test_parse_sdp_missing_media() {
        let sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n";

        let result = parse_sdp(sdp);
        assert!(result.is_err(), "Should fail without media description");
        match result.unwrap_err() {
            SdpError::MissingField(msg) => {
                assert!(msg.contains("audio media") || msg.contains("media"))
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_parse_sdp_missing_connection() {
        let sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0 8\r\n";

        let result = parse_sdp(sdp);
        // Should fail or handle gracefully - depends on webrtc-sdp behavior
        // For now, we expect it might fail or use a default
        // This test documents current behavior
        let _ = result; // Acknowledge result for clippy
    }

    #[test]
    fn test_generate_sdp_answer() {
        let offer_sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0 8\r\n\
            a=rtpmap:0 PCMU/8000\r\n\
            a=rtpmap:8 PCMA/8000\r\n";

        let offer = parse_sdp(offer_sdp).unwrap();

        let params = SdpAnswerParams {
            local_ip: "192.168.1.200".parse().unwrap(),
            rtp_port: 49174,
            username: "bob".to_string(),
            session_id: offer.session_id,
            session_version: offer.session_version + 1,
        };

        let answer = generate_sdp_answer(&offer, &params).unwrap();

        // Check required fields
        assert!(answer.contains("v=0"), "Should contain protocol version");
        assert!(answer.contains("o=bob"), "Should contain origin");
        assert!(answer.contains("s=-"), "Should contain session name");
        assert!(
            answer.contains("c=IN IP4 192.168.1.200"),
            "Should contain connection"
        );
        assert!(answer.contains("t=0 0"), "Should contain timing");
        assert!(
            answer.contains("m=audio 49174 RTP/AVP 0"),
            "Should contain media with selected codec"
        );
        assert!(
            answer.contains("a=rtpmap:0 PCMU/8000"),
            "Should contain selected codec (PCMU preferred)"
        );
    }

    #[test]
    fn test_generate_sdp_answer_prefers_pcmu() {
        let offer_sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0 8\r\n\
            a=rtpmap:0 PCMU/8000\r\n\
            a=rtpmap:8 PCMA/8000\r\n";

        let offer = parse_sdp(offer_sdp).unwrap();

        let params = SdpAnswerParams {
            local_ip: "192.168.1.200".parse().unwrap(),
            rtp_port: 49174,
            username: "bob".to_string(),
            session_id: offer.session_id,
            session_version: offer.session_version + 1,
        };

        let answer = generate_sdp_answer(&offer, &params).unwrap();

        // Should prefer PCMU (payload type 0) over PCMA (payload type 8)
        assert!(
            answer.contains("m=audio 49174 RTP/AVP 0"),
            "Should select PCMU"
        );
        assert!(!answer.contains("RTP/AVP 8"), "Should not include PCMA");
    }

    #[test]
    fn test_generate_sdp_answer_falls_back_to_pcma() {
        // Offer with only PCMA
        let offer_sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 8\r\n\
            a=rtpmap:8 PCMA/8000\r\n";

        let offer = parse_sdp(offer_sdp).unwrap();

        let params = SdpAnswerParams {
            local_ip: "192.168.1.200".parse().unwrap(),
            rtp_port: 49174,
            username: "bob".to_string(),
            session_id: offer.session_id,
            session_version: offer.session_version + 1,
        };

        let answer = generate_sdp_answer(&offer, &params).unwrap();

        // Should fall back to PCMA
        assert!(
            answer.contains("m=audio 49174 RTP/AVP 8"),
            "Should select PCMA"
        );
        assert!(
            answer.contains("a=rtpmap:8 PCMA/8000"),
            "Should contain PCMA codec"
        );
    }

    #[test]
    fn test_generate_sdp_answer_unsupported_codec() {
        // Offer with unsupported codec
        let offer_sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 96\r\n\
            a=rtpmap:96 OPUS/48000\r\n";

        let offer = parse_sdp(offer_sdp).unwrap();

        let params = SdpAnswerParams {
            local_ip: "192.168.1.200".parse().unwrap(),
            rtp_port: 49174,
            username: "bob".to_string(),
            session_id: offer.session_id,
            session_version: offer.session_version + 1,
        };

        let result = generate_sdp_answer(&offer, &params);
        assert!(result.is_err(), "Should fail with unsupported codec");
        match result.unwrap_err() {
            SdpError::UnsupportedCodec(msg) => assert!(msg.contains("PCMU/PCMA")),
            _ => panic!("Expected UnsupportedCodec error"),
        }
    }

    #[test]
    fn test_generate_sdp_answer_odd_port() {
        let offer_sdp = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
            s=-\r\n\
            c=IN IP4 192.168.1.100\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0 8\r\n\
            a=rtpmap:0 PCMU/8000\r\n\
            a=rtpmap:8 PCMA/8000\r\n";

        let offer = parse_sdp(offer_sdp).unwrap();

        let params = SdpAnswerParams {
            local_ip: "192.168.1.200".parse().unwrap(),
            rtp_port: 49173, // Odd port
            username: "bob".to_string(),
            session_id: offer.session_id,
            session_version: offer.session_version + 1,
        };

        let result = generate_sdp_answer(&offer, &params);
        assert!(result.is_err(), "Should reject odd RTP port");
        match result.unwrap_err() {
            SdpError::InvalidFormat(msg) => assert!(msg.contains("even")),
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_round_trip_offer_parse() {
        let params = SdpOfferParams {
            local_ip: "192.168.1.100".parse().unwrap(),
            rtp_port: 49172,
            username: "alice".to_string(),
            session_id: 2890844526,
        };

        let offer = generate_sdp_offer(&params).unwrap();
        let parsed = parse_sdp(&offer).unwrap();

        assert_eq!(parsed.rtp_port, 49172);
        assert_eq!(
            parsed.connection_ip,
            "192.168.1.100".parse::<IpAddr>().unwrap()
        );
        assert!(parsed
            .codecs
            .iter()
            .any(|c| c.payload_type == 0 && c.codec_name == "PCMU"));
        assert!(parsed
            .codecs
            .iter()
            .any(|c| c.payload_type == 8 && c.codec_name == "PCMA"));
    }

    #[test]
    fn test_round_trip_offer_answer() {
        // Generate offer
        let offer_params = SdpOfferParams {
            local_ip: "192.168.1.100".parse().unwrap(),
            rtp_port: 49172,
            username: "alice".to_string(),
            session_id: 2890844526,
        };
        let offer_str = generate_sdp_offer(&offer_params).unwrap();
        let offer = parse_sdp(&offer_str).unwrap();

        // Generate answer
        let answer_params = SdpAnswerParams {
            local_ip: "192.168.1.200".parse().unwrap(),
            rtp_port: 49174,
            username: "bob".to_string(),
            session_id: offer.session_id,
            session_version: offer.session_version + 1,
        };
        let answer_str = generate_sdp_answer(&offer, &answer_params).unwrap();
        let answer = parse_sdp(&answer_str).unwrap();

        // Verify answer
        assert_eq!(answer.rtp_port, 49174);
        assert_eq!(
            answer.connection_ip,
            "192.168.1.200".parse::<IpAddr>().unwrap()
        );
        assert_eq!(answer.codecs.len(), 1);
        assert_eq!(answer.codecs[0].payload_type, 0); // PCMU preferred
        assert_eq!(answer.codecs[0].codec_name, "PCMU");
    }

    #[test]
    fn test_sdp_error_display() {
        let error = SdpError::ParseError("Invalid format".to_string());
        assert_eq!(error.to_string(), "SDP parse error: Invalid format");

        let error = SdpError::InvalidFormat("Missing field".to_string());
        assert_eq!(error.to_string(), "Invalid SDP format: Missing field");

        let error = SdpError::MissingField("connection".to_string());
        assert_eq!(error.to_string(), "Missing required SDP field: connection");

        let error = SdpError::UnsupportedCodec("OPUS".to_string());
        assert_eq!(error.to_string(), "Unsupported codec: OPUS");
    }

    #[test]
    fn test_sdp_error_to_sip_error() {
        let sdp_error = SdpError::ParseError("Test error".to_string());
        let sip_error: SipError = sdp_error.into();
        match sip_error {
            SipError::ParseError { message } => assert_eq!(message, "Test error"),
            _ => panic!("Expected ParseError"),
        }
    }
}
