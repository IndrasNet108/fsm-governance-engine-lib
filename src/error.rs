//! Defines the custom error type for the FSM Governance Engine.

use std::fmt;

/// Custom error for FSM state transitions.
#[derive(Debug, PartialEq, Eq)]
pub enum FsmError {
    /// The attempted state transition is invalid.
    InvalidStateTransition,
    /// An invalid input was provided.
    InvalidInput,
    /// Insufficient quorum/members for the requested operation.
    InsufficientMembers,
    /// The requested transition cannot be performed because the system is in the wrong state.
    InvalidState,
    /// Number overflow detected while computing values.
    Overflow,
}

// Implement standard `Error` trait.
impl std::error::Error for FsmError {}

// Implement `Display` for user-friendly error messages.
impl fmt::Display for FsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FsmError::InvalidStateTransition => write!(f, "Invalid FSM state transition"),
            FsmError::InvalidInput => write!(f, "Invalid input provided"),
            FsmError::InsufficientMembers => write!(f, "Not enough members for quorum"),
            FsmError::InvalidState => write!(f, "Invalid state for requested operation"),
            FsmError::Overflow => write!(f, "Arithmetic overflow detected"),
        }
    }
}

/// Crate-level error for IndrasNet primitives.
#[derive(Debug, PartialEq, Eq)]
pub enum IndrasError {
    InvalidState,
    InvalidInput,
    InsufficientFunds,
    Overflow,
}

impl std::error::Error for IndrasError {}

impl fmt::Display for IndrasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IndrasError::InvalidState => write!(f, "Invalid grant state"),
            IndrasError::InvalidInput => write!(f, "Invalid input to grant handler"),
            IndrasError::InsufficientFunds => write!(f, "Insufficient funds for disbursement"),
            IndrasError::Overflow => write!(f, "Numeric overflow"),
        }
    }
}
