use crate::consts::NUM_LETTERS;
use crate::char_mapping::CharMapping;
use crate::util::letter_to_index;

#[derive(Debug)]
pub struct Rotor {
    /// Array of mappings between chars
    ///
    /// Each index maps to another index
    char_map: CharMapping,
    reverse_char_map: CharMapping,
    pos: usize,

    /// Positions at which turning over this rotor turns over
    /// the next rotor
    turnover_pos: Vec<usize>
}

impl Rotor {
    pub fn new(
        mappings: [(char, char); NUM_LETTERS],
        turnover_pos: Vec<char>,
        pos: usize,
    ) -> Rotor {
        let turnover_pos = turnover_pos
            .into_iter()
            .map(|c| letter_to_index(c).0)
            .collect();

        // Build the mappings
        let char_map = CharMapping::from(mappings);
        let reverse_char_map = CharMapping::from_reverse_of(&char_map);

        Rotor { char_map, reverse_char_map, turnover_pos, pos }
    }

    /// Convert a character sending it forwards through the system
    pub fn char_in(&self, c: usize) -> usize {
        self.char_map[(c + self.pos) % NUM_LETTERS]
    }

    pub fn char_out(&self, c: usize) -> usize {
        self.reverse_char_map[(c + self.pos) % NUM_LETTERS]
    }

    /// Turn over this rotor
    pub fn tick(&mut self) -> bool {
        self.pos = (self.pos + 1) % NUM_LETTERS;
        self.turnover_pos.contains(&self.pos)
    }
}
