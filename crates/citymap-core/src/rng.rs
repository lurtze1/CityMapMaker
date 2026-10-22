//! Seed mixing for the hashed per-object random streams (ADR-018).
//!
//! Every random value in the generator comes from a stream seeded by [`splitmix64`], so the
//! same master seed, stage and object always give the same numbers.

use std::num::NonZeroU64;

use rand_pcg::Pcg64Mcg;
use rand_pcg::rand_core::Rng;

use crate::id::Id;

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

/// The master seed of one generation run, as given on the command line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seed(u64);

impl Seed {
    /// Wraps a master seed.
    pub fn new(value: u64) -> Seed {
        Seed(value)
    }

    /// The master seed as a number.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// One object's own random stream: a `Pcg64Mcg` generator behind our own type (ADR-018,
/// ADR-032).
///
/// It is deliberately not `Clone`: a copied stream would repeat the same numbers for two
/// users, and no stage ever shares a stream (ARCHITECTURE.md §5).
#[derive(Debug)]
pub struct RngStream {
    generator: Pcg64Mcg,
}

impl RngStream {
    /// The next 64 random bits of this stream.
    pub fn next_u64(&mut self) -> u64 {
        self.generator.next_u64()
    }

    /// A decimal number from 0 up to but not including 1, from the top 53 bits of one draw,
    /// so every one of the 2^53 possible values is exact and equally likely (ADR-033).
    pub fn next_unit_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * UNIT_STEP
    }

    /// A whole number from 0 up to but not including `bound`, each value equally likely
    /// (Lemire 2019, ADR-033). An uneven draw is thrown away and drawn again, so one call
    /// can use more than one draw of the stream.
    pub fn next_below(&mut self, bound: NonZeroU64) -> u64 {
        let bound = bound.get();
        let uneven = bound.wrapping_neg() % bound;
        loop {
            let product = u128::from(self.next_u64()) * u128::from(bound);
            if product as u64 >= uneven {
                return (product >> 64) as u64;
            }
        }
    }
}

const UNIT_STEP: f64 = 1.0 / 9_007_199_254_740_992.0;

/// The random stream of object `id` in stage `stage` for master seed `seed`.
///
/// `stage` is the stage label of ARCHITECTURE.md §4, such as `"placement"` or
/// `"pack:peace-and-war"`. Equal inputs always give the same stream; a different seed,
/// stage or id gives an unrelated one.
pub fn rng_for(seed: Seed, stage: &str, id: Id) -> RngStream {
    let stage_salt = absorb_bytes(0, stage.as_bytes());
    let narrow = splitmix64(seed.value(), stage_salt, id.value());
    let wide = (u128::from(narrow) << 64) | u128::from(mix64(narrow));
    RngStream {
        generator: Pcg64Mcg::new(wide),
    }
}

/// Mixes one 64-bit word into a running hash state.
pub(crate) fn absorb(state: u64, word: u64) -> u64 {
    mix64(state ^ word)
}

/// Mixes a byte string into a running hash state: its length first, then its bytes in
/// 8-byte little-endian words, the last one padded with zeros.
pub(crate) fn absorb_bytes(state: u64, bytes: &[u8]) -> u64 {
    let mut state = absorb(state, length_word(bytes.len()));
    for chunk in bytes.chunks(8) {
        let mut word = [0u8; 8];
        word[..chunk.len()].copy_from_slice(chunk);
        state = absorb(state, u64::from_le_bytes(word));
    }
    state
}

