use crate::char_mapping::CharMapping;
use crate::consts::NUM_LETTERS;
use crate::util::letter_to_index;

#[derive(Debug, Clone)]
pub struct Rotor {
    /// Name of the rotor
    pub name: String,

    /// Array of mappings between chars
    ///
    /// Each index maps to another index
    char_map: CharMapping,

    /// Array of reverse mappings to speed up reverse lookup
    reverse_char_map: CharMapping,

    /// Current position of the rotor
    pos: usize,

    /// Positions at which turning over this rotor turns over
    /// the next rotor
    turnover_pos: Vec<usize>,

    /// Whether this rotor is capable of performing a double-stepping operation
    can_double_step: bool
}

impl Rotor {
    pub fn new(
        name: String,
        mappings: [(char, char); NUM_LETTERS],
        turnover_pos: Vec<char>,
        can_double_step: bool,
        pos: usize,
    ) -> Rotor {
        let turnover_pos = turnover_pos
            .into_iter()
            .map(|c| letter_to_index(c).0)
            .collect();

        // Build the mappings
        let char_map = CharMapping::from(mappings);
        let reverse_char_map = CharMapping::from_reverse_of(&char_map);

        Rotor {
            name,
            char_map,
            reverse_char_map,
            turnover_pos,
            can_double_step,
            pos,
        }
    }

    /// Convert a character sending it forwards through the system
    pub fn char_in(&self, c: usize) -> usize {
        let idx = (c + self.pos) % NUM_LETTERS;
        (self.char_map[idx] + NUM_LETTERS - self.pos) % NUM_LETTERS
    }

    pub fn char_out(&self, c: usize) -> usize {
        let idx = (c + self.pos) % NUM_LETTERS;
        (self.reverse_char_map[idx] + NUM_LETTERS - self.pos) % NUM_LETTERS
    }

    /// Turn over this rotor
    ///
    /// Returns whether the next one should be a regular step (`true`), which
    /// happens if this rotor reached its turnover position or a potential
    /// double step (`false`), which happens otherwise
    pub fn step(&mut self) -> bool {
        self.pos = (self.pos + 1) % NUM_LETTERS;
        self.turnover_pos.contains(&self.pos)
    }

    /// Perform a potential double step.
    ///
    /// The rotor is stepped, only if it is configured to be a double-stepping
    /// rotor (ie not the first or last), and if it is currently at a turnover
    /// position
    ///
    /// Returns whether the next one should be a regular step (`true`), which
    /// happens if this rotor stepped, or a double step (`false`), which
    /// happens otherwise
    pub fn double_step(&mut self) -> bool {
        if self.can_double_step && self.turnover_pos.contains(&(self.pos + 1)) {
            self.pos = (self.pos + 1) % NUM_LETTERS;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{consts::NUM_LETTERS, data::{get_rotor_config, RotorId}};

    use super::Rotor;

    #[test]
    fn inputs_are_symmetric() {
        let r = Rotor::new(
            "I".to_owned(),
            get_rotor_config(RotorId::I).1,
            get_rotor_config(RotorId::I).0,
            false,
            1,
        );

        for i in 0..NUM_LETTERS {
            let encoded = r.char_in(i);
            let decoded = r.char_out(encoded);
            assert_eq!(decoded, i);
        }
    }
}
