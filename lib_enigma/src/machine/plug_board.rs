use std::collections::BTreeMap;

use crate::letter::Letter;

#[derive(Debug)]
pub struct PlugBoard {
    // TODO: Use the CharMapping type for better performance
    char_map: BTreeMap<Letter, Letter>,
}

impl PlugBoard {
    pub fn new(mapping: &Vec<(Letter, Letter)>) -> Option<PlugBoard> {
        let expected_map_size = mapping.len() * 2;

        let built_map: BTreeMap<_, _> = mapping
            .clone()
            .into_iter()
            .map(|(a, b)| (b, a))
            .chain(mapping.clone())
            .collect();

        // Make sure there we got the right number of elements
        if built_map.len() != expected_map_size {
            return None;
        }

        Some(PlugBoard {
            char_map: built_map,
        })
    }

    pub fn map_char(&self, c: Letter) -> Letter {
        *self.char_map.get(&c).unwrap_or(&c)
    }
}
