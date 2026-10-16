//! Bounded facade-owned observation records and independent pull cursors.

use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;

use oxurack_core::{ChangeSource, LogicalTarget, OwnedChange};

/// Opaque one-based sequence number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObservationSequence(u64);
impl ObservationSequence {
    /// Numeric sequence value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}
/// Opaque one-based batch identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObservationBatchId(u64);
impl ObservationBatchId {
    /// Numeric batch value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}
/// Opaque patch-lifetime identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LifetimeId(u64);
impl LifetimeId {
    /// Numeric lifetime value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Origin retained for the current patch lifetime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchOrigin {
    /// Exact supplied path, without canonicalization.
    File(PathBuf),
    /// In-memory configuration.
    InMemory,
}
/// Current patch identity and selected configuration metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct PatchInfo {
    /// Current lifetime.
    pub lifetime: LifetimeId,
    /// Load origin.
    pub origin: PatchOrigin,
    /// Patch schema version.
    pub version: String,
    /// Seed used to initialize modules.
    pub master_seed: u64,
    /// Current BPM.
    pub bpm: f32,
}
/// Reason an old patch lifetime ended.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnloadReason {
    /// Replaced by another patch.
    Replace,
    /// Rebuilt from current snapshot.
    Restart,
    /// Explicitly unloaded.
    ExplicitUnload,
}
/// One observed state change.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum ObservationChange {
    /// Actual core producer event.
    Core(OwnedChange),
    /// Retired patch boundary.
    PatchUnloaded {
        old: PatchInfo,
        successor: LifetimeId,
        reason: UnloadReason,
    },
    /// Reset accepted.
    Reset { source: ChangeSource },
    /// Running state changed.
    RunningChanged { previous: bool, running: bool },
    /// BPM accepted.
    GlobalBpmChanged {
        previous: f32,
        bpm: f32,
        source: ChangeSource,
    },
}
/// One record in the ordered stream.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    /// Global sequence.
    pub sequence: ObservationSequence,
    /// Batch identity.
    pub batch: ObservationBatchId,
    /// True for the final record in this batch.
    pub last_in_batch: bool,
    /// Lifetime of this record.
    pub lifetime: LifetimeId,
    /// Sample boundary.
    pub sample: u64,
    /// Payload.
    pub change: ObservationChange,
}
/// Initial position for a reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationStart {
    /// Begin after the newest record.
    Now,
    /// Begin at the oldest retained record.
    OldestRetained,
}
/// Exact half-open interval omitted by retention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObservationGap {
    /// First missing sequence.
    pub from: ObservationSequence,
    /// First sequence after the gap.
    pub to: ObservationSequence,
}
/// One bounded read result.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationPage {
    /// Missing interval, if this cursor fell behind.
    pub gap: Option<ObservationGap>,
    /// At most the requested number of records.
    pub records: Vec<Observation>,
}
/// Typed failure for observation reads and counters.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationError {
    /// Cursor belongs to another rack.
    WrongFacade,
    /// Monotone identity space is exhausted.
    CounterExhausted { counter: ObservationCounter },
}
/// Counter that exhausted its checked identity space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationCounter {
    /// Record sequence.
    Sequence,
    /// Batch identity.
    Batch,
    /// Rack lifetime.
    Lifetime,
    /// Cable identity.
    CableId,
    /// Core staging position.
    StagePosition,
}
impl std::fmt::Display for ObservationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongFacade => f.write_str("observation cursor belongs to another rack"),
            Self::CounterExhausted { counter } => {
                write!(f, "observation unavailable: {counter:?} counter exhausted")
            }
        }
    }
}
impl std::error::Error for ObservationError {}

/// Independent reader position; clones share identity but copy position.
#[derive(Debug, Clone)]
pub struct ObservationCursor {
    facade: Arc<()>,
    next: u64,
}
/// Cable details with their owning lifetime.
#[derive(Debug, Clone, PartialEq)]
pub struct CableInfo {
    /// Lifetime containing this cable.
    pub lifetime: LifetimeId,
    /// Logical descriptor.
    pub cable: oxurack_core::ObservedCable,
}
/// Target with its owning lifetime.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationTarget {
    /// Lifetime containing this target.
    pub lifetime: LifetimeId,
    /// Logical target.
    pub target: LogicalTarget,
}
/// Exact tag-query matching rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagMatch {
    /// At least one requested tag.
    Any,
    /// Every requested tag.
    All,
}
/// Typed tag-query error.
#[non_exhaustive]
#[derive(Debug)]
pub enum TagQueryError {
    /// No tags supplied.
    EmptyQuery,
    /// One input failed core validation.
    InvalidTag {
        tag: String,
        source: oxurack_core::TagError,
    },
    /// No loaded patch.
    Core(oxurack_core::CoreError),
    /// Observation unavailable.
    Observation(ObservationError),
}

