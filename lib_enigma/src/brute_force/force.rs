use itertools::Itertools;
use rayon::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    letter::Letter,
    machine::{MachineState, ReflectorId},
    message::Message,
    EnigmaMachine, RotorId,
};

use super::{combinations::iter_possible_states, plug_options::PlugboardOptions};

/// Brute-force by trying all combinations until a match is found
pub fn force_combinations(
    plug_options: PlugboardOptions,
    rotors: Option<Vec<(Option<RotorId>, Option<Letter>)>>,
    reflector: Option<ReflectorId>,
    input: &Message,
    starting_string: &Option<Message>,
    ending_string: &Option<Message>,
    contained_string: &Option<Message>,
) -> Vec<MachineState> {
    // Change the reflector ID into a vec
    let reflector = if let Some(id) = reflector {
        vec![id]
    } else {
        ReflectorId::iter().collect_vec()
    };

    // If no rotors are specified, give an empty vec
    let rotors = rotors.unwrap_or(vec![]);

    // Split rotors from their starting positions
    let (rotor_ids, rotor_positions): (Vec<_>, Vec<_>) = rotors.into_iter().unzip();

    // Convert rotor IDs, such that they represent all combinations
    let rotor_ids = rotor_ids
        .into_iter()
        .map(|id| {
            if let Some(id) = id {
                // If it's a known value, just give that
                vec![id]
            } else {
                // Otherwise, give all possible values
                RotorId::iter().collect_vec()
            }
        })
        .collect_vec();

    // Convert rotor starts such that they also represent all combinations
    let rotor_positions = rotor_positions
        .into_iter()
        .map(|start| {
            if let Some(pos) = start {
                vec![pos]
            } else {
                Letter::iter().collect_vec()
            }
        })
        .collect_vec();

    // Generate all possible plug board wires
    let plugs: Vec<_> = Letter::iter()
        .combinations(2)
        .map(|combo| (combo[0], combo[1]))
        .collect();

    // Generate iterator over all possible plug combinations
    // Since the kind of iterator changes depending on the plug board, we need
    // to Box it or Rust can't determine the size. The simpler solution would
    // be to collect it to a Vec, but given that there are over 150 trillion
    // combinations, there is a ✨ slight possibility ✨ of using an obscene
    // amount of memory if we try to allocate them all at once.
    // Perhaps we could consider this possibility if we were using a machine
    // at least 12 petabytes of RAM, though
    let plug_combinations = match plug_options {
        PlugboardOptions::KnownConnections(connections) => Box::new([connections].into_iter())
            as Box<(dyn Iterator<Item = Vec<(Letter, Letter)>> + Send + 'static)>,
        PlugboardOptions::NumberInRange(range) => Box::new(
            range.flat_map(move |plug_count| plugs.clone().into_iter().combinations(plug_count)),
        ),
        PlugboardOptions::NumberInRangeInclusive(range) => Box::new(
            range.flat_map(move |plug_count| plugs.clone().into_iter().combinations(plug_count)),
        ),
    };

    // Iterate over all possible states and filter to states that match the
    // criteria
    iter_possible_states(plug_combinations, &reflector, &rotor_ids, &rotor_positions)
        .par_bridge()
        .filter(|state| {
            // Create a machine with the current state
            let mut machine = EnigmaMachine::from(state.clone());

            // If it matches
            check_machine(
                &mut machine,
                input,
                starting_string,
                ending_string,
                contained_string,
            )
        })
        .collect()
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
            if machine.try_consume(&input[i..(i + contained.len())], contained) {
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
    use crate::{
        EnigmaMachine, Letter, MachineState, Message, PlugboardOptions, ReflectorId, RotorId,
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
                (Some(RotorId::I), None),
                (Some(RotorId::II), None),
                (Some(RotorId::III), None),
            ]),
            Some(ReflectorId::C),
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
                (None, Some(Letter::A)),
                (None, Some(Letter::B)),
                (None, Some(Letter::C)),
            ]),
            Some(ReflectorId::C),
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
                (Some(RotorId::I), Some(Letter::A)),
                (Some(RotorId::II), Some(Letter::B)),
                (Some(RotorId::III), Some(Letter::C)),
            ]),
            None,
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
            vec![(Letter::A, Letter::B), (Letter::C, Letter::D)],
            vec![RotorId::I, RotorId::II, RotorId::III],
            vec![Letter::A, Letter::B, Letter::C],
            ReflectorId::C,
        );
        let mut machine = EnigmaMachine::from(state.clone());

        let encoded = machine.consume(&Message::from("Hello world".to_string()));

        let results = force_combinations(
            PlugboardOptions::NumberInRangeInclusive(2..=2),
            Some(vec![
                (Some(RotorId::I), Some(Letter::A)),
                (Some(RotorId::II), Some(Letter::B)),
                (Some(RotorId::III), Some(Letter::C)),
            ]),
            Some(ReflectorId::C),
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
                (Some(RotorId::I), None),
                (Some(RotorId::II), None),
                (Some(RotorId::III), None),
            ]),
            Some(ReflectorId::C),
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
                (Some(RotorId::I), Some(Letter::A)),
                (Some(RotorId::II), Some(Letter::B)),
                (Some(RotorId::III), Some(Letter::C)),
            ]),
            Some(ReflectorId::C),
            &encoded,
            &None,
            &None,
            &Some(Message::from("wor".to_string())),
        );

        assert_eq!(results, vec![state]);
    }
}
