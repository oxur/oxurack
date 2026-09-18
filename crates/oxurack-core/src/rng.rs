//! Deterministic seed derivation and module-local RNG construction.
//!
//! Each module instance derives its own seed from a master seed and
//! its instance name, ensuring reproducible randomness across runs.
//!
//! [`derive_seed`] produces a `u64` seed, while [`derive_module_rng`]
//! goes one step further and returns a ready-to-use [`SmallRng`].

use rand::SeedableRng;
use rand::rngs::SmallRng;

const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
const FNV_PRIME: u64 = 1_099_511_628_211;
const SEED_DOMAIN: &[u8] = b"oxurack.module-seed.v1";

/// Derive a deterministic seed from the versioned module-seed byte stream.
///
/// The stream is the ASCII domain, one zero byte, the little-endian master
/// seed, the little-endian UTF-8 byte length, and the unnormalised UTF-8 name.
/// This is a non-cryptographic derivation with a compatibility boundary at the
/// locked implementation and dependency versions.
pub fn derive_seed(master_seed: u64, instance_name: &str) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;

    for byte in SEED_DOMAIN
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .chain(master_seed.to_le_bytes())
        .chain((instance_name.len() as u64).to_le_bytes())
        .chain(instance_name.as_bytes().iter().copied())
    {
        hash = (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME);
    }

    hash
}

/// Derive a deterministic [`SmallRng`] for a module instance.
///
/// Combines [`derive_seed`] with [`SmallRng::seed_from_u64`] to
/// produce a fast, non-cryptographic RNG that is fully reproducible for the
/// same `(master_seed, instance_name)` pair under the locked `rand` version.
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
        assert_eq!(derive_seed(0, ""), 0xa854_da73_6b8b_efab);
    }

    #[test]
    fn test_derive_seed_golden_vectors() {
        let vectors = [
            (0, "", 0xa854_da73_6b8b_efab),
            (42, "tm_a", 0x2181_9f1b_fbd4_fcea),
            (42, "tm_b", 0x2181_9e1b_fbd4_fb37),
            (43, "tm_a", 0x5365_fb0e_c2a0_228b),
            (0, "vco_1", 0xfda4_2c6e_50ff_0d2a),
            (u64::MAX, "é", 0x1994_0875_7f0d_299d),
        ];

        for (master_seed, instance_name, expected) in vectors {
            assert_eq!(
                derive_seed(master_seed, instance_name),
                expected,
                "seed mismatch for master={master_seed}, name={instance_name:?}"
            );
        }
    }

    #[test]
    fn test_derive_seed_uses_utf8_byte_length() {
        assert_ne!(
            derive_seed(u64::MAX, "é"),
            derive_seed(u64::MAX, "e\u{301}")
        );
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
