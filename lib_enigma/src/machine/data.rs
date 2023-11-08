use super::{Reflector, Rotor};
use crate::letter::Letter;
use strum::{Display, EnumIter, EnumString, IntoStaticStr, EnumCount};

/// ID representing a rotor with a specific configuration
#[derive(Debug, Clone, Copy, EnumIter, EnumString, IntoStaticStr, Display, PartialEq, Eq)]
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

unsafe impl Send for RotorId {}

impl RotorId {
    /// Create a rotor in the given state from this rotor ID
    pub fn make_rotor(&self, starting_position: Letter, can_double_step: bool) -> Rotor {
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
    fn get_rotor_config(self: &RotorId) -> (Vec<Letter>, [(Letter, Letter); Letter::COUNT]) {
        match self {
            RotorId::I => (
                vec![Letter::R],
                [
                    (Letter::A, Letter::U),
                    (Letter::B, Letter::W),
                    (Letter::C, Letter::Y),
                    (Letter::D, Letter::G),
                    (Letter::E, Letter::A),
                    (Letter::F, Letter::D),
                    (Letter::G, Letter::F),
                    (Letter::H, Letter::P),
                    (Letter::I, Letter::V),
                    (Letter::J, Letter::Z),
                    (Letter::K, Letter::B),
                    (Letter::L, Letter::E),
                    (Letter::M, Letter::C),
                    (Letter::N, Letter::K),
                    (Letter::O, Letter::M),
                    (Letter::P, Letter::T),
                    (Letter::Q, Letter::H),
                    (Letter::R, Letter::X),
                    (Letter::S, Letter::S),
                    (Letter::T, Letter::L),
                    (Letter::U, Letter::R),
                    (Letter::V, Letter::I),
                    (Letter::W, Letter::N),
                    (Letter::X, Letter::Q),
                    (Letter::Y, Letter::O),
                    (Letter::Z, Letter::J),
                ],
            ),
            RotorId::II => (
                vec![Letter::F],
                [
                    (Letter::A, Letter::A),
                    (Letter::B, Letter::J),
                    (Letter::C, Letter::P),
                    (Letter::D, Letter::C),
                    (Letter::E, Letter::Z),
                    (Letter::F, Letter::W),
                    (Letter::G, Letter::R),
                    (Letter::H, Letter::L),
                    (Letter::I, Letter::F),
                    (Letter::J, Letter::B),
                    (Letter::K, Letter::D),
                    (Letter::L, Letter::K),
                    (Letter::M, Letter::O),
                    (Letter::N, Letter::T),
                    (Letter::O, Letter::Y),
                    (Letter::P, Letter::U),
                    (Letter::Q, Letter::Q),
                    (Letter::R, Letter::G),
                    (Letter::S, Letter::E),
                    (Letter::T, Letter::N),
                    (Letter::U, Letter::H),
                    (Letter::V, Letter::X),
                    (Letter::W, Letter::M),
                    (Letter::X, Letter::I),
                    (Letter::Y, Letter::V),
                    (Letter::Z, Letter::S),
                ],
            ),
            RotorId::III => (
                vec![Letter::W],
                [
                    (Letter::A, Letter::T),
                    (Letter::B, Letter::A),
                    (Letter::C, Letter::G),
                    (Letter::D, Letter::B),
                    (Letter::E, Letter::P),
                    (Letter::F, Letter::C),
                    (Letter::G, Letter::S),
                    (Letter::H, Letter::D),
                    (Letter::I, Letter::Q),
                    (Letter::J, Letter::E),
                    (Letter::K, Letter::U),
                    (Letter::L, Letter::F),
                    (Letter::M, Letter::V),
                    (Letter::N, Letter::N),
                    (Letter::O, Letter::Z),
                    (Letter::P, Letter::H),
                    (Letter::Q, Letter::Y),
                    (Letter::R, Letter::I),
                    (Letter::S, Letter::X),
                    (Letter::T, Letter::J),
                    (Letter::U, Letter::W),
                    (Letter::V, Letter::L),
                    (Letter::W, Letter::R),
                    (Letter::X, Letter::K),
                    (Letter::Y, Letter::O),
                    (Letter::Z, Letter::M),
                ],
            ),
            RotorId::IV => (
                vec![Letter::K],
                [
                    (Letter::A, Letter::H),
                    (Letter::B, Letter::Z),
                    (Letter::C, Letter::W),
                    (Letter::D, Letter::V),
                    (Letter::E, Letter::A),
                    (Letter::F, Letter::R),
                    (Letter::G, Letter::T),
                    (Letter::H, Letter::N),
                    (Letter::I, Letter::L),
                    (Letter::J, Letter::G),
                    (Letter::K, Letter::U),
                    (Letter::L, Letter::P),
                    (Letter::M, Letter::X),
                    (Letter::N, Letter::Q),
                    (Letter::O, Letter::C),
                    (Letter::P, Letter::E),
                    (Letter::Q, Letter::J),
                    (Letter::R, Letter::M),
                    (Letter::S, Letter::B),
                    (Letter::T, Letter::S),
                    (Letter::U, Letter::K),
                    (Letter::V, Letter::D),
                    (Letter::W, Letter::Y),
                    (Letter::X, Letter::O),
                    (Letter::Y, Letter::I),
                    (Letter::Z, Letter::F),
                ],
            ),
            RotorId::V => (
                vec![Letter::A],
                [
                    (Letter::A, Letter::Q),
                    (Letter::B, Letter::C),
                    (Letter::C, Letter::Y),
                    (Letter::D, Letter::L),
                    (Letter::E, Letter::X),
                    (Letter::F, Letter::W),
                    (Letter::G, Letter::E),
                    (Letter::H, Letter::N),
                    (Letter::I, Letter::F),
                    (Letter::J, Letter::T),
                    (Letter::K, Letter::Z),
                    (Letter::L, Letter::O),
                    (Letter::M, Letter::S),
                    (Letter::N, Letter::M),
                    (Letter::O, Letter::V),
                    (Letter::P, Letter::J),
                    (Letter::Q, Letter::U),
                    (Letter::R, Letter::D),
                    (Letter::S, Letter::K),
                    (Letter::T, Letter::G),
                    (Letter::U, Letter::I),
                    (Letter::V, Letter::A),
                    (Letter::W, Letter::R),
                    (Letter::X, Letter::P),
                    (Letter::Y, Letter::H),
                    (Letter::Z, Letter::B),
                ],
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, EnumString, IntoStaticStr, Display, PartialEq, Eq)]
pub enum ReflectorId {
    A,
    B,
    C,
}

unsafe impl Send for ReflectorId {}

impl ReflectorId {
    pub fn make_reflector(&self) -> Reflector {
        Reflector::new(self.get_reflector_config())
    }

    fn get_reflector_config(&self) -> [(Letter, Letter); Letter::COUNT / 2] {
        match self {
            ReflectorId::A => [
                (Letter::A, Letter::E),
                (Letter::B, Letter::J),
                (Letter::C, Letter::M),
                (Letter::D, Letter::Z),
                (Letter::F, Letter::L),
                (Letter::G, Letter::Y),
                (Letter::H, Letter::X),
                (Letter::I, Letter::V),
                (Letter::K, Letter::W),
                (Letter::N, Letter::R),
                (Letter::O, Letter::Q),
                (Letter::P, Letter::U),
                (Letter::S, Letter::T),
            ],
            ReflectorId::B => [
                (Letter::A, Letter::Y),
                (Letter::B, Letter::R),
                (Letter::C, Letter::U),
                (Letter::D, Letter::H),
                (Letter::E, Letter::Q),
                (Letter::F, Letter::S),
                (Letter::G, Letter::L),
                (Letter::I, Letter::P),
                (Letter::J, Letter::X),
                (Letter::K, Letter::N),
                (Letter::M, Letter::O),
                (Letter::T, Letter::Z),
                (Letter::V, Letter::W),
            ],
            ReflectorId::C => [
                (Letter::A, Letter::F),
                (Letter::B, Letter::V),
                (Letter::C, Letter::P),
                (Letter::D, Letter::J),
                (Letter::E, Letter::I),
                (Letter::G, Letter::O),
                (Letter::H, Letter::Y),
                (Letter::K, Letter::R),
                (Letter::L, Letter::Z),
                (Letter::M, Letter::X),
                (Letter::N, Letter::W),
                (Letter::Q, Letter::T),
                (Letter::S, Letter::U),
            ],
        }
    }
}
