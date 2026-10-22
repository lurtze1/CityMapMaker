//! Stable object ids: a kind plus a 64-bit hash of the kind, the parent ids and the keys.
//!
//! An `Id` names one object in a stage output. It never depends on a list position, so an
//! object keeps its id when other objects are added or removed (ARCHITECTURE.md §5, ADR-030).

use std::cmp::Ordering;
use std::fmt;

use crate::rng::mix64;

/// Which sort of object an id names.
///
/// Each kind has a fixed tag that goes into the hash and the ordering, so adding a kind
/// never changes an existing id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdKind {
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

impl IdKind {
    /// The fixed number that stands for this kind in hashes and in the ordering.
    pub const fn tag(self) -> u64 {
        match self {
            IdKind::Node => 1,
            IdKind::Street => 2,
            IdKind::Block => 3,
            IdKind::Parcel => 4,
            IdKind::Landmark => 5,
            IdKind::District => 6,
            IdKind::OpenSpace => 7,
            IdKind::Crossing => 8,
            IdKind::Road => 9,
            IdKind::River => 10,
        }
    }

    /// The lower-case name used in the text form of an id, such as `open_space`.
    pub const fn name(self) -> &'static str {
        match self {
            IdKind::Node => "node",
            IdKind::Street => "street",
            IdKind::Block => "block",
            IdKind::Parcel => "parcel",
            IdKind::Landmark => "landmark",
            IdKind::District => "district",
            IdKind::OpenSpace => "open_space",
            IdKind::Crossing => "crossing",
            IdKind::Road => "road",
            IdKind::River => "river",
        }
    }
}

/// The stable id of one object: its kind and a 64-bit hash.
///
/// Two ids are equal when kind and hash are equal. Ids sort by kind tag, then by hash.
/// The text form is `kind:hash` with the hash as 16 lower-case hex digits, for example
/// `landmark:7f3a09c2e4b1d856`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Id {
    kind: IdKind,
    value: u64,
}

impl Id {
    /// Derives the id of an object from its kind, its parent ids and its keys.
    ///
    /// The caller passes `parents` and `keys` in a canonical order; a different order gives
    /// a different id. Each key goes into the hash with its length in front, so the keys
    /// `["ab", "c"]` and `["a", "bc"]` give different ids.
    pub fn new(kind: IdKind, parents: &[Id], keys: &[&str]) -> Id {
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
        Id { kind, value: state }
    }

    /// The kind of object this id names.
    pub fn kind(self) -> IdKind {
        self.kind
    }

    /// The 64-bit hash part of the id.
    pub fn value(self) -> u64 {
        self.value
    }
}

fn absorb(state: u64, word: u64) -> u64 {
    mix64(state ^ word)
}

fn absorb_bytes(state: u64, bytes: &[u8]) -> u64 {
    let mut state = absorb(state, length_word(bytes.len()));
    for chunk in bytes.chunks(8) {
        let mut word = [0u8; 8];
        word[..chunk.len()].copy_from_slice(chunk);
        state = absorb(state, u64::from_le_bytes(word));
    }
    state
}

fn length_word(length: usize) -> u64 {
    u64::try_from(length).expect("invariant: a slice length fits in u64 on every target")
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind
            .tag()
            .cmp(&other.kind.tag())
            .then(self.value.cmp(&other.value))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{:016x}", self.kind.name(), self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Id, IdKind};

    #[test]
    fn known_answers_are_pinned() {
        let market = Id::new(IdKind::Landmark, &[], &["market"]);
        assert_eq!(market.value(), KNOWN_MARKET);
        let node_a = Id::new(IdKind::Node, &[], &["3", "4"]);
        let node_b = Id::new(IdKind::Node, &[], &["3", "5"]);
        let street = Id::new(IdKind::Street, &[node_a, node_b], &[]);
        assert_eq!(street.value(), KNOWN_STREET);
    }

    #[test]
    fn equal_inputs_give_equal_ids() {
        let first = Id::new(IdKind::District, &[], &["docks"]);
        let second = Id::new(IdKind::District, &[], &["docks"]);
        assert_eq!(first, second);
    }

    #[test]
    fn the_kind_is_part_of_the_id() {
        let landmark = Id::new(IdKind::Landmark, &[], &["docks"]);
        let district = Id::new(IdKind::District, &[], &["docks"]);
        assert_ne!(landmark.value(), district.value());
        assert_ne!(landmark, district);
    }

    #[test]
    fn key_boundaries_are_part_of_the_id() {
        let split_late = Id::new(IdKind::Road, &[], &["ab", "c"]);
        let split_early = Id::new(IdKind::Road, &[], &["a", "bc"]);
        let joined = Id::new(IdKind::Road, &[], &["abc"]);
        assert_ne!(split_late, split_early);
        assert_ne!(split_late, joined);
        assert_ne!(split_early, joined);
    }

    #[test]
    fn parents_tell_children_with_equal_keys_apart() {
        let block_a = Id::new(IdKind::Block, &[], &["a"]);
        let block_b = Id::new(IdKind::Block, &[], &["b"]);
        let parcel_in_a = Id::new(IdKind::Parcel, &[block_a], &["1"]);
        let parcel_in_b = Id::new(IdKind::Parcel, &[block_b], &["1"]);
        assert_ne!(parcel_in_a, parcel_in_b);
    }

    #[test]
    fn parent_order_is_part_of_the_id() {
        let node_a = Id::new(IdKind::Node, &[], &["a"]);
        let node_b = Id::new(IdKind::Node, &[], &["b"]);
        let forward = Id::new(IdKind::Street, &[node_a, node_b], &[]);
        let backward = Id::new(IdKind::Street, &[node_b, node_a], &[]);
        assert_ne!(forward, backward);
    }

    #[test]
    fn ids_sort_by_kind_first_then_by_hash() {
        let node = Id::new(IdKind::Node, &[], &["z"]);
        let river = Id::new(IdKind::River, &[], &["a"]);
        assert!(node < river);
        let mut landmarks = [
            Id::new(IdKind::Landmark, &[], &["market"]),
            Id::new(IdKind::Landmark, &[], &["temple"]),
        ];
        landmarks.sort();
        assert!(landmarks[0].value() < landmarks[1].value());
    }

    #[test]
    fn text_form_is_kind_colon_sixteen_hex_digits() {
        let market = Id::new(IdKind::Landmark, &[], &["market"]);
        assert_eq!(market.to_string(), format!("landmark:{KNOWN_MARKET:016x}"));
        let square = Id::new(IdKind::OpenSpace, &[], &["square"]);
        let text = square.to_string();
        assert!(text.starts_with("open_space:"));
        assert_eq!(text.len(), "open_space:".len() + 16);
    }

    const KNOWN_MARKET: u64 = 0x0B3A_1C74_F761_82BC;
    const KNOWN_STREET: u64 = 0xDF57_FA4A_DF91_3359;
}
