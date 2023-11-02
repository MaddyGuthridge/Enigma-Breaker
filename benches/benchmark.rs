use enigma::{EnigmaMachine, ReflectorId, RotorId, Letter};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.bench_function("encode 1000", |b| {
        let mut machine = EnigmaMachine::new(
            &vec![],
            &[RotorId::I, RotorId::II, RotorId::III],
            &[Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );

        let input = "A".repeat(1000);

        b.iter(|| {
            machine.consume(&input);
        });
    });

    c.bench_function("step all rotor combinations", |b| {
        let mut machine = EnigmaMachine::new(
            &vec![],
            &[RotorId::I, RotorId::II, RotorId::III],
            &[Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );

        b.iter(|| {
            // The rotors don't repeat for about 17000 steps
            // https://crypto.stackexchange.com/a/71395/112016
            for _ in 0..17000 {
                machine.step();
            }
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
