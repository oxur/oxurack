//! Patch persistence: data structures, validation, RON serialisation,
//! and file I/O.
//!
//! A [`Patch`] is a complete rack configuration that can be saved to
//! and loaded from a RON file. It describes the set of modules, their
//! parameter overrides, and the cables connecting them.
//!
//! The [`validate_patch`] function checks a patch against a
//! [`ModuleRegistry`](crate::ModuleRegistry) to ensure all module kinds
//! are registered, all cable endpoints reference existing ports, merge
//! policies are compatible, and the module graph is acyclic.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ── Patch data structures ────────────────────────────────────────────

/// A complete rack configuration that can be saved and loaded.
///
/// Contains the full description of a rack: which modules are
/// instantiated, how they are parameterised, and how they are wired
/// together.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Patch {
    /// Schema version for forward compatibility.
    pub version: String,
    /// Master RNG seed for deterministic randomness.
    pub master_seed: u64,
    /// Global tempo in beats per minute.
    pub bpm: f32,
    /// The modules in the rack.
    pub modules: Vec<ModuleConfig>,
    /// The cables connecting module ports.
    pub cables: Vec<CableConfig>,
}

/// Configuration for a single module instance within a patch.
///
/// Pairs a module kind (looked up in the
/// [`ModuleRegistry`](crate::ModuleRegistry)) with an instance name and
/// optional parameter overrides.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleConfig {
    /// The module kind name (e.g. `"vco"`, `"turing_machine"`).
    pub kind: String,
    /// Unique instance name within the patch (e.g. `"vco_1"`).
    pub instance_name: String,
    /// Parameter overrides (name -> value).
    pub parameters: HashMap<String, crate::ParameterValue>,
}

/// Configuration for a single cable connection within a patch.
///
/// Describes a connection from a source port on one module to a target
/// port on another module, with an optional inline transform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CableConfig {
    /// `(module_instance_name, port_name)` of the source.
    pub source: (String, String),
    /// `(module_instance_name, port_name)` of the target.
    pub target: (String, String),
    /// Optional signal transform applied on the cable.
    pub transform: Option<crate::CableTransform>,
}

// ── Patch validation ─────────────────────────────────────────────────

/// Validates a patch against the module registry.
///
/// # Checks
///
/// 1. All module kinds are registered.
/// 2. No duplicate instance names.
/// 3. All cable source/target ports exist in the registry's port schemas.
/// 4. Source and target value kinds are compatible (or a transform is
///    present).
/// 5. Merge policies are valid for multi-cable fan-in.
/// 6. No feedback cycles in the module graph.
///
/// # Errors
///
/// Returns the first [`PatchError`](crate::PatchError) encountered.
pub fn validate_patch(
    patch: &Patch,
    registry: &crate::ModuleRegistry,
) -> Result<(), crate::PatchError> {
    // 1. Check all module kinds are registered.
    for module in &patch.modules {
        let kind = crate::ModuleKind::from(module.kind.as_str());
        if !registry.contains(&kind) {
            return Err(crate::PatchError::UnknownModuleKind(module.kind.clone()));
        }
    }

    // 2. Check no duplicate instance names.
    let mut seen_names = std::collections::HashSet::new();
    for module in &patch.modules {
        if !seen_names.insert(&module.instance_name) {
            return Err(crate::PatchError::DuplicateInstanceName(
                module.instance_name.clone(),
            ));
        }
    }

    // 3. Validate cable endpoints and value kind compatibility.
    for cable in &patch.cables {
        validate_port_ref(patch, registry, &cable.source.0, &cable.source.1)?;
        validate_port_ref(patch, registry, &cable.target.0, &cable.target.1)?;

        let source_schema = find_port_schema(patch, registry, &cable.source.0, &cable.source.1)?;
        let target_schema = find_port_schema(patch, registry, &cable.target.0, &cable.target.1)?;

        // Check kind compatibility: mismatched kinds require a transform.
        if source_schema.value_kind != target_schema.value_kind && cable.transform.is_none() {
            return Err(crate::PatchError::KindMismatch {
                source_kind: source_schema.value_kind,
                target_kind: target_schema.value_kind,
            });
        }
    }

    // 4. Check merge policies for multi-cable fan-in.
    let mut cable_counts: HashMap<(String, String), usize> = HashMap::new();
    for cable in &patch.cables {
        *cable_counts.entry(cable.target.clone()).or_insert(0) += 1;
    }
    for ((module_name, port_name), count) in &cable_counts {
        if *count > 1 {
            let schema = find_port_schema(patch, registry, module_name, port_name)?;
            if schema.merge_policy == crate::MergePolicy::Reject {
                return Err(crate::PatchError::IllegalMerge {
                    module: module_name.clone(),
                    port: port_name.clone(),
                    kind: schema.value_kind,
                    policy: schema.merge_policy,
                });
            }
            if !schema.merge_policy.is_valid_for(schema.value_kind) {
                return Err(crate::PatchError::IllegalMerge {
                    module: module_name.clone(),
                    port: port_name.clone(),
                    kind: schema.value_kind,
                    policy: schema.merge_policy,
                });
            }
        }
    }

    // 5. Check for feedback cycles.
    check_patch_cycles(patch)?;

    Ok(())
}