impl std::fmt::Display for TagQueryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyQuery => formatter.write_str("at least one tag is required"),
            Self::InvalidTag { tag, source } => write!(formatter, "invalid tag {tag:?}: {source}"),
            Self::Core(error) => write!(formatter, "tag query failed: {error}"),
            Self::Observation(error) => write!(formatter, "tag query unavailable: {error}"),
        }
    }
}
impl std::error::Error for TagQueryError {}

#[derive(Debug)]
pub(super) struct Journal {
    pub(super) records: VecDeque<Observation>,
    pub(super) next_sequence: u64,
    pub(super) next_batch: u64,
    pub(super) next_lifetime: u64,
    pub(super) lifetime: LifetimeId,
    pub(super) current: Option<PatchInfo>,
    identity: Arc<()>,
    unavailable_counter: Option<ObservationCounter>,
}
impl Default for Journal {
    fn default() -> Self {
        Self {
            records: VecDeque::new(),
            next_sequence: 1,
            next_batch: 1,
            next_lifetime: 1,
            lifetime: LifetimeId(1),
            current: None,
            identity: Arc::new(()),
            unavailable_counter: None,
        }
    }
}
impl Journal {
    pub(super) fn mark_unavailable(&mut self, counter: ObservationCounter) {
        self.unavailable_counter = Some(counter);
    }
    fn unavailable_error(&self) -> Option<ObservationError> {
        self.unavailable_counter
            .map(|counter| ObservationError::CounterExhausted { counter })
    }
    pub(super) fn cursor(&self, start: ObservationStart) -> ObservationCursor {
        let next = match start {
            ObservationStart::Now => self.next_sequence,
            ObservationStart::OldestRetained => self
                .records
                .front()
                .map_or(self.next_sequence, |r| r.sequence.0),
        };
        ObservationCursor {
            facade: Arc::clone(&self.identity),
            next,
        }
    }
    pub(super) fn read(
        &self,
        cursor: &mut ObservationCursor,
        limit: NonZeroUsize,
    ) -> Result<ObservationPage, ObservationError> {
        if let Some(counter) = self.unavailable_counter {
            return Err(ObservationError::CounterExhausted { counter });
        }
        if !Arc::ptr_eq(&self.identity, &cursor.facade) {
            return Err(ObservationError::WrongFacade);
        }
        let oldest = self
            .records
            .front()
            .map_or(self.next_sequence, |r| r.sequence.0);
        let gap = (cursor.next < oldest).then_some(ObservationGap {
            from: ObservationSequence(cursor.next),
            to: ObservationSequence(oldest),
        });
        let start = cursor.next.max(oldest);
        let records: Vec<_> = self
            .records
            .iter()
            .filter(|r| r.sequence.0 >= start)
            .take(limit.get())
            .cloned()
            .collect();
        let next = records
            .last()
            .map_or(start, |r| r.sequence.0.saturating_add(1));
        cursor.next = next;
        Ok(ObservationPage { gap, records })
    }
    pub(super) fn append(
        &mut self,
        lifetime: LifetimeId,
        sample: u64,
        batch: ObservationBatchId,
        changes: Vec<ObservationChange>,
    ) -> Result<(), ObservationError> {
        let end = self.next_sequence.checked_add(changes.len() as u64).ok_or(
            ObservationError::CounterExhausted {
                counter: ObservationCounter::Sequence,
            },
        )?;
        for (index, change) in changes.into_iter().enumerate() {
            let sequence = self.next_sequence + index as u64;
            self.records.push_back(Observation {
                sequence: ObservationSequence(sequence),
                batch,
                last_in_batch: sequence + 1 == end,
                lifetime,
                sample,
                change,
            });
            if self.records.len() > 1024 {
                self.records.pop_front();
            }
        }
        self.next_sequence = end;
        Ok(())
    }
    pub(super) fn next_lifetime(&mut self) -> Option<LifetimeId> {
        if self.unavailable_counter.is_some() {
            return None;
        }
        let current = LifetimeId(self.next_lifetime);
        let Some(next) = self.next_lifetime.checked_add(1) else {
            self.mark_unavailable(ObservationCounter::Lifetime);
            return None;
        };
        self.next_lifetime = next;
        self.lifetime = current;
        Some(current)
    }
    pub(super) fn publish(
        &mut self,
        lifetime: LifetimeId,
        sample: u64,
        changes: Vec<ObservationChange>,
    ) {
        if self.unavailable_counter.is_some() {
            return;
        }
        let batch = ObservationBatchId(self.next_batch);
        let Some(next) = self.next_batch.checked_add(1) else {
            self.mark_unavailable(ObservationCounter::Batch);
            return;
        };
        if self.append(lifetime, sample, batch, changes).is_ok() {
            self.next_batch = next;
        } else {
            self.mark_unavailable(ObservationCounter::Sequence);
        }
    }
    pub(super) fn publish_staged(
        &mut self,
        lifetime: LifetimeId,
        sample: u64,
        prefix: Option<ObservationChange>,
        staged: Vec<oxurack_core::StagedChange>,
        emitted: u64,
        unavailable: bool,
    ) {
        if unavailable {
            self.mark_unavailable(ObservationCounter::StagePosition);
            return;
        }
        if self.unavailable_counter.is_some() {
            return;
        }
        let batch = ObservationBatchId(self.next_batch);
        let Some(next_batch) = self.next_batch.checked_add(1) else {
            self.mark_unavailable(ObservationCounter::Batch);
            return;
        };
        let prefix_count = u64::from(prefix.is_some());
        let Some(end) = self
            .next_sequence
            .checked_add(prefix_count)
            .and_then(|n| n.checked_add(emitted))
        else {
            self.mark_unavailable(ObservationCounter::Sequence);
            return;
        };
        let base = self.next_sequence;
        let mut pending = Vec::with_capacity(staged.len() + usize::from(prefix.is_some()));
        if let Some(change) = prefix {
            pending.push((base, change));
        }
        pending.extend(staged.into_iter().map(|item| {
            (
                base + prefix_count + item.position - 1,
                ObservationChange::Core(item.change),
            )
        }));
        for (sequence, change) in pending {
            self.records.push_back(Observation {
                sequence: ObservationSequence(sequence),
                batch,
                last_in_batch: sequence + 1 == end,
                lifetime,
                sample,
                change,
            });
            if self.records.len() > 1024 {
                self.records.pop_front();
            }
        }
        self.next_sequence = end;
        self.next_batch = next_batch;
    }
}

