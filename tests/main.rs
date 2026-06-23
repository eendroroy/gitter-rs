mod provision;

use crate::provision::run_provision;
use ctor::ctor;
use std::sync::Once;

static GLOBAL_INIT: Once = Once::new();

#[ctor(unsafe)]
fn global_test_setup() {
    GLOBAL_INIT.call_once(|| {
        println!("----- Creating git repositories for tests -----");
        run_provision();
    });
}

mod integration {
    mod exec_tests;
    mod filter_tests;
    mod gitter_tests;
    mod help_tests;
    mod list_tests;
}
