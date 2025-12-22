//! Example integration: DAO grant workflow with audit logging.

use fsm_governance_engine_lib::{
    audit::{AuditEntry, AuditTrail},
    grant::{Grant, GrantCategory, GrantDisbursementType, GrantVote, VoteType, VoterType},
    enums::GrantStatus,
};

fn main() {
    let mut grant = Grant::new(
        42,
        10,
        [1u8; 32],
        GrantCategory::Research,
        GrantDisbursementType::Escrow,
        GrantDisbursementType::Escrow, // reusing for demo
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
    println!("Audit trail entries: {}", trail.entries().len());
}
