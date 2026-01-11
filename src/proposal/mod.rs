//! Proposal module
//!
//! Provides proposal management functionality:
//! - types: ProposalStatus enum and Proposal struct
//! - lifecycle: Proposal lifecycle methods (new, activate, pass, reject, execute, cancel, archive)
//! - analytics: Proposal analytics and metrics
//! - amendment: Proposal amendment support
//! - template: Proposal template system

pub mod amendment;
pub mod analytics;
pub mod lifecycle;
pub mod template;
pub mod treasury;
pub mod types;

// Re-export types
pub use amendment::ProposalAmendment;
pub use analytics::{
    ProposalAnalyticsMetadata, ProposalAnalyticsStatus, ProposalAnalyticsType,
    onchain::initialize_proposal_analytics,
};
pub use template::{ProposalTemplate, TemplateField, TemplateFieldType};
pub use treasury::{TreasuryOperationData, TreasuryProposalType};
pub use types::{Proposal, ProposalStatus};
