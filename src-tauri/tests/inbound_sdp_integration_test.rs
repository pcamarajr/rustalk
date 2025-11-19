// Integration test for inbound SDP processing (IN-3.2)
// Tests SDP parsing, storage, and retrieval flow
// Note: Full integration with handle_incoming_invite requires registered state.
// This test focuses on verifying that SDP can be parsed, stored, and used to generate answers.

use rustalk_lib::domain::entities::call::Call;
use rustalk_lib::domain::traits::CredentialStore;
use rustalk_lib::infrastructure::sip::client::SipClient;
use rustalk_lib::infrastructure::sip::sdp::{generate_sdp_answer, parse_sdp, SdpAnswerParams};
use rustalk_lib::services::auth_service::AuthService;
use rustalk_lib::services::call_service::CallService;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Mutex;

// Mock credential store for testing
struct MockCredentialStore;

#[async_trait::async_trait]
impl CredentialStore for MockCredentialStore {
    async fn save(
        &self,
        _key: &str,
        _credentials: &rustalk_lib::domain::entities::credentials::Credentials,
    ) -> Result<(), rustalk_lib::domain::errors::CredentialStoreError> {
        Ok(())
    }

    async fn load(
        &self,
        _key: &str,
    ) -> Result<
        Option<rustalk_lib::domain::entities::credentials::Credentials>,
        rustalk_lib::domain::errors::CredentialStoreError,
    > {
        Ok(None)
    }

    async fn delete(
        &self,
        _key: &str,
    ) -> Result<(), rustalk_lib::domain::errors::CredentialStoreError> {
        Ok(())
    }

    async fn exists(
        &self,
        _key: &str,
    ) -> Result<bool, rustalk_lib::domain::errors::CredentialStoreError> {
        Ok(false)
    }
}

#[tokio::test]
async fn test_sdp_round_trip_parse_and_generate_answer() {
    // Test that we can parse an SDP offer and use it to generate an answer
    // This verifies the full SDP processing flow that will be used in IN-3.5

    // Valid SDP offer from incoming INVITE
    let sdp_offer_str = "v=0\r\n\
        o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
        s=-\r\n\
        c=IN IP4 192.168.1.100\r\n\
        t=0 0\r\n\
        m=audio 49172 RTP/AVP 0 8\r\n\
        a=rtpmap:0 PCMU/8000\r\n\
        a=rtpmap:8 PCMA/8000\r\n";

    // Parse SDP offer
    let parsed_sdp = parse_sdp(sdp_offer_str).expect("Should parse SDP offer");

    // Verify parsed SDP matches expected values
    assert_eq!(parsed_sdp.rtp_port, 49172, "RTP port should match");
    assert_eq!(
        parsed_sdp.connection_ip,
        "192.168.1.100".parse::<IpAddr>().unwrap(),
        "Connection IP should match"
    );
    assert_eq!(parsed_sdp.codecs.len(), 2, "Should have 2 codecs");
    assert_eq!(parsed_sdp.session_id, 2890844526, "Session ID should match");
    assert_eq!(
        parsed_sdp.session_version, 2890844526,
        "Session version should match"
    );

    // Verify codecs are extracted correctly
    let pcmu = parsed_sdp
        .codecs
        .iter()
        .find(|c| c.payload_type == 0)
        .expect("Should have PCMU codec");
    assert_eq!(pcmu.codec_name, "PCMU");
    assert_eq!(pcmu.clock_rate, 8000);

    let pcma = parsed_sdp
        .codecs
        .iter()
        .find(|c| c.payload_type == 8)
        .expect("Should have PCMA codec");
    assert_eq!(pcma.codec_name, "PCMA");
    assert_eq!(pcma.clock_rate, 8000);

    // Generate SDP answer using the parsed offer
    let answer_params = SdpAnswerParams {
        local_ip: "192.168.1.200".parse::<IpAddr>().unwrap(),
        rtp_port: 49174,
        username: "bob".to_string(),
        session_id: parsed_sdp.session_id,
        session_version: parsed_sdp.session_version + 1,
    };

    let sdp_answer = generate_sdp_answer(&parsed_sdp, &answer_params)
        .expect("Should generate SDP answer");

    // Verify answer is valid
    assert!(sdp_answer.contains("v=0"), "Answer should contain protocol version");
    assert!(sdp_answer.contains("o=bob"), "Answer should contain origin");
    assert!(
        sdp_answer.contains("c=IN IP4 192.168.1.200"),
        "Answer should contain connection"
    );
    assert!(
        sdp_answer.contains("m=audio 49174 RTP/AVP 0"),
        "Answer should contain media with selected codec (PCMU preferred)"
    );
    assert!(
        sdp_answer.contains("a=rtpmap:0 PCMU/8000"),
        "Answer should contain PCMU codec"
    );

    // Verify answer can be parsed
    let parsed_answer = parse_sdp(&sdp_answer).expect("Should parse SDP answer");
    assert_eq!(parsed_answer.rtp_port, 49174, "Answer RTP port should match");
    assert_eq!(
        parsed_answer.connection_ip,
        "192.168.1.200".parse::<IpAddr>().unwrap(),
        "Answer connection IP should match"
    );
    assert_eq!(parsed_answer.codecs.len(), 1, "Answer should have 1 codec");
    assert_eq!(
        parsed_answer.codecs[0].payload_type, 0,
        "Answer should select PCMU"
    );
}

#[tokio::test]
async fn test_call_entity_sdp_storage() {
    // Test that Call entity can store and retrieve SDP offer
    let mut call = Call::new_inbound(
        "sip:alice@example.com".to_string(),
        "call-id-123@example.com".to_string(),
        Some("from-tag-123".to_string()),
    );

    // Initially no SDP offer
    assert!(call.sdp_offer().is_none(), "Call should not have SDP offer initially");

    // Set SDP offer
    let sdp_offer = "v=0\r\n\
        o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
        s=-\r\n\
        c=IN IP4 192.168.1.100\r\n\
        t=0 0\r\n\
        m=audio 49172 RTP/AVP 0 8\r\n\
        a=rtpmap:0 PCMU/8000\r\n\
        a=rtpmap:8 PCMA/8000\r\n";
    call.set_sdp_offer(sdp_offer.to_string());

    // Verify SDP offer is stored
    assert!(call.sdp_offer().is_some(), "Call should have SDP offer after setting");
    assert_eq!(
        call.sdp_offer().unwrap(),
        sdp_offer,
        "Stored SDP should match original"
    );

    // Verify SDP can be parsed from stored value
    let parsed = parse_sdp(call.sdp_offer().unwrap()).expect("Should parse stored SDP");
    assert_eq!(parsed.rtp_port, 49172, "Parsed RTP port should match");
    assert_eq!(parsed.codecs.len(), 2, "Parsed should have 2 codecs");
}

// Note: Full integration test with handle_incoming_invite() requires:
// 1. Setting up registration state to Registered (complex in tests due to private fields)
// 2. Calling handle_incoming_invite() with valid SDP
// 3. Verifying SDP is parsed and stored via get_sdp_offer()
//
// The SDP parsing, storage in Call entity, and answer generation logic is tested above.
// The full INVITE flow with registration is tested through:
// - Unit tests in call_service.rs that test handle_incoming_invite validation
// - Manual testing with a real SIP server
// - End-to-end tests that set up full registration flow
//
// The get_sdp_offer() method is tested in call_service.rs unit tests.
