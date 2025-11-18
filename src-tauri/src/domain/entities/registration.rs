// Registration domain entity with state machine for SIP account registration lifecycle

use crate::domain::entities::credentials::Credentials;
use crate::domain::errors::SipError;
use std::time::{SystemTime, UNIX_EPOCH};

/// Registration state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationState {
    /// Initial state, no registration attempted
    Unregistered,
    /// Registration in progress (awaiting response)
    Registering,
    /// Successfully registered with SIP server
    Registered,
    /// Registration failed (with error reason)
    Failed(String),
    /// Registration expired (needs re-registration)
    Expired,
}

/// Registration entity managing SIP account registration lifecycle
#[derive(Debug, Clone)]
pub struct Registration {
    /// Current registration state
    state: RegistrationState,
    /// Credentials reference (optional, only set when registering/registered)
    credentials: Option<Credentials>,
    /// Expiration timestamp (from Contact header expires parameter)
    expires_at: Option<u64>,
    /// Last error message (for Failed state)
    last_error: Option<String>,
}

impl Registration {
    /// Create a new registration in Unregistered state
    pub fn new() -> Self {
        Self {
            state: RegistrationState::Unregistered,
            credentials: None,
            expires_at: None,
            last_error: None,
        }
    }

    /// Get current registration state
    pub fn state(&self) -> &RegistrationState {
        &self.state
    }

    /// Get credentials (if available)
    pub fn credentials(&self) -> Option<&Credentials> {
        self.credentials.as_ref()
    }

    /// Get expiration timestamp (if available)
    pub fn expires_at(&self) -> Option<u64> {
        self.expires_at
    }

    /// Get last error message (if available)
    pub fn last_error(&self) -> Option<&String> {
        self.last_error.as_ref()
    }

    /// Check if registration is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => return false, // If system time is invalid, consider not expired
            };
            now >= expires_at
        } else {
            false
        }
    }

    /// Transition to Registering state
    ///
    /// Valid transitions:
    /// - Unregistered → Registering
    /// - Failed → Registering (retry)
    /// - Expired → Registering (re-registration)
    ///
    /// # Arguments
    /// * `credentials` - SIP account credentials
    ///
    /// # Returns
    /// `Ok(())` if transition is valid, `Err(SipError)` otherwise
    pub fn start_registering(&mut self, credentials: Credentials) -> Result<(), SipError> {
        match &self.state {
            RegistrationState::Unregistered
            | RegistrationState::Failed(_)
            | RegistrationState::Expired => {
                self.state = RegistrationState::Registering;
                self.credentials = Some(credentials);
                self.last_error = None;
                Ok(())
            }
            _ => Err(SipError::InvalidMessage {
                reason: format!("Cannot transition from {:?} to Registering", self.state),
            }),
        }
    }

    /// Transition to Registered state
    ///
    /// Valid transitions:
    /// - Registering → Registered (on 200 OK)
    ///
    /// # Arguments
    /// * `expires_seconds` - Expiration time in seconds from Contact header
    ///
    /// # Returns
    /// `Ok(())` if transition is valid, `Err(SipError)` otherwise
    pub fn set_registered(&mut self, expires_seconds: Option<u32>) -> Result<(), SipError> {
        match &self.state {
            RegistrationState::Registering => {
                self.state = RegistrationState::Registered;
                // Calculate expiration timestamp
                if let Some(expires) = expires_seconds {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|e| SipError::InvalidMessage {
                            reason: format!("System time error: {}", e),
                        })?
                        .as_secs();
                    self.expires_at = Some(now + expires as u64);
                } else {
                    // Default to 3600 seconds if not provided
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|e| SipError::InvalidMessage {
                            reason: format!("System time error: {}", e),
                        })?
                        .as_secs();
                    self.expires_at = Some(now + 3600);
                }
                self.last_error = None;
                Ok(())
            }
            _ => Err(SipError::InvalidMessage {
                reason: format!("Cannot transition from {:?} to Registered", self.state),
            }),
        }
    }

    /// Transition to Failed state
    ///
    /// Valid transitions:
    /// - Registering → Failed (on error response)
    ///
    /// # Arguments
    /// * `error_message` - Error message describing the failure
    ///
    /// # Returns
    /// `Ok(())` if transition is valid, `Err(SipError)` otherwise
    pub fn set_failed(&mut self, error_message: String) -> Result<(), SipError> {
        match &self.state {
            RegistrationState::Registering => {
                self.state = RegistrationState::Failed(error_message.clone());
                self.last_error = Some(error_message);
                self.expires_at = None;
                Ok(())
            }
            _ => Err(SipError::InvalidMessage {
                reason: format!("Cannot transition from {:?} to Failed", self.state),
            }),
        }
    }

    /// Transition to Expired state
    ///
    /// Valid transitions:
    /// - Registered → Expired (when expires timestamp passed)
    ///
    /// # Returns
    /// `Ok(())` if transition is valid, `Err(SipError)` otherwise
    pub fn set_expired(&mut self) -> Result<(), SipError> {
        match &self.state {
            RegistrationState::Registered => {
                self.state = RegistrationState::Expired;
                self.expires_at = None;
                Ok(())
            }
            _ => Err(SipError::InvalidMessage {
                reason: format!("Cannot transition from {:?} to Expired", self.state),
            }),
        }
    }

    /// Transition to Unregistered state
    ///
    /// Valid transitions:
    /// - Registered → Unregistered (on explicit unregister)
    /// - Any state → Unregistered (reset)
    ///
    /// # Returns
    /// `Ok(())` - always succeeds (reset operation)
    pub fn set_unregistered(&mut self) -> Result<(), SipError> {
        self.state = RegistrationState::Unregistered;
        self.credentials = None;
        self.expires_at = None;
        self.last_error = None;
        Ok(())
    }

    /// Check and update state if registration has expired
    ///
    /// This method checks if the registration is expired and automatically
    /// transitions from Registered to Expired if needed.
    ///
    /// # Returns
    /// `true` if state was updated to Expired, `false` otherwise
    pub fn check_expiration(&mut self) -> bool {
        if matches!(self.state, RegistrationState::Registered) && self.is_expired() {
            let _ = self.set_expired();
            true
        } else {
            false
        }
    }
}

