//! Example integration: DAO grant workflow with audit logging.

use fsm_governance_engine_lib::{
    audit::{AuditEntry, AuditTrail},
    grant::{
        Grant, GrantCategory, GrantDisbursementType, GrantStatus, GrantType, GrantVote, VoteType,
        VoterType,
    },
};
use serde_json::Value;

fn main() {
    let mut grant = Grant::new(
        42,
        10,
        [1u8; 32],
        GrantCategory::Research,
        GrantType::Initial,
        GrantDisbursementType::Escrow,
        1_000,
        250,
        1_000,
    )
    .expect("create grant");

    let mut trail = AuditTrail::new();

    let entry = AuditEntry::new(
        grant.id,
        [0u8; 32],
        grant.status,
        GrantStatus::Approved,
        "approve",
        1_000,
        Some("committee".into()),
    );
    trail.record(entry).expect("approve log");
    grant.approve().unwrap();

    let entry = AuditEntry::new(
        grant.id,
        [1u8; 32],
        grant.status,
        GrantStatus::Active,
        "activate",
        1_100,
        Some("mesh_lead".into()),
    );
    trail.record(entry).expect("activate log");
    grant.activate().unwrap();

    let vote = GrantVote {
        grant_id: grant.id,
        voter_id: [2u8; 32],
        vote_type: VoteType::Approve,
        weight: GrantVote::calculate_weight(VoterType::MeshGroupMember),
        voter_type: VoterType::MeshGroupMember,
        cast_at: 1_200,
    };
    println!("Vote recorded: {:?}", vote);

    grant.disburse(1_250).unwrap();
    assert_eq!(grant.status, GrantStatus::Completed);

    trail.verify().expect("audit trail");

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
    println!("Audit trail entries: {}", lines.len());
}
