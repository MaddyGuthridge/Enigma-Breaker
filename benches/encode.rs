use enigma::{EnigmaMachine, ReflectorId, RotorId};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_encode(c: &mut Criterion) {
    c.bench_function("encode 1000", |b| {
        let mut machine = EnigmaMachine::new(
            &vec![],
            &[(RotorId::I, 'a'), (RotorId::II, 'b'), (RotorId::III, 'c')],
            ReflectorId::C,
        );

        let input = "A".repeat(1000);

        b.iter(|| {
            machine.consume(&input);
        });
    });
}

criterion_group!(benches, benchmark_encode);
criterion_main!(benches);
