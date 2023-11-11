use criterion::Criterion;
use lib_enigma::{EnigmaMachine, Letter, MachineState, Message, ReflectorId, RotorId};

pub fn bench_encipher(c: &mut Criterion) {
    c.bench_function("encode 1000", |b| {
        let mut machine = EnigmaMachine::from(MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        ));

        let input = Message::from("A".repeat(1000));

        b.iter(|| {
            machine.consume(&input);
        });
    });

    c.bench_function("step all rotor combinations", |b| {
        let mut machine = EnigmaMachine::from(MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        ));

        b.iter(|| {
            // The rotors don't repeat for about 17000 steps
            // https://crypto.stackexchange.com/a/71395/112016
            for _ in 0..17000 {
                machine.step();
            }
        });
    });
}
