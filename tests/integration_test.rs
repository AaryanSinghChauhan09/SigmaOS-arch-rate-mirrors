//! Integration tests for SigmaOS-arch-rate-mirrors.

use sigmaos_arch_rate_mirrors_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
