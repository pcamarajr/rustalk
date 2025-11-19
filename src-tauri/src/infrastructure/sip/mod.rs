// SIP infrastructure module
// Provides SIP message parsing, generation, transport, and client functionality

pub mod client;
pub mod invite;
pub mod message_builder;
pub mod message_receiver;
pub mod parser;
pub mod registration;
pub mod sdp;
pub mod tls;
pub mod transport;

pub use client::{SipClient, TransportType};
pub use invite::build_invite_request;
pub use message_builder::SipMessageBuilder;
pub use parser::{parse_message, parse_request, parse_response};
pub use registration::{
    generate_authorization, parse_www_authenticate, register_with_challenge, DigestChallenge,
    RegistrationResult,
};
pub use sdp::{
    generate_sdp_answer, generate_sdp_offer, parse_sdp, CodecInfo, ParsedSdp, SdpAnswerParams,
    SdpError, SdpOfferParams,
};
pub use tls::{create_tls_config, extract_hostname_from_credentials, extract_hostname_from_uri};
pub use transport::{SipTransport, TcpTransport, TlsTransport, UdpTransport};
