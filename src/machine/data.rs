use super::{consts::NUM_LETTERS, Rotor};
use strum::{EnumString, IntoStaticStr, Display};

/// ID representing a rotor with a specific configuration
#[derive(Debug, Clone, Copy, EnumString, IntoStaticStr, Display)]
#[allow(clippy::upper_case_acronyms)]
pub enum RotorId {
    /// Rotor I, turns over at R
    I,
    /// Rotor II, turns over at F
    II,
    /// Rotor III, turns over at W
    III,
    /// Rotor IV, turns over at K
    IV,
    /// Rotor V, turns over at A
    V,
}

impl RotorId {
    /// Create a rotor in the given state from this rotor ID
    pub fn make_rotor(&self, starting_position: usize, can_double_step: bool) -> Rotor {
        let (turnover_pos, mappings) = self.get_rotor_config();
        Rotor::new(
            self.to_string(),
            mappings,
            turnover_pos,
            can_double_step,
            starting_position,
        )
    }

    /// Returns a tuple containing the turnover positions and the character
    /// mappings of the rotor
    fn get_rotor_config(self: &RotorId) -> (Vec<char>, [(char, char); NUM_LETTERS]) {
        match self {
            RotorId::I => (vec!['R'], [
                ('a', 'u'),
                ('b', 'w'),
                ('c', 'y'),
                ('d', 'g'),
                ('e', 'a'),
                ('f', 'd'),
                ('g', 'f'),
                ('h', 'p'),
                ('i', 'v'),
                ('j', 'z'),
                ('k', 'b'),
                ('l', 'e'),
                ('m', 'c'),
                ('n', 'k'),
                ('o', 'm'),
                ('p', 't'),
                ('q', 'h'),
                ('r', 'x'),
                ('s', 's'),
                ('t', 'l'),
                ('u', 'r'),
                ('v', 'i'),
                ('w', 'n'),
                ('x', 'q'),
                ('y', 'o'),
                ('z', 'j'),
            ]),
            RotorId::II => (vec!['F'], [
                ('a', 'a'),
                ('b', 'j'),
                ('c', 'p'),
                ('d', 'c'),
                ('e', 'z'),
                ('f', 'w'),
                ('g', 'r'),
                ('h', 'l'),
                ('i', 'f'),
                ('j', 'b'),
                ('k', 'd'),
                ('l', 'k'),
                ('m', 'o'),
                ('n', 't'),
                ('o', 'y'),
                ('p', 'u'),
                ('q', 'q'),
                ('r', 'g'),
                ('s', 'e'),
                ('t', 'n'),
                ('u', 'h'),
                ('v', 'x'),
                ('w', 'm'),
                ('x', 'i'),
                ('y', 'v'),
                ('z', 's'),
            ]),
            RotorId::III => (vec!['W'], [
                ('a', 't'),
                ('b', 'a'),
                ('c', 'g'),
                ('d', 'b'),
                ('e', 'p'),
                ('f', 'c'),
                ('g', 's'),
                ('h', 'd'),
                ('i', 'q'),
                ('j', 'e'),
                ('k', 'u'),
                ('l', 'f'),
                ('m', 'v'),
                ('n', 'n'),
                ('o', 'z'),
                ('p', 'h'),
                ('q', 'y'),
                ('r', 'i'),
                ('s', 'x'),
                ('t', 'j'),
                ('u', 'w'),
                ('v', 'l'),
                ('w', 'r'),
                ('x', 'k'),
                ('y', 'o'),
                ('z', 'm'),
            ]),
            RotorId::IV => (vec!['K'], [
                ('a', 'h'),
                ('b', 'z'),
                ('c', 'w'),
                ('d', 'v'),
                ('e', 'a'),
                ('f', 'r'),
                ('g', 't'),
                ('h', 'n'),
                ('i', 'l'),
                ('j', 'g'),
                ('k', 'u'),
                ('l', 'p'),
                ('m', 'x'),
                ('n', 'q'),
                ('o', 'c'),
                ('p', 'e'),
                ('q', 'j'),
                ('r', 'm'),
                ('s', 'b'),
                ('t', 's'),
                ('u', 'k'),
                ('v', 'd'),
                ('w', 'y'),
                ('x', 'o'),
                ('y', 'i'),
                ('z', 'f'),
            ]),
            RotorId::V => (vec!['A'], [
                ('a', 'q'),
                ('b', 'c'),
                ('c', 'y'),
                ('d', 'l'),
                ('e', 'x'),
                ('f', 'w'),
                ('g', 'e'),
                ('h', 'n'),
                ('i', 'f'),
                ('j', 't'),
                ('k', 'z'),
                ('l', 'o'),
                ('m', 's'),
                ('n', 'm'),
                ('o', 'v'),
                ('p', 'j'),
                ('q', 'u'),
                ('r', 'd'),
                ('s', 'k'),
                ('t', 'g'),
                ('u', 'i'),
                ('v', 'a'),
                ('w', 'r'),
                ('x', 'p'),
                ('y', 'h'),
                ('z', 'b'),
            ]),
        }
    }
}


#[derive(Debug, Clone, Copy, EnumString, IntoStaticStr, Display)]
pub enum ReflectorId {
    A,
    B,
    C,
}


pub fn get_reflector_config(
    reflector_id: ReflectorId
) -> [(char, char); NUM_LETTERS / 2] {
    match reflector_id {
        ReflectorId::A => todo!(),
        ReflectorId::B => [
            ('a', 'y'),
            ('b', 'r'),
            ('c', 'u'),
            ('d', 'h'),
            ('e', 'q'),
            ('f', 's'),
            ('g', 'l'),
            ('i', 'p'),
            ('j', 'x'),
            ('k', 'n'),
            ('m', 'o'),
            ('t', 'z'),
            ('v', 'w'),
        ],
        ReflectorId::C => [
            ('a', 'f'),
            ('b', 'v'),
            ('c', 'p'),
            ('d', 'j'),
            ('e', 'i'),
            ('g', 'o'),
            ('h', 'y'),
            ('k', 'r'),
            ('l', 'z'),
            ('m', 'x'),
            ('n', 'w'),
            ('q', 't'),
            ('s', 'u'),
        ],
    }
}
