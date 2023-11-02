use std::collections::BTreeMap;

use crate::letter::Letter;

#[derive(Debug)]
pub struct PlugBoard {
    char_map: BTreeMap<Letter, Letter>,
}

impl PlugBoard {
    pub fn new(mapping: &Vec<(Letter, Letter)>) -> PlugBoard {
        let expected_map_size = mapping.len() * 2;

        let built_map: BTreeMap<_, _> = mapping
            .clone()
            .into_iter()
            .map(|(a, b)| (b, a))
            .chain(mapping.clone())
            .collect();

        // Make sure there we got the right number of elements
        assert_eq!(built_map.len(), expected_map_size);

        PlugBoard {
            char_map: built_map,
        }
    }

    pub fn map_char(&self, c: Letter) -> Letter {
        *self.char_map.get(&c).unwrap_or(&c)
    }
}
