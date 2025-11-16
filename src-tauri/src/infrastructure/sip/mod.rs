// SIP infrastructure module
// Provides SIP message parsing and generation using rsip library

pub mod message_builder;
pub mod parser;

pub use message_builder::SipMessageBuilder;
pub use parser::{parse_message, parse_request, parse_response};
