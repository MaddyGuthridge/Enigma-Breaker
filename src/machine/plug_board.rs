use std::collections::BTreeMap;

use crate::letter::Letter;

#[derive(Debug)]
pub struct PlugBoard {
    char_map: BTreeMap<Letter, Letter>,
}

impl PlugBoard {
    pub fn new(mapping: &Vec<(char, char)>) -> PlugBoard {
        let expected_map_size = mapping.len() * 2;

        let int_mappings: Vec<(Letter, Letter)> = mapping
            .iter()
            .map(|(a, b)| {
                (
                    Letter::from_char(*a).unwrap().0,
                    Letter::from_char(*b).unwrap().0,
                )
            })
            .collect();

        let built_map: BTreeMap<_, _> = int_mappings
            .clone()
            .into_iter()
            .map(|(a, b)| (b, a))
            .chain(int_mappings)
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
