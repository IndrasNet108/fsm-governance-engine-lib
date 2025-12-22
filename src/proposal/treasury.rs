//! Treasury Proposal types
//!
//! Specialized proposal types for Treasury operations

use crate::error::FsmError;
use std::marker::PhantomData;

/// Treasury Proposal Type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreasuryProposalType {
    /// Withdraw funds from treasury
    Withdrawal,
    /// Deposit funds to treasury
    Deposit,
    /// Transfer funds between treasuries
    Transfer,
    /// Grant capability for treasury operations
    GrantCapability,
    /// Revoke capability
    RevokeCapability,
    /// Update treasury configuration
    UpdateConfig,
}

/// Treasury Proposal Operation Data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreasuryOperationData<P> {
    pub operation_type: TreasuryProposalType,
    pub amount: Option<u64>,             // For withdrawal, deposit, transfer
    pub target_treasury: Option<P>,      // For transfer
    pub capability_grantee: Option<P>,   // For grant/revoke capability
    pub capability_type: Option<String>, // For grant capability
    pub expires_at: Option<i64>,         // For grant capability
    pub description: String,             // Operation description
    _phantom: PhantomData<P>,
}

impl<P> TreasuryOperationData<P> {
    pub fn validate(&self, current_time: i64) -> Result<(), FsmError> {
        match self.operation_type {
            TreasuryProposalType::Withdrawal
            | TreasuryProposalType::Deposit
            | TreasuryProposalType::Transfer => {
                if !(self.amount.is_some()) {
                    return Err(FsmError::InvalidInput);
                }
                if !(self.amount.unwrap() > 0) {
                    return Err(FsmError::InvalidInput);
                }
                if self.operation_type == TreasuryProposalType::Transfer {
                    if !(self.target_treasury.is_some()) {
                        return Err(FsmError::InvalidInput);
                    }
                }
            }
            TreasuryProposalType::GrantCapability => {
                if !(self.capability_grantee.is_some()) {
                    return Err(FsmError::InvalidInput);
                }
                if !(self.capability_type.is_some()) {
                    return Err(FsmError::InvalidInput);
                }
                if !(self.expires_at.is_some()) {
                    return Err(FsmError::InvalidInput);
                }
                if let Some(exp) = self.expires_at {
                    if !(exp > current_time) {
                        return Err(FsmError::InvalidInput);
                    }
                }
            }
            TreasuryProposalType::RevokeCapability => {
                if !(self.capability_grantee.is_some()) {
                    return Err(FsmError::InvalidInput);
                }
            }
            TreasuryProposalType::UpdateConfig => {
                // No specific requirements for config updates
            }
        }
        if !(!self.description.is_empty()) {
            return Err(FsmError::InvalidInput);
        }
        if !(self.description.len() <= 200) {
            return Err(FsmError::InvalidInput);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FsmError;
    use std::marker::PhantomData;

    fn create_test_pubkey(seed: u8) -> u8 {
        seed
    }

    #[test]
    fn test_treasury_operation_data_withdrawal() {
        let data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::Withdrawal,
            amount: Some(1000),
            target_treasury: None,
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Withdraw for expenses".to_string(),
            _phantom: PhantomData,
        };
        // Validation would require Clock, so skip in unit test
        assert_eq!(data.operation_type, TreasuryProposalType::Withdrawal);
        assert_eq!(data.amount, Some(1000));
    }

    #[test]
    fn test_treasury_operation_data_transfer() {
        let target = 1u8; // Using u8 placeholder for Pubkey
        let data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::Transfer,
            amount: Some(5000),
            target_treasury: Some(target),
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Transfer to secondary treasury".to_string(),
            _phantom: PhantomData,
        };
        assert_eq!(data.operation_type, TreasuryProposalType::Transfer);
        assert_eq!(data.target_treasury, Some(target));
    }

    #[test]
    fn test_treasury_operation_data_validate_withdrawal() {
        let data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::Withdrawal,
            amount: Some(1000),
            target_treasury: None,
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Withdrawal description".to_string(),
            _phantom: PhantomData,
        };
        assert!(data.validate(0).is_ok());

        let invalid_data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::Withdrawal,
            amount: None, // Invalid
            target_treasury: None,
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Withdrawal description".to_string(),
            _phantom: PhantomData,
        };
        assert_eq!(
            invalid_data.validate(0).unwrap_err(),
            FsmError::InvalidInput
        );
    }

    #[test]
    fn test_treasury_operation_data_validate_transfer() {
        let target = create_test_pubkey(2);
        let data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::Transfer,
            amount: Some(500),
            target_treasury: Some(target),
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Transfer description".to_string(),
            _phantom: PhantomData,
        };
        assert!(data.validate(0).is_ok());

        let invalid_data_no_target = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::Transfer,
            amount: Some(500),
            target_treasury: None, // Invalid
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "Transfer description".to_string(),
            _phantom: PhantomData,
        };
        assert_eq!(
            invalid_data_no_target.validate(0).unwrap_err(),
            FsmError::InvalidInput
        );
    }

    #[test]
    fn test_treasury_operation_data_validate_grant_capability() {
        let grantee = create_test_pubkey(3);
        let expires = 1000;
        let data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::GrantCapability,
            amount: None,
            target_treasury: None,
            capability_grantee: Some(grantee),
            capability_type: Some("Admin".to_string()),
            expires_at: Some(expires),
            description: "Grant capability description".to_string(),
            _phantom: PhantomData,
        };
        assert!(data.validate(expires - 1).is_ok()); // current_time < expires

        let invalid_data_expired = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::GrantCapability,
            amount: None,
            target_treasury: None,
            capability_grantee: Some(grantee),
            capability_type: Some("Admin".to_string()),
            expires_at: Some(expires),
            description: "Grant capability description".to_string(),
            _phantom: PhantomData,
        };
        assert_eq!(
            invalid_data_expired.validate(expires + 1).unwrap_err(),
            FsmError::InvalidInput
        );
    }

    #[test]
    fn test_treasury_operation_data_validate_revoke_capability() {
        let grantee = create_test_pubkey(4);
        let data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::RevokeCapability,
            amount: None,
            target_treasury: None,
            capability_grantee: Some(grantee),
            capability_type: None,
            expires_at: None,
            description: "Revoke capability description".to_string(),
            _phantom: PhantomData,
        };
        assert!(data.validate(0).is_ok());

        let invalid_data_no_grantee = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::RevokeCapability,
            amount: None,
            target_treasury: None,
            capability_grantee: None, // Invalid
            capability_type: None,
            expires_at: None,
            description: "Revoke capability description".to_string(),
            _phantom: PhantomData,
        };
        assert_eq!(
            invalid_data_no_grantee.validate(0).unwrap_err(),
            FsmError::InvalidInput
        );
    }

    #[test]
    fn test_treasury_operation_data_validate_description_empty() {
        let data = TreasuryOperationData::<u8> {
            operation_type: TreasuryProposalType::Withdrawal,
            amount: Some(100),
            target_treasury: None,
            capability_grantee: None,
            capability_type: None,
            expires_at: None,
            description: "".to_string(), // Invalid
            _phantom: PhantomData,
        };
        assert_eq!(data.validate(0).unwrap_err(), FsmError::InvalidInput);
    }
}
