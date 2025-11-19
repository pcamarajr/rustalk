// Call domain entity with state machine for SIP call lifecycle

use crate::domain::errors::SipError;
use std::time::SystemTime;

/// Call identifier (newtype wrapper)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallId(String);

impl CallId {
    /// Create a new CallId from a string
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for CallId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for CallId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// Call direction enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallDirection {
    /// Outbound call (initiated by us)
    Outbound,
    /// Inbound call (received from remote)
    Inbound,
}

/// Call state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallState {
    /// Initial state, no call active
    Idle,
    /// INVITE sent, waiting for response (100 Trying or 180 Ringing)
    Ringing,
    /// Received provisional response (180 Ringing), waiting for 200 OK
    Connecting,
    /// Call established (200 OK received)
    Active,
    /// Call temporarily suspended (future)
    OnHold,
    /// Call terminated
    Ended,
}

/// Call entity managing SIP call lifecycle
#[derive(Debug, Clone)]
pub struct Call {
    /// Call identifier
    id: CallId,
    /// Call direction (Outbound or Inbound)
    direction: CallDirection,
    /// Remote phone number or URI
    remote_number: String,
    /// Current call state
    state: CallState,
    /// Call start time (set when call becomes Active)
    start_time: Option<SystemTime>,
    /// Call end time (set when call ends)
    end_time: Option<SystemTime>,
    /// SIP Call-ID header value
    call_id_header: Option<String>,
    /// SIP From tag
    from_tag: Option<String>,
    /// SIP To tag (set when call is answered)
    to_tag: Option<String>,
    /// Local SIP URI
    local_uri: Option<String>,
    /// Remote SIP URI
    remote_uri: Option<String>,
    /// SDP offer from incoming INVITE (raw string, stored for later answer generation)
    sdp_offer: Option<String>,
}

impl Call {
    /// Create a new outbound call in Idle state
    ///
    /// # Arguments
    /// * `number` - Remote phone number or URI
    ///
    /// # Returns
    /// A new Call instance in Idle state
    pub fn new_outbound(number: String) -> Self {
        // Generate a simple CallId from timestamp
        let call_id = format!(
            "call_{}",
            SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );

        Self {
            id: CallId::new(call_id),
            direction: CallDirection::Outbound,
            remote_number: number,
            state: CallState::Idle,
            start_time: None,
            end_time: None,
            call_id_header: None,
            from_tag: None,
            to_tag: None,
            local_uri: None,
            remote_uri: None,
            sdp_offer: None,
        }
    }

    /// Create a new inbound call in Ringing state
    ///
    /// Inbound calls start in Ringing state because the caller is ringing us.
    ///
    /// # Arguments
    /// * `remote_number` - Remote phone number or URI (from Request-URI or From header)
    /// * `call_id_header` - SIP Call-ID header value from INVITE
    /// * `from_tag` - SIP From tag from INVITE (optional)
    ///
    /// # Returns
    /// A new Call instance in Ringing state
    pub fn new_inbound(
        remote_number: String,
        call_id_header: String,
        from_tag: Option<String>,
    ) -> Self {
        // Generate a simple CallId from timestamp
        let call_id = format!(
            "call_{}",
            SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );

        Self {
            id: CallId::new(call_id),
            direction: CallDirection::Inbound,
            remote_number,
            state: CallState::Ringing, // Inbound calls start in Ringing state
            start_time: None,
            end_time: None,
            call_id_header: Some(call_id_header),
            from_tag,
            to_tag: None,
            local_uri: None,
            remote_uri: None,
            sdp_offer: None,
        }
    }

    /// Get the call ID
    pub fn id(&self) -> &CallId {
        &self.id
    }

    /// Get the call direction
    pub fn direction(&self) -> &CallDirection {
        &self.direction
    }

    /// Get the remote number
    pub fn remote_number(&self) -> &str {
        &self.remote_number
    }

    /// Get the current call state
    pub fn state(&self) -> &CallState {
        &self.state
    }

    /// Get the start time (if available)
    pub fn start_time(&self) -> Option<SystemTime> {
        self.start_time
    }

    /// Get the end time (if available)
    pub fn end_time(&self) -> Option<SystemTime> {
        self.end_time
    }

    /// Get the SIP Call-ID header value
    pub fn call_id_header(&self) -> Option<&String> {
        self.call_id_header.as_ref()
    }

    /// Get the SIP From tag
    pub fn from_tag(&self) -> Option<&String> {
        self.from_tag.as_ref()
    }

