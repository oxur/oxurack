//! Deterministic seed derivation and module-local RNG construction.
//!
//! Each module instance derives its own seed from a master seed and
//! its instance name, ensuring reproducible randomness across runs.
//!
//! [`derive_seed`] produces a `u64` seed, while [`derive_module_rng`]
//! goes one step further and returns a ready-to-use [`SmallRng`].

use std::hash::{Hash, Hasher};

use ahash::AHasher;
use rand::rngs::SmallRng;
use rand::SeedableRng;

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

/// Derive a deterministic [`SmallRng`] for a module instance.
///
/// Combines [`derive_seed`] with [`SmallRng::seed_from_u64`] to
/// produce a fast, non-cryptographic RNG that is fully reproducible
/// given the same `(master_seed, instance_name)` pair.
///
/// # Examples
///
/// ```
/// use oxurack_core::derive_module_rng;
/// use rand::RngExt;
///
/// let mut rng = derive_module_rng(42, "vco_1");
/// let _value: f32 = rng.random();
/// ```
pub fn derive_module_rng(master_seed: u64, instance_name: &str) -> SmallRng {
    let seed = derive_seed(master_seed, instance_name);
    SmallRng::seed_from_u64(seed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngExt;

    // ── derive_seed ─────────────────────────────────────────────

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

    // ── derive_module_rng ───────────────────────────────────────

    #[test]
    fn test_derive_module_rng_same_inputs_same_sequence() {
        let mut rng_a = derive_module_rng(42, "turing_1");
        let mut rng_b = derive_module_rng(42, "turing_1");

        let seq_a: Vec<f64> = (0..10).map(|_| rng_a.random()).collect();
        let seq_b: Vec<f64> = (0..10).map(|_| rng_b.random()).collect();

        assert_eq!(seq_a, seq_b, "same inputs should produce same sequence");
    }

    #[test]
    fn test_derive_module_rng_different_inputs_different_sequence() {
        let mut rng_a = derive_module_rng(42, "turing_1");
        let mut rng_b = derive_module_rng(42, "turing_2");

        let seq_a: Vec<f64> = (0..10).map(|_| rng_a.random()).collect();
        let seq_b: Vec<f64> = (0..10).map(|_| rng_b.random()).collect();

        assert_ne!(
            seq_a, seq_b,
            "different inputs should produce different sequences"
        );
    }

    #[test]
    fn test_derive_module_rng_different_master_seed() {
        let mut rng_a = derive_module_rng(0, "lfo_1");
        let mut rng_b = derive_module_rng(1, "lfo_1");

        let seq_a: Vec<f64> = (0..10).map(|_| rng_a.random()).collect();
        let seq_b: Vec<f64> = (0..10).map(|_| rng_b.random()).collect();

        assert_ne!(
            seq_a, seq_b,
            "different master seeds should produce different sequences"
        );
    }
}
