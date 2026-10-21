//! Unit newtypes: lengths in the model are `Metres`, never a bare `f64`.
//!
//! A `Metres` value is always finite, within `±MAX_ABS_METRES`, and rounded to the
//! centimetre, so it serialises and compares the same way on every run (ADR-011, ADR-020).

use std::cmp::Ordering;
use std::fmt;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Largest absolute length `Metres::new` accepts, one million kilometres.
pub const MAX_ABS_METRES: f64 = 1e9;

/// Quantisation steps per metre: values are rounded to 1 / 100 m, the ADR-020 quantum.
pub const STEPS_PER_METRE: f64 = 100.0;

/// A length in metres: finite, within `±MAX_ABS_METRES`, rounded to the centimetre.
#[derive(Clone, Copy, Debug)]
pub struct Metres(f64);

/// Why a float could not become a `Metres` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitError {
    /// The value was NaN, positive infinity or negative infinity.
    NotFinite,
    /// The value was finite but larger than `MAX_ABS_METRES` in absolute size.
    OutOfRange,
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitError::NotFinite => write!(f, "length is NaN or infinite"),
            UnitError::OutOfRange => {
                write!(f, "length is outside ±{MAX_ABS_METRES} m")
            }
        }
    }
}

impl std::error::Error for UnitError {}

impl Metres {
    /// Checks `x` and rounds it to the centimetre, half away from zero.
    ///
    /// # Errors
    ///
    /// `UnitError::NotFinite` for NaN or an infinity, `UnitError::OutOfRange` when
    /// `|x| > MAX_ABS_METRES`.
    pub fn new(x: f64) -> Result<Metres, UnitError> {
        if !x.is_finite() {
            return Err(UnitError::NotFinite);
        }
        if x.abs() > MAX_ABS_METRES {
            return Err(UnitError::OutOfRange);
        }
        Ok(Metres((x * STEPS_PER_METRE).round() / STEPS_PER_METRE))
    }

    /// The stored, already rounded value in metres.
    pub fn get(self) -> f64 {
        self.0
    }
}

impl PartialEq for Metres {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Metres {}

impl PartialOrd for Metres {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Metres {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Serialize for Metres {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for Metres {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Metres, D::Error> {
        let x = f64::deserialize(deserializer)?;
        Metres::new(x).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn metres(x: f64) -> Metres {
        Metres::new(x).expect("test input is a valid length")
    }

    fn json_round_trip(m: Metres) -> Metres {
        let text = serde_json::to_string(&m).expect("a Metres value always serialises");
        serde_json::from_str(&text).expect("a serialised Metres value always parses")
    }

    #[test]
    fn nan_is_rejected() {
        assert_eq!(Metres::new(f64::NAN), Err(UnitError::NotFinite));
    }

    #[test]
    fn infinities_are_rejected() {
        assert_eq!(Metres::new(f64::INFINITY), Err(UnitError::NotFinite));
        assert_eq!(Metres::new(f64::NEG_INFINITY), Err(UnitError::NotFinite));
    }

    #[test]
    fn values_beyond_the_limit_are_rejected() {
        assert_eq!(Metres::new(1e9 + 1.0), Err(UnitError::OutOfRange));
        assert_eq!(Metres::new(-1e9 - 1.0), Err(UnitError::OutOfRange));
    }

    #[test]
    fn the_limit_itself_is_accepted() {
        assert_eq!(metres(1e9).get().to_bits(), 1e9_f64.to_bits());
    }

    #[test]
    fn half_a_centimetre_rounds_away_from_zero() {
        assert_eq!(metres(0.005).get().to_bits(), 0.01_f64.to_bits());
        assert_eq!(metres(-0.005).get().to_bits(), (-0.01_f64).to_bits());
    }

    #[test]
    fn a_negative_value_round_trips() {
        let m = metres(-12.34);
        assert_eq!(json_round_trip(m).get().to_bits(), m.get().to_bits());
    }

    #[test]
    fn ordering_follows_the_number_line() {
        assert!(metres(-1.0) < metres(0.0));
        assert!(metres(0.0) < metres(2.5));
        assert_eq!(metres(2.5), metres(2.5));
    }

    #[test]
    fn fixed_values_round_trip_through_json() {
        for x in [0.0, 0.01, 1.0, 123.45, -987.65, 1e9, -1e9] {
            let m = metres(x);
            assert_eq!(json_round_trip(m).get().to_bits(), m.get().to_bits());
        }
    }

    #[test]
    fn invalid_json_value_is_a_serde_error() {
        let result: Result<Metres, _> = serde_json::from_str("1e10");
        assert!(result.is_err());
    }

    proptest! {
        #[test]
        fn json_round_trip_keeps_the_bits(x in -1e9..=1e9_f64) {
            let m = metres(x);
            prop_assert_eq!(json_round_trip(m).get().to_bits(), m.get().to_bits());
        }
    }
}
