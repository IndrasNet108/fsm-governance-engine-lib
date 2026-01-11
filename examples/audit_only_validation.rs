//! Example: audit-only validation without domain execution.

use fsm_governance_engine_lib::audit::{AuditEntry, AuditTrail};
use fsm_governance_engine_lib::grant::types::GrantStatus;
use serde_json::Value;

fn main() {
    let mut trail = AuditTrail::new();

    let entry = AuditEntry::new(
        99,
        [0u8; 32],
        GrantStatus::Pending,
        GrantStatus::Approved,
        "approve",
        1_000,
        Some("audit-only".to_string()),
    );

    trail.record(entry).expect("record audit entry");
    trail.verify().expect("verify trail");

    let lines: Vec<String> = trail
        .entries()
        .iter()
        .map(|item| serde_json::to_string(item).expect("serialize audit entry"))
        .collect();

    for line in &lines {
        let value: Value = serde_json::from_str(line).expect("parse audit entry");
        assert!(value.get("grant_id").is_some());
        assert!(value.get("from_state").is_some());
        assert!(value.get("to_state").is_some());
        assert!(value.get("action").is_some());
        assert!(value.get("timestamp").is_some());
    }

    println!("Audit JSONL:");
    for line in &lines {
        println!("{}", line);
    }
    println!("Audit-only validation complete: {} entries", lines.len());
}
