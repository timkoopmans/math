use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use logarithm::decimal::FixedPoint;
use checked_decimal_macro::*;

criterion_group!(
    benches,
    bench_ln,
);
criterion_main!(benches);

fn bench_ln(c: &mut Criterion) {
    let mut group = c.benchmark_group("msb fixed point integer");

    for integer in [u64::MAX >> 1].iter() {
        let fixed_point = FixedPoint::new(*integer as u128);
        let parameter = "u64::MAX >> 1";

        group.bench_with_input(
            BenchmarkId::new("leading zeros", parameter),
            &fixed_point,
            |b, _s| {
                b.iter(|| fixed_point.msb());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bitwise", parameter),
            &fixed_point,
            |b, _s| {
                b.iter(|| fixed_point.msb_shift());
            },
        );
    }
    group.finish();
}
