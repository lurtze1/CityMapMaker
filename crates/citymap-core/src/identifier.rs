//! Stable object identifiers: a kind plus a 64-bit hash of the kind, the parent identifiers
//! and the keys. An identifier never depends on a list position, so an object keeps its
//! identifier when other objects are added or removed (ARCHITECTURE.md §5, ADR-030).

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

use serde::{Serialize, Serializer};

use crate::random::{absorb, absorb_bytes, length_word};

/// Which sort of object an identifier names. Each kind has a fixed tag that goes into the hash and
/// the ordering, so adding a kind never changes an existing identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentifierKind {
    /// A point of the street graph, anchored to a cell.
    Node,
    /// An edge of the street graph between two nodes.
    Street,
    /// A face of the street graph.
    Block,
    /// A subdivision of a block.
    Parcel,
    /// A landmark from the spec.
    Landmark,
    /// A district from the spec.
    District,
    /// An open space from the spec: plaza, market square, green, parade ground.
    OpenSpace,
    /// A place where a road crosses a river.
    Crossing,
    /// A road from the spec.
    Road,
    /// A river from the spec.
    River,
}

impl IdentifierKind {
    /// The fixed number that stands for this kind in hashes and in the ordering.
    pub const fn tag(self) -> u64 {
        match self {
            IdentifierKind::Node => 1,
            IdentifierKind::Street => 2,
            IdentifierKind::Block => 3,
            IdentifierKind::Parcel => 4,
            IdentifierKind::Landmark => 5,
            IdentifierKind::District => 6,
            IdentifierKind::OpenSpace => 7,
            IdentifierKind::Crossing => 8,
            IdentifierKind::Road => 9,
            IdentifierKind::River => 10,
        }
    }

    /// The lower-case name used in the text form of an identifier, such as `open_space`.
    pub const fn name(self) -> &'static str {
        match self {
            IdentifierKind::Node => "node",
            IdentifierKind::Street => "street",
            IdentifierKind::Block => "block",
            IdentifierKind::Parcel => "parcel",
            IdentifierKind::Landmark => "landmark",
            IdentifierKind::District => "district",
            IdentifierKind::OpenSpace => "open_space",
            IdentifierKind::Crossing => "crossing",
            IdentifierKind::Road => "road",
            IdentifierKind::River => "river",
        }
    }
}

/// The stable identifier of one object: its kind and a 64-bit hash. Identifiers sort by kind
/// tag, then by hash. The text form is `kind:hash` with the hash as 16 lower-case hex digits,
/// for example `landmark:7f3a09c2e4b1d856`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Identifier {
    kind: IdentifierKind,
    value: u64,
}

impl Identifier {
    /// Derives an identifier from its kind, parent identifiers and keys, in the caller's
    /// canonical order; another order gives another identifier. Each key is hashed with its
    /// length first, so the keys `["ab", "c"]` and `["a", "bc"]` give different identifiers.
    pub fn new(kind: IdentifierKind, parents: &[Identifier], keys: &[&str]) -> Identifier {
        let mut state = absorb(0, kind.tag());
        state = absorb(state, length_word(parents.len()));
        for parent in parents {
            state = absorb(state, parent.kind.tag());
            state = absorb(state, parent.value);
        }
        state = absorb(state, length_word(keys.len()));
        for key in keys {
            state = absorb_bytes(state, key.as_bytes());
        }
        Identifier { kind, value: state }
    }

    /// The kind of object this identifier names.
    pub fn kind(self) -> IdentifierKind {
        self.kind
    }

    /// The 64-bit hash part of the identifier.
    pub fn value(self) -> u64 {
        self.value
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind
            .tag()
            .cmp(&other.kind.tag())
            .then(self.value.cmp(&other.value))
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{:016x}", self.kind.name(), self.value)
    }
}

