//! Governance state modules
//!
//! Governance management for the DAO:
//! - On-chain: Metadata, policies, committees, security board
//! - Off-chain: Advanced analytics, optimization, recommendations
//!
//! Includes: analytics, voting, participation

pub mod analytics;
pub mod participation;
pub mod proposal_lifecycle;
pub mod quorum;
pub mod security_board;
pub mod security_committees;
pub mod security_excellence;
pub mod security_policies;
pub mod voting;

// Re-exports (specific to avoid ambiguous glob re-exports)
pub use analytics::{
    GovernanceAnalyticsMetadata, GovernanceAnalyticsStatus, GovernanceAnalyticsType,
    onchain::initialize_governance_analytics,
};
pub use participation::{
    GovernanceParticipationMetadata, GovernanceParticipationStatus, GovernanceParticipationType,
    onchain::initialize_governance_participation,
};
pub use proposal_lifecycle::{
    ProposalLifecycleMetadata, ProposalLifecycleStage, onchain as proposal_lifecycle_onchain,
};
pub use quorum::{QuorumCalculationMethod, QuorumMetadata, onchain as quorum_onchain};
pub use security_board::{
    SecurityBoardDecisionMetadata, SecurityBoardDecisionStatus, SecurityBoardMemberMetadata,
    SecurityBoardMemberRole, onchain as security_board_onchain,
};
pub use security_committees::{
    CommitteeMemberRole, SecurityCommitteeMetadata, onchain as security_committees_onchain,
};
pub use security_excellence::{SecurityExcellenceMetadata, onchain as security_excellence_onchain};
pub use security_policies::{
    SecurityPolicyMetadata, SecurityPolicyStatus, onchain as security_policies_onchain,
};
pub use voting::{
    GovernanceVotingMetadata, GovernanceVotingStatus, GovernanceVotingType,
    onchain::initialize_governance_voting,
};
