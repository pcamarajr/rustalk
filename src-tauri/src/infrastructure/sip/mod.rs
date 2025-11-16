// SIP infrastructure module
// Provides SIP message parsing, generation, transport, and client functionality

pub mod client;
pub mod message_builder;
pub mod parser;
pub mod transport;

pub use client::{SipClient, TransportType};
pub use message_builder::SipMessageBuilder;
pub use parser::{parse_message, parse_request, parse_response};
pub use transport::{SipTransport, TcpTransport, TlsTransport, UdpTransport};
