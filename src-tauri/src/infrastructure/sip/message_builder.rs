// SIP message builder using rsip library
// Provides a builder pattern for constructing SIP messages

use crate::domain::errors::SipError;
use crate::infrastructure::sip::parser::parse_message;

/// Builder for constructing SIP messages
pub struct SipMessageBuilder {
    method: Option<String>,
    uri: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl SipMessageBuilder {
    /// Create a new SIP message builder
    pub fn new() -> Self {
        Self {
            method: None,
            uri: None,
            headers: Vec::new(),
            body: None,
        }
    }

    /// Set the SIP method (REGISTER, INVITE, BYE, etc.)
    pub fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    /// Set the request URI
    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = Some(uri.to_string());
        self
    }

    /// Add a header
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    /// Set the message body
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Build the SIP message and return raw bytes
    pub fn build(self) -> Result<Vec<u8>, SipError> {
        // Validate required fields for requests
        let method = self.method.ok_or_else(|| SipError::MissingHeader {
            header: "Method".to_string(),
        })?;

        let uri = self.uri.ok_or_else(|| SipError::MissingHeader {
            header: "Request-URI".to_string(),
        })?;

        // Build the SIP message as a string
        let mut message = format!("{} {} SIP/2.0\r\n", method, uri);

        // Add all headers
        for (name, value) in self.headers {
            message.push_str(&format!("{}: {}\r\n", name, value));
        }

        // Add body if provided
        let body = self.body.unwrap_or_default();
        if !body.is_empty() {
            message.push_str(&format!("Content-Length: {}\r\n", body.len()));
        } else {
            message.push_str("Content-Length: 0\r\n");
        }

        // End of headers
        message.push_str("\r\n");

        // Add body if present
        if !body.is_empty() {
            message.push_str(&body);
        }

        // Parse the message to validate it, then convert to bytes
        let sip_message = parse_message(message.as_bytes())?;
        let message_bytes: Vec<u8> = sip_message.into();
        Ok(message_bytes)
    }
}

impl Default for SipMessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_register_request() {
        let message = SipMessageBuilder::new()
            .method("REGISTER")
            .uri("sip:example.com")
            .header("Via", "SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds")
            .header("Max-Forwards", "70")
            .header("To", "<sip:user@example.com>")
            .header("From", "<sip:user@example.com>;tag=1928301774")
            .header("Call-ID", "a84b4c76e66710")
            .header("CSeq", "1 REGISTER")
            .header("Contact", "<sip:user@client.example.com>")
            .build();

        assert!(message.is_ok(), "Should build REGISTER request");
        
        // Verify it can be parsed back
        let bytes = message.unwrap();
        let parsed = parse_message(&bytes);
        assert!(parsed.is_ok(), "Built message should be parseable");
    }

    #[test]
    fn test_build_invite_request_with_sdp() {
        let sdp_body = "v=0\r\n\
            o=alice 2890844526 2890844526 IN IP4 client.example.com\r\n\
            s=-\r\n\
            c=IN IP4 192.0.2.101\r\n\
            t=0 0\r\n\
            m=audio 49172 RTP/AVP 0\r\n\
            a=rtpmap:0 PCMU/8000\r\n";

        let message = SipMessageBuilder::new()
            .method("INVITE")
            .uri("sip:bob@example.com")
            .header("Via", "SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds")
            .header("Max-Forwards", "70")
            .header("To", "<sip:bob@example.com>")
            .header("From", "<sip:alice@example.com>;tag=1928301774")
            .header("Call-ID", "a84b4c76e66710")
            .header("CSeq", "1 INVITE")
            .header("Contact", "<sip:alice@client.example.com>")
            .header("Content-Type", "application/sdp")
            .body(sdp_body)
            .build();

        assert!(message.is_ok(), "Should build INVITE request with SDP");
        
        let bytes = message.unwrap();
        let parsed = parse_message(&bytes);
        assert!(parsed.is_ok(), "Built message should be parseable");
    }

    #[test]
    fn test_build_bye_request() {
        let message = SipMessageBuilder::new()
            .method("BYE")
            .uri("sip:bob@example.com")
            .header("Via", "SIP/2.0/UDP client.example.com:5060;branch=z9hG4bK776asdhds")
            .header("Max-Forwards", "70")
            .header("To", "<sip:bob@example.com>;tag=a6c85cf")
            .header("From", "<sip:alice@example.com>;tag=1928301774")
            .header("Call-ID", "a84b4c76e66710")
            .header("CSeq", "2 BYE")
            .build();

        assert!(message.is_ok(), "Should build BYE request");
        
        let bytes = message.unwrap();
        let parsed = parse_message(&bytes);
        assert!(parsed.is_ok(), "Built message should be parseable");
    }

    #[test]
    fn test_build_missing_method() {
        let message = SipMessageBuilder::new()
            .uri("sip:example.com")
            .build();

        assert!(message.is_err(), "Should fail without method");
        match message.unwrap_err() {
            SipError::MissingHeader { header } => {
                assert_eq!(header, "Method");
            }
            _ => panic!("Expected MissingHeader error"),
        }
    }

    #[test]
    fn test_build_missing_uri() {
        let message = SipMessageBuilder::new()
            .method("REGISTER")
            .build();

        assert!(message.is_err(), "Should fail without URI");
        match message.unwrap_err() {
            SipError::MissingHeader { header } => {
                assert_eq!(header, "Request-URI");
            }
            _ => panic!("Expected MissingHeader error"),
        }
    }
}
