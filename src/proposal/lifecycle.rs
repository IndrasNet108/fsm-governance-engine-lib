//! Proposal lifecycle methods
use super::types::{Proposal, ProposalStatus};
use crate::error::FsmError;
use std::marker::PhantomData;
impl<P> Proposal<P> {
    /// Create a new proposal with current time
    pub fn new(
        id: u64,
        title: String,
        description: String,
        proposal_type: String,
        author: P,
    ) -> Result<Proposal<P>, FsmError> {
        Self::new_with_time(id, title, description, proposal_type, author, 0)
    }
    /// Create a new proposal with specified time
    pub fn new_with_time(
        id: u64,
        title: String,
        description: String,
        proposal_type: String,
        author: P,
        current_time: i64,
    ) -> Result<Proposal<P>, FsmError> {
        if !(!title.is_empty()) {
            return Err(FsmError::InvalidInput);
        }
        if !(title.len() <= 200) {
            return Err(FsmError::InvalidInput);
        }
        if !(!description.is_empty()) {
            return Err(FsmError::InvalidInput);
        }
        if !(description.len() <= 2000) {
            return Err(FsmError::InvalidInput);
        }
        if !(!proposal_type.is_empty()) {
            return Err(FsmError::InvalidInput);
        }
        if !(proposal_type.len() <= 50) {
            return Err(FsmError::InvalidInput);
        }
        Ok(Self {
            id,
            title,
            description,
            proposal_type,
            author,
            created_at: current_time,
            updated_at: None,
            submitted_at: None,
            cancelled_at: None,
            executed_at: None,
            archived_at: None,
            voting_duration: 7 * 24 * 3600, // 7 days default
            status: ProposalStatus::Draft,
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            last_tallied_at: None,
            cancellation_reason: None,
            execution_data: None,
            expires_at: None,
            idea_id: None,
            treasury_operation: None,
            _phantom: PhantomData,
        })
    }
    /// Activate proposal (move from Draft to Active)
    pub fn activate(&mut self, min_quorum: u64, total_members: u64) -> Result<(), FsmError> {
        self.activate_with_time(min_quorum, total_members, 0)
    }
    /// Activate proposal with specified time
    pub fn activate_with_time(
        &mut self,
        min_quorum: u64,
        total_members: u64,
        current_time: i64,
    ) -> Result<(), FsmError> {
        if !(self.status == ProposalStatus::Draft) {
            return Err(FsmError::InvalidInput);
        }
        if !(total_members >= min_quorum) {
            return Err(FsmError::InsufficientMembers);
        }
        if !(min_quorum > 0) {
            return Err(FsmError::InvalidInput);
        }
        if !(total_members > 0) {
            return Err(FsmError::InvalidInput);
        }

        self.status = ProposalStatus::Active;
        self.submitted_at = Some(current_time);
        Ok(())
    }
    /// Pass proposal (move from Active to Passed)
    pub fn pass(&mut self) -> Result<(), FsmError> {
        self.pass_with_time(0)
    }
    /// Pass proposal with specified time
    pub fn pass_with_time(&mut self, current_time: i64) -> Result<(), FsmError> {
        if !(self.status == ProposalStatus::Active) {
            return Err(FsmError::InvalidInput);
        }

        // Check that voting is completed
        let voting_end = self.created_at + self.voting_duration;
        if !(current_time >= voting_end) {
            return Err(FsmError::InvalidState);
        }

        self.status = ProposalStatus::Passed;
        Ok(())
    }
    /// Reject proposal (move from Active to Rejected)
    pub fn reject(&mut self) -> Result<(), FsmError> {
        self.reject_with_time(0)
    }
    /// Reject proposal with specified time
    pub fn reject_with_time(&mut self, current_time: i64) -> Result<(), FsmError> {
        if !(self.status == ProposalStatus::Active) {
            return Err(FsmError::InvalidInput);
        }

        // Check that voting is completed
        let voting_end = self.created_at + self.voting_duration;
        if !(current_time >= voting_end) {
            return Err(FsmError::InvalidState);
        }

        self.status = ProposalStatus::Rejected;
        Ok(())
    }
    /// Execute proposal (move from Passed to Executed)
    pub fn execute(&mut self) -> Result<(), FsmError> {
        self.execute_with_time(0)
    }
    /// Execute proposal with specified time
    pub fn execute_with_time(&mut self, current_time: i64) -> Result<(), FsmError> {
        if !(self.status == ProposalStatus::Passed) {
            return Err(FsmError::InvalidInput);
        }
        if !(self.executed_at.is_none()) {
            return Err(FsmError::InvalidState);
        }

        self.status = ProposalStatus::Executed;
        self.executed_at = Some(current_time);
        Ok(())
    }
    /// Cancel proposal (move from Draft or Active to Cancelled)
    pub fn cancel(&mut self, reason: String) -> Result<(), FsmError> {
        self.cancel_with_time(reason, 0)
    }
    /// Cancel proposal with specified time
    pub fn cancel_with_time(&mut self, reason: String, current_time: i64) -> Result<(), FsmError> {
        if !(self.status == ProposalStatus::Draft || self.status == ProposalStatus::Active) {
            return Err(FsmError::InvalidInput);
        }
        self.status = ProposalStatus::Cancelled;
        self.cancelled_at = Some(current_time);
        self.cancellation_reason = Some(reason);
        Ok(())
    }
    /// Archive proposal (move from Executed, Rejected, or Cancelled to Archived)
    pub fn archive(&mut self) -> Result<(), FsmError> {
        self.archive_with_time(0)
    }
    /// Archive proposal with specified time
    pub fn archive_with_time(&mut self, current_time: i64) -> Result<(), FsmError> {
        if !(self.status == ProposalStatus::Executed
            || self.status == ProposalStatus::Rejected
            || self.status == ProposalStatus::Cancelled)
        {
            return Err(FsmError::InvalidInput);
        }
        self.status = ProposalStatus::Archived;
        self.archived_at = Some(current_time);
        Ok(())
    }
    /// Check if proposal has expired and auto-archive if needed
    /// Returns true if proposal was archived, false otherwise
    pub fn check_and_auto_archive(&mut self, current_time: i64) -> Result<bool, FsmError> {
        if let Some(expires_at) = self.expires_at {
            if current_time >= expires_at {
                // Only auto-archive if in a finalizable state
                if self.status == ProposalStatus::Executed
                    || self.status == ProposalStatus::Rejected
                    || self.status == ProposalStatus::Cancelled
                {
                    self.archive_with_time(current_time)?;
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    /// Set expiration time for proposal
    pub fn set_expiration(&mut self, expires_at: Option<i64>) -> Result<(), FsmError> {
        if let Some(exp) = expires_at {
            if !(exp > self.created_at) {
                return Err(FsmError::InvalidInput);
            }
        }
        self.expires_at = expires_at;
        Ok(())
    }
    /// Automatically transition Active proposal to Passed/Rejected based on votes
    /// This checks voting period end and vote counts
    pub fn auto_transition_after_voting(&mut self, current_time: i64) -> Result<bool, FsmError> {
        if self.status != ProposalStatus::Active {
            return Ok(false);
        }
        // Check if voting period has ended
        // Use submitted_at if available (when proposal was activated), otherwise created_at
        let voting_start = self.submitted_at.unwrap_or(self.created_at);
        let voting_end = voting_start
            .checked_add(self.voting_duration)
            .ok_or(FsmError::Overflow)?;
        if current_time >= voting_end {
            // Determine result based on votes
            if self.yes_votes > self.no_votes {
                self.pass_with_time(current_time)?;
                return Ok(true);
            } else if self.no_votes > self.yes_votes {
                self.reject_with_time(current_time)?;
                return Ok(true);
            } else {
                // Tied - set status to Tied
                self.status = ProposalStatus::Tied;
                self.last_tallied_at = Some(current_time);
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// Check if proposal can be auto-activated (for future use)
    /// Currently returns false - activation requires manual call
    pub fn can_auto_activate(&self) -> bool {
        // Future: could check conditions like minimum support, time since creation, etc.
        false
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
    fn test_proposal_new_with_time() {
        let author = create_test_pubkey(1);
        let proposal = Proposal::<u8>::new_with_time(
            1,
            "Test Proposal".to_string(),
            "Test Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        assert_eq!(proposal.id);
        assert_eq!(proposal.title, "Test Proposal");
        assert_eq!(proposal.author, author);
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert_eq!(proposal.created_at);
    }
    #[test]
    fn test_proposal_new_validation_empty_title() {
        let author = create_test_pubkey(1);
        let result = Proposal::<u8>::new_with_time(
            1,
            String::new(), // Invalid: empty
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        );
        assert_eq!(result.unwrap_err(), FsmError::InvalidInput);
    }
    #[test]
    fn test_proposal_activate_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        assert!(proposal.activate_with_time(10, 20).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Active);
        assert_eq!(proposal.submitted_at, Some(2000));
    }
    #[test]
    fn test_proposal_activate_insufficient_members() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        // total_members < min_quorum
        assert_eq!(
            proposal.activate_with_time(20, 10).unwrap_err(),
            FsmError::InsufficientMembers
        );
    }
    #[test]
    fn test_proposal_pass_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();

        // Pass after voting duration
        let voting_end = proposal.created_at + proposal.voting_duration;
        assert!(proposal.pass_with_time(voting_end + 1).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }
    #[test]
    fn test_proposal_pass_before_voting_end() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();

        // Try to pass before voting ends - should fail
        assert_eq!(
            proposal.pass_with_time(2000).unwrap_err(),
            FsmError::InvalidState
        );
    }
    #[test]
    fn test_proposal_reject_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;

        assert!(proposal.reject_with_time(voting_end + 1).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }
    #[test]
    fn test_proposal_cancel_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();

        assert!(
            proposal
                .cancel_with_time("Changed mind".to_string())
                .is_ok()
        );
        assert_eq!(proposal.status, ProposalStatus::Cancelled);
        assert_eq!(
            proposal.cancellation_reason,
            Some("Changed mind".to_string())
        );
        assert_eq!(proposal.cancelled_at, Some(3000));
    }
    #[test]
    fn test_proposal_execute_with_time() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();

        assert!(proposal.execute_with_time(5000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Executed);
        assert_eq!(proposal.executed_at, Some(5000));
        // execution_data is set separately in real usage
    }
    #[test]
    fn test_proposal_archive_with_time_executed() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();
        proposal.execute_with_time(3000).unwrap();

        // Can archive executed proposal
        assert!(proposal.archive_with_time(4000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Archived);
        assert_eq!(proposal.archived_at, Some(4000));
    }
    #[test]
    fn test_proposal_archive_with_time_rejected() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();

        // Can archive rejected proposal
        assert!(proposal.archive_with_time(4000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }
    #[test]
    fn test_proposal_archive_with_time_cancelled() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        proposal
            .cancel_with_time("Changed mind".to_string())
            .unwrap();

        // Can archive cancelled proposal
        assert!(proposal.archive_with_time(4000).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }
    #[test]
    fn test_proposal_archive_invalid_status() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        // Cannot archive Draft or Active proposal
        assert_eq!(
            proposal.archive_with_time(4000).unwrap_err(),
            FsmError::InvalidInput
        );

        proposal.activate_with_time(10, 20).unwrap();
        assert_eq!(
            proposal.archive_with_time(4000).unwrap_err(),
            FsmError::InvalidInput
        );
    }
    #[test]
    fn test_proposal_new_with_time_all_fields() {
        let author = create_test_pubkey(5);
        let proposal = Proposal::<u8>::new_with_time(
            999,
            "Title".to_string(),
            "Description".to_string(),
            "type".to_string(),
            author,
            5000,
        )
        .unwrap();

        assert_eq!(proposal.id);
        assert_eq!(proposal.title, "Title");
        assert_eq!(proposal.description, "Description");
        assert_eq!(proposal.proposal_type, "type");
        assert_eq!(proposal.author, author);
        assert_eq!(proposal.created_at);
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert_eq!(proposal.voting_duration, 7 * 24 * 3600);
    }
    #[test]
    fn test_proposal_activate_with_time_zero_quorum() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        // Zero quorum should fail
        assert_eq!(
            proposal.activate_with_time(0, 10).unwrap_err(),
            FsmError::InvalidInput
        );
    }
    #[test]
    fn test_proposal_activate_with_time_zero_total_members() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        // Zero total members should fail
        assert_eq!(
            proposal.activate_with_time(10, 0).unwrap_err(),
            FsmError::InvalidInput
        );
    }
    #[test]
    fn test_proposal_pass_with_time_exact_voting_end() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();

        // Pass exactly at voting end
        let voting_end = proposal.created_at + proposal.voting_duration;
        assert!(proposal.pass_with_time(voting_end).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }
    #[test]
    fn test_proposal_execute_with_time_already_executed() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();
        proposal.execute_with_time(5000).unwrap();

        // Try to execute again - should fail
        assert_eq!(
            proposal.execute_with_time(6000).unwrap_err(),
            FsmError::InvalidState
        );
    }
    #[test]
    fn test_proposal_cancel_with_time_draft() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        // Cancel from Draft
        assert!(proposal.cancel_with_time("Reason".to_string()).is_ok());
        assert_eq!(proposal.status, ProposalStatus::Cancelled);
        assert_eq!(proposal.cancellation_reason, Some("Reason".to_string()));
    }
    #[test]
    fn test_proposal_cancel_with_time_invalid_status() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.pass_with_time(voting_end + 1).unwrap();

        // Cannot cancel Passed proposal
        assert_eq!(
            proposal.cancel_with_time("Reason".to_string()).unwrap_err(),
            FsmError::InvalidInput
        );
    }
    // ========== New lifecycle methods tests ==========
    #[test]
    fn test_proposal_set_expiration() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        // Set expiration in the future
        assert!(proposal.set_expiration(Some(5000)).is_ok());
        assert_eq!(proposal.expires_at, Some(5000));

