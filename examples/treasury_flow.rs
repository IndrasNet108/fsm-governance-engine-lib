//! Example: treasury operation validation.

use fsm_governance_engine_lib::proposal::treasury::{TreasuryOperationData, TreasuryProposalType};

fn main() {
    let operation = TreasuryOperationData::new(
        TreasuryProposalType::Transfer,
        Some(5_000),
        Some(7u8),
        None,
        None,
        None,
        "Transfer to ops treasury".to_string(),
    );

    operation.validate(0).expect("treasury validation");
    println!(
        "Treasury operation validated: {:?}",
        operation.operation_type
    );
}
