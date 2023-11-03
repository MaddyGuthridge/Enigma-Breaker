use itertools::Itertools;
use num::iter::Range;
use strum::IntoEnumIterator;

use crate::{
    letter::Letter,
    machine::{MachineState, ReflectorId},
    message::Message,
    EnigmaMachine, RotorId,
};

use super::unknown::Unknown;

/// Brute-force by trying all combinations until a match is found
pub fn force_combinations(
    num_plugs: Range<usize>,
    rotors: Option<Vec<(Unknown<RotorId>, Unknown<Letter>)>>,
    reflector: Unknown<ReflectorId>,
    input: &Message,
    starting_string: &Option<Message>,
    ending_string: &Option<Message>,
    contained_string: &Option<Message>,
) -> Vec<MachineState> {
    let mut matches: Vec<MachineState> = Vec::default();

    // Assume 3 unknown rotors if no rotors are specified
    let rotors = rotors.unwrap_or(vec![
        (Unknown::Unknown, Unknown::Unknown),
        (Unknown::Unknown, Unknown::Unknown),
        (Unknown::Unknown, Unknown::Unknown),
    ]);

    // Split rotors from their starting positions
    let (rotor_ids, rotor_positions): (Vec<_>, Vec<_>) = rotors.into_iter().unzip();

    // Generate all possible plug board wires
    let plugs: Vec<_> = Letter::iter()
        .combinations(2)
        .map(|combo| (combo[0], combo[1]))
        .collect();

    // For each potential number of plugs
    for plug_count in num_plugs {
        // For every possible combination of plugs with that count
        for plugs in plugs
            .iter()
            .combinations(plug_count)
            .map(|v| v.into_iter().cloned().collect_vec())
        {
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
        machine.jump_forwards(input.len() - end.len());
        if !machine.try_consume(input, end) {
            return false;
        }
        machine.reset();
    }

    if let Some(contained) = contained_string {
        let mut found_match = false;
        for i in 0..(input.len() - contained.len()) {
            machine.jump_forwards(i);
            if machine.try_consume(input, contained) {
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