    /// Get the SIP To tag
    pub fn to_tag(&self) -> Option<&String> {
        self.to_tag.as_ref()
    }

    /// Get the local SIP URI
    pub fn local_uri(&self) -> Option<&String> {
        self.local_uri.as_ref()
    }

    /// Get the remote SIP URI
    pub fn remote_uri(&self) -> Option<&String> {
        self.remote_uri.as_ref()
    }

    /// Set SIP Call-ID header value
    pub fn set_call_id_header(&mut self, call_id: String) {
        self.call_id_header = Some(call_id);
    }

    /// Set SIP From tag
    pub fn set_from_tag(&mut self, tag: String) {
        self.from_tag = Some(tag);
    }

    /// Set SIP To tag
    pub fn set_to_tag(&mut self, tag: String) {
        self.to_tag = Some(tag);
    }

    /// Set local SIP URI
    pub fn set_local_uri(&mut self, uri: String) {
        self.local_uri = Some(uri);
    }

    /// Set remote SIP URI
    pub fn set_remote_uri(&mut self, uri: String) {
        self.remote_uri = Some(uri);
    }

    /// Get the SDP offer (if available)
    pub fn sdp_offer(&self) -> Option<&str> {
        self.sdp_offer.as_deref()
    }

    /// Set the SDP offer
    pub fn set_sdp_offer(&mut self, offer: String) {
        self.sdp_offer = Some(offer);
    }

    /// Check if a transition to the given state is valid
    ///
    /// Validates transitions for both outbound and inbound calls.
    /// Inbound calls start in Ringing state, so they can transition directly to Active or Ended.
    ///
    /// # Arguments
    /// * `target_state` - The target state to transition to
    ///
    /// # Returns
    /// `true` if the transition is valid, `false` otherwise
    pub fn can_transition_to(&self, target_state: &CallState) -> bool {
        match (&self.state, target_state) {
            // Idle → Ringing (when INVITE sent - outbound only)
            (CallState::Idle, CallState::Ringing) => true,
            // Ringing → Connecting (on 180 Ringing - outbound only)
            (CallState::Ringing, CallState::Connecting) => true,
            // Ringing → Active (on 200 OK, skipping Connecting - both inbound and outbound)
            (CallState::Ringing, CallState::Active) => true,
            // Connecting → Active (on 200 OK - outbound only)
            (CallState::Connecting, CallState::Active) => true,
            // Any → Ended (on BYE, error, or timeout)
            (_, CallState::Ended) => true,
            // Any → OnHold (future feature)
            (CallState::Active, CallState::OnHold) => true,
            // OnHold → Active (resume call)
            (CallState::OnHold, CallState::Active) => true,
            // Invalid transitions
            _ => false,
        }
    }

    /// Transition to Ringing state
    ///
    /// Valid transitions:
    /// - Idle → Ringing (when INVITE sent)
    ///
    /// # Returns
    /// `Ok(())` if transition is valid, `Err(SipError)` otherwise
    pub fn transition_to_ringing(&mut self) -> Result<(), SipError> {
        if !self.can_transition_to(&CallState::Ringing) {
            return Err(SipError::InvalidMessage {
                reason: format!("Cannot transition from {:?} to Ringing", self.state),
            });
        }
        self.state = CallState::Ringing;
        Ok(())
    }

    /// Transition to Connecting state
    ///
    /// Valid transitions:
    /// - Ringing → Connecting (on 180 Ringing)
    ///
    /// # Returns
    /// `Ok(())` if transition is valid, `Err(SipError)` otherwise
    pub fn transition_to_connecting(&mut self) -> Result<(), SipError> {
        if !self.can_transition_to(&CallState::Connecting) {
            return Err(SipError::InvalidMessage {
                reason: format!("Cannot transition from {:?} to Connecting", self.state),
            });
        }
        self.state = CallState::Connecting;
        Ok(())
    }

    /// Transition to Active state
    ///
    /// Valid transitions:
    /// - Ringing → Active (on 200 OK, skipping Connecting)
    /// - Connecting → Active (on 200 OK)
    ///
    /// # Returns
    /// `Ok(())` if transition is valid, `Err(SipError)` otherwise
    pub fn transition_to_active(&mut self) -> Result<(), SipError> {
        if !self.can_transition_to(&CallState::Active) {
            return Err(SipError::InvalidMessage {
                reason: format!("Cannot transition from {:?} to Active", self.state),
            });
        }
        self.state = CallState::Active;
        self.start_time = Some(SystemTime::now());
        Ok(())
    }

