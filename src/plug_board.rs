use std::collections::BTreeMap;

use crate::util::letter_to_index;

#[derive(Debug)]
pub struct PlugBoard {
    char_map: BTreeMap<usize, usize>,
}

impl PlugBoard {
    pub fn new(mapping: &Vec<(char, char)>) -> PlugBoard {

        let expected_map_size = mapping.len() * 2;

        let int_mappings: Vec<(usize, usize)> = mapping
            .iter()
            .map(|(a, b)| (letter_to_index(*a).0, letter_to_index(*b).0))
            .collect();

        let built_map: BTreeMap<_, _> = int_mappings
            .clone()
            .into_iter()
            .map(|(a, b)| (b, a))
            .chain(int_mappings)
            .collect();

        // Make sure there we got the right number of elements
        assert_eq!(built_map.len(), expected_map_size);

        PlugBoard { char_map: built_map }
    }

    pub fn map_char(&self, c: usize) -> usize {
        *self.char_map.get(&c).unwrap_or(&c)
    }
}
