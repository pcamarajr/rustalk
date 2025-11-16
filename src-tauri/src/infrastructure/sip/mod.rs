// SIP infrastructure module
// Provides SIP message parsing and generation using rsip library

pub mod parser;
pub mod message_builder;

pub use parser::{parse_message, parse_request, parse_response};
pub use message_builder::SipMessageBuilder;

