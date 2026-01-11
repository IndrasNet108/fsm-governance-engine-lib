//! # FSM Governance Engine
//!
//! A standalone, reusable Finite State Machine (FSM) library for declarative,
//! validation-only governance process checks with auditability.

pub mod audit;
pub mod definition;
pub mod enums;
pub mod error;
pub mod fsm;
pub mod governance;
pub mod grant;
pub mod proposal;

// Re-export key types for easy access
pub use audit::{AuditEntry, AuditTrail};
pub use definition::{
    FsmDefaults, FsmDefinition, FsmInvariant, FsmTransition, FsmTransitionMetadata,
    FsmTransitionRef,
};
pub use enums::IdeaStatus;
pub use error::FsmError;
pub use grant::{Grant, GrantDisbursementType, GrantStatus, GrantVote, VoteType};
