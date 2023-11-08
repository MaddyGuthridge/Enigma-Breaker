use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use lib_enigma::{EnigmaMachine, Letter, MachineState, ReflectorId, RotorId, Message, force_combinations, PlugboardOptions};

pub fn bench_brute_force(c: &mut Criterion) {

    let mut group = c.benchmark_group("brute force");

    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10);

    group.bench_function("brute force unknown rotors", |b| {
        // Encode the message
        let mut machine = EnigmaMachine::from(MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        ));

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        b.iter(|| {
            force_combinations(
                PlugboardOptions::KnownConnections(vec![]),
                Some(vec![
                    (None, None),
                    (None, None),
                    (None, None),
                ]),
                Some(ReflectorId::C),
                &encoded,
                &Some(Message::from("Hello".to_string())),
                &None,
                &None,
            );
        });
    });

    group.finish();
}

criterion_group!(benches, bench_brute_force);
criterion_main!(benches);
