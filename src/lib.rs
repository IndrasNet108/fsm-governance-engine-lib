//! # FSM Governance Engine
//!
//! A standalone, reusable Finite State Machine (FSM) engine for validating
//! and managing state transitions in governance processes.

pub mod enums;
pub mod error;
pub mod fsm;
pub mod grant;
pub mod audit;

// Re-export key types for easy access
pub use enums::{GrantStatus, IdeaStatus};
pub use error::FsmError;
pub use grant::{Grant, GrantDisbursementType, GrantVote, VoteType};
pub use audit::{AuditEntry, AuditTrail};