/// Validates that a port reference (module instance name + port name)
/// exists in the patch and registry.
fn validate_port_ref(
    patch: &Patch,
    registry: &crate::ModuleRegistry,
    module_name: &str,
    port_name: &str,
) -> Result<(), crate::PatchError> {
    let module = patch
        .modules
        .iter()
        .find(|m| m.instance_name == module_name)
        .ok_or_else(|| crate::PatchError::UnknownPort {
            module: module_name.to_string(),
            port: port_name.to_string(),
        })?;

    let kind = crate::ModuleKind::from(module.kind.as_str());
    let reg = registry.get(&kind).ok_or_else(|| {
        crate::PatchError::UnknownModuleKind(module.kind.clone())
    })?;

    if !reg.port_schemas.iter().any(|p| p.name == port_name) {
        return Err(crate::PatchError::UnknownPort {
            module: module_name.to_string(),
            port: port_name.to_string(),
        });
    }

    Ok(())
}

/// Finds the [`PortSchema`](crate::PortSchema) for a given module
/// instance name and port name.
fn find_port_schema<'a>(
    patch: &Patch,
    registry: &'a crate::ModuleRegistry,
    module_name: &str,
    port_name: &str,
) -> Result<&'a crate::PortSchema, crate::PatchError> {
    let module = patch
        .modules
        .iter()
        .find(|m| m.instance_name == module_name)
        .ok_or_else(|| crate::PatchError::UnknownPort {
            module: module_name.to_string(),
            port: port_name.to_string(),
        })?;

    let kind = crate::ModuleKind::from(module.kind.as_str());
    let reg = registry.get(&kind).ok_or_else(|| {
        crate::PatchError::UnknownModuleKind(module.kind.clone())
    })?;

    reg.port_schemas
        .iter()
        .find(|p| p.name == port_name)
        .ok_or_else(|| crate::PatchError::UnknownPort {
            module: module_name.to_string(),
            port: port_name.to_string(),
        })
}

/// Validates that the patch's cable graph has no cycles.
/// Uses Kahn's algorithm (topological sort) with a proper queue on
/// module instance names. When a cycle is detected, runs Tarjan's SCC
/// to report only the modules actually participating in cycles.
fn check_patch_cycles(patch: &Patch) -> Result<(), crate::PatchError> {
    use std::collections::{HashSet, VecDeque};

    let mut in_degree: HashMap<&str, usize> = patch
        .modules
        .iter()
        .map(|m| (m.instance_name.as_str(), 0))
        .collect();
    let mut dependents: HashMap<&str, Vec<&str>> = HashMap::new();

    for cable in &patch.cables {
        let source = cable.source.0.as_str();
        let target = cable.target.0.as_str();
        if source == target {
            continue; // self-loop within same module, skip
        }
        dependents.entry(source).or_default().push(target);
        *in_degree.entry(target).or_insert(0) += 1;
    }

    // Kahn's algorithm with deterministic ordering via BTreeSet-style
    // initial seeding and sorted insertion.
    let mut queue: VecDeque<&str> = {
        let mut seeds: Vec<&str> = in_degree
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(name, _)| *name)
            .collect();
        seeds.sort();
        seeds.into_iter().collect()
    };
    let mut resolved = HashSet::new();

    while let Some(node) = queue.pop_front() {
        resolved.insert(node);
        if let Some(deps) = dependents.get(node) {
            // Collect newly freed nodes, sort them, then extend queue.
            let mut newly_free: Vec<&str> = Vec::new();
            for dep in deps {
                if let Some(deg) = in_degree.get_mut(dep) {
                    *deg -= 1;
                    if *deg == 0 {
                        newly_free.push(dep);
                    }
                }
            }
            newly_free.sort();
            for n in newly_free {
                queue.push_back(n);
            }
        }
    }

    if resolved.len() != patch.modules.len() {
        // Cycle detected -- use Tarjan's SCC to find exactly which
        // modules participate in cycles.
        let unresolved: HashSet<&str> = in_degree
            .keys()
            .filter(|name| !resolved.contains(**name))
            .copied()
            .collect();
        let cycle_members =
            find_patch_cycle_members(&unresolved, &dependents);
        return Err(crate::PatchError::FeedbackCycle(cycle_members));
    }

    Ok(())
}

