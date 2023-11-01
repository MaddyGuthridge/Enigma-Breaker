use super::char_mapping::CharMapping;
use crate::{consts::NUM_LETTERS, letter::Letter};

#[derive(Debug)]
pub struct Reflector {
    char_map: CharMapping,
}

impl Reflector {
    pub fn new(mapping: [(Letter, Letter); NUM_LETTERS / 2]) -> Reflector {
        let char_map = CharMapping::from(
            mapping
                .into_iter()
                .map(|(a, b)| (b, a))
                .chain(mapping)
                .collect::<Vec<(Letter, Letter)>>(),
        );

        Reflector { char_map }
    }

    pub fn reflect(&self, c: Letter) -> Letter {
        self.char_map[c]
    }
}