impl super::Oxurack {
    pub(super) fn flush_staging(&mut self) {
        let (changes, unavailable) = {
            let Some(mut staging) = self
                .app
                .world_mut()
                .get_resource_mut::<oxurack_core::ObservationStaging>()
            else {
                return;
            };
            let (_, records, unavailable) = staging.take_packet();
            (
                records
                    .into_iter()
                    .map(|record| ObservationChange::Core(record.change))
                    .collect::<Vec<_>>(),
                unavailable,
            )
        };
        if !unavailable && !changes.is_empty() {
            self.observation.publish(
                self.observation.lifetime,
                self.sample_time().completed_samples(),
                changes,
            );
        } else if unavailable {
            self.observation
                .mark_unavailable(ObservationCounter::StagePosition);
        }
    }
    /// Creates an independent observation reader at the requested position.
    pub fn observe(&self, start: ObservationStart) -> Result<ObservationCursor, ObservationError> {
        if let Some(error) = self.observation.unavailable_error() {
            return Err(error);
        }
        Ok(self.observation.cursor(start))
    }
    /// Reads at most `limit` retained records and advances only this cursor.
    pub fn read_observations(
        &self,
        cursor: &mut ObservationCursor,
        limit: NonZeroUsize,
    ) -> Result<ObservationPage, ObservationError> {
        self.observation.read(cursor, limit)
    }
    /// Returns current patch identity, if a patch is loaded.
    pub fn patch_info(&self) -> Result<Option<PatchInfo>, ObservationError> {
        if let Some(error) = self.observation.unavailable_error() {
            return Err(error);
        }
        Ok(self.observation.current.clone())
    }
    /// Reports loaded membership independently of journal availability.
    #[must_use]
    pub fn has_patch(&self) -> bool {
        self.loaded_identity.is_some()
    }

