//! Deterministic seed derivation for module-local RNG.
//!
//! Each module instance derives its own seed from a master seed and
//! its instance name, ensuring reproducible randomness across runs.

use std::hash::{Hash, Hasher};

use ahash::AHasher;

/// Derive a deterministic seed by hashing `master_seed` and
/// `instance_name` together.
///
/// The same `(master_seed, instance_name)` pair always produces the
/// same output, but different instance names yield different seeds.
pub fn derive_seed(master_seed: u64, instance_name: &str) -> u64 {
    let mut hasher = AHasher::default();
    master_seed.hash(&mut hasher);
    instance_name.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_seed_deterministic() {
        let a = derive_seed(42, "vco_1");
        let b = derive_seed(42, "vco_1");
        assert_eq!(a, b);
    }

    #[test]
    fn test_derive_seed_differs_for_different_names() {
        let a = derive_seed(42, "vco_1");
        let b = derive_seed(42, "vco_2");
        assert_ne!(a, b);
    }

    #[test]
    fn test_derive_seed_differs_for_different_master_seeds() {
        let a = derive_seed(0, "lfo_1");
        let b = derive_seed(1, "lfo_1");
        assert_ne!(a, b);
    }

    #[test]
    fn test_derive_seed_empty_name() {
        // Should still produce a valid u64; just checks it doesn't panic.
        let seed = derive_seed(0, "");
        // The seed should be non-zero (hash of 0u64 + empty string), but
        // the exact value is hasher-dependent. Just verify determinism.
        assert_eq!(seed, derive_seed(0, ""));
    }
}
