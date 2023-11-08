use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    letter::Letter,
    machine::{MachineState, ReflectorId, PlugBoard},
    message::Message,
    EnigmaMachine, RotorId,
};

use super::{plug_options::PlugboardOptions, unknown::Unknown};

/// Brute-force by trying all combinations until a match is found
pub fn force_combinations(
    plug_options: PlugboardOptions,
    rotors: Option<Vec<(Unknown<RotorId>, Unknown<Letter>)>>,
    reflector: Unknown<ReflectorId>,
    input: &Message,
    starting_string: &Option<Message>,
    ending_string: &Option<Message>,
    contained_string: &Option<Message>,
) -> Vec<MachineState> {
    let mut matches: Vec<MachineState> = Vec::default();

    // If no rotors are specified, give an empty vec
    let rotors = rotors.unwrap_or(vec![]);

    // Split rotors from their starting positions
    let (rotor_ids, rotor_positions): (Vec<_>, Vec<_>) = rotors.into_iter().unzip();

    // Generate all possible plug board wires
    let plugs: Vec<_> = Letter::iter()
        .combinations(2)
        .map(|combo| (combo[0], combo[1]))
        .collect();

    // Iterator over all possible plug combinations
    // Since the kind of iterator changes depending on the plug board, we need
    // to Box it or Rust can't determine the size. The simpler solution would
    // be to collect it to a Vec, but given that there are over 150 trillion
    // combinations, there is a ✨ slight possibility ✨ of using an obscene
    // amount of memory if we try to allocate them all at once.
    // Perhaps we could consider this possibility if we were using a machine
    // at least 12 petabytes of RAM, though
    let plug_combinations = match plug_options {
        PlugboardOptions::KnownConnections(connections) => {
            Box::new([connections].into_iter()) as Box<dyn Iterator<Item = Vec<(Letter, Letter)>>>
        }
        PlugboardOptions::NumberInRange(range) => Box::new(range.flat_map(|plug_count| {
            plugs
                .iter()
                .combinations(plug_count)
                .map(|v| v.into_iter().cloned().collect_vec())
        })),
        PlugboardOptions::NumberInRangeInclusive(range) => Box::new(range.flat_map(|plug_count| {
            plugs
                .iter()
                .combinations(plug_count)
                .map(|v| v.into_iter().cloned().collect_vec())
        })),
    };

    // For every combination of plugs
    for plugs in plug_combinations {

        // If this plug combination is illegal
        if PlugBoard::new(&plugs).is_none() {
            continue;
        }

        // For all possible reflectors
        for reflect in &reflector {
            // For all possible rotor IDs
            for rotors in rotor_ids.iter().multi_cartesian_product() {
                // For all possible positions for each rotor
                for positions in rotor_positions.iter().multi_cartesian_product() {
                    // Create a machine with the current state
                    let mut machine = EnigmaMachine::from(MachineState::new(
                        plugs.clone(),
                        rotors.clone(),
                        positions,
                        reflect,
                    ));

                    // If it matches
                    if check_machine(
                        &mut machine,
                        input,
                        starting_string,
                        ending_string,
                        contained_string,
                    ) {
                        matches.push(machine.get_starting_state());
                    }
                }
            }
        }
    }

    matches
}

