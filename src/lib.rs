//! # FSM Governance Engine
//!
//! A standalone, reusable Finite State Machine (FSM) library for declarative,
//! validation-only governance process checks with auditability.

pub mod enums;
pub mod error;
pub mod fsm;
pub mod grant;
pub mod audit;
pub mod definition;

// Re-export key types for easy access
pub use enums::IdeaStatus;
pub use error::FsmError;
pub use grant::{Grant, GrantDisbursementType, GrantStatus, GrantVote, VoteType};
pub use audit::{AuditEntry, AuditTrail};
pub use definition::{
    FsmDefaults, FsmDefinition, FsmInvariant, FsmTransition, FsmTransitionMetadata, FsmTransitionRef,
};
