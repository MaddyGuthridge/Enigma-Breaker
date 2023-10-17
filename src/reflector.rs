use crate::{char_mapping::CharMapping, consts::NUM_LETTERS};

#[derive(Debug)]
pub struct Reflector {
    char_map: CharMapping,
}

impl Reflector {
    pub fn new(mapping: [(char, char); NUM_LETTERS / 2]) -> Reflector {

        let char_map = CharMapping::from(
            mapping
                .into_iter()
                .map(|(a, b)| (b, a))
                .chain(mapping)
                .collect::<Vec<(char, char)>>()
        );

        Reflector { char_map }
    }

    pub fn reflect(&self, c: usize) -> usize {
        self.char_map[c]
    }
}
