//! Integration tests for SigmaOS-word-processor.

use sigmaos_word_processor_lib::{Component, ComponentLifecycle};

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
