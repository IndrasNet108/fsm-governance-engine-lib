#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(definition) = serde_json::from_slice::<fsm_governance_engine_lib::FsmDefinition>(data)
    {
        let _ = definition.validate();
    }
});
