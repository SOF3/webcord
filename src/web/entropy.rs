use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use ring::hmac;

pub(super) struct Entropy(hmac::Key);

#[allow(unused)]
impl Entropy {
    pub fn new(entropy: &[u8; 32]) -> Self {
        Self(hmac::Key::new(hmac::HMAC_SHA512, &entropy[..]))
    }

    pub fn hash<H: Hash>(&self, h: H) -> Vec<u8> {
        let mut hasher = DefaultHasher::new();
        h.hash(&mut hasher);
        let hash = hasher.finish();

        Vec::from(hmac::sign(&self.0, &hash.to_le_bytes()[..]).as_ref())
    }
}
