//! Simple grant vote types for FSM governance.

use crate::grant::voting_types::VoteType;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum VoterType {
    MeshGroupMember,
    DaoMember,
    IdeaAuthor,
    Expert,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GrantVote {
    pub grant_id: u64,
    pub voter_id: [u8; 32],
    pub vote_type: VoteType,
    pub weight: u64,
    pub voter_type: VoterType,
    pub cast_at: i64,
}

impl GrantVote {
    pub fn calculate_weight(voter_type: VoterType) -> u64 {
        match voter_type {
            VoterType::MeshGroupMember => 2,
            _ => 1,
        }
    }

    pub fn calculate_final_weight(base_weight: u64, multiplier: u64) -> u64 {
        base_weight.saturating_mul(multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights() {
        assert_eq!(GrantVote::calculate_weight(VoterType::MeshGroupMember), 2);
        assert_eq!(GrantVote::calculate_weight(VoterType::DaoMember), 1);
    }

    #[test]
    fn final_weight() {
        assert_eq!(GrantVote::calculate_final_weight(2, 3), 6);
        assert_eq!(GrantVote::calculate_final_weight(1, 5), 5);
    }

    #[test]
    fn serialization() {
        let vote = GrantVote {
            grant_id: 1,
            voter_id: [1u8; 32],
            vote_type: VoteType::Approve,
            weight: 2,
            voter_type: VoterType::MeshGroupMember,
            cast_at: 1000,
        };
        let bytes = vote.try_to_vec().expect("serialization");
        let decoded = GrantVote::try_from_slice(&bytes).expect("deserialization");
        assert_eq!(decoded, vote);
    }
}
