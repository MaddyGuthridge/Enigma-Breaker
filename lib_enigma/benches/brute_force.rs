use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use lib_enigma::{
    force_combinations, EnigmaMachine, Letter, MachineState, Message, PlugboardOptions,
    ReflectorId, RotorId,
};

pub fn bench_brute_force(c: &mut Criterion) {
    let mut group = c.benchmark_group("brute force");

    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10);

    group.bench_function("unknown rotors", |b| {
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
                Some(vec![(None, None), (None, None), (None, None)]),
                Some(ReflectorId::C),
                &encoded,
                &Some(Message::from("Hello".to_string())),
                &None,
                &None,
            );
        });
    });

    group.bench_function("unknown plug board", |b| {
        // Encode the message
        let mut machine = EnigmaMachine::from(MachineState::new(
            vec![
                (Letter::F, Letter::U),
                (Letter::T, Letter::A),
                (Letter::Y, Letter::K),
            ],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        ));

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        b.iter(|| {
            force_combinations(
                PlugboardOptions::NumberInRangeInclusive(3..=3),
                Some(vec![
                    (Some(RotorId::I), Some(Letter::A)),
                    (Some(RotorId::II), Some(Letter::B)),
                    (Some(RotorId::III), Some(Letter::C)),
                ]),
                Some(ReflectorId::C),
                &encoded,
                &Some(Message::from("Hello world".to_string())),
                &None,
                &None,
            );
        });
    });

    group.finish();
}

criterion_group!(benches, bench_brute_force);
criterion_main!(benches);
