//! Proposal Amendment module
//!
//! Handles amendments to proposals during the Draft phase
use crate::error::FsmError;
use std::marker::PhantomData;
/// Proposal Amendment account structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposalAmendment<P> {
    pub amendment_id: u64,
    pub proposal_id: u64,
    pub author: P,
    pub content: String,
    pub created_at: i64,
    _phantom: PhantomData<P>,
}
impl<P> ProposalAmendment<P> {
    /// Create a new proposal amendment
    pub fn new(
        amendment_id: u64,
        proposal_id: u64,
        author: P,
        content: String,
    ) -> Result<ProposalAmendment<P>, FsmError> {
        Self::new_with_time(amendment_id, proposal_id, author, content, 0)
    }
    /// Create a new proposal amendment with specified time
    pub fn new_with_time(
        amendment_id: u64,
        proposal_id: u64,
        author: P,
        content: String,
        current_time: i64,
    ) -> Result<ProposalAmendment<P>, FsmError> {
        if content.is_empty() {
            return Err(FsmError::InvalidInput);
        }
        if content.len() > 2000 {
            return Err(FsmError::InvalidInput);
        }
        Ok(Self {
            amendment_id,
            proposal_id,
            author,
            content,
            created_at: current_time,
            _phantom: PhantomData,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FsmError;
    fn create_test_pubkey(seed: u8) -> u8 {
        seed
    }
    #[test]
    fn test_proposal_amendment_new_with_time() {
        let author = create_test_pubkey(1);
        let amendment = ProposalAmendment::<u8>::new_with_time(
            1,
            100,
            author,
            "Amendment content".to_string(),
            1000,
        )
        .unwrap();
        assert_eq!(amendment.amendment_id, 1);
        assert_eq!(amendment.proposal_id, 100);
        assert_eq!(amendment.author, author);
        assert_eq!(amendment.content, "Amendment content");
        assert_eq!(amendment.created_at, 1000);
    }
    #[test]
    fn test_proposal_amendment_validation_empty_content() {
        let author = create_test_pubkey(1);
        let result = ProposalAmendment::<u8>::new_with_time(1, 100, author, String::new(), 1000);
        assert_eq!(result.unwrap_err(), FsmError::InvalidInput);
    }
    #[test]
    fn test_proposal_amendment_validation_content_too_long() {
        let author = create_test_pubkey(1);
        let content = "a".repeat(2001);
        let result = ProposalAmendment::<u8>::new_with_time(1, 100, author, content, 1000);
        assert_eq!(result.unwrap_err(), FsmError::InvalidInput);
    }
}
