//! Example: governance voting metadata initialization.

use fsm_governance_engine_lib::governance::voting::{
    GovernanceVotingMetadata, GovernanceVotingStatus, GovernanceVotingType, onchain,
};

fn main() {
    let mut voting = GovernanceVotingMetadata {
        voting_id: 0,
        proposal_id: 0,
        voting_type: GovernanceVotingType::SimpleMajority,
        status: GovernanceVotingStatus::Open,
        created_at: 0,
        voting_data_hash: [0u8; 32],
    };

    let data_hash = [9u8; 32];
    onchain::initialize_governance_voting(
        &mut voting,
        100,
        200,
        GovernanceVotingType::SuperMajority,
        data_hash,
        1_000,
    )
    .expect("initialize voting");

    assert_eq!(voting.voting_id, 100);
    assert_eq!(voting.status, GovernanceVotingStatus::Open);
    println!("Voting initialized for proposal {}", voting.proposal_id);
}
