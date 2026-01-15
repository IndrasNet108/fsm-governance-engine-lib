use fsm_governance_engine_lib::{
    FsmDefaults, FsmDefinition, FsmError, FsmInvariant, FsmTransition, FsmTransitionRef,
};

fn base_definition() -> FsmDefinition {
    FsmDefinition {
        states: vec!["A".into(), "B".into()],
        transitions: vec![FsmTransition {
            from: "A".into(),
            to: "B".into(),
            action: "go".into(),
            guard: None,
            metadata: None,
        }],
        defaults: None,
        invariants: Vec::new(),
    }
}

macro_rules! invalid_test {
    ($name:ident, $builder:expr) => {
        #[test]
        fn $name() {
            let definition = $builder();
            assert_eq!(definition.validate(), Err(FsmError::InvalidInput));
        }
    };
}

invalid_test!(invalid_empty_states, || {
    let mut definition = base_definition();
    definition.states.clear();
    definition
});

invalid_test!(invalid_empty_transitions, || {
    let mut definition = base_definition();
    definition.transitions.clear();
    definition
});

invalid_test!(invalid_transition_empty_from, || {
    let mut definition = base_definition();
    definition.transitions[0].from = "".into();
    definition
});

invalid_test!(invalid_transition_empty_to, || {
    let mut definition = base_definition();
    definition.transitions[0].to = "".into();
    definition
});

invalid_test!(invalid_transition_empty_action, || {
    let mut definition = base_definition();
    definition.transitions[0].action = "".into();
    definition
});

invalid_test!(invalid_transition_whitespace_from, || {
    let mut definition = base_definition();
    definition.transitions[0].from = "  ".into();
    definition
});

invalid_test!(invalid_transition_whitespace_to, || {
    let mut definition = base_definition();
    definition.transitions[0].to = "  ".into();
    definition
});

invalid_test!(invalid_transition_whitespace_action, || {
    let mut definition = base_definition();
    definition.transitions[0].action = "  ".into();
    definition
});

invalid_test!(invalid_transition_unknown_from, || {
    let mut definition = base_definition();
    definition.transitions[0].from = "C".into();
    definition
});

invalid_test!(invalid_transition_unknown_to, || {
    let mut definition = base_definition();
    definition.transitions[0].to = "C".into();
    definition
});

invalid_test!(invalid_transition_unknown_both, || {
    let mut definition = base_definition();
    definition.transitions[0].from = "C".into();
    definition.transitions[0].to = "D".into();
    definition
});

invalid_test!(invalid_defaults_unknown_initial, || {
    let mut definition = base_definition();
    definition.defaults = Some(FsmDefaults {
        initial_state: Some("C".into()),
    });
    definition
});

invalid_test!(invalid_defaults_empty_initial, || {
    let mut definition = base_definition();
    definition.defaults = Some(FsmDefaults {
        initial_state: Some("".into()),
    });
    definition
});

invalid_test!(invalid_transition_invalid_in_multi_action, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "B".into(),
        to: "A".into(),
        action: "".into(),
        guard: None,
        metadata: None,
    });
    definition
});

invalid_test!(invalid_transition_invalid_in_multi_unknown_from, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "C".into(),
        to: "A".into(),
        action: "back".into(),
        guard: None,
        metadata: None,
    });
    definition
});

invalid_test!(invalid_invariant_unknown_kind, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "unknown_rule".into(),
        states: vec![],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_terminal_outbound_simple, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "B".into(),
        to: "A".into(),
        action: "back".into(),
        guard: None,
        metadata: None,
    });
    definition.invariants = vec![FsmInvariant {
        kind: "terminal_states".into(),
        states: vec!["B".into()],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_terminal_outbound_two_states, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "A".into(),
        to: "B".into(),
        action: "again".into(),
        guard: None,
        metadata: None,
    });
    definition.invariants = vec![FsmInvariant {
        kind: "terminal_states".into(),
        states: vec!["A".into(), "B".into()],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_required_transition_missing, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "required_transitions".into(),
        states: vec![],
        transitions: vec![FsmTransitionRef {
            from: "B".into(),
            to: "A".into(),
        }],
        description: None,
    }];
    definition
});

invalid_test!(invalid_required_transition_missing_second, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "required_transitions".into(),
        states: vec![],
        transitions: vec![
            FsmTransitionRef {
                from: "A".into(),
                to: "B".into(),
            },
            FsmTransitionRef {
                from: "B".into(),
                to: "A".into(),
            },
        ],
        description: None,
    }];
    definition
});

invalid_test!(invalid_forbidden_transition_present, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "forbidden_transitions".into(),
        states: vec![],
        transitions: vec![FsmTransitionRef {
            from: "A".into(),
            to: "B".into(),
        }],
        description: None,
    }];
    definition
});