    /// Transition to Ended state
    ///
    /// Valid transitions:
    /// - Any → Ended (on BYE, error, or timeout)
    ///
    /// # Returns
    /// `Ok(())` - always succeeds (any state can end)
    pub fn transition_to_ended(&mut self) -> Result<(), SipError> {
        self.state = CallState::Ended;
        self.end_time = Some(SystemTime::now());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_call() -> Call {
        Call::new_outbound("sip:bob@example.com".to_string())
    }

    #[test]
    fn test_new_outbound_call() {
        let call = create_test_call();
        assert!(matches!(call.state(), CallState::Idle));
        assert!(matches!(call.direction(), CallDirection::Outbound));
        assert_eq!(call.remote_number(), "sip:bob@example.com");
        assert!(call.start_time().is_none());
        assert!(call.end_time().is_none());
    }

    #[test]
    fn test_idle_to_ringing() {
        let mut call = create_test_call();
        assert!(call.transition_to_ringing().is_ok());
        assert!(matches!(call.state(), CallState::Ringing));
    }

    #[test]
    fn test_ringing_to_connecting() {
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        assert!(call.transition_to_connecting().is_ok());
        assert!(matches!(call.state(), CallState::Connecting));
    }

    #[test]
    fn test_ringing_to_active() {
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        assert!(call.transition_to_active().is_ok());
        assert!(matches!(call.state(), CallState::Active));
        assert!(call.start_time().is_some());
    }

    #[test]
    fn test_connecting_to_active() {
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        call.transition_to_connecting().unwrap();
        assert!(call.transition_to_active().is_ok());
        assert!(matches!(call.state(), CallState::Active));
        assert!(call.start_time().is_some());
    }

    #[test]
    fn test_any_to_ended() {
        let mut call = create_test_call();
        // Test from Idle
        assert!(call.transition_to_ended().is_ok());
        assert!(matches!(call.state(), CallState::Ended));
        assert!(call.end_time().is_some());

        // Test from Ringing
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        assert!(call.transition_to_ended().is_ok());
        assert!(matches!(call.state(), CallState::Ended));

        // Test from Connecting
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        call.transition_to_connecting().unwrap();
        assert!(call.transition_to_ended().is_ok());
        assert!(matches!(call.state(), CallState::Ended));

        // Test from Active
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        call.transition_to_active().unwrap();
        assert!(call.transition_to_ended().is_ok());
        assert!(matches!(call.state(), CallState::Ended));
    }

    #[test]
    fn test_invalid_idle_to_active() {
        let mut call = create_test_call();
        assert!(call.transition_to_active().is_err());
    }

    #[test]
    fn test_invalid_ended_to_ringing() {
        let mut call = create_test_call();
        call.transition_to_ended().unwrap();
        assert!(call.transition_to_ringing().is_err());
    }

    #[test]
    fn test_invalid_connecting_to_ringing() {
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        call.transition_to_connecting().unwrap();
        assert!(call.transition_to_ringing().is_err());
    }

    #[test]
    fn test_can_transition_to() {
        let call = create_test_call();
        assert!(call.can_transition_to(&CallState::Ringing));
        assert!(!call.can_transition_to(&CallState::Active));
        assert!(!call.can_transition_to(&CallState::Connecting));
        assert!(call.can_transition_to(&CallState::Ended));
    }

    #[test]
    fn test_can_transition_to_ringing() {
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        assert!(call.can_transition_to(&CallState::Connecting));
        assert!(call.can_transition_to(&CallState::Active));
        assert!(call.can_transition_to(&CallState::Ended));
        assert!(!call.can_transition_to(&CallState::Idle));
    }

    #[test]
    fn test_can_transition_to_connecting() {
        let mut call = create_test_call();
        call.transition_to_ringing().unwrap();
        call.transition_to_connecting().unwrap();
        assert!(call.can_transition_to(&CallState::Active));
        assert!(call.can_transition_to(&CallState::Ended));
        assert!(!call.can_transition_to(&CallState::Ringing));
    }

    #[test]
    fn test_sip_header_setters() {
        let mut call = create_test_call();
        call.set_call_id_header("abc123@example.com".to_string());
        call.set_from_tag("from-tag-123".to_string());
        call.set_to_tag("to-tag-456".to_string());
        call.set_local_uri("sip:alice@example.com".to_string());
        call.set_remote_uri("sip:bob@example.com".to_string());

        assert_eq!(
            call.call_id_header(),
            Some(&"abc123@example.com".to_string())
        );
        assert_eq!(call.from_tag(), Some(&"from-tag-123".to_string()));
        assert_eq!(call.to_tag(), Some(&"to-tag-456".to_string()));
        assert_eq!(call.local_uri(), Some(&"sip:alice@example.com".to_string()));
        assert_eq!(call.remote_uri(), Some(&"sip:bob@example.com".to_string()));
    }

    #[test]
    fn test_call_id_newtype() {
        let call_id1 = CallId::new("call-123".to_string());
        let call_id2 = CallId::from("call-123");
        let call_id3 = CallId::from("call-456");

        assert_eq!(call_id1.as_str(), "call-123");
        assert_eq!(call_id1, call_id2);
        assert_ne!(call_id1, call_id3);
    }

    #[test]
    fn test_new_inbound_call() {
        let call = Call::new_inbound(
            "sip:alice@example.com".to_string(),
            "abc123@example.com".to_string(),
            Some("from-tag-123".to_string()),
        );
        assert!(matches!(call.state(), CallState::Ringing));
        assert!(matches!(call.direction(), CallDirection::Inbound));
        assert_eq!(call.remote_number(), "sip:alice@example.com");
        assert_eq!(
            call.call_id_header(),
            Some(&"abc123@example.com".to_string())
        );
        assert_eq!(call.from_tag(), Some(&"from-tag-123".to_string()));
        assert!(call.start_time().is_none());
        assert!(call.end_time().is_none());
    }

    #[test]
    fn test_new_inbound_call_no_from_tag() {
        let call = Call::new_inbound(
            "sip:alice@example.com".to_string(),
            "abc123@example.com".to_string(),
            None,
        );
        assert!(matches!(call.state(), CallState::Ringing));
        assert!(matches!(call.direction(), CallDirection::Inbound));
        assert!(call.from_tag().is_none());
    }

    #[test]
    fn test_inbound_call_ringing_to_active() {
        let mut call = Call::new_inbound(
            "sip:alice@example.com".to_string(),
            "abc123@example.com".to_string(),
            Some("from-tag-123".to_string()),
        );
        // Inbound calls start in Ringing, can transition directly to Active
        assert!(call.transition_to_active().is_ok());
        assert!(matches!(call.state(), CallState::Active));
        assert!(call.start_time().is_some());
    }

    #[test]
    fn test_inbound_call_ringing_to_ended() {
        let mut call = Call::new_inbound(
            "sip:alice@example.com".to_string(),
            "abc123@example.com".to_string(),
            Some("from-tag-123".to_string()),
        );
        // Inbound calls can be rejected/ended from Ringing
        assert!(call.transition_to_ended().is_ok());
        assert!(matches!(call.state(), CallState::Ended));
        assert!(call.end_time().is_some());
    }

    #[test]
    fn test_sdp_offer_getter_setter() {
        let mut call = Call::new_inbound(
            "sip:alice@example.com".to_string(),
            "abc123@example.com".to_string(),
            Some("from-tag-123".to_string()),
        );
        // Initially no SDP offer
        assert!(call.sdp_offer().is_none());

        // Set SDP offer
        let sdp = "v=0\r\no=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\ns=-\r\nc=IN IP4 192.168.1.100\r\nt=0 0\r\nm=audio 49172 RTP/AVP 0 8\r\na=rtpmap:0 PCMU/8000\r\na=rtpmap:8 PCMA/8000\r\n".to_string();
        call.set_sdp_offer(sdp.clone());

        // Verify SDP offer is stored
        assert_eq!(call.sdp_offer(), Some(sdp.as_str()));
    }

    #[test]
    fn test_sdp_offer_outbound_call() {
        let mut call = Call::new_outbound("sip:bob@example.com".to_string());
        // Outbound calls don't have SDP offer initially
        assert!(call.sdp_offer().is_none());

        // Can still set SDP offer if needed
        let sdp = "v=0\r\no=bob 2890844527 2890844527 IN IP4 192.168.1.200\r\ns=-\r\nc=IN IP4 192.168.1.200\r\nt=0 0\r\nm=audio 49174 RTP/AVP 0 8\r\na=rtpmap:0 PCMU/8000\r\na=rtpmap:8 PCMA/8000\r\n".to_string();
        call.set_sdp_offer(sdp.clone());
        assert_eq!(call.sdp_offer(), Some(sdp.as_str()));
    }
}
