//! Example: governance proposal lifecycle with audit JSONL output.

use fsm_governance_engine_lib::proposal::types::{Proposal, ProposalStatus};
use serde::Serialize;

#[derive(Serialize)]
struct AuditEntry {
    proposal_id: u64,
    from_state: ProposalStatus,
    to_state: ProposalStatus,
    action: &'static str,
    timestamp: i64,
}

fn main() {
    let mut proposal = Proposal::new_with_time(
        1,
        "Budget Allocation".to_string(),
        "Allocate funds for infra".to_string(),
        "treasury".to_string(),
        42u64,
        0,
    )
    .expect("proposal create");

    proposal.voting_duration = 1;

    let mut audit = Vec::new();

    audit.push(AuditEntry {
        proposal_id: proposal.id,
        from_state: ProposalStatus::Draft,
        to_state: ProposalStatus::Active,
        action: "activate",
        timestamp: 1,
    });
    proposal.activate_with_time(1, 10, 10).expect("activate");

    audit.push(AuditEntry {
        proposal_id: proposal.id,
        from_state: ProposalStatus::Active,
        to_state: ProposalStatus::Passed,
        action: "pass",
        timestamp: 11,
    });
    proposal.pass_with_time(11).expect("pass");

    audit.push(AuditEntry {
        proposal_id: proposal.id,
        from_state: ProposalStatus::Passed,
        to_state: ProposalStatus::Executed,
        action: "execute",
        timestamp: 12,
    });
    proposal.execute_with_time(12).expect("execute");

    assert_eq!(proposal.status, ProposalStatus::Executed);
    println!("Audit JSONL:");
    for entry in &audit {
        let line = serde_json::to_string(entry).expect("serialize audit entry");
        println!("{}", line);
    }
    println!("Proposal lifecycle complete: {:?}", proposal.status);
}
