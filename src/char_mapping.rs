use std::ops::{Deref, DerefMut};

use crate::{consts::NUM_LETTERS, util::letter_to_index};

pub const INVALID_MAPPING: usize = usize::MAX;

#[derive(Debug)]
pub struct CharMapping([usize; NUM_LETTERS]);

impl Deref for CharMapping {
    type Target = [usize; NUM_LETTERS];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharMapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for CharMapping {
    fn default() -> Self {
        Self([INVALID_MAPPING; NUM_LETTERS])
    }
}

fn add_to_mapping(
    mapping: &mut [usize; NUM_LETTERS],
    from: char,
    to: char
) {
    let c_a = letter_to_index(from).0;
    let c_b = letter_to_index(to).0;
    if mapping[c_a] != INVALID_MAPPING {
        let existing = mapping[c_a];
        panic!(
            "Cannot map char {from:?} to {to:?}, {from:?} already maps to {existing:?}",
        )
    }
    mapping[c_a] = c_b;
    mapping[letter_to_index(from).0] = letter_to_index(to).0;
}

impl From<[(char, char); NUM_LETTERS]> for CharMapping {
    fn from(
        chars: [(char, char); NUM_LETTERS],
    ) -> CharMapping {
        let mut mapping = [INVALID_MAPPING; NUM_LETTERS];

        for (from, to) in chars {
            add_to_mapping(&mut mapping, from, to)
        }

        CharMapping(mapping)
    }
}

impl From<Vec<(char, char)>> for CharMapping {
    fn from(
        chars: Vec<(char, char)>,
    ) -> CharMapping {
        assert_eq!(chars.len(), NUM_LETTERS);

        let mut mapping = [INVALID_MAPPING; NUM_LETTERS];

        for (from, to) in chars {
            add_to_mapping(&mut mapping, from, to)
        }

        CharMapping(mapping)
    }
}

impl CharMapping {
    pub fn from_reverse_of(map: &CharMapping) -> CharMapping {
        let mut new = CharMapping::default();

        for (i, c) in map.iter().enumerate() {
            new[*c] = i;
        }

        new
    }
}
