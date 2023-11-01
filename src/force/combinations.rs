use crate::{RotorId, letter::Letter};
use strum::IntoEnumIterator;

use super::unknown::Unknown;


/// Brute-force by trying all combinations until a match is found
pub fn force_combinations(
    rotors: Option<Vec<(Unknown<RotorId>, Unknown<Letter>)>>,
) {}
