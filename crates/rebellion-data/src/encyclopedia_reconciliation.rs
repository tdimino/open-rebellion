//! Internal reconciliation between the source-derived Encyclopedia index and
//! the optional opaque content catalog.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::encyclopedia::{CatalogBinding, EncyclopediaCatalog as ContentCatalog, TopicId};
use crate::encyclopedia_catalog::{EncyclopediaCatalog as SourceCatalog, EncyclopediaSourceTable};

pub const CANONICAL_OBJECT_COUNT: usize = 356;
pub const CANONICAL_AVAILABLE_COUNT: usize = 346;
pub const KNOWN_NONMEMBER_FLEET_OBJECT_ID: u32 = 0x0800_0004;
pub const KNOWN_UNAVAILABLE_MISSION_OBJECT_IDS: [u32; 10] = [
    0x4100_0001,
    0x4200_0002,
    0x4300_0003,
    0x4400_0004,
    0x7100_0043,
    0x6400_0044,
    0x7200_0045,
    0x7200_0046,
    0x7300_0082,
    0x6500_0083,
];

const DAT_ID_MASK: u32 = 0x00ff_ffff;
const KNOWN_UNAVAILABLE_MISSION_SOURCES: [(u32, EncyclopediaSourceTable); 10] = [
    (0x4100_0001, EncyclopediaSourceTable::Missions),
    (0x4200_0002, EncyclopediaSourceTable::Missions),
    (0x4300_0003, EncyclopediaSourceTable::Missions),
    (0x4400_0004, EncyclopediaSourceTable::Missions),
    (0x7100_0043, EncyclopediaSourceTable::Missions),
    (0x6400_0044, EncyclopediaSourceTable::Missions),
    (0x7200_0045, EncyclopediaSourceTable::Missions),
    (0x7200_0046, EncyclopediaSourceTable::Missions),
    (0x7300_0082, EncyclopediaSourceTable::Missions),
    (0x6500_0083, EncyclopediaSourceTable::Missions),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaUnavailableReason {
    NoVerifiedContent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncyclopediaContentAvailability {
    Available(TopicId),
    Unavailable(EncyclopediaUnavailableReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconciledEncyclopediaEntry {
    object_id: u32,
    availability: EncyclopediaContentAvailability,
}

impl ReconciledEncyclopediaEntry {
    #[must_use]
    pub const fn object_id(&self) -> u32 {
        self.object_id
    }

    #[must_use]
    pub const fn availability(&self) -> &EncyclopediaContentAvailability {
        &self.availability
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconciledEncyclopediaCatalog {
    entries: Vec<ReconciledEncyclopediaEntry>,
    by_object_id: BTreeMap<u32, usize>,
    available_count: usize,
}

impl ReconciledEncyclopediaCatalog {
    /// Entries in the authoritative source catalog's order.
    #[must_use]
    pub fn entries(&self) -> &[ReconciledEncyclopediaEntry] {
        &self.entries
    }

    #[must_use]
    pub fn entry(&self, object_id: u32) -> Option<&ReconciledEncyclopediaEntry> {
        self.by_object_id
            .get(&object_id)
            .map(|index| &self.entries[*index])
    }

    #[must_use]
    pub const fn available_count(&self) -> usize {
        self.available_count
    }

    #[must_use]
    pub fn unavailable_count(&self) -> usize {
        self.entries.len() - self.available_count
    }

    pub fn unavailable_object_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries.iter().filter_map(|entry| {
            matches!(
                entry.availability,
                EncyclopediaContentAvailability::Unavailable(_)
            )
            .then_some(entry.object_id)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaReconciliationError {
    code: &'static str,
    object_id: Option<u32>,
    detail: String,
}

impl EncyclopediaReconciliationError {
    fn new(code: &'static str, object_id: Option<u32>, detail: impl Into<String>) -> Self {
        Self {
            code,
            object_id,
            detail: detail.into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub const fn object_id(&self) -> Option<u32> {
        self.object_id
    }
}

impl fmt::Display for EncyclopediaReconciliationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(object_id) = self.object_id {
            write!(
                formatter,
                "{} for object {object_id:#010x}: {}",
                self.code, self.detail
            )
        } else {
            write!(formatter, "{}: {}", self.code, self.detail)
        }
    }
}

impl std::error::Error for EncyclopediaReconciliationError {}

/// Join the authoritative source index to the validated optional content
/// catalog without allowing content availability to change membership/order.
pub fn reconcile_encyclopedia_content(
    source_catalog: &SourceCatalog,
    content_catalog: &ContentCatalog,
) -> Result<ReconciledEncyclopediaCatalog, EncyclopediaReconciliationError> {
    let source_objects = source_catalog
        .entries
        .iter()
        .map(|entry| SourceObject {
            object_id: entry.object_id,
            source_table: entry.source_table(),
            raw_dat_id: entry.raw_dat_id(),
        })
        .collect::<Vec<_>>();
    let topic_ids = content_catalog.topics.keys().cloned().collect();
    let policy = ReconciliationPolicy::new(
        &KNOWN_UNAVAILABLE_MISSION_SOURCES,
        CANONICAL_OBJECT_COUNT,
        CANONICAL_AVAILABLE_COUNT,
        BindingIdentity::new("fleet_definitions", 4, "viewer_faction"),
    );

    reconcile_source_objects(
        &source_objects,
        &content_catalog.bindings,
        &topic_ids,
        &policy,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SourceObject {
    object_id: u32,
    source_table: EncyclopediaSourceTable,
    raw_dat_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BindingIdentity {
    family: String,
    dat_id: u32,
    variant: String,
}

impl BindingIdentity {
    fn new(family: &str, dat_id: u32, variant: &str) -> Self {
        Self {
            family: family.to_owned(),
            dat_id,
            variant: variant.to_owned(),
        }
    }

    fn from_binding(binding: &CatalogBinding) -> Self {
        Self::new(&binding.family, binding.dat_id, &binding.variant)
    }
}

impl fmt::Display for BindingIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "({}, {}, {})",
            self.family, self.dat_id, self.variant
        )
    }
}

struct ReconciliationPolicy {
    unavailable: BTreeMap<u32, EncyclopediaSourceTable>,
    expected_membership: usize,
    expected_available: usize,
    required_nonmember: BindingIdentity,
}

impl ReconciliationPolicy {
    fn new(
        unavailable: &[(u32, EncyclopediaSourceTable)],
        expected_membership: usize,
        expected_available: usize,
        required_nonmember: BindingIdentity,
    ) -> Self {
        Self {
            unavailable: unavailable.iter().copied().collect(),
            expected_membership,
            expected_available,
            required_nonmember,
        }
    }
}

fn reconcile_source_objects(
    source_objects: &[SourceObject],
    bindings: &[CatalogBinding],
    topic_ids: &BTreeSet<TopicId>,
    policy: &ReconciliationPolicy,
) -> Result<ReconciledEncyclopediaCatalog, EncyclopediaReconciliationError> {
    if source_objects.len() != policy.expected_membership {
        return Err(EncyclopediaReconciliationError::new(
            "source_membership_count",
            None,
            format!(
                "source catalog has {} objects; expected {}",
                source_objects.len(),
                policy.expected_membership
            ),
        ));
    }

    let mut bindings_by_key = BTreeMap::new();
    let mut bound_topic_ids = BTreeSet::new();
    for binding in bindings {
        if !topic_ids.contains(&binding.topic_id) {
            return Err(EncyclopediaReconciliationError::new(
                "dangling_topic_reference",
                None,
                format!(
                    "binding {} references absent topic {:?}",
                    BindingIdentity::from_binding(binding),
                    binding.topic_id.0
                ),
            ));
        }
        let key = BindingIdentity::from_binding(binding);
        if bindings_by_key.contains_key(&key) {
            return Err(EncyclopediaReconciliationError::new(
                "ambiguous_content_binding",
                None,
                format!("binding key {key} occurs more than once"),
            ));
        }
        if !bound_topic_ids.insert(binding.topic_id.clone()) {
            return Err(EncyclopediaReconciliationError::new(
                "ambiguous_content_binding",
                None,
                format!(
                    "topic {:?} is targeted by more than one binding key",
                    binding.topic_id.0
                ),
            ));
        }
        bindings_by_key.insert(key.clone(), &binding.topic_id);
    }
    if let Some(unbound_topic) = topic_ids
        .iter()
        .find(|topic_id| !bound_topic_ids.contains(*topic_id))
    {
        return Err(EncyclopediaReconciliationError::new(
            "unexpected_content_orphan",
            None,
            format!("topic {:?} has no content binding", unbound_topic.0),
        ));
    }

    let mut seen_object_ids = BTreeSet::new();
    let mut matched_bindings = BTreeSet::new();
    let mut seen_unavailable = BTreeSet::new();
    let mut entries = Vec::with_capacity(source_objects.len());
    let mut available_count = 0;

    for source in source_objects {
        if !seen_object_ids.insert(source.object_id) {
            return Err(EncyclopediaReconciliationError::new(
                "duplicate_source_object",
                Some(source.object_id),
                "source catalog contains the object identity more than once",
            ));
        }
        let key = binding_identity_for_source(source)?;

        let availability = if let Some(expected_table) = policy.unavailable.get(&source.object_id) {
            if expected_table != &source.source_table {
                return Err(EncyclopediaReconciliationError::new(
                    "source_provenance_mismatch",
                    Some(source.object_id),
                    format!(
                        "known unavailable object came from {:?}, expected {:?}",
                        source.source_table, expected_table
                    ),
                ));
            }
            if bindings_by_key.contains_key(&key) {
                return Err(EncyclopediaReconciliationError::new(
                    "unexpected_available_content",
                    Some(source.object_id),
                    format!("known unavailable object unexpectedly resolves through {key}"),
                ));
            }
            seen_unavailable.insert(source.object_id);
            EncyclopediaContentAvailability::Unavailable(
                EncyclopediaUnavailableReason::NoVerifiedContent,
            )
        } else {
            let topic_id = bindings_by_key.get(&key).ok_or_else(|| {
                EncyclopediaReconciliationError::new(
                    "missing_content_binding",
                    Some(source.object_id),
                    format!("no exact content binding for {key}"),
                )
            })?;
            if !matched_bindings.insert(key.clone()) {
                return Err(EncyclopediaReconciliationError::new(
                    "ambiguous_source_join",
                    Some(source.object_id),
                    format!("multiple source objects resolve through {key}"),
                ));
            }
            available_count += 1;
            EncyclopediaContentAvailability::Available((*topic_id).clone())
        };
        entries.push(ReconciledEncyclopediaEntry {
            object_id: source.object_id,
            availability,
        });
    }

    if seen_unavailable.len() != policy.unavailable.len()
        || policy
            .unavailable
            .keys()
            .any(|object_id| !seen_unavailable.contains(object_id))
    {
        return Err(EncyclopediaReconciliationError::new(
            "missing_unavailable_source_object",
            None,
            "the source catalog does not contain the complete exact unavailable set",
        ));
    }
    if available_count != policy.expected_available {
        return Err(EncyclopediaReconciliationError::new(
            "available_content_count",
            None,
            format!(
                "resolved {available_count} available objects; expected {}",
                policy.expected_available
            ),
        ));
    }

    let mut accounted_nonmember_bindings = 0;
    for key in bindings_by_key.keys() {
        if matched_bindings.contains(key) {
            continue;
        }
        if key == &policy.required_nonmember {
            accounted_nonmember_bindings += 1;
            continue;
        }
        return Err(EncyclopediaReconciliationError::new(
            "unexpected_content_orphan",
            None,
            format!("content binding {key} has no authoritative source member"),
        ));
    }
    if accounted_nonmember_bindings != 1 {
        return Err(EncyclopediaReconciliationError::new(
            "missing_fleet_accounting",
            Some(KNOWN_NONMEMBER_FLEET_OBJECT_ID),
            format!(
                "required nonmember binding {} is absent",
                policy.required_nonmember
            ),
        ));
    }

    let by_object_id = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| (entry.object_id, index))
        .collect();
    Ok(ReconciledEncyclopediaCatalog {
        entries,
        by_object_id,
        available_count,
    })
}

fn binding_identity_for_source(
    source: &SourceObject,
) -> Result<BindingIdentity, EncyclopediaReconciliationError> {
    let family = (source.object_id >> 24) as u8;
    if !source_family_matches_table(source.source_table, family) {
        return Err(EncyclopediaReconciliationError::new(
            "source_provenance_mismatch",
            Some(source.object_id),
            format!(
                "family {family:#04x} is incompatible with {:?}",
                source.source_table
            ),
        ));
    }

    let dat_id = if source.raw_dat_id >> 24 == 0 {
        if source.object_id & DAT_ID_MASK != source.raw_dat_id {
            return Err(EncyclopediaReconciliationError::new(
                "source_provenance_mismatch",
                Some(source.object_id),
                format!(
                    "compound identity low bits do not preserve raw DAT id {}",
                    source.raw_dat_id
                ),
            ));
        }
        source.raw_dat_id
    } else {
        if source.raw_dat_id != source.object_id {
            return Err(EncyclopediaReconciliationError::new(
                "source_provenance_mismatch",
                Some(source.object_id),
                format!(
                    "precombined raw DAT id {:#010x} differs from the compound identity",
                    source.raw_dat_id
                ),
            ));
        }
        source.raw_dat_id & DAT_ID_MASK
    };

    let (binding_family, variant) = binding_role(source.source_table);
    Ok(BindingIdentity::new(binding_family, dat_id, variant))
}

const fn binding_role(source_table: EncyclopediaSourceTable) -> (&'static str, &'static str) {
    match source_table {
        EncyclopediaSourceTable::Systems => ("systems_world_locations", "default"),
        EncyclopediaSourceTable::CapitalShips => ("capital_ship_classes", "default"),
        EncyclopediaSourceTable::Fighters => ("fighter_classes", "default"),
        EncyclopediaSourceTable::DefenseFacilities => ("defense_facilities", "default"),
        EncyclopediaSourceTable::ManufacturingFacilities => ("manufacturing_facilities", "default"),
        EncyclopediaSourceTable::ProductionFacilities => ("production_facilities", "default"),
        EncyclopediaSourceTable::Missions => ("mission_definitions", "viewer_faction"),
        EncyclopediaSourceTable::Troops => ("troop_classes", "default"),
        EncyclopediaSourceTable::MajorCharacters => ("major_characters", "default"),
        EncyclopediaSourceTable::MinorCharacters => ("minor_characters", "default"),
        EncyclopediaSourceTable::SpecialForces => ("special_force_classes", "default"),
    }
}

fn source_family_matches_table(source_table: EncyclopediaSourceTable, family: u8) -> bool {
    match source_table {
        EncyclopediaSourceTable::Systems => matches!(family, 0x90 | 0x92),
        EncyclopediaSourceTable::CapitalShips => matches!(family, 0x14 | 0x18),
        EncyclopediaSourceTable::Fighters => family == 0x1c,
        EncyclopediaSourceTable::DefenseFacilities => (0x22..=0x25).contains(&family),
        EncyclopediaSourceTable::ManufacturingFacilities => (0x28..=0x2a).contains(&family),
        EncyclopediaSourceTable::ProductionFacilities => matches!(family, 0x2c | 0x2d),
        EncyclopediaSourceTable::Missions => (0x40..0x80).contains(&family),
        EncyclopediaSourceTable::Troops => family == 0x10,
        EncyclopediaSourceTable::MajorCharacters => (0x30..=0x35).contains(&family),
        EncyclopediaSourceTable::MinorCharacters => family == 0x38,
        EncyclopediaSourceTable::SpecialForces => family == 0x3c,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::encyclopedia::{parse_catalog, CatalogBinding, TopicId};
    use crate::encyclopedia_catalog::{
        load_encyclopedia_catalog, EncyclopediaSourceTable, EncyclopediaSourceTable::*,
    };

    fn source(
        object_id: u32,
        source_table: EncyclopediaSourceTable,
        raw_dat_id: u32,
    ) -> SourceObject {
        SourceObject {
            object_id,
            source_table,
            raw_dat_id,
        }
    }

    fn binding(family: &str, dat_id: u32, variant: &str, topic_id: &str) -> CatalogBinding {
        CatalogBinding {
            family: family.to_owned(),
            dat_id,
            variant: variant.to_owned(),
            topic_id: TopicId(topic_id.to_owned()),
        }
    }

    fn topic_ids(bindings: &[CatalogBinding]) -> BTreeSet<TopicId> {
        bindings
            .iter()
            .map(|binding| binding.topic_id.clone())
            .collect()
    }

    fn fleet_binding() -> CatalogBinding {
        binding("fleet_definitions", 4, "viewer_faction", "fleet-content")
    }

    fn policy(
        unavailable: &[(u32, EncyclopediaSourceTable)],
        expected_membership: usize,
        expected_available: usize,
    ) -> ReconciliationPolicy {
        ReconciliationPolicy::new(
            unavailable,
            expected_membership,
            expected_available,
            BindingIdentity::new("fleet_definitions", 4, "viewer_faction"),
        )
    }

    #[test]
    fn table_qualified_raw_ids_resolve_collisions_without_name_or_object_id_guessing() {
        let sources = [
            source(0x9000_0007, Systems, 7),
            source(0x4000_0007, Missions, 7),
        ];
        let bindings = [
            binding("systems_world_locations", 7, "default", "system-content"),
            binding(
                "mission_definitions",
                7,
                "viewer_faction",
                "mission-content",
            ),
            fleet_binding(),
        ];

        let result = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 2, 2),
        )
        .unwrap();

        assert_eq!(result.entries()[0].object_id(), 0x9000_0007);
        assert_eq!(
            result.entries()[0].availability(),
            &EncyclopediaContentAvailability::Available(TopicId("system-content".into()))
        );
        assert_eq!(result.entries()[1].object_id(), 0x4000_0007);
        assert_eq!(
            result.entries()[1].availability(),
            &EncyclopediaContentAvailability::Available(TopicId("mission-content".into()))
        );
    }

    #[test]
    fn every_typed_source_role_uses_its_verified_binding_family_and_variant() {
        let sources = [
            source(0x9000_0001, Systems, 1),
            source(0x1400_0002, CapitalShips, 2),
            source(0x1c00_0003, Fighters, 3),
            source(0x2200_0004, DefenseFacilities, 4),
            source(0x2800_0005, ManufacturingFacilities, 5),
            source(0x2c00_0006, ProductionFacilities, 6),
            source(0x4000_0007, Missions, 7),
            source(0x1000_0008, Troops, 8),
            source(0x3000_0009, MajorCharacters, 9),
            source(0x3800_000a, MinorCharacters, 10),
            source(0x3c00_000b, SpecialForces, 11),
        ];
        let bindings = [
            binding("systems_world_locations", 1, "default", "systems"),
            binding("capital_ship_classes", 2, "default", "capital-ships"),
            binding("fighter_classes", 3, "default", "fighters"),
            binding("defense_facilities", 4, "default", "defense"),
            binding("manufacturing_facilities", 5, "default", "manufacturing"),
            binding("production_facilities", 6, "default", "production"),
            binding("mission_definitions", 7, "viewer_faction", "missions"),
            binding("troop_classes", 8, "default", "troops"),
            binding("major_characters", 9, "default", "major"),
            binding("minor_characters", 10, "default", "minor"),
            binding("special_force_classes", 11, "default", "special-forces"),
            fleet_binding(),
        ];

        let result = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 11, 11),
        )
        .unwrap();

        assert_eq!(result.available_count(), 11);
        assert_eq!(
            result
                .entries()
                .iter()
                .map(ReconciledEncyclopediaEntry::object_id)
                .collect::<Vec<_>>(),
            sources
                .iter()
                .map(|source| source.object_id)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn precombined_raw_id_is_verified_before_its_binding_dat_id_is_extracted() {
        let sources = [source(0x9200_0109, Systems, 0x9200_0109)];
        let bindings = [
            binding(
                "systems_world_locations",
                0x109,
                "default",
                "system-content",
            ),
            fleet_binding(),
        ];

        let result = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 1, 1),
        )
        .unwrap();

        assert_eq!(result.available_count(), 1);
    }

    #[test]
    fn exact_known_missions_are_typed_unavailable_without_fabricated_topics() {
        let mut sources = vec![source(0x9000_0007, Systems, 7)];
        sources.extend(
            KNOWN_UNAVAILABLE_MISSION_OBJECT_IDS
                .map(|object_id| source(object_id, Missions, object_id & DAT_ID_MASK)),
        );
        let bindings = [
            binding("systems_world_locations", 7, "default", "system-content"),
            fleet_binding(),
        ];
        let unavailable = KNOWN_UNAVAILABLE_MISSION_OBJECT_IDS.map(|id| (id, Missions));

        let result = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&unavailable, 11, 1),
        )
        .unwrap();

        assert_eq!(result.available_count(), 1);
        assert_eq!(result.unavailable_count(), 10);
        assert_eq!(
            result.unavailable_object_ids().collect::<Vec<_>>(),
            KNOWN_UNAVAILABLE_MISSION_OBJECT_IDS
        );
        for object_id in KNOWN_UNAVAILABLE_MISSION_OBJECT_IDS {
            assert_eq!(
                result.entry(object_id).unwrap().availability(),
                &EncyclopediaContentAvailability::Unavailable(
                    EncyclopediaUnavailableReason::NoVerifiedContent,
                )
            );
        }
    }

    #[test]
    fn an_unexpected_missing_join_is_rejected_instead_of_becoming_unavailable() {
        let sources = [source(0x9000_0007, Systems, 7)];
        let bindings = [fleet_binding()];

        let error = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 1, 1),
        )
        .unwrap_err();

        assert_eq!(error.code(), "missing_content_binding");
        assert_eq!(error.object_id(), Some(0x9000_0007));
    }

    #[test]
    fn duplicate_binding_keys_are_rejected_as_ambiguous() {
        let sources = [source(0x9000_0007, Systems, 7)];
        let bindings = [
            binding("systems_world_locations", 7, "default", "first"),
            binding("systems_world_locations", 7, "default", "second"),
            fleet_binding(),
        ];

        let error = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 1, 1),
        )
        .unwrap_err();

        assert_eq!(error.code(), "ambiguous_content_binding");
    }

    #[test]
    fn distinct_member_keys_cannot_alias_the_same_content_topic() {
        let sources = [
            source(0x9000_0007, Systems, 7),
            source(0x1000_0008, Troops, 8),
        ];
        let bindings = [
            binding("systems_world_locations", 7, "default", "shared"),
            binding("troop_classes", 8, "default", "shared"),
            fleet_binding(),
        ];

        let error = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 2, 2),
        )
        .unwrap_err();

        assert_eq!(error.code(), "ambiguous_content_binding");
    }

    #[test]
    fn member_and_fleet_keys_cannot_alias_the_same_content_topic() {
        let sources = [source(0x9000_0007, Systems, 7)];
        let bindings = [
            binding("systems_world_locations", 7, "default", "fleet-content"),
            fleet_binding(),
        ];

        let error = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 1, 1),
        )
        .unwrap_err();

        assert_eq!(error.code(), "ambiguous_content_binding");
    }

    #[test]
    fn every_provided_topic_requires_exactly_one_binding() {
        let sources = [source(0x9000_0007, Systems, 7)];
        let bindings = [
            binding("systems_world_locations", 7, "default", "system-content"),
            fleet_binding(),
        ];
        let topics = BTreeSet::from([
            TopicId("system-content".into()),
            TopicId("fleet-content".into()),
            TopicId("unexpected-orphan".into()),
        ]);

        let error =
            reconcile_source_objects(&sources, &bindings, &topics, &policy(&[], 1, 1)).unwrap_err();

        assert_eq!(error.code(), "unexpected_content_orphan");
    }

    #[test]
    fn a_binding_to_an_absent_topic_is_rejected_before_reconciliation() {
        let sources = [source(0x9000_0007, Systems, 7)];
        let bindings = [
            binding("systems_world_locations", 7, "default", "missing-topic"),
            fleet_binding(),
        ];
        let topics = BTreeSet::from([TopicId("fleet-content".into())]);

        let error =
            reconcile_source_objects(&sources, &bindings, &topics, &policy(&[], 1, 1)).unwrap_err();

        assert_eq!(error.code(), "dangling_topic_reference");
    }

    #[test]
    fn an_unexpected_orphan_binding_is_rejected() {
        let sources = [source(0x9000_0007, Systems, 7)];
        let bindings = [
            binding("systems_world_locations", 7, "default", "system-content"),
            binding("troop_classes", 99, "default", "orphan-content"),
            fleet_binding(),
        ];

        let error = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 1, 1),
        )
        .unwrap_err();

        assert_eq!(error.code(), "unexpected_content_orphan");
    }

    #[test]
    fn fleet_is_required_accounting_but_never_source_membership() {
        let sources = [source(0x9000_0007, Systems, 7)];
        let bindings = [
            binding("systems_world_locations", 7, "default", "system-content"),
            fleet_binding(),
        ];

        let result = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 1, 1),
        )
        .unwrap();

        assert!(result.entry(KNOWN_NONMEMBER_FLEET_OBJECT_ID).is_none());

        let without_fleet = &bindings[..1];
        let error = reconcile_source_objects(
            &sources,
            without_fleet,
            &topic_ids(without_fleet),
            &policy(&[], 1, 1),
        )
        .unwrap_err();
        assert_eq!(error.code(), "missing_fleet_accounting");
    }

    #[test]
    fn count_preserving_wrong_source_identity_is_rejected() {
        let sources = [
            source(0x9000_0007, Systems, 7),
            source(0x1000_0008, Troops, 8),
        ];
        let bindings = [
            binding("systems_world_locations", 7, "default", "system-content"),
            binding("troop_classes", 8, "default", "troop-content"),
            fleet_binding(),
        ];
        let mut adversary = sources;
        adversary[1] = source(0x1000_0009, Troops, 9);

        let error = reconcile_source_objects(
            &adversary,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 2, 2),
        )
        .unwrap_err();

        assert_eq!(error.code(), "missing_content_binding");
        assert_eq!(error.object_id(), Some(0x1000_0009));
    }

    #[test]
    fn source_provenance_mismatch_is_rejected_even_if_a_binding_key_would_match() {
        let sources = [source(0x4000_0007, Systems, 7)];
        let bindings = [
            binding("systems_world_locations", 7, "default", "system-content"),
            fleet_binding(),
        ];

        let error = reconcile_source_objects(
            &sources,
            &bindings,
            &topic_ids(&bindings),
            &policy(&[], 1, 1),
        )
        .unwrap_err();

        assert_eq!(error.code(), "source_provenance_mismatch");
        assert_eq!(error.object_id(), Some(0x4000_0007));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires immutable owned inputs and the accepted validated catalog"]
    fn owned_loader_reconciles_exact_p65_order_with_optional_content() {
        use std::fs;
        use std::path::PathBuf;

        let owned_root = std::env::var_os("E56_OWNED_REBELLION_ROOT")
            .map(PathBuf::from)
            .expect("E56_OWNED_REBELLION_ROOT must point to the owned installation root");
        let content_catalog_path = std::env::var_os("E56_OWNED_ENCYCLOPEDIA_CATALOG")
            .map(PathBuf::from)
            .expect("E56_OWNED_ENCYCLOPEDIA_CATALOG must point to accepted catalog.json");
        let staged = tempfile::tempdir().unwrap();
        fs::copy(
            owned_root.join("TEXTSTRA.DLL"),
            staged.path().join("TEXTSTRA.DLL"),
        )
        .unwrap();
        for filename in [
            "SYSTEMSD.DAT",
            "CAPSHPSD.DAT",
            "FIGHTSD.DAT",
            "DEFFACSD.DAT",
            "MANFACSD.DAT",
            "PROFACSD.DAT",
            "MISSNSD.DAT",
            "TROOPSD.DAT",
            "MJCHARSD.DAT",
            "MNCHARSD.DAT",
            "SPECFCSD.DAT",
        ] {
            fs::copy(
                owned_root.join("GData").join(filename),
                staged.path().join(filename),
            )
            .unwrap();
        }

        let source_catalog = load_encyclopedia_catalog(staged.path()).unwrap();
        let content_bytes = fs::read(content_catalog_path).unwrap();
        let content_catalog = parse_catalog(&content_bytes).unwrap();
        let result = reconcile_encyclopedia_content(&source_catalog, &content_catalog).unwrap();

        assert_eq!(source_catalog.entries.len(), CANONICAL_OBJECT_COUNT);
        assert_eq!(result.entries().len(), CANONICAL_OBJECT_COUNT);
        assert_eq!(result.available_count(), CANONICAL_AVAILABLE_COUNT);
        assert_eq!(
            result.unavailable_count(),
            KNOWN_UNAVAILABLE_MISSION_OBJECT_IDS.len()
        );
        assert_eq!(source_catalog.entries_for(0x6f).len(), 356);
        assert_eq!(source_catalog.entries_for(0x70).len(), 200);
        assert_eq!(source_catalog.entries_for(0x71).len(), 38);
        assert_eq!(source_catalog.entries_for(0x72).len(), 14);
        assert_eq!(source_catalog.entries_for(0x73).len(), 25);
        assert_eq!(source_catalog.entries_for(0x74).len(), 10);
        assert_eq!(source_catalog.entries_for(0x75).len(), 69);
        assert_eq!(
            result.unavailable_object_ids().collect::<BTreeSet<_>>(),
            KNOWN_UNAVAILABLE_MISSION_OBJECT_IDS
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            ordered_identity_fnv64(
                result
                    .entries()
                    .iter()
                    .map(ReconciledEncyclopediaEntry::object_id)
            ),
            0x0abb_eede_cc0b_f1e9,
        );
    }

    fn ordered_identity_fnv64(ids: impl IntoIterator<Item = u32>) -> u64 {
        ids.into_iter().fold(0xcbf2_9ce4_8422_2325, |mut hash, id| {
            for byte in id.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
            hash
        })
    }
}
