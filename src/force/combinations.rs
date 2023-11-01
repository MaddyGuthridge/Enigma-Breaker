use crate::{RotorId, letter::Letter, machine::ReflectorId};

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

    
}
