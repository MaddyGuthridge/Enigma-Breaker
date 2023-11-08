use criterion::{criterion_group, criterion_main, Criterion};
use lib_enigma::{EnigmaMachine, Letter, MachineState, ReflectorId, RotorId, Message, force_combinations, PlugboardOptions, Unknown};

pub fn bench_brute_force(c: &mut Criterion) {
    c.bench_function("brute force unknown rotors", |b| {
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
                    (Unknown::Known(RotorId::I), Unknown::Known(Letter::A)),
                    (Unknown::Unknown, Unknown::Unknown),
                    (Unknown::Unknown, Unknown::Unknown),
                ]),
                Unknown::Known(ReflectorId::C),
                &encoded,
                &Some(Message::from("Hello".to_string())),
                &None,
                &None,
            );
        });
    });
}

criterion_group!(benches, bench_brute_force);
criterion_main!(benches);
