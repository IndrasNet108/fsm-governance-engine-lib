//! Audit trail helpers for FSM transitions.
//! 
//! Records every state change for grants and allows verification of the sequence.

use crate::enums::GrantStatus;
use crate::error::FsmError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// Immutable audit entry representing one transition.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AuditEntry {
    pub grant_id: u64,
    pub actor: [u8; 32],
    pub from_state: GrantStatus,
    pub to_state: GrantStatus,
    pub action: String,
    pub timestamp: i64,
    pub metadata: Option<String>,
}

impl AuditEntry {
    /// Build a new entry.
    pub fn new(
        grant_id: u64,
        actor: [u8; 32],
        from_state: GrantStatus,
        to_state: GrantStatus,
        action: &'static str,
        timestamp: i64,
        metadata: Option<String>,
    ) -> Self {
        Self {
            grant_id,
            actor,
            from_state,
            to_state,
            action: action.to_string(),
            timestamp,
            metadata,
        }
    }
}

/// In-memory audit trail for FSM transitions.
#[derive(Default, Clone, Debug)]
pub struct AuditTrail {
    entries: Vec<AuditEntry>,
}

impl AuditTrail {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Append an audit entry after verifying the transition is permitted.
    pub fn record(&mut self, entry: AuditEntry) -> Result<(), FsmError> {
        entry
            .from_state
            .validate_transition(entry.to_state)
            .map_err(|_| FsmError::InvalidStateTransition)?;
        self.entries.push(entry);
        Ok(())
    }

    /// Verify that history only contains valid transitions and is monotonically increasing.
    pub fn verify(&self) -> Result<(), FsmError> {
        for window in self.entries.windows(2) {
            let first = &window[0];
            let second = &window[1];
            if first.grant_id != second.grant_id {
                continue;
            }
            if first.to_state != second.from_state {
                return Err(FsmError::InvalidStateTransition);
            }
        }
        Ok(())
    }

    /// Provide slice of entries for export.
    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grant::types::GrantStatus;

    fn sample_entry(from: GrantStatus, to: GrantStatus) -> AuditEntry {
        AuditEntry::new(
            1,
            [0u8; 32],
            from,
            to,
            "test",
            1_000,
            Some("metadata".to_string()),
        )
    }

    #[test]
    fn record_permitted_transition() {
        let mut trail = AuditTrail::new();
        let entry = sample_entry(GrantStatus::Pending, GrantStatus::Approved);
        assert!(trail.record(entry).is_ok());
        assert_eq!(trail.entries().len(), 1);
    }

    #[test]
    fn record_invalid_transition() {
        let mut trail = AuditTrail::new();
        let entry = sample_entry(GrantStatus::Approved, GrantStatus::Pending);
        assert!(trail.record(entry).is_err());
    }

    #[test]
    fn verify_chain_success() {
        let mut trail = AuditTrail::new();
        let first = sample_entry(GrantStatus::Pending, GrantStatus::Approved);
        trail.record(first).unwrap();
        let second = sample_entry(GrantStatus::Approved, GrantStatus::Active);
        trail.record(second).unwrap();
        assert!(trail.verify().is_ok());
    }

    #[test]
    fn verify_chain_gap() {
        let mut trail = AuditTrail::new();
        let first = sample_entry(GrantStatus::Pending, GrantStatus::Approved);
        trail.record(first).unwrap();
        let gap = sample_entry(GrantStatus::Active, GrantStatus::Completed);
        trail.entries.push(gap);
        assert!(trail.verify().is_err());
    }

    #[test]
    fn serialize_deserialize_json() {
        let entry = sample_entry(GrantStatus::Pending, GrantStatus::Approved);
        let json = serde_json::to_string(&entry).unwrap();
        let parsed: AuditEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, parsed);
    }

    #[test]
    fn trail_contains_multiple_grants() {
        let mut trail = AuditTrail::new();
        trail.record(sample_entry(GrantStatus::Pending, GrantStatus::Approved)).unwrap();
        trail.record(AuditEntry::new(
            2,
            [1u8; 32],
            GrantStatus::Pending,
            GrantStatus::Approved,
            "approve",
            2_000,
            None,
        ))
        .unwrap();
        assert_eq!(trail.entries().len(), 2);
        assert!(trail.verify().is_ok());
    }
}
