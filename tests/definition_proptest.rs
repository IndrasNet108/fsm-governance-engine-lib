use fsm_governance_engine_lib::{FsmDefinition, FsmError, FsmTransition};
use proptest::prelude::*;
use proptest::string::string_regex;

fn base_definition(action: String, from: &str, to: &str) -> FsmDefinition {
    FsmDefinition {
        states: vec!["A".into(), "B".into()],
        transitions: vec![FsmTransition {
            from: from.into(),
            to: to.into(),
            action,
            guard: None,
            metadata: None,
        }],
        defaults: None,
        invariants: Vec::new(),
    }
}

proptest! {
    #[test]
    fn prop_valid_structure(action in string_regex("[a-z]{1,8}").unwrap()) {
        let definition = base_definition(action, "A", "B");
        prop_assert!(definition.validate_structure().is_ok());
    }

    #[test]
    fn prop_whitespace_action_invalid(action in string_regex("\\s{1,4}").unwrap()) {
        let definition = base_definition(action, "A", "B");
        prop_assert_eq!(definition.validate_structure(), Err(FsmError::InvalidInput));
    }

    #[test]
    fn prop_unknown_from_invalid(action in string_regex("[a-z]{1,8}").unwrap(),
                                 from in string_regex("[C-Z]{1,2}").unwrap()) {
        let definition = base_definition(action, &from, "B");
        prop_assert_eq!(definition.validate_structure(), Err(FsmError::InvalidInput));
    }
}
