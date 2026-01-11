//! Example: governance proposal lifecycle.

use fsm_governance_engine_lib::proposal::types::{Proposal, ProposalStatus};

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

    proposal.activate_with_time(1, 10, 10).expect("activate");
    proposal.pass_with_time(11).expect("pass");
    proposal.execute_with_time(12).expect("execute");

    assert_eq!(proposal.status, ProposalStatus::Executed);
    println!("Proposal lifecycle complete: {:?}", proposal.status);
}
