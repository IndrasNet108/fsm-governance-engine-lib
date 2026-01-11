//! FSM Grant lifecycle helpers.

use crate::error::FsmError;
use crate::grant::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub type EntityId = [u8; 32];

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Grant {
    pub id: u64,
    pub idea_id: u64,
    pub mesh_group_id: EntityId,
    pub category: GrantCategory,
    pub status: GrantStatus,
    pub base_amount: u64,
    pub reputation_bonus: u64,
    pub total_amount: u64,
    pub disbursed_amount: u64,
    pub grant_type: GrantType,
    pub disbursement_type: GrantDisbursementType,
    pub verification_status: VerificationStatus,
    pub created_at: i64,
    pub enabled: bool,
}

impl Grant {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u64,
        idea_id: u64,
        mesh_group_id: EntityId,
        category: GrantCategory,
        grant_type: GrantType,
        disbursement_type: GrantDisbursementType,
        base_amount: u64,
        reputation_bonus: u64,
        created_at: i64,
    ) -> Result<Self, FsmError> {
        let total_amount = base_amount
            .checked_add(reputation_bonus)
            .ok_or(FsmError::Overflow)?;
        Ok(Self {
            id,
            idea_id,
            mesh_group_id,
            category,
            status: GrantStatus::Pending,
            base_amount,
            reputation_bonus,
            total_amount,
            disbursed_amount: 0,
            grant_type,
            disbursement_type,
            verification_status: VerificationStatus::Pending,
            created_at,
            enabled: true,
        })
    }

    pub fn approve(&mut self) -> Result<(), FsmError> {
        if self.status != GrantStatus::Pending {
            return Err(FsmError::InvalidState);
        }
        self.status = GrantStatus::Approved;
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), FsmError> {
        if self.status != GrantStatus::Approved {
            return Err(FsmError::InvalidState);
        }
        self.status = GrantStatus::Active;
        Ok(())
    }

    pub fn disburse(&mut self, amount: u64) -> Result<(), FsmError> {
        if self.status != GrantStatus::Active {
            return Err(FsmError::InvalidState);
        }
        let new_amount = self
            .disbursed_amount
            .checked_add(amount)
            .ok_or(FsmError::Overflow)?;
        if new_amount > self.total_amount {
            return Err(FsmError::InvalidInput);
        }
        self.disbursed_amount = new_amount;
        if self.disbursed_amount == self.total_amount {
            self.status = GrantStatus::Completed;
        }
        Ok(())
    }
}

impl Default for Grant {
    fn default() -> Self {
        Self {
            id: 0,
            idea_id: 0,
            mesh_group_id: [0u8; 32],
            category: GrantCategory::Research,
            status: GrantStatus::Pending,
            base_amount: 0,
            reputation_bonus: 0,
            total_amount: 0,
            disbursed_amount: 0,
            grant_type: GrantType::Initial,
            disbursement_type: GrantDisbursementType::Standard,
            verification_status: VerificationStatus::Pending,
            created_at: 0,
            enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_id() -> EntityId {
        [2u8; 32]
    }

    #[test]
    fn lifecycle_flow() {
        let mut grant = Grant::new(
            1,
            10,
            sample_id(),
            GrantCategory::Research,
            GrantType::Initial,
            GrantDisbursementType::Standard,
            1_000,
            250,
            1_000,
        )
        .expect("create");

        grant.approve().unwrap();
        grant.activate().unwrap();
        assert!(grant.disburse(1_250).is_ok());
        assert_eq!(grant.status, GrantStatus::Completed);
    }

    #[test]
    fn disburse_underflow() {
        let mut grant = Grant::new(
            2,
            5,
            sample_id(),
            GrantCategory::Development,
            GrantType::Core,
            GrantDisbursementType::Escrow,
            2_000,
            0,
            2_000,
        )
        .unwrap();

        grant.approve().unwrap();
        grant.activate().unwrap();
        assert!(grant.disburse(3_000).is_err());
    }
}
