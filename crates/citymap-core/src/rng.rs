//! Seed mixing for the hashed per-object random streams (ADR-018).
//!
//! Every random value in the generator comes from a stream seeded by [`splitmix64`], so the
//! same master seed, stage and object always give the same numbers.

/// The `SplitMix64` increment: 2^64 divided by the golden ratio, rounded to an odd number.
pub const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// The `SplitMix64` finaliser (Steele, Lea and Flood 2014; Vigna's reference `splitmix64.c`).
///
/// It scrambles all 64 bits of `z` so that inputs differing in one bit give unrelated outputs.
pub const fn mix64(z: u64) -> u64 {
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Combines a master seed, a stage salt and an object id into one 64-bit stream seed.
///
/// The formula is
/// `mix64(mix64(mix64(master_seed.wrapping_add(GOLDEN_GAMMA)) ^ stage_salt) ^ object_id)`.
/// The `Id` type (#6) and `rng_for(stage, id)` (#9) build on this function.
pub fn splitmix64(master_seed: u64, stage_salt: u64, object_id: u64) -> u64 {
    mix64(mix64(mix64(master_seed.wrapping_add(GOLDEN_GAMMA)) ^ stage_salt) ^ object_id)
}

#[cfg(test)]
mod tests {
    use super::{GOLDEN_GAMMA, mix64, splitmix64};

    #[test]
    fn first_output_from_state_zero_matches_reference() {
        assert_eq!(
            mix64(0u64.wrapping_add(GOLDEN_GAMMA)),
            0xE220_A839_7B1D_CDAF
        );
    }

    #[test]
    fn splitmix64_outputs_are_pinned() {
        assert_eq!(splitmix64(0, 0, 0), 0x33FE_8BD4_F9C5_7863);
        assert_eq!(splitmix64(1, 2, 3), 0x9E71_840A_CAA5_F697);
        assert_eq!(splitmix64(u64::MAX, 0xDEAD_BEEF, 42), 0x6CDD_563B_D010_989B);
    }

    #[test]
    fn mix64_gives_distinct_outputs_for_distinct_inputs() {
        let inputs = [0, 1, 2, GOLDEN_GAMMA, u64::MAX];
        let outputs = inputs.map(mix64);
        for (i, a) in outputs.iter().enumerate() {
            for b in &outputs[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }
}
