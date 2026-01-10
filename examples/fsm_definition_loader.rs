use std::fs;

use fsm_governance_engine_lib::FsmDefinition;

fn main() {
    let raw = fs::read_to_string("docs/example_fsm_definition.json")
        .expect("failed to read example definition");
    let definition: FsmDefinition = serde_json::from_str(&raw)
        .expect("failed to parse definition JSON");

    definition.validate().expect("definition failed validation");

    println!("Loaded {} states and {} transitions.", definition.states.len(), definition.transitions.len());
}
