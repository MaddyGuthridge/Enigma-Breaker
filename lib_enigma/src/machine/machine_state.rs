use std::fmt::Display;

use crate::{Letter, ReflectorId, RotorId};

/// Represents the state of an enigma machine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachineState {
    pub plug_map: Vec<(Letter, Letter)>,
    pub rotor_ids: Vec<RotorId>,
    pub rotor_starts: Vec<Letter>,
    pub reflector_id: ReflectorId,
}

impl MachineState {
    /// Create a new machine state
    pub fn new(
        plug_map: Vec<(Letter, Letter)>,
        rotor_ids: Vec<RotorId>,
        rotor_starts: Vec<Letter>,
        reflector_id: ReflectorId,
    ) -> MachineState {
        MachineState {
            plug_map,
            rotor_ids,
            rotor_starts,
            reflector_id,
        }
    }
}

unsafe impl Send for MachineState {}

impl Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reflector_id)?;

        if !self.rotor_ids.is_empty() {
            write!(f, " --rotor-ids")?;
            for (id, start) in self.rotor_ids.iter().zip(&self.rotor_starts) {
                write!(f, " {}:{}", id, start)?;
            }
        }

        if !self.plug_map.is_empty() {
            write!(f, " --plug-map")?;
            for (a, b) in &self.plug_map {
                write!(f, " {}{}", a, b)?;
            }
        }

        write!(f, "")
    }
}
