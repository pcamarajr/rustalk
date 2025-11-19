// G.711 codec implementation (PCMU/PCMA)
// ITU-T G.711 standard for audio encoding/decoding

use crate::domain::errors::RtpError;

/// G.711 codec type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G711Type {
    /// PCMU (μ-law) - payload type 0
    Pcmu,
    /// PCMA (A-law) - payload type 8
    Pcma,
}

/// Trait for audio codecs
pub trait Codec {
    /// Encode audio samples to codec format
    /// # Arguments
    /// * `samples` - Input audio samples (linear PCM, 16-bit)
    /// # Returns
    /// Encoded bytes
    fn encode(&self, samples: &[i16]) -> Result<Vec<u8>, RtpError>;

    /// Decode codec bytes to audio samples
    /// # Arguments
    /// * `data` - Encoded audio bytes
    /// # Returns
    /// Decoded audio samples (linear PCM, 16-bit)
    fn decode(&self, data: &[u8]) -> Result<Vec<i16>, RtpError>;

    /// Get the payload type for this codec
    fn payload_type(&self) -> u8;

    /// Get the clock rate (sample rate) for this codec
    fn clock_rate(&self) -> u32;
}

/// G.711 codec implementation
#[derive(Clone)]
pub struct G711Codec {
    codec_type: G711Type,
}

impl G711Codec {
    /// Create a new G.711 codec
    pub fn new(codec_type: G711Type) -> Self {
        Self { codec_type }
    }

    /// Create PCMU codec
    pub fn pcmu() -> Self {
        Self::new(G711Type::Pcmu)
    }

    /// Create PCMA codec
    pub fn pcma() -> Self {
        Self::new(G711Type::Pcma)
    }
}

impl Codec for G711Codec {
    fn encode(&self, samples: &[i16]) -> Result<Vec<u8>, RtpError> {
        match self.codec_type {
            G711Type::Pcmu => encode_pcmu(samples),
            G711Type::Pcma => encode_pcma(samples),
        }
    }

    fn decode(&self, data: &[u8]) -> Result<Vec<i16>, RtpError> {
        match self.codec_type {
            G711Type::Pcmu => decode_pcmu(data),
            G711Type::Pcma => decode_pcma(data),
        }
    }

    fn payload_type(&self) -> u8 {
        match self.codec_type {
            G711Type::Pcmu => 0,
            G711Type::Pcma => 8,
        }
    }

    fn clock_rate(&self) -> u32 {
        8000 // G.711 standard sample rate
    }
}

/// Encode PCM samples to μ-law (PCMU)
/// Based on ITU-T G.711 standard
fn encode_pcmu(samples: &[i16]) -> Result<Vec<u8>, RtpError> {
    let mut encoded = Vec::with_capacity(samples.len());
    for &sample in samples {
        encoded.push(linear_to_ulaw(sample));
    }
    Ok(encoded)
}

/// Decode μ-law (PCMU) to PCM samples
fn decode_pcmu(data: &[u8]) -> Result<Vec<i16>, RtpError> {
    let mut decoded = Vec::with_capacity(data.len());
    for &byte in data {
        decoded.push(ulaw_to_linear(byte));
    }
    Ok(decoded)
}

/// Encode PCM samples to A-law (PCMA)
/// Based on ITU-T G.711 standard
fn encode_pcma(samples: &[i16]) -> Result<Vec<u8>, RtpError> {
    let mut encoded = Vec::with_capacity(samples.len());
    for &sample in samples {
        encoded.push(linear_to_alaw(sample));
    }
    Ok(encoded)
}

/// Decode A-law (PCMA) to PCM samples
fn decode_pcma(data: &[u8]) -> Result<Vec<i16>, RtpError> {
    let mut decoded = Vec::with_capacity(data.len());
    for &byte in data {
        decoded.push(alaw_to_linear(byte));
    }
    Ok(decoded)
}

/// Convert linear PCM sample to μ-law
/// Based on ITU-T G.711 Appendix I
fn linear_to_ulaw(sample: i16) -> u8 {
    let sign = if sample < 0 { 0x80 } else { 0x00 };
    let magnitude = if sample < 0 {
        // Handle -32768 specially to avoid overflow
        if sample == i16::MIN {
            0x8000u16
        } else {
            (-sample) as u16
        }
    } else {
        sample as u16
    };

    // Clamp to 14 bits (0x3fff = 16383)
    let magnitude = magnitude.min(0x3fff);

    // Bias the magnitude
    let biased = magnitude + 33;

    // Find exponent (0-7)
    let mut exponent = 0u8;
    let mut exp_mag = biased;
    while exp_mag > 0x1f {
        exp_mag >>= 1;
        exponent += 1;
        if exponent >= 7 {
            break;
        }
    }

    // Mantissa is 4 bits (top 4 bits of exp_mag)
    let mantissa = (exp_mag >> 1) as u8 & 0x0f;

    // Combine: sign (1 bit) + exponent (3 bits) + mantissa (4 bits)
    let ulaw_byte = sign | (exponent << 4) | mantissa;

    // Invert all bits
    !ulaw_byte
}

