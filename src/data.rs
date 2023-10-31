use crate::consts::NUM_LETTERS;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum RotorId {
    I,
    II,
    III,
    IV,
    V,
}

impl From<&str> for RotorId {
    fn from(value: &str) -> Self {
        match value {
            "I" => RotorId::I,
            "II" => RotorId::II,
            "III" => RotorId::III,
            "IV" => RotorId::IV,
            "V" => RotorId::V,
            _ => panic!("Unknown rotor id {value:?}")
        }
    }
}

impl From<&String> for RotorId {
    fn from(value: &String) -> Self {
        let str_value: &str = value.as_str();
        str_value.into()
    }
}

pub fn get_rotor_config(rotor_id: RotorId) -> (Vec<char>, [(char, char); NUM_LETTERS]) {
    match rotor_id {
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

#[derive(Debug, Clone, Copy)]
pub enum ReflectorId {
    A,
    B,
    C,
}

impl From<&str> for ReflectorId {
    fn from(value: &str) -> Self {
        match value {
            "A" => ReflectorId::A,
            "B" => ReflectorId::B,
            "C" => ReflectorId::C,
            _ => panic!("Unknown reflector id {value:?}")
        }
    }
}

impl From<&String> for ReflectorId {
    fn from(value: &String) -> Self {
        let str_value: &str = value.as_str();
        str_value.into()
    }
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
