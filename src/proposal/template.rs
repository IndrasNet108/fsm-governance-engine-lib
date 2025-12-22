//! Proposal Template module
//!
//! Handles proposal templates for standardized proposal creation

use crate::error::FsmError;
use std::marker::PhantomData;

/// Template field definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateField<P> {
    pub name: String,
    pub description: String,
    pub field_type: TemplateFieldType,
    pub required: bool,
    _phantom: PhantomData<P>,
}

/// Template field type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplateFieldType {
    Text,
    Number,
    Date,
    Choice, // For dropdown/choice fields (choices stored separately if needed)
}

/// Proposal Template account structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposalTemplate<P> {
    pub template_id: u64,
    pub name: String,
    pub description: String,
    pub proposal_type: String,
    pub fields: Vec<TemplateField<P>>, // Max 20 fields
    pub created_by: P,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub is_active: bool,
    _phantom: PhantomData<P>,
}

impl<P> ProposalTemplate<P> {
    /// Create a new proposal template
    pub fn new(
        template_id: u64,
        name: String,
        description: String,
        proposal_type: String,
        fields: Vec<TemplateField<P>>,
        created_by: P,
    ) -> Result<Self, FsmError> {
        Self::new_with_time(
            template_id,
            name,
            description,
            proposal_type,
            fields,
            created_by,
            0, // current_time placeholder
        )
    }

    /// Create a new proposal template with specified time
    pub fn new_with_time(
        template_id: u64,
        name: String,
        description: String,
        proposal_type: String,
        fields: Vec<TemplateField<P>>,
        created_by: P,
        current_time: i64,
    ) -> Result<Self, FsmError> {
        if !(!name.is_empty()) {
            return Err(FsmError::InvalidInput);
        }
        if !(name.len() <= 100) {
            return Err(FsmError::InvalidInput);
        }
        if !(!description.is_empty()) {
            return Err(FsmError::InvalidInput);
        }
        if !(description.len() <= 500) {
            return Err(FsmError::InvalidInput);
        }
        if !(!proposal_type.is_empty()) {
            return Err(FsmError::InvalidInput);
        }
        if !(proposal_type.len() <= 50) {
            return Err(FsmError::InvalidInput);
        }
        if !(fields.len() <= 20) {
            return Err(FsmError::InvalidInput);
        } // Max 20 fields

        // Validate all fields
        for field in &fields {
            if !(!field.name.is_empty()) {
                return Err(FsmError::InvalidInput);
            }
            if !(field.name.len() <= 50) {
                return Err(FsmError::InvalidInput);
            }
            if !(field.description.len() <= 200) {
                return Err(FsmError::InvalidInput);
            }
        }

        Ok(Self {
            template_id,
            name,
            description,
            proposal_type,
            fields,
            created_by,
            created_at: current_time,
            updated_at: None,
            is_active: true,
            _phantom: PhantomData,
        })
    }

    /// Update template
    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        fields: Option<Vec<TemplateField<P>>>,
    ) -> Result<(), FsmError> {
        self.update_with_time(name, description, fields, 0) // current_time placeholder
    }

    /// Update template with specified time
    pub fn update_with_time(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        fields: Option<Vec<TemplateField<P>>>,
        current_time: i64,
    ) -> Result<(), FsmError> {
        if let Some(new_name) = name {
            if !(!new_name.is_empty()) {
                return Err(FsmError::InvalidInput);
            }
            if !(new_name.len() <= 100) {
                return Err(FsmError::InvalidInput);
            }
            self.name = new_name;
        }

        if let Some(new_description) = description {
            if !(!new_description.is_empty()) {
                return Err(FsmError::InvalidInput);
            }
            if !(new_description.len() <= 500) {
                return Err(FsmError::InvalidInput);
            }
            self.description = new_description;
        }

        if let Some(new_fields) = fields {
            if !(new_fields.len() <= 20) {
                return Err(FsmError::InvalidInput);
            }
            // Validate all fields
            for field in &new_fields {
                if !(!field.name.is_empty()) {
                    return Err(FsmError::InvalidInput);
                }
                if !(field.name.len() <= 50) {
                    return Err(FsmError::InvalidInput);
                }
                if !(field.description.len() <= 200) {
                    return Err(FsmError::InvalidInput);
                }
            }
            self.fields = new_fields;
        }

        self.updated_at = Some(current_time);
        Ok(())
    }

    /// Deactivate template
    pub fn deactivate(&mut self) -> Result<(), FsmError> {
        self.deactivate_with_time(0) // current_time placeholder
    }

    /// Deactivate template with specified time
    pub fn deactivate_with_time(&mut self, current_time: i64) -> Result<(), FsmError> {
        if !(self.is_active) {
            return Err(FsmError::InvalidState);
        }
        self.is_active = false;
        self.updated_at = Some(current_time);
        Ok(())
    }

    /// Activate template
    pub fn activate(&mut self) -> Result<(), FsmError> {
        self.activate_with_time(0) // current_time placeholder
    }

    /// Activate template with specified time
    pub fn activate_with_time(&mut self, current_time: i64) -> Result<(), FsmError> {
        if !(!self.is_active) {
            return Err(FsmError::InvalidState);
        }
        self.is_active = true;
        self.updated_at = Some(current_time);
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

    fn create_test_field(name: &str) -> TemplateField<u8> {
        TemplateField {
            name: name.to_string(),
            description: "Test field".to_string(),
            field_type: TemplateFieldType::Text,
            required: false,
            _phantom: PhantomData,
        }
    }

    #[test]
    fn test_proposal_template_new_with_time() {
        let author = create_test_pubkey(1);
        let fields = vec![create_test_field("field1"), create_test_field("field2")];

        let template = ProposalTemplate::<u8>::new_with_time(
            1,
            "Test Template".to_string(),
            "Test Description".to_string(),
            "governance".to_string(),
            fields,
            author,
            1000,
        )
        .unwrap();

        assert_eq!(template.template_id, 1);
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.fields.len(), 2);
        assert_eq!(template.created_by, author);
        assert_eq!(template.created_at, 1000);
        assert!(template.is_active);
    }

    #[test]
    fn test_proposal_template_validation_too_many_fields() {
        let author = create_test_pubkey(1);
        let fields: Vec<TemplateField<u8>> = (0..21)
            .map(|i| create_test_field(&format!("field{}", i)))
            .collect();

        let result = ProposalTemplate::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            fields,
            author,
            1000,
        );
        assert_eq!(result.unwrap_err(), FsmError::InvalidInput);
    }

    #[test]
    fn test_proposal_template_update() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::<u8>::new_with_time(
            1,
            "Old Name".to_string(),
            "Old Description".to_string(),
            "governance".to_string(),
            vec![create_test_field("field1")],
            author,
            1000,
        )
        .unwrap();

        assert!(
            template
                .update_with_time(
                    Some("New Name".to_string()),
                    Some("New Description".to_string()),
                    None,
                    2000,
                )
                .is_ok()
        );

        assert_eq!(template.name, "New Name");
        assert_eq!(template.description, "New Description");
        assert!(template.updated_at.is_some());
    }

    #[test]
    fn test_proposal_template_deactivate() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            vec![],
            author,
            1000,
        )
        .unwrap();

        assert!(template.deactivate_with_time(2000).is_ok());
        assert!(!template.is_active);
        assert!(template.updated_at.is_some());
    }

    #[test]
    fn test_proposal_template_activate() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::<u8>::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            vec![],
            author,
            1000,
        )
        .unwrap();

        template.is_active = false;
        assert!(template.activate_with_time(2000).is_ok());
        assert!(template.is_active);
    }
}
