use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use logarithm::fixed_point::FixedPoint;
use checked_decimal_macro::*;

criterion_group!(
    benches,
    bench_ln,
);
criterion_main!(benches);

fn bench_ln(c: &mut Criterion) {
    let mut group = c.benchmark_group("ln fixed point integer");

    for integer in [u64::MAX].iter() {
        let fixed_point = FixedPoint::new(*integer as u128);
        let parameter = "u64::MAX";

        group.bench_with_input(
            BenchmarkId::new("iterative approximation", parameter),
            &fixed_point,
            |b, _s| {
                b.iter(|| fixed_point.ln());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("table lookup", parameter),
            &fixed_point,
            |b, _s| {
                b.iter(|| fixed_point.ln_tables());
            },
        );
    }
    group.finish();
}
