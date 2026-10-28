//! Length types, all in metres: `SignedLength` for the x and y of points and vectors, `Length`
//! for sizes and distances (zero or more). Both are finite, rounded to the centimetre and
//! compare the same way on every run (ADR-011, ADR-020, ADR-038). `Gradient` is a fall in
//! metres per kilometre and `Weight` a rule's weight, both with the rules of `Length`
//! (ADR-056, ADR-059). `Rotation` is a turn in degrees, rounded to 0.01 degrees (ADR-061).

use std::cmp::Ordering;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Largest absolute value a length type accepts, one million kilometres.
pub const MAXIMUM_ABSOLUTE_METRES: f64 = 1e9;

/// Quantisation steps per metre: values are rounded to 1 / 100 m, the ADR-020 quantum.
pub const STEPS_PER_METRE: f64 = 100.0;

/// A signed length in metres, such as the x of a point: finite, within
/// `±MAXIMUM_ABSOLUTE_METRES`, rounded to the centimetre.
#[derive(Clone, Copy, Debug)]
pub struct SignedLength(f64);

/// A length in metres that is never negative, such as a width or a distance: finite, at most
/// `MAXIMUM_ABSOLUTE_METRES`, rounded to the centimetre.
#[derive(Clone, Copy, Debug)]
pub struct Length(f64);

/// A fall of the ground in metres per kilometre, such as a slope's: zero or more, with the
/// same range and rounding as `Length`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Gradient(Length);

/// How much a soft rule counts against the other soft rules of its object: zero or more,
/// with the same range and rounding as `Length` (ADR-059).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Weight(Length);

/// A turn in degrees, such as a pinned footprint's, with the range and the 0.01 rounding of
/// `SignedLength`, so it reads back the same on every run (ADR-061). Validation keeps it from
/// 0 up to 360.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Rotation(SignedLength);

/// Why a float could not become a length, a gradient, a weight or a rotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum UnitError {
    /// The value was NaN, positive infinity or negative infinity.
    #[error("value is NaN or infinite")]
    NotFinite,
    /// The value was finite but larger than `MAXIMUM_ABSOLUTE_METRES` in absolute size.
    #[error("value is outside ±{MAXIMUM_ABSOLUTE_METRES}")]
    OutOfRange,
    /// The value was below zero where only zero or more is allowed.
    #[error("value is negative")]
    Negative,
}

impl SignedLength {
    /// Checks `value` and rounds it to the centimetre, half away from zero. A result of
    /// negative zero is stored as `0.0`, so every value that rounds to zero is equal.
    ///
    /// # Errors
    ///
    /// `UnitError::NotFinite` for NaN or an infinity, `UnitError::OutOfRange` when
    /// `|value| > MAXIMUM_ABSOLUTE_METRES`.
    pub fn new(value: f64) -> Result<SignedLength, UnitError> {
        quantise(value).map(SignedLength)
    }

    /// The stored, already rounded value in metres.
    pub fn get(self) -> f64 {
        self.0
    }

    /// The value in whole quantisation steps (centimetres), exact for comparisons.
    pub fn steps(self) -> i64 {
        (self.0 * STEPS_PER_METRE).round() as i64
    }

    /// The size of this signed length, its absolute value, as a `Length`.
    pub fn absolute(self) -> Length {
        Length(self.0.abs())
    }
}

impl Length {
    /// Checks `value` and rounds it to the centimetre, half away from zero. Negative zero
    /// becomes `0.0`; any other value below zero is rejected, never flipped.
    ///
    /// # Errors
    ///
    /// `UnitError::Negative` below zero, `UnitError::NotFinite` for NaN or an infinity,
    /// `UnitError::OutOfRange` above `MAXIMUM_ABSOLUTE_METRES`.
    pub fn new(value: f64) -> Result<Length, UnitError> {
        if value < 0.0 && value.is_finite() {
            return Err(UnitError::Negative);
        }
        quantise(value).map(Length)
    }

    /// The stored, already rounded value in metres.
    pub fn get(self) -> f64 {
        self.0
    }
}

