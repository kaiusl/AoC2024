// don't delete this use, divan don't register benchmarks without it
use aoc2024;

fn main() {
    // Run registered benchmarks.
    #[cfg(feature = "divan")]
    divan::main();
}
