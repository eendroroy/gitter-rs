use crate::provision::run_provision;
use ctor::ctor;
use std::sync::Once;

mod provision;

static GLOBAL_INIT: Once = Once::new();

#[ctor(unsafe)]
fn global_test_setup() {
    GLOBAL_INIT.call_once(|| {
        println!("----- Creating git repositories for tests -----");
        run_provision();
    });
}

mod integration {
    pub mod exec_tests;
    pub mod filter_tests;
    pub mod gitter_tests;
    pub mod help_tests;
    pub mod list_tests;
}