fn quantise(value: f64) -> Result<f64, UnitError> {
    if !value.is_finite() {
        return Err(UnitError::NotFinite);
    }
    if value.abs() > MAXIMUM_ABSOLUTE_METRES {
        return Err(UnitError::OutOfRange);
    }
    let rounded = (value * STEPS_PER_METRE).round() / STEPS_PER_METRE;
    Ok(without_negative_zero(rounded))
}

fn without_negative_zero(value: f64) -> f64 {
    if value.total_cmp(&-0.0) == Ordering::Equal {
        0.0
    } else {
        value
    }
}

impl PartialEq for SignedLength {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for SignedLength {}

impl PartialOrd for SignedLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SignedLength {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Serialize for SignedLength {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for SignedLength {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<SignedLength, D::Error> {
        let value = f64::deserialize(deserializer)?;
        SignedLength::new(value).map_err(D::Error::custom)
    }
}

impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Length {}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Serialize for Length {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for Length {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Length, D::Error> {
        let value = f64::deserialize(deserializer)?;
        Length::new(value).map_err(D::Error::custom)
    }
}

impl Gradient {
    /// Checks `metres_per_kilometre` with the rules of `Length::new`.
    ///
    /// # Errors
    ///
    /// The errors of `Length::new`.
    pub fn new(metres_per_kilometre: f64) -> Result<Gradient, UnitError> {
        Length::new(metres_per_kilometre).map(Gradient)
    }

    /// The stored, already rounded fall in metres per kilometre.
    pub fn metres_per_kilometre(self) -> f64 {
        self.0.get()
    }
}

impl Rotation {
    /// Checks `degrees` with the rules of `SignedLength::new`.
    ///
    /// # Errors
    ///
    /// The errors of `SignedLength::new`.
    pub fn new(degrees: f64) -> Result<Rotation, UnitError> {
        SignedLength::new(degrees).map(Rotation)
    }

    /// The stored, already rounded turn in degrees.
    pub fn degrees(self) -> f64 {
        self.0.get()
    }
}

impl Weight {
    /// The weight a rule has when the file gives none.
    pub const DEFAULT: Weight = Weight(Length(1.0));

    /// Checks `value` with the rules of `Length::new`.
    ///
    /// # Errors
    ///
    /// The errors of `Length::new`.
    pub fn new(value: f64) -> Result<Weight, UnitError> {
        Length::new(value).map(Weight)
    }

    /// The stored, already rounded weight.
    pub fn get(self) -> f64 {
        self.0.get()
    }
}

impl Default for Weight {
    fn default() -> Weight {
        Weight::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn a_rotation_is_rounded_to_a_hundredth_of_a_degree() {
        let rotation = Rotation::new(12.345_6).expect("a finite turn");
        assert_eq!(rotation, Rotation::new(12.35).expect("a finite turn"));
        assert_eq!(Rotation::new(f64::NAN), Err(UnitError::NotFinite));
        assert_eq!(Rotation::new(-0.0).map(Rotation::degrees), Ok(0.0));
    }

    fn signed_length(value: f64) -> SignedLength {
        SignedLength::new(value).expect("test input is a valid length")
    }

    fn json_round_trip(length: SignedLength) -> SignedLength {
        let text = serde_json::to_string(&length).expect("a SignedLength value always serialises");
        serde_json::from_str(&text).expect("a serialised SignedLength value always parses")
    }

    #[test]
    fn nan_is_rejected() {
        assert_eq!(SignedLength::new(f64::NAN), Err(UnitError::NotFinite));
    }

    #[test]
    fn infinities_are_rejected() {
        assert_eq!(SignedLength::new(f64::INFINITY), Err(UnitError::NotFinite));
        assert_eq!(
            SignedLength::new(f64::NEG_INFINITY),
            Err(UnitError::NotFinite)
        );
    }

    #[test]
    fn values_beyond_the_limit_are_rejected() {
        assert_eq!(SignedLength::new(1e9 + 1.0), Err(UnitError::OutOfRange));
        assert_eq!(SignedLength::new(-1e9 - 1.0), Err(UnitError::OutOfRange));
    }

    #[test]
    fn the_limit_itself_is_accepted() {
        assert_eq!(signed_length(1e9).get().to_bits(), 1e9_f64.to_bits());
    }

    #[test]
    fn half_a_centimetre_rounds_away_from_zero() {
        assert_eq!(signed_length(0.005).get().to_bits(), 0.01_f64.to_bits());
        assert_eq!(signed_length(-0.005).get().to_bits(), (-0.01_f64).to_bits());
    }

    #[test]
    fn values_that_round_to_zero_become_positive_zero() {
        for value in [-0.0, -0.001, -0.004_999] {
            assert_eq!(signed_length(value).get().to_bits(), 0.0_f64.to_bits());
            assert_eq!(signed_length(value), signed_length(0.0));
        }
    }

    #[test]
    fn a_negative_value_round_trips() {
        let length = signed_length(-12.34);
        assert_eq!(
            json_round_trip(length).get().to_bits(),
            length.get().to_bits()
        );
    }

    #[test]
    fn ordering_follows_the_number_line() {
        assert!(signed_length(-1.0) < signed_length(0.0));
        assert!(signed_length(0.0) < signed_length(2.5));
        assert_eq!(signed_length(2.5), signed_length(2.5));
    }

    #[test]
    fn fixed_values_round_trip_through_json() {
        for value in [0.0, 0.01, 1.0, 123.45, -987.65, 1e9, -1e9] {
            let length = signed_length(value);
            assert_eq!(
                json_round_trip(length).get().to_bits(),
                length.get().to_bits()
            );
        }
    }

    #[test]
    fn invalid_json_value_is_a_serde_error() {
        let result: Result<SignedLength, _> = serde_json::from_str("1e10");
        assert!(result.is_err());
    }

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid non-negative length")
    }

    #[test]
    fn a_negative_length_is_rejected_not_flipped() {
        assert_eq!(Length::new(-18.0), Err(UnitError::Negative));
        assert_eq!(Length::new(-0.001), Err(UnitError::Negative));
    }

    #[test]
    fn length_accepts_zero_and_negative_zero() {
        assert_eq!(length(0.0), length(-0.0));
        assert_eq!(length(-0.0).get().to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn length_shares_the_other_checks() {
        assert_eq!(Length::new(f64::NAN), Err(UnitError::NotFinite));
        assert_eq!(Length::new(f64::NEG_INFINITY), Err(UnitError::NotFinite));
        assert_eq!(Length::new(1e9 + 1.0), Err(UnitError::OutOfRange));
        assert_eq!(length(0.005).get().to_bits(), 0.01_f64.to_bits());
    }

    #[test]
    fn a_negative_length_in_json_is_a_serde_error() {
        let result: Result<Length, _> = serde_json::from_str("-18");
        assert!(result.is_err());
    }

    #[test]
    fn the_absolute_value_of_a_signed_length_is_a_length() {
        assert_eq!(signed_length(-12.34).absolute(), length(12.34));
        assert_eq!(signed_length(5.0).absolute(), length(5.0));
    }

    proptest! {
        #[test]
        fn length_json_round_trip_keeps_the_bits(value in 0.0..=1e9_f64) {
            let original = length(value);
            let text = serde_json::to_string(&original).expect("a Length always serialises");
            let read: Length = serde_json::from_str(&text).expect("a serialised Length parses");
            prop_assert_eq!(read.get().to_bits(), original.get().to_bits());
        }

        #[test]
        fn absolute_is_never_negative_and_keeps_the_size(value in -1e9..=1e9_f64) {
            let absolute = signed_length(value).absolute();
            prop_assert!(absolute.get() >= 0.0);
            prop_assert_eq!(absolute.get().to_bits(), signed_length(value).get().abs().to_bits());
        }

        #[test]
        fn json_round_trip_keeps_the_bits(value in -1e9..=1e9_f64) {
            let length = signed_length(value);
            prop_assert_eq!(json_round_trip(length).get().to_bits(), length.get().to_bits());
        }
    }
}
