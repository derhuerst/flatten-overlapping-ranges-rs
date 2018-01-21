#[macro_use]
extern crate bencher;
extern crate flatten_overlapping_ranges;

use bencher::Bencher;
use flatten_overlapping_ranges::flatten;

fn simple(b: &mut Bencher) {
    let simple: Vec<(&char, usize, usize)> = vec![
        (&'a', 0, 7),
        (&'b', 2, 12),
        (&'c', 5, 5),
        (&'d', 12, 7)
    ];

    b.iter(|| flatten(&simple));
}

benchmark_group!(benches, simple);
benchmark_main!(benches);