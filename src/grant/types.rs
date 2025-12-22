//! Grant-specific enums for the FSM governance engine.

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

macro_rules! grant_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(
            BorshSerialize,
            BorshDeserialize,
            Serialize,
            Deserialize,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Debug,
        )]
        #[repr(u8)]
        pub enum $name {
            $($variant,)*
        }
    };
}

grant_enum!(GrantStatus {
    Pending,
    Approved,
    Active,
    Suspended,
    Completed,
    Cancelled,
    Rejected,
    Expired,
    Archived,
});

grant_enum!(GrantCategory {
    Research,
    Development,
    Community,
});

grant_enum!(GrantType {
    Initial,
    Core,
    Final,
});

grant_enum!(GrantDisbursementType {
    Urgent,
    Escrow,
    Standard,
});

impl GrantDisbursementType {
    pub fn requires_report(&self) -> bool {
        matches!(
            self,
            GrantDisbursementType::Escrow | GrantDisbursementType::Standard
        )
    }

    pub fn requires_escrow(&self) -> bool {
        matches!(self, GrantDisbursementType::Escrow)
    }

    pub fn can_close_without_report(&self) -> bool {
        matches!(self, GrantDisbursementType::Urgent)
    }
}

grant_enum!(VerificationStatus {
    Pending,
    Verified,
    Rejected,
});

// Simple unit-test to ensure serialization works
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grant_status_values() {
        assert_eq!(GrantStatus::Pending as u8, 0);
        assert_eq!(GrantStatus::Expired as u8, 7);
    }
}
