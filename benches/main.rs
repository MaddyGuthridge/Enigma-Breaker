mod brute_force;
mod encipher;

use brute_force::bench_brute_force;
use criterion::{criterion_group, criterion_main};
use encipher::bench_encipher;


criterion_group!(benches, bench_brute_force, bench_encipher);
criterion_main!(benches);
