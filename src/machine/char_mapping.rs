use std::ops::{Index, IndexMut};

use strum::EnumCount;

use crate::letter::Letter;

#[derive(Debug, Clone)]
pub struct CharMapping([Letter; Letter::COUNT]);

impl Index<Letter> for CharMapping {
    type Output = Letter;

    fn index(&self, index: Letter) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Letter> for CharMapping {
    fn index_mut(&mut self, index: Letter) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl IntoIterator for CharMapping {
    type Item = Letter;

    type IntoIter = <[Letter; Letter::COUNT] as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a CharMapping {
    type Item = &'a Letter;

    type IntoIter = std::slice::Iter<'a, Letter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Default for CharMapping {
    fn default() -> Self {
        Self([Letter::A; Letter::COUNT])
    }
}

impl From<[(char, char); Letter::COUNT]> for CharMapping {
    fn from(chars: [(char, char); Letter::COUNT]) -> CharMapping {
        let mut mapping = CharMapping::default();

        for (from, to) in chars {
            mapping.add_to_mapping(from, to);
        }

        mapping
    }
}

impl From<[(Letter, Letter); Letter::COUNT]> for CharMapping {
    fn from(chars: [(Letter, Letter); Letter::COUNT]) -> CharMapping {
        let mut mapping = CharMapping::default();

        for (from, to) in chars {
            mapping[from] = to;
            mapping[to] = from;
        }

        mapping
    }
}

impl From<Vec<(char, char)>> for CharMapping {
    fn from(chars: Vec<(char, char)>) -> CharMapping {
        assert_eq!(chars.len(), Letter::COUNT);

        let mut mapping = CharMapping::default();

        for (from, to) in chars {
            mapping.add_to_mapping(from, to);
        }

        mapping
    }
}

impl From<Vec<(Letter, Letter)>> for CharMapping {
    fn from(chars: Vec<(Letter, Letter)>) -> CharMapping {
        assert_eq!(chars.len(), Letter::COUNT);

        let mut mapping = CharMapping::default();

        for (from, to) in chars {
            mapping[from] = to;
            mapping[to] = from;
        }

        mapping
    }
}

impl CharMapping {
    pub fn from_reverse_of(map: &CharMapping) -> CharMapping {
        let mut new = CharMapping::default();

        for (i, c) in map.0.iter().enumerate() {
            new[*c] = Letter::from_usize(i).unwrap();
        }

        new
    }

    fn add_to_mapping(&mut self, from: char, to: char) {
        let c_a = Letter::from_char(from).unwrap().0;
        let c_b = Letter::from_char(to).unwrap().0;
        // FIXME: Add some way to check if it is valid or not
        // if self[c_a] != INVALID_MAPPING {
        //     let existing = self[c_a];
        //     panic!("Cannot map char {from:?} to {to:?}, {from:?} already maps to {existing:?}",)
        // }
        self[c_a] = c_b;
        self[c_b] = c_a;
    }
}