impl Serialize for Identifier {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// A stage output named the same identifier twice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("duplicate identifier {identifier}")]
pub struct DuplicateIdentifier {
    /// The first identifier that appeared a second time, in input order.
    pub identifier: Identifier,
}

/// Checks that no identifier appears twice in `identifiers`, one stage output
/// (ARCHITECTURE.md §5, ADR-031).
///
/// # Errors
///
/// `DuplicateIdentifier` naming the first identifier, in input order, that appears a second time.
pub fn check_unique(identifiers: &[Identifier]) -> Result<(), DuplicateIdentifier> {
    let mut seen = BTreeSet::new();
    for &identifier in identifiers {
        if !seen.insert(identifier) {
            return Err(DuplicateIdentifier { identifier });
        }
    }
    Ok(())
}

/// Stops the program with the `DuplicateIdentifier` message when `identifiers` contains a
/// duplicate, in debug builds only; release builds skip the check (ARCHITECTURE.md §5).
///
/// # Panics
///
/// In a debug build, when `check_unique(identifiers)` returns an error.
pub fn debug_assert_unique(identifiers: &[Identifier]) {
    if cfg!(debug_assertions)
        && let Err(duplicate) = check_unique(identifiers)
    {
        panic!("{duplicate}");
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{
        DuplicateIdentifier, Identifier, IdentifierKind, check_unique, debug_assert_unique,
    };

    #[test]
    fn known_answers_are_pinned() {
        let market = Identifier::new(IdentifierKind::Landmark, &[], &["market"]);
        assert_eq!(market.value(), KNOWN_MARKET);
        let node_a = Identifier::new(IdentifierKind::Node, &[], &["3", "4"]);
        let node_b = Identifier::new(IdentifierKind::Node, &[], &["3", "5"]);
        let street = Identifier::new(IdentifierKind::Street, &[node_a, node_b], &[]);
        assert_eq!(street.value(), KNOWN_STREET);
    }

    #[test]
    fn equal_inputs_give_equal_identifiers() {
        let first = Identifier::new(IdentifierKind::District, &[], &["docks"]);
        let second = Identifier::new(IdentifierKind::District, &[], &["docks"]);
        assert_eq!(first, second);
    }

    #[test]
    fn the_kind_is_part_of_the_identifier() {
        let landmark = Identifier::new(IdentifierKind::Landmark, &[], &["docks"]);
        let district = Identifier::new(IdentifierKind::District, &[], &["docks"]);
        assert_ne!(landmark.value(), district.value());
        assert_ne!(landmark, district);
    }

    #[test]
    fn key_boundaries_are_part_of_the_identifier() {
        let split_late = Identifier::new(IdentifierKind::Road, &[], &["ab", "c"]);
        let split_early = Identifier::new(IdentifierKind::Road, &[], &["a", "bc"]);
        let joined = Identifier::new(IdentifierKind::Road, &[], &["abc"]);
        assert_ne!(split_late, split_early);
        assert_ne!(split_late, joined);
        assert_ne!(split_early, joined);
    }

    #[test]
    fn parents_tell_children_with_equal_keys_apart() {
        let block_a = Identifier::new(IdentifierKind::Block, &[], &["a"]);
        let block_b = Identifier::new(IdentifierKind::Block, &[], &["b"]);
        let parcel_in_a = Identifier::new(IdentifierKind::Parcel, &[block_a], &["1"]);
        let parcel_in_b = Identifier::new(IdentifierKind::Parcel, &[block_b], &["1"]);
        assert_ne!(parcel_in_a, parcel_in_b);
    }

    #[test]
    fn parent_order_is_part_of_the_identifier() {
        let node_a = Identifier::new(IdentifierKind::Node, &[], &["a"]);
        let node_b = Identifier::new(IdentifierKind::Node, &[], &["b"]);
        let forward = Identifier::new(IdentifierKind::Street, &[node_a, node_b], &[]);
        let backward = Identifier::new(IdentifierKind::Street, &[node_b, node_a], &[]);
        assert_ne!(forward, backward);
    }

    #[test]
    fn identifiers_sort_by_kind_first_then_by_hash() {
        let node = Identifier::new(IdentifierKind::Node, &[], &["z"]);
        let river = Identifier::new(IdentifierKind::River, &[], &["a"]);
        assert!(node < river);
        let mut landmarks = [
            Identifier::new(IdentifierKind::Landmark, &[], &["market"]),
            Identifier::new(IdentifierKind::Landmark, &[], &["temple"]),
        ];
        landmarks.sort();
        assert!(landmarks[0].value() < landmarks[1].value());
    }

    #[test]
    fn text_form_is_kind_colon_sixteen_hex_digits() {
        let market = Identifier::new(IdentifierKind::Landmark, &[], &["market"]);
        assert_eq!(market.to_string(), format!("landmark:{KNOWN_MARKET:016x}"));
        let square = Identifier::new(IdentifierKind::OpenSpace, &[], &["square"]);
        let text = square.to_string();
        assert!(text.starts_with("open_space:"));
        assert_eq!(text.len(), "open_space:".len() + 16);
    }

    fn landmark(name: &str) -> Identifier {
        Identifier::new(IdentifierKind::Landmark, &[], &[name])
    }

    #[test]
    fn an_empty_list_is_unique() {
        assert_eq!(check_unique(&[]), Ok(()));
    }

    #[test]
    fn distinct_identifiers_are_unique() {
        let identifiers = [landmark("market"), landmark("temple"), landmark("gate")];
        assert_eq!(check_unique(&identifiers), Ok(()));
    }

    #[test]
    fn a_duplicate_is_named_in_the_error() {
        let identifiers = [landmark("market"), landmark("temple"), landmark("market")];
        let error = check_unique(&identifiers).unwrap_err();
        assert_eq!(
            error,
            DuplicateIdentifier {
                identifier: landmark("market")
            }
        );
        assert_eq!(
            error.to_string(),
            format!("duplicate identifier {}", landmark("market"))
        );
    }

    #[test]
    fn the_first_repeated_identifier_is_reported() {
        let identifiers = [
            landmark("gate"),
            landmark("temple"),
            landmark("temple"),
            landmark("gate"),
        ];
        assert_eq!(
            check_unique(&identifiers),
            Err(DuplicateIdentifier {
                identifier: landmark("temple")
            })
        );
    }

    #[test]
    fn debug_assert_unique_accepts_distinct_identifiers() {
        debug_assert_unique(&[landmark("market"), landmark("temple")]);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "duplicate identifier landmark:")]
    fn debug_assert_unique_stops_on_a_duplicate() {
        debug_assert_unique(&[landmark("market"), landmark("market")]);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn distinct_landmark_names_never_collide(
            names in prop::collection::btree_set("[a-z_]{1,24}", 0..200)
        ) {
            let identifiers: Vec<Identifier> = names.iter().map(|name| landmark(name)).collect();
            prop_assert_eq!(check_unique(&identifiers), Ok(()));
        }

        #[test]
        fn distinct_cells_never_collide(
            cells in prop::collection::btree_set((-500i32..500, -500i32..500), 0..200)
        ) {
            let identifiers: Vec<Identifier> = cells
                .iter()
                .map(|(x, y)| {
                    Identifier::new(IdentifierKind::Node, &[], &[&x.to_string(), &y.to_string()])
                })
                .collect();
            prop_assert_eq!(check_unique(&identifiers), Ok(()));
        }

        #[test]
        fn parcels_of_one_block_never_collide(count in 0usize..200, block_key in "[a-z]{1,8}") {
            let block = Identifier::new(IdentifierKind::Block, &[], &[&block_key]);
            let identifiers: Vec<Identifier> = (1..=count)
                .map(|k| Identifier::new(IdentifierKind::Parcel, &[block], &[&k.to_string()]))
                .collect();
            prop_assert_eq!(check_unique(&identifiers), Ok(()));
        }
    }

    const KNOWN_MARKET: u64 = 0x0B3A_1C74_F761_82BC;
    const KNOWN_STREET: u64 = 0xDF57_FA4A_DF91_3359;
}
