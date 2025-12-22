//! Governance Parameters module
//!
//! Adaptive governance parameters:
//! - quorum_percentage - quorum percentage (0-100)
//! - vote_duration_hours - voting duration in hours
//! - delegate_weight_percentage - delegate weight (0-100)
//! - early_quorum_enabled - early quorum enabled

use crate::error::FsmError;

/// Adaptive governance parameters
///
/// Governance parameters are not static - they adapt based on metrics and AI recommendations.
/// This is a shift from "code as law" to "code as living process".

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceParams {
    pub quorum_percentage: u8,          // Quorum percentage (0-100)
    pub vote_duration_hours: u64,       // Voting duration in hours
    pub delegate_weight_percentage: u8, // Delegate weight (0-100)
    pub early_quorum_enabled: bool,     // Early quorum enabled
    pub update_timestamp: i64,          // Last update time
}

impl GovernanceParams {
    /// Create new governance parameters
    pub fn new(
        quorum_percentage: u8,
        vote_duration_hours: u64,
        delegate_weight_percentage: u8,
        early_quorum_enabled: bool,
        current_time: i64,
    ) -> Result<Self, FsmError> {
        // Validate parameters
        if !(quorum_percentage > 0 && quorum_percentage <= 100) {
            return Err(FsmError::InvalidInput);
        }
        if !((24..=720).contains(&vote_duration_hours)) {
            // 24 hours - 30 days
            return Err(FsmError::InvalidInput);
        }
        if !(delegate_weight_percentage <= 100) {
            return Err(FsmError::InvalidInput);
        }

        Ok(Self {
            quorum_percentage,
            vote_duration_hours,
            delegate_weight_percentage,
            early_quorum_enabled,
            update_timestamp: current_time,
        })
    }

    /// Update governance parameters
    ///
    /// Updates parameters with validation and constraints:
    /// - Maximum change per update: ±10%
    /// - Minimum interval between changes: 24 hours (checked off-chain)
    pub fn update(
        &mut self,
        quorum_percentage: Option<u8>,
        vote_duration_hours: Option<u64>,
        delegate_weight_percentage: Option<u8>,
        early_quorum_enabled: Option<bool>,
        current_time: i64,
    ) -> Result<(), FsmError> {
        // Update parameters with validation
        if let Some(quorum) = quorum_percentage {
            if !(quorum > 0 && quorum <= 100) {
                return Err(FsmError::InvalidInput);
            }
            // Constraint: maximum change ±10%
            let diff = quorum.abs_diff(self.quorum_percentage);
            if !(diff <= 10) {
                return Err(FsmError::InvalidInput);
            }
            self.quorum_percentage = quorum;
        }

        if let Some(duration) = vote_duration_hours {
            if !((24..=720).contains(&duration)) {
                return Err(FsmError::InvalidInput);
            }
            self.vote_duration_hours = duration;
        }

        if let Some(weight) = delegate_weight_percentage {
            if !(weight <= 100) {
                return Err(FsmError::InvalidInput);
            }
            self.delegate_weight_percentage = weight;
        }

        if let Some(enabled) = early_quorum_enabled {
            self.early_quorum_enabled = enabled;
        }

        // Update timestamp
        self.update_timestamp = current_time;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_params_new() {
        let params = GovernanceParams::new(
            50,   // quorum_percentage
            168,  // vote_duration_hours (7 days)
            30,   // delegate_weight_percentage
            true, // early_quorum_enabled
            1000, // current_time
        )
        .unwrap();

        assert_eq!(params.quorum_percentage, 50);
        assert_eq!(params.vote_duration_hours, 168);
        assert_eq!(params.delegate_weight_percentage, 30);
        assert_eq!(params.early_quorum_enabled, true);
        assert_eq!(params.update_timestamp, 1000);
    }

    #[test]
    fn test_governance_params_new_validation() {
        // Test quorum_percentage = 0
        assert_eq!(
            GovernanceParams::new(0, 168, 30, true, 1000).unwrap_err(),
            FsmError::InvalidInput
        );

        // Test quorum_percentage > 100
        assert_eq!(
            GovernanceParams::new(101, 168, 30, true, 1000).unwrap_err(),
            FsmError::InvalidInput
        );

        // Test vote_duration_hours < 24
        assert_eq!(
            GovernanceParams::new(50, 23, 30, true, 1000).unwrap_err(),
            FsmError::InvalidInput
        );

        // Test vote_duration_hours > 720
        assert_eq!(
            GovernanceParams::new(50, 721, 30, true, 1000).unwrap_err(),
            FsmError::InvalidInput
        );

        // Test delegate_weight_percentage > 100
        assert_eq!(
            GovernanceParams::new(50, 168, 101, true, 1000).unwrap_err(),
            FsmError::InvalidInput
        );
    }

    #[test]
    fn test_governance_params_update() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Update quorum_percentage within limit (±10%)
        assert!(params.update(Some(55), None, None, None, 2000).is_ok());
        assert_eq!(params.quorum_percentage, 55);
        assert_eq!(params.update_timestamp, 2000);

        // Update vote_duration_hours
        assert!(params.update(None, Some(240), None, None, 3000).is_ok());
        assert_eq!(params.vote_duration_hours, 240);

        // Update delegate_weight_percentage
        assert!(params.update(None, None, Some(40), None, 4000).is_ok());
        assert_eq!(params.delegate_weight_percentage, 40);

        // Update early_quorum_enabled
        assert!(params.update(None, None, None, Some(false), 5000).is_ok());
        assert_eq!(params.early_quorum_enabled, false);
    }

