// SIP message parser using rsip library
// Converts raw SIP message bytes into structured types

use crate::domain::errors::SipError;
use rsip::SipMessage;

/// Parse a raw SIP message from bytes
/// Returns a parsed SipMessage or a SipError if parsing fails
pub fn parse_message(bytes: &[u8]) -> Result<SipMessage, SipError> {
    SipMessage::try_from(bytes).map_err(|e| SipError::ParseError {
        message: format!("Failed to parse SIP message: {}", e),
    })
}

/// Parse a SIP request message
/// Validates that the message is a request (not a response)
pub fn parse_request(bytes: &[u8]) -> Result<SipMessage, SipError> {
    let message = parse_message(bytes)?;

    match &message {
        SipMessage::Request(_) => Ok(message),
        SipMessage::Response(_) => Err(SipError::InvalidMessage {
            reason: "Expected request but got response".to_string(),
        }),
    }
}

/// Parse a SIP response message
/// Validates that the message is a response (not a request)
pub fn parse_response(bytes: &[u8]) -> Result<SipMessage, SipError> {
    let message = parse_message(bytes)?;

    match &message {
        SipMessage::Response(_) => Ok(message),
        SipMessage::Request(_) => Err(SipError::InvalidMessage {
            reason: "Expected response but got request".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register_request() {
        let register_msg = b"REGISTER sip:example.com SIP/2.0\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds\r\n\
            Max-Forwards: 70\r\n\
            To: <sip:user@example.com>\r\n\
            From: <sip:user@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 REGISTER\r\n\
            Contact: <sip:user@client.example.com>\r\n\
            Content-Length: 0\r\n\r\n";

        let result = parse_request(register_msg);
        assert!(result.is_ok(), "Should parse REGISTER request");
        
        let message = result.unwrap();
        match message {
            SipMessage::Request(request) => {
                assert_eq!(request.method.to_string(), "REGISTER");
            }
            SipMessage::Response(_) => panic!("Expected request"),
        }
    }

    #[test]
    fn test_parse_invite_request_with_sdp() {
        let invite_msg = b"INVITE sip:bob@example.com SIP/2.0\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds\r\n\
            Max-Forwards: 70\r\n\
            To: <sip:bob@example.com>\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 INVITE\r\n\
            Contact: <sip:alice@client.example.com>\r\n\
            Content-Type: application/sdp\r\n\
            Content-Length: 142\r\n\r\n\
            v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 client.example.com\r\n\
            s=-\r\n\
            c=IN IP4 192.0.2.101\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0\r\n\
            a=rtpmap:0 PCMU/8000\r\n";

        let result = parse_request(invite_msg);
        assert!(result.is_ok(), "Should parse INVITE request with SDP");
        
        let message = result.unwrap();
        match message {
            SipMessage::Request(request) => {
                assert_eq!(request.method.to_string(), "INVITE");
            }
            SipMessage::Response(_) => panic!("Expected request"),
        }
    }

    #[test]
    fn test_parse_bye_request() {
        let bye_msg = b"BYE sip:bob@example.com SIP/2.0\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds\r\n\
            Max-Forwards: 70\r\n\
            To: <sip:bob@example.com>;tag=a6c85cf\r\n\
            From: <sip:alice@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 2 BYE\r\n\
            Content-Length: 0\r\n\r\n";

        let result = parse_request(bye_msg);
        assert!(result.is_ok(), "Should parse BYE request");
        
        let message = result.unwrap();
        match message {
            SipMessage::Request(request) => {
                assert_eq!(request.method.to_string(), "BYE");
            }
            SipMessage::Response(_) => panic!("Expected request"),
        }
    }

    #[test]
    fn test_parse_200_ok_response() {
        let ok_response = b"SIP/2.0 200 OK\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds;received=192.0.2.101\r\n\
            To: <sip:user@example.com>;tag=1928301774\r\n\
            From: <sip:user@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 REGISTER\r\n\
            Contact: <sip:user@client.example.com>;expires=3600\r\n\
            Content-Length: 0\r\n\r\n";

        let result = parse_response(ok_response);
        assert!(result.is_ok(), "Should parse 200 OK response");
        
        let message = result.unwrap();
        match message {
            SipMessage::Response(response) => {
                assert!(response.status_code.to_string().starts_with("200"));
            }
            SipMessage::Request(_) => panic!("Expected response"),
        }
    }

    #[test]
    fn test_parse_401_unauthorized_response() {
        let unauthorized_response = b"SIP/2.0 401 Unauthorized\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds;received=192.0.2.101\r\n\
            To: <sip:user@example.com>;tag=1928301774\r\n\
            From: <sip:user@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 REGISTER\r\n\
            WWW-Authenticate: Digest realm=\"example.com\", nonce=\"dcd98b7102dd2f0e8b11d0f600bfb0c093\"\r\n\
            Content-Length: 0\r\n\r\n";

        let result = parse_response(unauthorized_response);
        assert!(result.is_ok(), "Should parse 401 Unauthorized response");
        
        let message = result.unwrap();
        match message {
            SipMessage::Response(response) => {
                assert!(response.status_code.to_string().starts_with("401"));
            }
            SipMessage::Request(_) => panic!("Expected response"),
        }
    }

    #[test]
    fn test_parse_malformed_message() {
        let malformed = b"This is not a valid SIP message\r\n\r\n";
        
        let result = parse_message(malformed);
        assert!(result.is_err(), "Should fail to parse malformed message");
        
        match result.unwrap_err() {
            SipError::ParseError { .. } => {}
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_parse_request_with_response() {
        let response = b"SIP/2.0 200 OK\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds\r\n\
            To: <sip:user@example.com>;tag=1928301774\r\n\
            From: <sip:user@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 REGISTER\r\n\
            Content-Length: 0\r\n\r\n";

        let result = parse_request(response);
        assert!(result.is_err(), "Should fail when parsing response as request");
        
        match result.unwrap_err() {
            SipError::InvalidMessage { .. } => {}
            _ => panic!("Expected InvalidMessage"),
        }
    }

    #[test]
    fn test_parse_response_with_request() {
        let request = b"REGISTER sip:example.com SIP/2.0\r\n\
            Via: SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds\r\n\
            Max-Forwards: 70\r\n\
            To: <sip:user@example.com>\r\n\
            From: <sip:user@example.com>;tag=1928301774\r\n\
            Call-ID: a84b4c76e66710\r\n\
            CSeq: 1 REGISTER\r\n\
            Content-Length: 0\r\n\r\n";

        let result = parse_response(request);
        assert!(result.is_err(), "Should fail when parsing request as response");
        
        match result.unwrap_err() {
            SipError::InvalidMessage { .. } => {}
            _ => panic!("Expected InvalidMessage"),
        }
    }
}

