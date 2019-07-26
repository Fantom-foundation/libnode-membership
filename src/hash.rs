use bincode;
use failure::Fail;
use serde::{Deserialize, Serialize};
use sha3::{digest::generic_array::transmute, Digest, Sha3_256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

#[derive(Debug, Fail)]
pub enum Error {
    /// A serialization error in `compute_hash`.
    #[fail(display = "Serialization error: {}", _0)]
    ComputeHashSerialize(bincode::Error),
}

/// Computes the hash of serializable data.
pub fn compute_hash<B: Serialize>(b: &B) -> Result<Hash, Error> {
    let mut hasher = Sha3_256::new();
    let ser = bincode::serialize(b).map_err(|err| Error::ComputeHashSerialize(err))?;
    hasher.input(ser);
    let r = hasher.result();
    Ok(Hash(unsafe { transmute(r) }))
}