/// Uses Tarjan's SCC algorithm on the unresolved subgraph to find
/// modules that are actually in cycles (SCCs of size > 1 or self-loops).
fn find_patch_cycle_members(
    unresolved: &std::collections::HashSet<&str>,
    dependents: &HashMap<&str, Vec<&str>>,
) -> Vec<String> {
    use std::collections::HashSet;

    struct TarjanState<'a> {
        unresolved: &'a HashSet<&'a str>,
        dependents: &'a HashMap<&'a str, Vec<&'a str>>,
        index_counter: usize,
        stack: Vec<&'a str>,
        on_stack: HashSet<&'a str>,
        indices: HashMap<&'a str, usize>,
        lowlinks: HashMap<&'a str, usize>,
        sccs: Vec<Vec<&'a str>>,
    }

    impl<'a> TarjanState<'a> {
        fn visit(&mut self, node: &'a str) {
            self.indices.insert(node, self.index_counter);
            self.lowlinks.insert(node, self.index_counter);
            self.index_counter += 1;
            self.stack.push(node);
            self.on_stack.insert(node);

            if let Some(deps) = self.dependents.get(node) {
                for &dep in deps {
                    if !self.unresolved.contains(dep) {
                        continue;
                    }
                    if !self.indices.contains_key(dep) {
                        self.visit(dep);
                        let dep_low = self.lowlinks[dep];
                        let node_low = self.lowlinks.get_mut(node).expect("just inserted");
                        *node_low = (*node_low).min(dep_low);
                    } else if self.on_stack.contains(dep) {
                        let dep_idx = self.indices[dep];
                        let node_low = self.lowlinks.get_mut(node).expect("just inserted");
                        *node_low = (*node_low).min(dep_idx);
                    }
                }
            }

            if self.lowlinks[node] == self.indices[node] {
                let mut scc = Vec::new();
                loop {
                    let w = self.stack.pop().expect("stack non-empty");
                    self.on_stack.remove(w);
                    scc.push(w);
                    if w == node {
                        break;
                    }
                }
                self.sccs.push(scc);
            }
        }
    }

    let mut state = TarjanState {
        unresolved,
        dependents,
        index_counter: 0,
        stack: Vec::new(),
        on_stack: HashSet::new(),
        indices: HashMap::new(),
        lowlinks: HashMap::new(),
        sccs: Vec::new(),
    };

    let mut sorted_unresolved: Vec<&str> = unresolved.iter().copied().collect();
    sorted_unresolved.sort();
    for &node in &sorted_unresolved {
        if !state.indices.contains_key(node) {
            state.visit(node);
        }
    }

    let mut result: Vec<String> = state
        .sccs
        .iter()
        .filter(|scc| scc.len() > 1)
        .flatten()
        .map(|name| name.to_string())
        .collect();
    result.sort();
    result
}

// ── RON serialisation ────────────────────────────────────────────────

/// Serialises a [`Patch`] to a human-readable RON string.
///
/// # Errors
///
/// Returns [`CoreError::Patch`](crate::CoreError::Patch) if
/// serialisation fails (should not happen for well-formed patches).
pub fn serialize_patch(patch: &Patch) -> Result<String, crate::CoreError> {
    ron::ser::to_string_pretty(patch, ron::ser::PrettyConfig::default())
        .map_err(|e| crate::CoreError::Patch(crate::PatchError::Serialize(e.to_string())))
}

