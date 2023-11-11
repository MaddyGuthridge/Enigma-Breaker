use crate::letter::Letter;

use super::char_mapping::CharMapping;

#[derive(Debug)]
pub struct PlugBoard {
    // TODO: Use the CharMapping type for better performance
    char_map: CharMapping,
}

impl PlugBoard {
    pub fn new(mapping: &[(Letter, Letter)]) -> Option<PlugBoard> {
        let mut built_map = CharMapping::default();

        for (a, b) in mapping
        {
            // If the mapping already exists
            if built_map[*a] != *a || built_map[*b] != *b {
                return None;
            }
            built_map[*a] = *b;
            built_map[*b] = *a;
        }

        Some(PlugBoard {
            char_map: built_map,
        })
    }

    pub fn map_char(&self, c: Letter) -> Letter {
        self.char_map[c]
    }
}
