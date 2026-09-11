use rand::fill;
use sha2::{Digest, Sha256};

pub struct SessionToken {
    pub raw: String,

    pub hash: String,
}

pub fn generate_session_token() -> SessionToken {
    let mut bytes = [0u8; 32];

    fill(&mut bytes);

    let raw = hex::encode(bytes);

    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());

    let hash = hex::encode(hasher.finalize());

    SessionToken { raw, hash }
}