/// Deserialises a [`Patch`] from a RON string.
///
/// # Errors
///
/// Returns [`PatchError::Deserialize`](crate::PatchError::Deserialize)
/// if the input is not valid RON or does not match the `Patch` schema.
pub fn deserialize_patch(ron_str: &str) -> Result<Patch, crate::PatchError> {
    ron::from_str(ron_str).map_err(|e| crate::PatchError::Deserialize(e.to_string()))
}

/// Saves a patch to a file in RON format.
///
/// Creates or overwrites the file at `path`.
///
/// # Errors
///
/// Returns [`CoreError`](crate::CoreError) on serialisation or I/O
/// failure.
pub fn save_patch_to_file(
    patch: &Patch,
    path: &std::path::Path,
) -> Result<(), crate::CoreError> {
    let ron_str = serialize_patch(patch)?;
    std::fs::write(path, ron_str)
        .map_err(|e| crate::CoreError::Patch(crate::PatchError::Io(e)))?;
    Ok(())
}

/// Loads a patch from a RON file.
///
/// # Errors
///
/// Returns [`CoreError`](crate::CoreError) on I/O or parse failure.
pub fn load_patch_from_file(path: &std::path::Path) -> Result<Patch, crate::CoreError> {
    let ron_str = std::fs::read_to_string(path)
        .map_err(|e| crate::CoreError::Patch(crate::PatchError::Io(e)))?;
    deserialize_patch(&ron_str).map_err(crate::CoreError::Patch)
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MergePolicy, ModuleRegistry, OxurackModule, ParameterSchema, ParameterValue,
        PortDirection, PortSchema, ValueKind,
    };
    use pretty_assertions::assert_eq;

    // ── Test module definitions ──────────────────────────────────

    /// A dummy VCO module for testing.
    struct TestVco;

    impl OxurackModule for TestVco {
        const KIND: &'static str = "vco";
        const DISPLAY_NAME: &'static str = "VCO";
        const DESCRIPTION: &'static str = "Test oscillator";

        fn port_schema() -> &'static [PortSchema] {
            &[
                PortSchema {
                    name: "pitch",
                    direction: PortDirection::Input,
                    value_kind: ValueKind::Float,
                    merge_policy: MergePolicy::LastWins,
                    description: "Pitch CV input",
                },
                PortSchema {
                    name: "out",
                    direction: PortDirection::Output,
                    value_kind: ValueKind::Bipolar,
                    merge_policy: MergePolicy::Reject,
                    description: "Audio output",
                },
            ]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[ParameterSchema {
                name: "waveform",
                description: "Oscillator waveform",
                default: ParameterValue::Int(0),
            }]
        }
    }

    /// A dummy filter module for testing.
    struct TestFilter;

    impl OxurackModule for TestFilter {
        const KIND: &'static str = "filter";
        const DISPLAY_NAME: &'static str = "Filter";
        const DESCRIPTION: &'static str = "Test filter";

        fn port_schema() -> &'static [PortSchema] {
            &[
                PortSchema {
                    name: "in",
                    direction: PortDirection::Input,
                    value_kind: ValueKind::Bipolar,
                    merge_policy: MergePolicy::Reject,
                    description: "Audio input",
                },
                PortSchema {
                    name: "cutoff",
                    direction: PortDirection::Input,
                    value_kind: ValueKind::Float,
                    merge_policy: MergePolicy::Average,
                    description: "Cutoff CV",
                },
                PortSchema {
                    name: "out",
                    direction: PortDirection::Output,
                    value_kind: ValueKind::Bipolar,
                    merge_policy: MergePolicy::Reject,
                    description: "Audio output",
                },
            ]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[ParameterSchema {
                name: "resonance",
                description: "Filter resonance",
                default: ParameterValue::Float(0.0),
            }]
        }
    }

    /// A dummy mixer module with a Reject-merge input for testing.
    struct TestMixer;

    impl OxurackModule for TestMixer {
        const KIND: &'static str = "mixer";
        const DISPLAY_NAME: &'static str = "Mixer";

        fn port_schema() -> &'static [PortSchema] {
            &[
                PortSchema {
                    name: "in",
                    direction: PortDirection::Input,
                    value_kind: ValueKind::Bipolar,
                    merge_policy: MergePolicy::Reject,
                    description: "Single input",
                },
                PortSchema {
                    name: "out",
                    direction: PortDirection::Output,
                    value_kind: ValueKind::Bipolar,
                    merge_policy: MergePolicy::Reject,
                    description: "Output",
                },
            ]
        }

        fn parameter_schema() -> &'static [ParameterSchema] {
            &[]
        }
    }

    /// Helper: builds a [`ModuleRegistry`] with the test modules.
    fn test_registry() -> ModuleRegistry {
        let mut registry = ModuleRegistry::default();
        registry.register::<TestVco>();
        registry.register::<TestFilter>();
        registry.register::<TestMixer>();
        registry
    }

    /// Helper: builds a valid two-module patch (VCO -> Filter).
    fn valid_patch() -> Patch {
        Patch {
            version: "1.0".to_string(),
            master_seed: 42,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_1".to_string(),
                    parameters: HashMap::from([(
                        "waveform".to_string(),
                        ParameterValue::Int(1),
                    )]),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "filter_1".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![CableConfig {
                source: ("vco_1".to_string(), "out".to_string()),
                target: ("filter_1".to_string(), "in".to_string()),
                transform: None,
            }],
        }
    }

    // ── Milestone 5.1: Data structure tests ─────────────────────

    #[test]
    fn test_patch_clone_eq() {
        let patch = valid_patch();
        let cloned = patch.clone();
        assert_eq!(patch, cloned);
    }

    // ── Milestone 5.3: RON serialisation tests ──────────────────

    #[test]
    fn test_patch_ron_roundtrip() {
        let patch = valid_patch();
        let ron_str = serialize_patch(&patch).expect("serialisation should succeed");
        let deserialized = deserialize_patch(&ron_str).expect("deserialisation should succeed");
        assert_eq!(patch, deserialized);
    }

    #[test]
    fn test_patch_ron_human_readable() {
        let patch = valid_patch();
        let ron_str = serialize_patch(&patch).expect("serialisation should succeed");

        assert!(
            ron_str.contains("modules"),
            "expected 'modules' in RON output:\n{ron_str}"
        );
        assert!(
            ron_str.contains("cables"),
            "expected 'cables' in RON output:\n{ron_str}"
        );
        assert!(
            ron_str.contains("vco"),
            "expected 'vco' in RON output:\n{ron_str}"
        );
        assert!(
            ron_str.contains("filter"),
            "expected 'filter' in RON output:\n{ron_str}"
        );
        assert!(
            ron_str.contains("120"),
            "expected bpm '120' in RON output:\n{ron_str}"
        );
    }

    #[test]
    fn test_patch_ron_roundtrip_with_transform() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 90.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_1".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "filter_1".to_string(),
                    parameters: HashMap::from([(
                        "resonance".to_string(),
                        ParameterValue::Float(0.5),
                    )]),
                },
            ],
            cables: vec![CableConfig {
                source: ("vco_1".to_string(), "out".to_string()),
                target: ("filter_1".to_string(), "in".to_string()),
                transform: Some(crate::CableTransform::Affine {
                    factor: 0.5,
                    offset: 0.25,
                }),
            }],
        };

        let ron_str = serialize_patch(&patch).expect("serialisation should succeed");
        let deserialized = deserialize_patch(&ron_str).expect("deserialisation should succeed");
        assert_eq!(patch, deserialized);
    }

    #[test]
    fn test_deserialize_invalid_ron() {
        let result = deserialize_patch("this is not valid RON {{{");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::Deserialize(_)),
            "expected Deserialize, got: {err:?}"
        );
    }

    // ── Milestone 5.2: Validation tests ─────────────────────────

    #[test]
    fn test_validate_valid_patch() {
        let patch = valid_patch();
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_ok(), "valid patch should pass: {result:?}");
    }

    #[test]
    fn test_validate_unknown_module_kind() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![ModuleConfig {
                kind: "reverb".to_string(),
                instance_name: "reverb_1".to_string(),
                parameters: HashMap::new(),
            }],
            cables: vec![],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::UnknownModuleKind(ref k) if k == "reverb"),
            "expected UnknownModuleKind(\"reverb\"), got: {err:?}"
        );
    }

    #[test]
    fn test_validate_duplicate_instance_name() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "osc".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "osc".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::DuplicateInstanceName(ref n) if n == "osc"),
            "expected DuplicateInstanceName(\"osc\"), got: {err:?}"
        );
    }

    #[test]
    fn test_validate_unknown_port() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_1".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "filter_1".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![CableConfig {
                source: ("vco_1".to_string(), "nonexistent".to_string()),
                target: ("filter_1".to_string(), "in".to_string()),
                transform: None,
            }],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::UnknownPort { ref port, .. } if port == "nonexistent"),
            "expected UnknownPort with port 'nonexistent', got: {err:?}"
        );
    }

    #[test]
    fn test_validate_unknown_port_module_not_found() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![ModuleConfig {
                kind: "vco".to_string(),
                instance_name: "vco_1".to_string(),
                parameters: HashMap::new(),
            }],
            cables: vec![CableConfig {
                source: ("missing_module".to_string(), "out".to_string()),
                target: ("vco_1".to_string(), "pitch".to_string()),
                transform: None,
            }],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::UnknownPort { ref module, .. } if module == "missing_module"),
            "expected UnknownPort for missing_module, got: {err:?}"
        );
    }

    #[test]
    fn test_validate_kind_mismatch_without_transform() {
        // VCO out is Bipolar, Filter cutoff is Float -- mismatch without transform.
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_1".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "filter_1".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![CableConfig {
                source: ("vco_1".to_string(), "out".to_string()),
                target: ("filter_1".to_string(), "cutoff".to_string()),
                transform: None,
            }],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::KindMismatch { .. }),
            "expected KindMismatch, got: {err:?}"
        );
    }

    #[test]
    fn test_validate_kind_mismatch_with_transform_ok() {
        // Same mismatch but with a transform -- should pass.
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_1".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "filter_1".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![CableConfig {
                source: ("vco_1".to_string(), "out".to_string()),
                target: ("filter_1".to_string(), "cutoff".to_string()),
                transform: Some(crate::CableTransform::Bipolarize),
            }],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_ok(), "mismatch with transform should pass: {result:?}");
    }

    #[test]
    fn test_validate_reject_multi_cable() {
        // Two cables targeting a Reject-merge port.
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_1".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_2".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "mixer".to_string(),
                    instance_name: "mixer_1".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![
                CableConfig {
                    source: ("vco_1".to_string(), "out".to_string()),
                    target: ("mixer_1".to_string(), "in".to_string()),
                    transform: None,
                },
                CableConfig {
                    source: ("vco_2".to_string(), "out".to_string()),
                    target: ("mixer_1".to_string(), "in".to_string()),
                    transform: None,
                },
            ],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::IllegalMerge { ref port, .. } if port == "in"),
            "expected IllegalMerge on 'in', got: {err:?}"
        );
    }

    #[test]
    fn test_validate_average_multi_cable_ok() {
        // Two cables targeting an Average-merge Float port -- should pass.
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_1".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "vco_2".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "filter_1".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![
                CableConfig {
                    source: ("vco_1".to_string(), "out".to_string()),
                    target: ("filter_1".to_string(), "cutoff".to_string()),
                    // Bipolar -> Float requires a transform.
                    transform: Some(crate::CableTransform::Bipolarize),
                },
                CableConfig {
                    source: ("vco_2".to_string(), "out".to_string()),
                    target: ("filter_1".to_string(), "cutoff".to_string()),
                    transform: Some(crate::CableTransform::Bipolarize),
                },
            ],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(
            result.is_ok(),
            "Average merge on Float port with 2 cables should pass: {result:?}"
        );
    }

    #[test]
    fn test_validate_feedback_cycle() {
        // A -> B -> A cycle.
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "a".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "b".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![
                CableConfig {
                    source: ("a".to_string(), "out".to_string()),
                    target: ("b".to_string(), "in".to_string()),
                    transform: None,
                },
                CableConfig {
                    source: ("b".to_string(), "out".to_string()),
                    target: ("a".to_string(), "pitch".to_string()),
                    // Bipolar -> Float requires transform.
                    transform: Some(crate::CableTransform::Bipolarize),
                },
            ],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::PatchError::FeedbackCycle(_)),
            "expected FeedbackCycle, got: {err:?}"
        );
    }

    #[test]
    fn test_validate_no_cycle_linear() {
        // A -> B -> C, no cycle.
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "a".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "b".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "mixer".to_string(),
                    instance_name: "c".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![
                CableConfig {
                    source: ("a".to_string(), "out".to_string()),
                    target: ("b".to_string(), "in".to_string()),
                    transform: None,
                },
                CableConfig {
                    source: ("b".to_string(), "out".to_string()),
                    target: ("c".to_string(), "in".to_string()),
                    transform: None,
                },
            ],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_ok(), "linear chain should not be a cycle: {result:?}");
    }

    #[test]
    fn test_validate_empty_patch() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![],
            cables: vec![],
        };
        let registry = test_registry();
        let result = validate_patch(&patch, &registry);
        assert!(result.is_ok(), "empty patch should be valid: {result:?}");
    }

    // ── Milestone 5.4: File I/O tests ───────────────────────────

    #[test]
    fn test_save_and_load_file() {
        let patch = valid_patch();
        let path = std::env::temp_dir().join("oxurack_test_patch.ron");

        save_patch_to_file(&patch, &path).expect("save should succeed");
        let loaded = load_patch_from_file(&path).expect("load should succeed");
        assert_eq!(patch, loaded);

        // Clean up.
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let path = std::path::Path::new("/tmp/oxurack_nonexistent_file_12345.ron");
        let result = load_patch_from_file(path);
        assert!(result.is_err(), "loading nonexistent file should fail");
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::CoreError::Patch(crate::PatchError::Io(_))),
            "expected PatchError::Io, got: {err:?}"
        );
    }

    #[test]
    fn test_load_malformed_ron_file() {
        let path = std::env::temp_dir().join("oxurack_test_malformed.ron");
        std::fs::write(&path, "this is { not [ valid RON").expect("write should succeed");

        let result = load_patch_from_file(&path);
        assert!(result.is_err(), "loading malformed RON should fail");
        let err = result.unwrap_err();
        assert!(
            matches!(err, crate::CoreError::Patch(crate::PatchError::Deserialize(_))),
            "expected PatchError::Deserialize, got: {err:?}"
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_save_and_load_roundtrip_with_all_parameter_types() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 99,
            bpm: 140.0,
            modules: vec![ModuleConfig {
                kind: "vco".to_string(),
                instance_name: "vco_1".to_string(),
                parameters: HashMap::from([
                    ("float_param".to_string(), ParameterValue::Float(0.75)),
                    ("int_param".to_string(), ParameterValue::Int(42)),
                    ("bool_param".to_string(), ParameterValue::Bool(true)),
                    (
                        "string_param".to_string(),
                        ParameterValue::String("hello".to_string()),
                    ),
                    (
                        "scale_param".to_string(),
                        ParameterValue::Scale(crate::Scale::major(2)),
                    ),
                ]),
            }],
            cables: vec![],
        };
        let path = std::env::temp_dir().join("oxurack_test_patch_params.ron");

        save_patch_to_file(&patch, &path).expect("save should succeed");
        let loaded = load_patch_from_file(&path).expect("load should succeed");
        assert_eq!(patch, loaded);

        // Clean up.
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_ron_roundtrip_all_cable_transforms() {
        let transforms = vec![
            Some(crate::CableTransform::Affine {
                factor: 2.0,
                offset: 0.1,
            }),
            Some(crate::CableTransform::Invert),
            Some(crate::CableTransform::Clamp {
                min: 0.0,
                max: 1.0,
            }),
            Some(crate::CableTransform::Threshold { threshold: 0.5 }),
            Some(crate::CableTransform::GateToFloat),
            Some(crate::CableTransform::Unipolar),
            Some(crate::CableTransform::Bipolarize),
            None,
        ];

        for (i, transform) in transforms.into_iter().enumerate() {
            let patch = Patch {
                version: "1.0".to_string(),
                master_seed: 0,
                bpm: 120.0,
                modules: vec![
                    ModuleConfig {
                        kind: "vco".to_string(),
                        instance_name: "vco_1".to_string(),
                        parameters: HashMap::new(),
                    },
                    ModuleConfig {
                        kind: "filter".to_string(),
                        instance_name: "filter_1".to_string(),
                        parameters: HashMap::new(),
                    },
                ],
                cables: vec![CableConfig {
                    source: ("vco_1".to_string(), "out".to_string()),
                    target: ("filter_1".to_string(), "in".to_string()),
                    transform,
                }],
            };

            let ron_str = serialize_patch(&patch)
                .unwrap_or_else(|e| panic!("serialisation failed for transform #{i}: {e}"));
            let deserialized = deserialize_patch(&ron_str)
                .unwrap_or_else(|e| panic!("deserialisation failed for transform #{i}: {e}"));
            assert_eq!(patch, deserialized, "roundtrip failed for transform #{i}");
        }
    }

    // ── check_patch_cycles edge cases ──────────────────────────────

    #[test]
    fn test_check_patch_cycles_self_loop_ignored() {
        // A cable from a module to itself is a self-loop, which should
        // be ignored by the cycle checker (it doesn't create a cycle
        // in the inter-module graph).
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![ModuleConfig {
                kind: "filter".to_string(),
                instance_name: "f".to_string(),
                parameters: HashMap::new(),
            }],
            cables: vec![CableConfig {
                source: ("f".to_string(), "out".to_string()),
                target: ("f".to_string(), "in".to_string()),
                transform: None,
            }],
        };
        let result = check_patch_cycles(&patch);
        assert!(result.is_ok(), "self-loop should not be treated as a cycle: {result:?}");
    }

    #[test]
    fn test_check_patch_cycles_three_node_cycle() {
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "a".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "b".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "mixer".to_string(),
                    instance_name: "c".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![
                CableConfig {
                    source: ("a".to_string(), "out".to_string()),
                    target: ("b".to_string(), "in".to_string()),
                    transform: None,
                },
                CableConfig {
                    source: ("b".to_string(), "out".to_string()),
                    target: ("c".to_string(), "in".to_string()),
                    transform: None,
                },
                CableConfig {
                    source: ("c".to_string(), "out".to_string()),
                    target: ("a".to_string(), "pitch".to_string()),
                    transform: None,
                },
            ],
        };
        let result = check_patch_cycles(&patch);
        assert!(result.is_err(), "A->B->C->A should be detected as a cycle");
        let err = result.unwrap_err();
        if let crate::PatchError::FeedbackCycle(modules) = &err {
            assert_eq!(modules.len(), 3, "all three modules should be in the cycle");
        } else {
            panic!("expected FeedbackCycle, got: {err:?}");
        }
    }

    #[test]
    fn test_check_patch_cycles_only_reports_cycle_members() {
        // A->B->A cycle. B->C->D downstream of B.
        // Only A and B should be reported, NOT C and D.
        let patch = Patch {
            version: "1.0".to_string(),
            master_seed: 0,
            bpm: 120.0,
            modules: vec![
                ModuleConfig {
                    kind: "vco".to_string(),
                    instance_name: "a".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "filter".to_string(),
                    instance_name: "b".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "mixer".to_string(),
                    instance_name: "c".to_string(),
                    parameters: HashMap::new(),
                },
                ModuleConfig {
                    kind: "mixer".to_string(),
                    instance_name: "d".to_string(),
                    parameters: HashMap::new(),
                },
            ],
            cables: vec![
                // A -> B
                CableConfig {
                    source: ("a".to_string(), "out".to_string()),
                    target: ("b".to_string(), "in".to_string()),
                    transform: None,
                },
                // B -> A (creates cycle)
                CableConfig {
                    source: ("b".to_string(), "out".to_string()),
                    target: ("a".to_string(), "pitch".to_string()),
                    transform: None,
                },
                // B -> C (downstream of cycle)
                CableConfig {
                    source: ("b".to_string(), "out".to_string()),
                    target: ("c".to_string(), "in".to_string()),
                    transform: None,
                },
                // C -> D (downstream of cycle)
                CableConfig {
                    source: ("c".to_string(), "out".to_string()),
                    target: ("d".to_string(), "in".to_string()),
                    transform: None,
                },
            ],
        };
        let result = check_patch_cycles(&patch);
        assert!(result.is_err(), "should detect cycle");
        let err = result.unwrap_err();
        if let crate::PatchError::FeedbackCycle(modules) = &err {
            assert_eq!(
                modules,
                &["a".to_string(), "b".to_string()],
                "only A and B should be in the cycle, got: {modules:?}"
            );
        } else {
            panic!("expected FeedbackCycle, got: {err:?}");
        }
    }
}
