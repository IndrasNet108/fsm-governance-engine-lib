//! Grant module for the standalone FSM governance engine.
//! It exposes a simple Grant data model and lifecycle helpers without DAO-specific
//! dependencies.

pub mod lifecycle;
pub mod types;
pub mod vote;
pub mod voting_types;

pub use lifecycle::Grant;
pub use types::*;
pub use vote::{GrantVote, VoterType};
pub use voting_types::VoteType;
