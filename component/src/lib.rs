use bindings::*;
use std::time::*;
use std::thread::sleep;

struct TestCase;

impl TestComponent for TestCase {
    fn run() -> f64 {
        let instant1 = Instant::now();
        sleep(Duration::from_secs(2));
        let elapsed = instant1.elapsed().as_secs_f64();
        elapsed
    }
}

bindings::export!(TestCase);
