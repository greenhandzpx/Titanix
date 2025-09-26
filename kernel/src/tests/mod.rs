use alloc::vec::Vec;

use crate::sync::mutex::SpinNoIrqLock;

mod concurrent;

type TestFn = fn();

static TESTS: SpinNoIrqLock<Option<Vec<(&'static str, TestFn)>>> = SpinNoIrqLock::new(None);

/// Register a test
#[macro_export]
macro_rules! register_test {
    ($name:expr, $fn:path) => {{
        fn wrapper() {
            $fn();
        }
        $crate::tests::add_test($name, wrapper);
    }};
}

fn add_test(name: &'static str, f: TestFn) {
    TESTS.lock().as_mut().unwrap().push((name, f));
}

fn run_all_tests() {
    let tests = TESTS.lock().as_ref().unwrap().clone();
    for (name, f) in tests {
        println!("========== Running test: {}... ==========", name);
        f();
        println!("========== Running test: {} Done ==========", name);
    }
}

pub fn init() {
    *TESTS.lock() = Some(Vec::new());

    concurrent::init();

    run_all_tests();
}