    #[test]
    fn test_governance_params_new_validation_quorum_too_high() {
        let result = GovernanceParams::new(
            101, // Invalid: > 100
            168, 50, false, 1000,
        );
        assert_eq!(result.unwrap_err(), FsmError::InvalidInput);
    }

    #[test]
    fn test_governance_params_new_validation_duration_too_low() {
        let result = GovernanceParams::new(
            50, 23, // Invalid: < 24
            50, false, 1000,
        );
        assert_eq!(result.unwrap_err(), FsmError::InvalidInput);
    }

    #[test]
    fn test_governance_params_new_validation_duration_too_high() {
        let result = GovernanceParams::new(
            50, 721, // Invalid: > 720
            50, false, 1000,
        );
        assert_eq!(result.unwrap_err(), FsmError::InvalidInput);
    }

    #[test]
    fn test_governance_params_update_quorum_limit() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Try to update quorum_percentage beyond ±10% limit
        assert_eq!(
            params.update(Some(61), None, None, None, 2000).unwrap_err(),
            FsmError::InvalidInput
        ); // +11%
        assert_eq!(
            params.update(Some(39), None, None, None, 2000).unwrap_err(),
            FsmError::InvalidInput
        ); // -11%

        // Update within limit should work
        assert!(params.update(Some(60), None, None, None, 2000).is_ok()); // +10%
        assert!(params.update(Some(50), None, None, None, 3000).is_ok()); // -10%
    }

    #[test]
    fn test_governance_params_update_validation() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Test invalid quorum_percentage
        assert_eq!(
            params.update(Some(0), None, None, None, 2000).unwrap_err(),
            FsmError::InvalidInput
        );
        assert_eq!(
            params
                .update(Some(101), None, None, None, 2000)
                .unwrap_err(),
            FsmError::InvalidInput
        );

        // Test invalid vote_duration_hours
        assert_eq!(
            params.update(None, Some(23), None, None, 2000).unwrap_err(),
            FsmError::InvalidInput
        );
        assert_eq!(
            params
                .update(None, Some(721), None, None, 2000)
                .unwrap_err(),
            FsmError::InvalidInput
        );

        // Test invalid delegate_weight_percentage
        assert_eq!(
            params
                .update(None, None, Some(101), None, 2000)
                .unwrap_err(),
            FsmError::InvalidInput
        );
    }

    #[test]
    fn test_governance_params_update_multiple_fields() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Update multiple fields at once
        assert!(
            params
                .update(Some(55), Some(240), Some(35), Some(false), 2000)
                .is_ok()
        );

        assert_eq!(params.quorum_percentage, 55);
        assert_eq!(params.vote_duration_hours, 240);
        assert_eq!(params.delegate_weight_percentage, 35);
        assert_eq!(params.early_quorum_enabled, false);
        assert_eq!(params.update_timestamp, 2000);
    }

    #[test]
    fn test_governance_params_update_boundary_values() {
        // Start with quorum = 10 to allow testing boundary value 1 (within ±10% limit)
        let mut params = GovernanceParams::new(10, 168, 30, true, 1000).unwrap();

        // Test boundary value for quorum = 1 (10 - 1 = 9, within ±10% limit)
        assert!(params.update(Some(1), None, None, None, 2000).is_ok());
        assert_eq!(params.quorum_percentage, 1);

        // Now test quorum = 100 (1 -> 100 is too big, need to go step by step)
        // First go to 11 (1 + 10 = 11, within limit)
        assert!(params.update(Some(11), None, None, None, 3000).is_ok());
        assert_eq!(params.quorum_percentage, 11);

        // Test boundary values for duration (24 and 720)
        assert!(params.update(None, Some(24), None, None, 4000).is_ok());
        assert_eq!(params.vote_duration_hours, 24);

        assert!(params.update(None, Some(720), None, None, 5000).is_ok());
        assert_eq!(params.vote_duration_hours, 720);

        // Test boundary value for delegate weight (0 and 100)
        assert!(params.update(None, None, Some(0), None, 6000).is_ok());
        assert_eq!(params.delegate_weight_percentage, 0);

        assert!(params.update(None, None, Some(100), None, 7000).is_ok());
        assert_eq!(params.delegate_weight_percentage, 100);
    }

    #[test]
    fn test_governance_params_update_quorum_exact_limits() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Test exact ±10% limits
        assert!(params.update(Some(60), None, None, None, 2000).is_ok()); // +10%
        assert_eq!(params.quorum_percentage, 60);

        assert!(params.update(Some(50), None, None, None, 3000).is_ok()); // -10%
        assert_eq!(params.quorum_percentage, 50);

        // Test beyond limits
        assert_eq!(
            params.update(Some(61), None, None, None, 4000).unwrap_err(),
            FsmError::InvalidInput
        ); // +11%
        assert_eq!(
            params.update(Some(39), None, None, None, 5000).unwrap_err(),
            FsmError::InvalidInput
        ); // -11%
    }

    #[test]
    fn test_governance_params_structure() {
        let params = GovernanceParams::new(75, 336, 50, false, 5000).unwrap();

        assert_eq!(params.quorum_percentage, 75);
        assert_eq!(params.vote_duration_hours, 336);
        assert_eq!(params.delegate_weight_percentage, 50);
        assert_eq!(params.early_quorum_enabled, false);
        assert_eq!(params.update_timestamp, 5000);
    }

    #[test]
    fn test_governance_params_new_validation_quorum_zero() {
        assert_eq!(
            GovernanceParams::new(0, 168, 30, true, 1000).unwrap_err(),
            FsmError::InvalidInput
        );
    }

    #[test]
    fn test_governance_params_new_validation_delegate_weight_zero_allowed() {
        // delegate_weight_percentage = 0 is allowed (can be 0-100)
        assert!(GovernanceParams::new(50, 168, 0, true, 1000).is_ok());
    }

    #[test]
    fn test_governance_params_new_with_time_all_fields() {
        let params = GovernanceParams::new(75, 336, 50, false, 5000).unwrap();

        assert_eq!(params.quorum_percentage, 75);
        assert_eq!(params.vote_duration_hours, 336);
        assert_eq!(params.delegate_weight_percentage, 50);
        assert_eq!(params.early_quorum_enabled, false);
        assert_eq!(params.update_timestamp, 5000);
    }

    #[test]
    fn test_governance_params_update_preserves_unchanged_fields() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Update only quorum_percentage
        assert!(params.update(Some(55), None, None, None, 2000).is_ok());

        assert_eq!(params.quorum_percentage, 55);
        assert_eq!(params.vote_duration_hours, 168); // Unchanged
        assert_eq!(params.delegate_weight_percentage, 30); // Unchanged
        assert_eq!(params.early_quorum_enabled, true); // Unchanged
        assert_eq!(params.update_timestamp, 2000);
    }

    #[test]
    fn test_governance_params_update_quorum_too_large_change() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Try to change quorum by more than 10% - should fail
        assert_eq!(
            params.update(Some(65), None, None, None, 2000).unwrap_err(),
            FsmError::InvalidInput
        );
        assert_eq!(
            params.update(Some(39), None, None, None, 2000).unwrap_err(),
            FsmError::InvalidInput
        );

        assert_eq!(params.quorum_percentage, 50); // Should remain unchanged
    }

    #[test]
    fn test_governance_params_update_vote_duration_boundary() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Test minimum boundary (24 hours)
        assert!(params.update(None, Some(24), None, None, 2000).is_ok());
        assert_eq!(params.vote_duration_hours, 24);

        // Test maximum boundary (720 hours = 30 days)
        assert!(params.update(None, Some(720), None, None, 3000).is_ok());
        assert_eq!(params.vote_duration_hours, 720);
    }

    #[test]
    fn test_governance_params_update_vote_duration_invalid() {
        let mut params = GovernanceParams::new(50, 168, 30, true, 1000).unwrap();

        // Test below minimum (23 hours)
        assert_eq!(
            params.update(None, Some(23), None, None, 2000).unwrap_err(),
            FsmError::InvalidInput
        );

        // Test above maximum (721 hours)
        assert_eq!(
            params
                .update(None, Some(721), None, None, 3000)
                .unwrap_err(),
            FsmError::InvalidInput
        );
    }
}