/// A slice length as a 64-bit word for hashing.
pub(crate) fn length_word(length: usize) -> u64 {
    u64::try_from(length).expect("invariant: a slice length fits in u64 on every target")
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use std::num::NonZeroU64;

    use super::{GOLDEN_GAMMA, RngStream, Seed, mix64, rng_for, splitmix64};
    use crate::id::{Id, IdKind};

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

    fn landmark(name: &str) -> Id {
        Id::new(IdKind::Landmark, &[], &[name])
    }

    fn draws(seed: u64, stage: &str, id: Id) -> [u64; 16] {
        let mut stream = rng_for(Seed::new(seed), stage, id);
        std::array::from_fn(|_| stream.next_u64())
    }

    #[test]
    fn first_draws_are_pinned() {
        let mut stream = rng_for(Seed::new(1), "placement", landmark("market"));
        assert_eq!(stream.next_u64(), 0x59DD_D13A_34C8_EA45);
        assert_eq!(stream.next_u64(), 0xD791_D68E_18D7_E061);
        assert_eq!(stream.next_u64(), 0xE3CB_E1A7_9C98_1BCD);
    }

    #[test]
    fn seed_stage_and_id_each_change_the_stream() {
        let base = draws(1, "placement", landmark("market"));
        assert_ne!(base, draws(2, "placement", landmark("market")));
        assert_ne!(base, draws(1, "labels", landmark("market")));
        assert_ne!(base, draws(1, "placement", landmark("temple")));
    }

    fn stream(seed: u64, stage: &str) -> RngStream {
        rng_for(Seed::new(seed), stage, landmark("market"))
    }

    fn bound(n: u64) -> NonZeroU64 {
        NonZeroU64::new(n).expect("invariant: test bounds are not zero")
    }

    #[test]
    fn first_unit_draws_are_pinned() {
        let mut stream = stream(1, "placement");
        assert_eq!(
            stream.next_unit_f64().to_bits(),
            0.351_040_913_305_755_4_f64.to_bits()
        );
        assert_eq!(
            stream.next_unit_f64().to_bits(),
            0.842_069_062_892_590_6_f64.to_bits()
        );
        assert_eq!(
            stream.next_unit_f64().to_bits(),
            0.889_829_734_249_901_f64.to_bits()
        );
    }

    #[test]
    fn first_below_draws_are_pinned() {
        let mut stream = stream(1, "labels");
        let picks: Vec<u64> = (0..5).map(|_| stream.next_below(bound(6))).collect();
        assert_eq!(picks, [4, 3, 3, 2, 5]);
    }

    #[test]
    fn uneven_draws_are_drawn_again() {
        let mut stream = stream(1, "labels");
        assert_eq!(
            stream.next_below(bound((1 << 63) + 1)),
            4_297_050_426_214_607_501
        );
    }

    #[test]
    fn a_bound_of_one_always_gives_zero() {
        let mut stream = stream(1, "labels");
        for _ in 0..100 {
            assert_eq!(stream.next_below(bound(1)), 0);
        }
    }

    #[test]
    fn six_thousand_draws_below_six_spread_evenly() {
        let mut stream = stream(1, "spread");
        let mut counts = [0u32; 6];
        for _ in 0..6000 {
            let pick = usize::try_from(stream.next_below(bound(6)))
                .expect("invariant: a value below 6 fits in usize");
            counts[pick] += 1;
        }
        for count in counts {
            assert!((800..=1200).contains(&count), "counts {counts:?}");
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn unit_draws_lie_in_zero_to_one(seed in any::<u64>()) {
            let mut stream = stream(seed, "placement");
            for _ in 0..16 {
                let x = stream.next_unit_f64();
                prop_assert!((0.0..1.0).contains(&x));
            }
        }

        #[test]
        fn below_draws_stay_below_the_bound(seed in any::<u64>(), n in 1u64..) {
            let mut stream = stream(seed, "placement");
            for _ in 0..16 {
                prop_assert!(stream.next_below(bound(n)) < n);
            }
        }

        #[test]
        fn equal_inputs_give_equal_streams(
            seed in any::<u64>(),
            stage in "[a-z:_-]{1,24}",
            name in "[a-z_]{1,24}",
        ) {
            prop_assert_eq!(
                draws(seed, &stage, landmark(&name)),
                draws(seed, &stage, landmark(&name))
            );
        }

        #[test]
        fn different_ids_give_different_streams(
            seed in any::<u64>(),
            stage in "[a-z:_-]{1,24}",
            first in "[a-z_]{1,24}",
            second in "[a-z_]{1,24}",
        ) {
            prop_assume!(first != second);
            prop_assert_ne!(
                draws(seed, &stage, landmark(&first)),
                draws(seed, &stage, landmark(&second))
            );
        }

        #[test]
        fn different_stages_give_different_streams(
            seed in any::<u64>(),
            first in "[a-z:_-]{1,24}",
            second in "[a-z:_-]{1,24}",
            name in "[a-z_]{1,24}",
        ) {
            prop_assume!(first != second);
            prop_assert_ne!(
                draws(seed, &first, landmark(&name)),
                draws(seed, &second, landmark(&name))
            );
        }
    }
}
