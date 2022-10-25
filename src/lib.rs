pub mod gene;
pub mod generic_search;
pub mod maze;

/// # Timed
/// Time the execution of a given expression using `std::time::Instant` as a stopwatch.
#[macro_export]
macro_rules! timed {
    ($e:expr) => {
        let now = std::time::Instant::now();
        println!("Started stopwatch.");

        $e;

        let elapsed = now.elapsed().as_millis();
        println!("Elapsed: {} ms.", elapsed);
    };

    ($b:block) => {
        let now = std::time::Instant::now();
        println!("Started stopwatch.");

        $b;

        let elapsed = now.elapsed().as_millis();
        println!("Elapsed: {} ms.", elapsed);
    };

    ($s:stmt) => {
        let now = std::time::Instant::now();
        println!("Started stopwatch.");

        $s;

        let elapsed = now.elapsed().as_millis();
        println!("Elapsed: {} ms.", elapsed);
    };
}
