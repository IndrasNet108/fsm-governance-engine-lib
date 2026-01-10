//! Declarative FSM definitions for validation-only workflows.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::error::FsmError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FsmDefinition {
    pub states: Vec<String>,
    pub transitions: Vec<FsmTransition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<FsmDefaults>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invariants: Vec<FsmInvariant>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FsmDefaults {
    #[serde(rename = "initialState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FsmTransition {
    pub from: String,
    pub to: String,
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FsmTransitionMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FsmTransitionMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FsmInvariant {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions: Vec<FsmTransitionRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FsmTransitionRef {
    pub from: String,
    pub to: String,
}

impl FsmDefinition {
    pub fn validate(&self) -> Result<(), FsmError> {
        self.validate_structure()?;
        self.validate_invariants()?;
        Ok(())
    }

    pub fn validate_structure(&self) -> Result<(), FsmError> {
        if self.states.is_empty() || self.transitions.is_empty() {
            return Err(FsmError::InvalidInput);
        }

        let state_set: HashSet<&str> = self.states.iter().map(|s| s.as_str()).collect();

        for transition in &self.transitions {
            if transition.from.trim().is_empty()
                || transition.to.trim().is_empty()
                || transition.action.trim().is_empty()
            {
                return Err(FsmError::InvalidInput);
            }

            if !state_set.contains(transition.from.as_str())
                || !state_set.contains(transition.to.as_str())
            {
                return Err(FsmError::InvalidInput);
            }
        }

        if let Some(defaults) = &self.defaults {
            if let Some(initial_state) = &defaults.initial_state {
                if !state_set.contains(initial_state.as_str()) {
                    return Err(FsmError::InvalidInput);
                }
            }
        }

        Ok(())
    }

    pub fn validate_invariants(&self) -> Result<(), FsmError> {
        if self.invariants.is_empty() {
            return Ok(());
        }

        let transition_set: HashSet<(&str, &str)> = self
            .transitions
            .iter()
            .map(|t| (t.from.as_str(), t.to.as_str()))
            .collect();

        let mut adjacency: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
        for transition in &self.transitions {
            adjacency
                .entry(transition.from.as_str())
                .or_default()
                .push(transition.to.as_str());
        }

        for invariant in &self.invariants {
            match invariant.kind.as_str() {
                "terminal_states" => {
                    for state in &invariant.states {
                        if let Some(outbound) = adjacency.get(state.as_str()) {
                            if !outbound.is_empty() {
                                return Err(FsmError::InvalidInput);
                            }
                        }
                    }
                }
                "required_transitions" => {
                    for transition in &invariant.transitions {
                        if !transition_set.contains(&(
                            transition.from.as_str(),
                            transition.to.as_str(),
                        )) {
                            return Err(FsmError::InvalidInput);
                        }
                    }
                }
                "forbidden_transitions" => {
                    for transition in &invariant.transitions {
                        if transition_set.contains(&(
                            transition.from.as_str(),
                            transition.to.as_str(),
                        )) {
                            return Err(FsmError::InvalidInput);
                        }
                    }
                }
                "forbidden_cycles" => {
                    for state in &invariant.states {
                        if has_cycle_from(state.as_str(), &adjacency) {
                            return Err(FsmError::InvalidInput);
                        }
                    }
                }
                "self_transitions_required" => {
                    let states: Vec<&str> = if invariant.states.is_empty() {
                        self.states.iter().map(|s| s.as_str()).collect()
                    } else {
                        invariant.states.iter().map(|s| s.as_str()).collect()
                    };

                    for state in states {
                        if !transition_set.contains(&(state, state)) {
                            return Err(FsmError::InvalidInput);
                        }
                    }
                }
                _ => return Err(FsmError::InvalidInput),
            }
        }

        Ok(())
    }
}

fn has_cycle_from(start: &str, adjacency: &std::collections::HashMap<&str, Vec<&str>>) -> bool {
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert(start);

    if let Some(neighbors) = adjacency.get(start) {
        for neighbor in neighbors {
            if *neighbor == start {
                return true;
            }
            if dfs_reaches_start(neighbor, start, adjacency, &mut visited) {
                return true;
            }
        }
    }

    false
}

fn dfs_reaches_start<'a>(
    current: &'a str,
    start: &'a str,
    adjacency: &std::collections::HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
) -> bool {
    if let Some(neighbors) = adjacency.get(current) {
        for neighbor in neighbors {
            if neighbor == &start {
                return true;
            }
            if visited.insert(neighbor) && dfs_reaches_start(neighbor, start, adjacency, visited) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_structure_success() {
        let definition = FsmDefinition {
            states: vec!["Draft".into(), "Review".into(), "Approved".into()],
            transitions: vec![FsmTransition {
                from: "Draft".into(),
                to: "Review".into(),
                action: "submit".into(),
                guard: None,
                metadata: None,
            }],
            defaults: Some(FsmDefaults {
                initial_state: Some("Draft".into()),
            }),
            invariants: Vec::new(),
        };

        assert!(definition.validate_structure().is_ok());
    }

    #[test]
    fn test_validate_structure_unknown_state() {
        let definition = FsmDefinition {
            states: vec!["Draft".into(), "Review".into()],
            transitions: vec![FsmTransition {
                from: "Draft".into(),
                to: "Approved".into(),
                action: "submit".into(),
                guard: None,
                metadata: None,
            }],
            defaults: None,
            invariants: Vec::new(),
        };

        assert_eq!(definition.validate_structure(), Err(FsmError::InvalidInput));
    }

    #[test]
    fn test_validate_invariants_terminal_state() {
        let definition = FsmDefinition {
            states: vec!["Draft".into(), "Archived".into()],
            transitions: vec![FsmTransition {
                from: "Draft".into(),
                to: "Archived".into(),
                action: "archive".into(),
                guard: None,
                metadata: None,
            }],
            defaults: None,
            invariants: vec![FsmInvariant {
                kind: "terminal_states".into(),
                states: vec!["Archived".into()],
                transitions: Vec::new(),
                description: None,
            }],
        };

        assert!(definition.validate_invariants().is_ok());
    }

    #[test]
    fn test_validate_invariants_terminal_state_violation() {
        let definition = FsmDefinition {
            states: vec!["Draft".into(), "Archived".into()],
            transitions: vec![FsmTransition {
                from: "Archived".into(),
                to: "Draft".into(),
                action: "restore".into(),
                guard: None,
                metadata: None,
            }],
            defaults: None,
            invariants: vec![FsmInvariant {
                kind: "terminal_states".into(),
                states: vec!["Archived".into()],
                transitions: Vec::new(),
                description: None,
            }],
        };

        assert_eq!(definition.validate_invariants(), Err(FsmError::InvalidInput));
    }
}