impl Default for Registration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::credentials::{Credentials, TransportProtocol};

    fn create_test_credentials() -> Credentials {
        Credentials::new(
            "sip.example.com".to_string(),
            5060,
            TransportProtocol::Udp,
            "user1".to_string(),
            "password123".to_string(),
        )
    }

    #[test]
    fn test_new_registration() {
        let reg = Registration::new();
        assert!(matches!(reg.state(), RegistrationState::Unregistered));
        assert!(reg.credentials().is_none());
        assert!(reg.expires_at().is_none());
        assert!(reg.last_error().is_none());
    }

    #[test]
    fn test_unregistered_to_registering() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();

        assert!(reg.start_registering(creds.clone()).is_ok());
        assert!(matches!(reg.state(), RegistrationState::Registering));
        assert_eq!(reg.credentials(), Some(&creds));
    }

    #[test]
    fn test_registering_to_registered() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();

        assert!(reg.set_registered(Some(3600)).is_ok());
        assert!(matches!(reg.state(), RegistrationState::Registered));
        assert!(reg.expires_at().is_some());
    }

    #[test]
    fn test_registering_to_registered_default_expires() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();

        assert!(reg.set_registered(None).is_ok());
        assert!(matches!(reg.state(), RegistrationState::Registered));
        assert!(reg.expires_at().is_some());
    }

    #[test]
    fn test_registering_to_failed() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();

        assert!(reg.set_failed("401 Unauthorized".to_string()).is_ok());
        assert!(matches!(reg.state(), RegistrationState::Failed(_)));
        assert!(reg.last_error().is_some());
        assert_eq!(reg.last_error().unwrap(), "401 Unauthorized");
    }

    #[test]
    fn test_registered_to_expired() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();
        reg.set_registered(Some(1)).unwrap(); // 1 second expiration

        // Wait for expiration
        std::thread::sleep(std::time::Duration::from_secs(2));

        assert!(reg.set_expired().is_ok());
        assert!(matches!(reg.state(), RegistrationState::Expired));
    }

    #[test]
    fn test_registered_to_unregistered() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();
        reg.set_registered(Some(3600)).unwrap();

        assert!(reg.set_unregistered().is_ok());
        assert!(matches!(reg.state(), RegistrationState::Unregistered));
        assert!(reg.credentials().is_none());
    }

    #[test]
    fn test_failed_to_registering() {
        let mut reg = Registration::new();
        let creds1 = create_test_credentials();
        reg.start_registering(creds1).unwrap();
        reg.set_failed("401 Unauthorized".to_string()).unwrap();

        let creds2 = create_test_credentials();
        assert!(reg.start_registering(creds2).is_ok());
        assert!(matches!(reg.state(), RegistrationState::Registering));
    }

    #[test]
    fn test_expired_to_registering() {
        let mut reg = Registration::new();
        let creds1 = create_test_credentials();
        reg.start_registering(creds1).unwrap();
        reg.set_registered(Some(1)).unwrap();

        // Wait for expiration
        std::thread::sleep(std::time::Duration::from_secs(2));
        reg.set_expired().unwrap();

        let creds2 = create_test_credentials();
        assert!(reg.start_registering(creds2).is_ok());
        assert!(matches!(reg.state(), RegistrationState::Registering));
    }

    #[test]
    fn test_invalid_transition_registered_to_registering() {
        let mut reg = Registration::new();
        let creds1 = create_test_credentials();
        reg.start_registering(creds1).unwrap();
        reg.set_registered(Some(3600)).unwrap();

        let creds2 = create_test_credentials();
        assert!(reg.start_registering(creds2).is_err());
    }

    #[test]
    fn test_invalid_transition_unregistered_to_registered() {
        let mut reg = Registration::new();
        assert!(reg.set_registered(Some(3600)).is_err());
    }

    #[test]
    fn test_invalid_transition_unregistered_to_failed() {
        let mut reg = Registration::new();
        assert!(reg.set_failed("Error".to_string()).is_err());
    }

    #[test]
    fn test_invalid_transition_registering_to_expired() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();

        assert!(reg.set_expired().is_err());
    }

    #[test]
    fn test_check_expiration() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();
        reg.set_registered(Some(1)).unwrap(); // 1 second expiration

        // Wait for expiration
        std::thread::sleep(std::time::Duration::from_secs(2));

        assert!(reg.check_expiration());
        assert!(matches!(reg.state(), RegistrationState::Expired));
    }

    #[test]
    fn test_check_expiration_not_expired() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();
        reg.set_registered(Some(3600)).unwrap();

        assert!(!reg.check_expiration());
        assert!(matches!(reg.state(), RegistrationState::Registered));
    }

    #[test]
    fn test_is_expired() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();
        reg.set_registered(Some(1)).unwrap(); // 1 second expiration

        // Wait for expiration
        std::thread::sleep(std::time::Duration::from_secs(2));

        assert!(reg.is_expired());
    }

    #[test]
    fn test_is_not_expired() {
        let mut reg = Registration::new();
        let creds = create_test_credentials();
        reg.start_registering(creds).unwrap();
        reg.set_registered(Some(3600)).unwrap();

        assert!(!reg.is_expired());
    }
}
