use bincode;
use failure::Fail;
use serde::{Deserialize, Serialize};
use sha3::{digest::generic_array::transmute, Digest, Sha3_256};

/// Type of hash commonly used within the library.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

/// Hashing errors.
#[derive(Debug, Fail)]
pub enum Error {
    /// A serialization error in `compute_hash`.
    #[fail(display = "Serialization error: {}", _0)]
    ComputeHashSerialize(bincode::Error),
}

/// Computes the hash of serializable data.
pub fn compute_hash<B: Serialize>(b: &B) -> Result<Hash, Error> {
    let mut hasher = Sha3_256::new();
    let ser = bincode::serialize(b).map_err(Error::ComputeHashSerialize)?;
    hasher.input(ser);
    let r = hasher.result();
    Ok(Hash(unsafe { transmute(r) }))
}

#[cfg(test)]
mod tests {
    use super::compute_hash;
    use sha3::{Digest, Sha3_256};

    #[test]
    fn compute_hash_works() {
        let b = "Hash me";
        // Compute the hash using the function.
        let hash1 = compute_hash(&b).unwrap();
        // Compute the same hash inline.
        let mut hasher = Sha3_256::new();
        let ser = bincode::serialize(b).unwrap();
        hasher.input(ser);
        let hash2 = hasher.result();
        // The two hashes should be the same.
        assert_eq!(hash1.0, hash2.as_slice());
    }
}
