//! What a wall reads from its fields (ADR-063).

use crate::support::{length, wall};

#[test]
fn a_separate_ring_without_a_gap_gets_15_metres() {
    let mut inner = wall("keep_wall", &["old_town"]);
    inner.separate_ring = true;
    assert_eq!(inner.gap(), Some(length(15.0)));
}

#[test]
fn a_separate_ring_with_a_gap_keeps_its_own() {
    let mut inner = wall("keep_wall", &["old_town"]);
    inner.separate_ring = true;
    inner.gap_metres = Some(length(8.0));
    assert_eq!(inner.gap(), Some(length(8.0)));
}

#[test]
fn a_wall_that_shares_the_outer_line_has_no_gap() {
    assert_eq!(wall("keep_wall", &["old_town"]).gap(), None);
}
