use std::fs;

use fsm_governance_engine_lib::FsmDefinition;

fn validate_definition(path: &str) {
    let raw = fs::read_to_string(path).expect("read definition");
    let definition: FsmDefinition = serde_json::from_str(&raw).expect("parse definition");
    definition.validate().expect("validate definition");
}

#[test]
fn validate_example_definitions() {
    let definitions = [
        "docs/example_fsm_definition.json",
        "examples/definitions/governance_lifecycle.json",
        "examples/definitions/voting.json",
        "examples/definitions/treasury_flow.json",
        "examples/definitions/audit_only.json",
    ];

    for path in definitions {
        validate_definition(path);
    }
}