/// Convert μ-law to linear PCM sample
fn ulaw_to_linear(ulaw_byte: u8) -> i16 {
    // Invert all bits
    let ulaw = !ulaw_byte;

    let sign = (ulaw & 0x80) as i16;
    let exponent = ((ulaw >> 4) & 0x07) as u8;
    let mantissa = (ulaw & 0x0f) as u16;

    // Reconstruct linear value
    let linear = ((mantissa << 1) + 33) << (exponent as u16);
    let linear_i16 = linear as i16;

    if sign != 0 {
        -(linear_i16 - 33)
    } else {
        linear_i16 - 33
    }
}

/// Convert linear PCM sample to A-law
/// Based on ITU-T G.711 Appendix II
fn linear_to_alaw(sample: i16) -> u8 {
    let sign = if sample < 0 { 0x80 } else { 0x00 };
    let magnitude = if sample < 0 {
        // Handle -32768 specially to avoid overflow
        if sample == i16::MIN {
            0x8000u16
        } else {
            (-sample) as u16
        }
    } else {
        sample as u16
    };

    // Clamp to 13 bits (0x1fff = 8191)
    let magnitude = magnitude.min(0x1fff);

    // Bias the magnitude
    let biased = magnitude + 33;

    // Find exponent (0-7)
    let mut exponent = 0u8;
    let mut exp_mag = biased;
    while exp_mag > 0x1f {
        exp_mag >>= 1;
        exponent += 1;
        if exponent >= 7 {
            break;
        }
    }

    // Mantissa is 4 bits
    let mantissa = (exp_mag >> 1) as u8 & 0x0f;

    // Combine: sign (1 bit) + exponent (3 bits) + mantissa (4 bits)
    let alaw_byte = sign | (exponent << 4) | mantissa;

    // Invert even bits
    alaw_byte ^ 0x55
}

/// Convert A-law to linear PCM sample
fn alaw_to_linear(alaw_byte: u8) -> i16 {
    // Invert even bits
    let alaw = alaw_byte ^ 0x55;

    let sign = (alaw & 0x80) as i16;
    let exponent = ((alaw >> 4) & 0x07) as u8;
    let mantissa = (alaw & 0x0f) as u16;

    // Reconstruct linear value
    let linear = ((mantissa << 1) + 33) << (exponent as u16);
    let linear_i16 = linear as i16;

    if sign != 0 {
        -(linear_i16 - 33)
    } else {
        linear_i16 - 33
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcmu_encode_decode_roundtrip() {
        let codec = G711Codec::pcmu();
        let original: Vec<i16> = vec![0, 1000, -1000, 16384, -16384, 32767, -32768];
        let encoded = codec.encode(&original).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        // G.711 is lossy, so we check approximate values
        assert_eq!(decoded.len(), original.len());
        for (orig, dec) in original.iter().zip(decoded.iter()) {
            let diff = (orig - dec).abs();
            // Allow some tolerance for lossy compression
            assert!(diff < 100, "Difference too large: {} vs {}", orig, dec);
        }
    }

    #[test]
    fn test_pcma_encode_decode_roundtrip() {
        let codec = G711Codec::pcma();
        let original: Vec<i16> = vec![0, 1000, -1000, 16384, -16384, 32767, -32768];
        let encoded = codec.encode(&original).unwrap();
        let decoded = codec.decode(&encoded).unwrap();

        assert_eq!(decoded.len(), original.len());
        for (orig, dec) in original.iter().zip(decoded.iter()) {
            let diff = (orig - dec).abs();
            assert!(diff < 100, "Difference too large: {} vs {}", orig, dec);
        }
    }

    #[test]
    fn test_pcmu_payload_type() {
        let codec = G711Codec::pcmu();
        assert_eq!(codec.payload_type(), 0);
        assert_eq!(codec.clock_rate(), 8000);
    }

    #[test]
    fn test_pcma_payload_type() {
        let codec = G711Codec::pcma();
        assert_eq!(codec.payload_type(), 8);
        assert_eq!(codec.clock_rate(), 8000);
    }

    #[test]
    fn test_ulaw_zero() {
        let encoded = linear_to_ulaw(0);
        let decoded = ulaw_to_linear(encoded);
        assert_eq!(decoded, 0);
    }

    #[test]
    fn test_alaw_zero() {
        let encoded = linear_to_alaw(0);
        let decoded = alaw_to_linear(encoded);
        assert_eq!(decoded, 0);
    }
}

