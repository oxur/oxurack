//! Deterministic current-rack query helpers.

use std::collections::BTreeSet;

use oxurack_core::Tag;

use crate::TagMatch;

pub(super) fn tags_match(actual: &BTreeSet<Tag>, wanted: &BTreeSet<Tag>, mode: TagMatch) -> bool {
    match mode {
        TagMatch::Any => actual.iter().any(|tag| wanted.contains(tag)),
        TagMatch::All => wanted.iter().all(|tag| actual.contains(tag)),
    }
}