invalid_test!(invalid_forbidden_transition_present_second, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "forbidden_transitions".into(),
        states: vec![],
        transitions: vec![
            FsmTransitionRef {
                from: "C".into(),
                to: "A".into(),
            },
            FsmTransitionRef {
                from: "A".into(),
                to: "B".into(),
            },
        ],
        description: None,
    }];
    definition
});

invalid_test!(invalid_forbidden_cycle_simple, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "B".into(),
        to: "A".into(),
        action: "back".into(),
        guard: None,
        metadata: None,
    });
    definition.invariants = vec![FsmInvariant {
        kind: "forbidden_cycles".into(),
        states: vec!["A".into()],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_forbidden_cycle_long, || {
    let mut definition = base_definition();
    definition.states.push("C".into());
    definition.transitions.push(FsmTransition {
        from: "B".into(),
        to: "C".into(),
        action: "next".into(),
        guard: None,
        metadata: None,
    });
    definition.transitions.push(FsmTransition {
        from: "C".into(),
        to: "A".into(),
        action: "loop".into(),
        guard: None,
        metadata: None,
    });
    definition.invariants = vec![FsmInvariant {
        kind: "forbidden_cycles".into(),
        states: vec!["A".into(), "B".into(), "C".into()],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_self_transition_required_missing_specified, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "self_transitions_required".into(),
        states: vec!["A".into()],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_self_transition_required_missing_all, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "self_transitions_required".into(),
        states: vec![],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(
    invalid_self_transition_required_missing_second_state,
    || {
        let mut definition = base_definition();
        definition.transitions.push(FsmTransition {
            from: "A".into(),
            to: "A".into(),
            action: "stay".into(),
            guard: None,
            metadata: None,
        });
        definition.invariants = vec![FsmInvariant {
            kind: "self_transitions_required".into(),
            states: vec!["A".into(), "B".into()],
            transitions: vec![],
            description: None,
        }];
        definition
    }
);

invalid_test!(invalid_forbidden_cycle_self_transition, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "A".into(),
        to: "A".into(),
        action: "stay".into(),
        guard: None,
        metadata: None,
    });
    definition.invariants = vec![FsmInvariant {
        kind: "forbidden_cycles".into(),
        states: vec!["A".into()],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_terminal_outbound_with_multiple, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "B".into(),
        to: "A".into(),
        action: "reset".into(),
        guard: None,
        metadata: None,
    });
    definition.transitions.push(FsmTransition {
        from: "A".into(),
        to: "A".into(),
        action: "loop".into(),
        guard: None,
        metadata: None,
    });
    definition.invariants = vec![FsmInvariant {
        kind: "terminal_states".into(),
        states: vec!["B".into()],
        transitions: vec![],
        description: None,
    }];
    definition
});

invalid_test!(invalid_required_transition_missing_swap, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "required_transitions".into(),
        states: vec![],
        transitions: vec![FsmTransitionRef {
            from: "B".into(),
            to: "A".into(),
        }],
        description: None,
    }];
    definition
});

invalid_test!(invalid_transition_unknown_in_multi_to, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "A".into(),
        to: "C".into(),
        action: "go".into(),
        guard: None,
        metadata: None,
    });
    definition
});

invalid_test!(invalid_transition_unknown_in_multi_from, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "C".into(),
        to: "B".into(),
        action: "go".into(),
        guard: None,
        metadata: None,
    });
    definition
});

invalid_test!(invalid_transition_unknown_in_multi_both, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "C".into(),
        to: "D".into(),
        action: "go".into(),
        guard: None,
        metadata: None,
    });
    definition
});

invalid_test!(invalid_transition_empty_in_multi, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "".into(),
        to: "B".into(),
        action: "go".into(),
        guard: None,
        metadata: None,
    });
    definition
});

invalid_test!(invalid_transition_whitespace_in_multi, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "A".into(),
        to: " ".into(),
        action: "go".into(),
        guard: None,
        metadata: None,
    });
    definition
});

invalid_test!(invalid_required_transition_missing_two, || {
    let mut definition = base_definition();
    definition.invariants = vec![FsmInvariant {
        kind: "required_transitions".into(),
        states: vec![],
        transitions: vec![
            FsmTransitionRef {
                from: "A".into(),
                to: "B".into(),
            },
            FsmTransitionRef {
                from: "B".into(),
                to: "B".into(),
            },
            FsmTransitionRef {
                from: "A".into(),
                to: "A".into(),
            },
        ],
        description: None,
    }];
    definition
});

invalid_test!(invalid_forbidden_transition_present_with_two, || {
    let mut definition = base_definition();
    definition.transitions.push(FsmTransition {
        from: "B".into(),
        to: "A".into(),
        action: "back".into(),
        guard: None,
        metadata: None,
    });
    definition.invariants = vec![FsmInvariant {
        kind: "forbidden_transitions".into(),
        states: vec![],
        transitions: vec![
            FsmTransitionRef {
                from: "A".into(),
                to: "B".into(),
            },
            FsmTransitionRef {
                from: "B".into(),
                to: "A".into(),
            },
        ],
        description: None,
    }];
    definition
});