/// Check to see if the given machine matches the criteria for the decoded
/// values
///
/// * `machine`: the enigma machine to check against
/// * `input`: the input string to check
/// * `starting_string`: string expected to be at the start of the decoded
/// input
/// * `ending_string`: string expected to be at the end of the decoded input
/// * `contained_string`: string expected to be contained somewhere in the
/// decoded input. All possible positions are checked.
#[inline]
fn check_machine(
    machine: &mut EnigmaMachine,
    input: &Message,
    starting_string: &Option<Message>,
    ending_string: &Option<Message>,
    contained_string: &Option<Message>,
) -> bool {
    if let Some(start) = starting_string {
        if !machine.try_consume(input, start) {
            return false;
        }
        machine.reset();
    }

    if let Some(end) = ending_string {
        machine.jump_forwards(&input[..(input.len() - end.len())]);
        if !machine.try_consume(&input[(input.len() - end.len())..], end) {
            return false;
        }
        machine.reset();
    }

    if let Some(contained) = contained_string {
        let mut found_match = false;
        for i in 0..(input.len() - contained.len()) {
            machine.jump_forwards(&input[..i]);
            if machine.try_consume(&input[i..(i+contained.len())], contained) {
                found_match = true;
                break;
            }
            machine.reset();
        }
        if !found_match {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::force_combinations;
    use crate::brute_force::Unknown;
    use crate::{
        EnigmaMachine, Letter, MachineState, Message, PlugboardOptions,
        ReflectorId, RotorId,
    };

    #[test]
    fn unknown_rotor_starts() {
        // Encode the message
        let state = MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );
        let mut machine = EnigmaMachine::from(state.clone());

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        let results = force_combinations(
            PlugboardOptions::KnownConnections(vec![]),
            Some(vec![
                (Unknown::Known(RotorId::I), Unknown::Unknown),
                (Unknown::Known(RotorId::II), Unknown::Unknown),
                (Unknown::Known(RotorId::III), Unknown::Unknown),
            ]),
            Unknown::Known(ReflectorId::C),
            &encoded,
            &Some(Message::from("Hello".to_string())),
            &None,
            &None,
        );

        assert_eq!(results, vec![state]);
    }

    #[test]
    fn unknown_rotor_ids() {
        // Encode the message
        let state = MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );
        let mut machine = EnigmaMachine::from(state.clone());

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        let results = force_combinations(
            PlugboardOptions::KnownConnections(vec![]),
            Some(vec![
                (Unknown::Unknown, Unknown::Known(Letter::A)),
                (Unknown::Unknown, Unknown::Known(Letter::B)),
                (Unknown::Unknown, Unknown::Known(Letter::C)),
            ]),
            Unknown::Known(ReflectorId::C),
            &encoded,
            &Some(Message::from("Hello".to_string())),
            &None,
            &None,
        );

        assert_eq!(results, vec![state]);
    }

    #[test]
    fn unknown_reflector() {
        // Encode the message
        let state = MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );
        let mut machine = EnigmaMachine::from(state.clone());

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        let results = force_combinations(
            PlugboardOptions::KnownConnections(vec![]),
            Some(vec![
                (Unknown::Known(RotorId::I), Unknown::Known(Letter::A)),
                (Unknown::Known(RotorId::II), Unknown::Known(Letter::B)),
                (Unknown::Known(RotorId::III), Unknown::Known(Letter::C)),
            ]),
            Unknown::Unknown,
            &encoded,
            &Some(Message::from("Hello".to_string())),
            &None,
            &None,
        );

        assert_eq!(results, vec![state]);
    }

    #[test]
    fn unknown_plugs() {
        // Encode the message
        let state = MachineState::new(
            vec![
                (Letter::A, Letter::B),
                (Letter::C, Letter::D),
            ],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );
        let mut machine = EnigmaMachine::from(state.clone());

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        let results = force_combinations(
            PlugboardOptions::NumberInRangeInclusive(2..=2),
            Some(vec![
                (Unknown::Known(RotorId::I), Unknown::Known(Letter::A)),
                (Unknown::Known(RotorId::II), Unknown::Known(Letter::B)),
                (Unknown::Known(RotorId::III), Unknown::Known(Letter::C)),
            ]),
            Unknown::Known(ReflectorId::C),
            &encoded,
            &Some(Message::from("Hello".to_string())),
            &None,
            &None,
        );

        // There are many plug board combinations that work, since we don't use
        // all chars in the encoded message, meaning unused chars could be
        // wired in any way, and it wouldn't affect the encoding
        assert!(results.contains(&state));
    }

    #[test]
    fn message_end() {
        // Encode the message
        let state = MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );
        let mut machine = EnigmaMachine::from(state.clone());

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        let results = force_combinations(
            PlugboardOptions::KnownConnections(vec![]),
            Some(vec![
                (Unknown::Known(RotorId::I), Unknown::Unknown),
                (Unknown::Known(RotorId::II), Unknown::Unknown),
                (Unknown::Known(RotorId::III), Unknown::Unknown),
            ]),
            Unknown::Known(ReflectorId::C),
            &encoded,
            &None,
            &Some(Message::from("world".to_string())),
            &None,
        );

        assert_eq!(results, vec![state]);
    }

    #[test]
    fn message_contains() {
        // Encode the message
        let state = MachineState::new(
            vec![],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );
        let mut machine = EnigmaMachine::from(state.clone());

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        let results = force_combinations(
            // Specify all options to simplify debugging
            PlugboardOptions::KnownConnections(vec![]),
            Some(vec![
                (Unknown::Known(RotorId::I), Unknown::Known(Letter::A)),
                (Unknown::Known(RotorId::II), Unknown::Known(Letter::B)),
                (Unknown::Known(RotorId::III), Unknown::Known(Letter::C)),
            ]),
            Unknown::Known(ReflectorId::C),
            &encoded,
            &None,
            &None,
            &Some(Message::from("wor".to_string())),
        );

        assert_eq!(results, vec![state]);
    }
}