    /// Lists live cables in patch-vector order with their stable IDs.
    pub fn cables(&mut self) -> Result<Vec<CableInfo>, ObservationError> {
        if let Some(error) = self.observation.unavailable_error() {
            return Err(error);
        }
        let lifetime = self.observation.lifetime;
        let world = self.app.world_mut();
        let mut query = world.query::<(
            bevy_ecs::prelude::Entity,
            &oxurack_core::Cable,
            Option<&oxurack_core::CableOrder>,
        )>();
        let mut entities: Vec<_> = query
            .iter(world)
            .map(|(entity, _, order)| (order.map_or(usize::MAX, |order| order.0), entity))
            .collect();
        entities.sort_unstable();
        Ok(entities
            .into_iter()
            .filter_map(|(_, entity)| {
                oxurack_core::observation::describe_cable(world, entity)
                    .map(|cable| CableInfo { lifetime, cable })
            })
            .collect())
    }

    /// Finds rack, module, and cable labels using exact validated tag strings.
    pub fn find_tags(
        &mut self,
        mode: TagMatch,
        tags: &[&str],
    ) -> Result<Vec<ObservationTarget>, TagQueryError> {
        if tags.is_empty() {
            return Err(TagQueryError::EmptyQuery);
        }
        let mut wanted = std::collections::BTreeSet::new();
        for raw in tags {
            let tag = oxurack_core::Tag::new(*raw).map_err(|source| TagQueryError::InvalidTag {
                tag: (*raw).to_owned(),
                source,
            })?;
            wanted.insert(tag);
        }
        if let Some(error) = self.observation.unavailable_error() {
            return Err(TagQueryError::Observation(error));
        }
        if !self.has_patch() {
            return Err(TagQueryError::Core(oxurack_core::CoreError::NoPatchLoaded));
        }
        let lifetime = self.observation.lifetime;
        let world = self.app.world_mut();
        let mut result = Vec::new();
        if let Some(meta) = world.get_resource::<oxurack_core::RackMeta>()
            && super::queries::tags_match(&meta.tags, &wanted, mode)
        {
            result.push(ObservationTarget {
                lifetime,
                target: LogicalTarget::Rack,
            });
        }
        let handle = world
            .get_resource::<oxurack_core::PatchHandle>()
            .cloned()
            .ok_or(TagQueryError::Core(oxurack_core::CoreError::NoPatchLoaded))?;
        let mut modules: Vec<_> = handle.modules.into_iter().collect();
        modules.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, entity) in modules {
            if world
                .get::<oxurack_core::Tags>(entity)
                .is_some_and(|set| super::queries::tags_match(&set.0, &wanted, mode))
            {
                result.push(ObservationTarget {
                    lifetime,
                    target: LogicalTarget::Module(name),
                });
            }
        }
        let mut query = world.query::<(
            bevy_ecs::prelude::Entity,
            &oxurack_core::Cable,
            Option<&oxurack_core::CableOrder>,
        )>();
        let cable_entities: Vec<_> = query.iter(world).map(|(entity, _, _)| entity).collect();
        let mut cable_targets = Vec::new();
        for entity in cable_entities {
            if world
                .get::<oxurack_core::Tags>(entity)
                .is_some_and(|set| super::queries::tags_match(&set.0, &wanted, mode))
                && let Some(cable) = oxurack_core::observation::describe_cable(world, entity)
            {
                cable_targets.push(cable);
            }
        }
        cable_targets.sort_by(|a, b| {
            (&a.source, &a.target, a.occurrence).cmp(&(&b.source, &b.target, b.occurrence))
        });
        result.extend(cable_targets.into_iter().map(|cable| ObservationTarget {
            lifetime,
            target: LogicalTarget::Cable(cable),
        }));
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_counter_exhaustion_is_typed_and_terminal() {
        let mut journal = Journal {
            next_sequence: u64::MAX,
            ..Journal::default()
        };
        journal.publish(
            LifetimeId(1),
            0,
            vec![ObservationChange::Reset {
                source: ChangeSource::DirectApi,
            }],
        );
        assert_eq!(
            journal.unavailable_error(),
            Some(ObservationError::CounterExhausted {
                counter: ObservationCounter::Sequence
            })
        );

        let mut journal = Journal {
            next_batch: u64::MAX,
            ..Journal::default()
        };
        journal.publish(
            LifetimeId(1),
            0,
            vec![ObservationChange::Reset {
                source: ChangeSource::DirectApi,
            }],
        );
        assert_eq!(
            journal.unavailable_error(),
            Some(ObservationError::CounterExhausted {
                counter: ObservationCounter::Batch
            })
        );

        let mut journal = Journal {
            next_lifetime: u64::MAX,
            ..Journal::default()
        };
        assert_eq!(journal.next_lifetime(), None);
        assert_eq!(
            journal.unavailable_error(),
            Some(ObservationError::CounterExhausted {
                counter: ObservationCounter::Lifetime
            })
        );
    }
}
