use itertools::Itertools;

use crate::{RotorId, letter::Letter, machine::ReflectorId, EnigmaMachine};

use super::unknown::Unknown;


/// Brute-force by trying all combinations until a match is found
pub fn force_combinations(
    rotors: Option<Vec<(Unknown<RotorId>, Unknown<Letter>)>>,
    reflector: Unknown<ReflectorId>,
) {
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
