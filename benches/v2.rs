use criterion::{criterion_group, criterion_main, Criterion};
use arkanium_hash::v2::{arkanium_hash, ScratchPad};

const FIXED_INPUT: &[u8] = b"Hello World from arkanium hash v2!";

fn bench_zero_input(c: &mut Criterion) {
    let mut scratch_pad = ScratchPad::default();
    let input = [0u8; 112];
    c.bench_function("v2::zero_input", |b| b.iter(|| arkanium_hash(&input, &mut scratch_pad)));
}

fn bench_fixed_input(c: &mut Criterion) {
    let mut scratch_pad = ScratchPad::default();
    c.bench_function("v2::fixed_input", |b| b.iter(|| arkanium_hash(FIXED_INPUT, &mut scratch_pad)));
}

criterion_group!(benches, bench_zero_input, bench_fixed_input);
criterion_main!(benches);