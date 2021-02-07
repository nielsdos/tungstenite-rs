//! Data type for shared messages.

use bytes::Bytes;

/// An enum representing a payload.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Payload {
    /// Owned data in the payload.
    Owned(Vec<u8>),
    /// Data in payload is possibly shared with others.
    Shared(Bytes),
}

impl Payload {
    /// Gets the length of the payload.
    pub fn len(&self) -> usize {
        match self {
            Self::Owned(v) => v.len(),
            Self::Shared(b) => b.len(),
        }
    }

    /// Gets a slice reference for the payload.
    pub fn as_ref(&self) -> &[u8] {
        match self {
            Self::Owned(v) => v.as_ref(),
            Self::Shared(b) => b.as_ref(),
        }
    }

    /// Gets a vector (possibly cloned) from the payload.
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Payload::Owned(v) => v.clone(),
            Payload::Shared(b) => b.to_vec(),
        }
    }

    /// Converts the payload into a vector.
    pub fn into_vec(self) -> Vec<u8> {
        match self {
            Payload::Owned(v) => v,
            Payload::Shared(b) => b.to_vec(),
        }
    }
}

impl From<Vec<u8>> for Payload {
    fn from(v: Vec<u8>) -> Self {
        Payload::Owned(v)
    }
}

impl From<String> for Payload {
    fn from(s: String) -> Self {
        Payload::Owned(s.into_bytes())
    }
}

impl From<&str> for Payload {
    fn from(s: &str) -> Self {
        Payload::Owned(s.as_bytes().into())
    }
}

impl From<&[u8]> for Payload {
    fn from(s: &[u8]) -> Self {
        Payload::Owned(s.into())
    }
}

impl From<Bytes> for Payload {
    fn from(b: Bytes) -> Self {
        Payload::Shared(b)
    }
}
