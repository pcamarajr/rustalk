// RTP (Real-time Transport Protocol) infrastructure
// Provides RTP session management and audio streaming

pub mod codec;
pub mod session;

pub use codec::{Codec, G711Codec, G711Type};
pub use session::{RtpSession, RtpSessionConfig};