        // Clear expiration
        assert!(proposal.set_expiration(None).is_ok());
        assert_eq!(proposal.expires_at, None);
    }
    #[test]
    fn test_proposal_set_expiration_invalid() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        // Expiration before creation should fail
        assert_eq!(
            proposal.set_expiration(Some(500)).unwrap_err(),
            FsmError::InvalidInput
        );
    }
    #[test]
    fn test_proposal_check_and_auto_archive_expired() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();

        // Set expiration in the past
        proposal.expires_at = Some(5000);

        // Should auto-archive
        assert!(proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Archived);
    }
    #[test]
    fn test_proposal_check_and_auto_archive_not_expired() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        let voting_end = proposal.created_at + proposal.voting_duration;
        proposal.reject_with_time(voting_end + 1).unwrap();

        // Set expiration in the future
        proposal.expires_at = Some(10000);

        // Should not archive
        assert!(!proposal.check_and_auto_archive(6000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }
    #[test]
    fn test_proposal_auto_transition_after_voting_yes_wins() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        proposal.yes_votes = 100;
        proposal.no_votes = 50;

        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;

        // Should auto-transition to Passed
        assert!(
            proposal
                .auto_transition_after_voting(voting_end + 1)
                .unwrap()
        );
        assert_eq!(proposal.status, ProposalStatus::Passed);
    }
    #[test]
    fn test_proposal_auto_transition_after_voting_no_wins() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        proposal.yes_votes = 50;
        proposal.no_votes = 100;

        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;

        // Should auto-transition to Rejected
        assert!(
            proposal
                .auto_transition_after_voting(voting_end + 1)
                .unwrap()
        );
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }
    #[test]
    fn test_proposal_auto_transition_after_voting_tied() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        proposal.yes_votes = 100;
        proposal.no_votes = 100;

        let voting_end = proposal.submitted_at.unwrap() + proposal.voting_duration;

        // Should auto-transition to Tied
        assert!(
            proposal
                .auto_transition_after_voting(voting_end + 1)
                .unwrap()
        );
        assert_eq!(proposal.status, ProposalStatus::Tied);
    }
    #[test]
    fn test_proposal_auto_transition_before_voting_end() {
        let author = create_test_pubkey(1);
        let mut proposal = Proposal::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            author,
            1000,
        )
        .unwrap();

        proposal.activate_with_time(10, 20).unwrap();
        proposal.yes_votes = 100;
        proposal.no_votes = 50;

        // Try to auto-transition before voting ends - should not transition
        assert!(!proposal.auto_transition_after_voting(2000).unwrap());
        assert_eq!(proposal.status, ProposalStatus::Active);
    }
}
