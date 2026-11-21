//! SigmaOS-word-processor — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_word_processor_lib::{Component, ComponentLifecycle};

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-word-processor: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-word-processor: ok");
}
