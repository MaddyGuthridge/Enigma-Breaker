use itertools::Itertools;

use crate::{RotorId, letter::Letter, machine::ReflectorId, EnigmaMachine};

use super::unknown::Unknown;


/// Brute-force by trying all combinations until a match is found
pub fn force_combinations(
    rotors: Option<Vec<(Unknown<RotorId>, Unknown<Letter>)>>,
    reflector: Unknown<ReflectorId>,
) {
    let mut matches: Vec<bool> = Vec::default();

    // Assume 3 unknown rotors if no rotors are specified
    let rotors = rotors.unwrap_or(vec![
        (Unknown::Unknown, Unknown::Unknown),
        (Unknown::Unknown, Unknown::Unknown),
        (Unknown::Unknown, Unknown::Unknown),
    ]);

    // Split rotors from their starting positions
    let (rotor_ids, rotor_positions): (Vec<_>, Vec<_>) = rotors.into_iter().unzip();

    // For all possible reflectors
    for reflect in reflector {
        for rotors in rotor_ids.iter().multi_cartesian_product() {
            for positions in rotor_positions.iter().multi_cartesian_product() {
                let machine = EnigmaMachine::new(
                    &vec![],
                    &rotors,
                    &positions,
                    reflect,
                );
            }
        }
    }
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
    input: &String,
    starting_string: &Option<String>,
    ending_string: &Option<String>,
    contained_string: &Option<String>,
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
