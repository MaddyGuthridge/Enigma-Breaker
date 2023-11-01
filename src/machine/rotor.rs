use super::char_mapping::CharMapping;
use crate::consts::NUM_LETTERS;
use crate::letter::Letter;

#[derive(Debug, Clone)]
pub struct Rotor {
    /// Name of the rotor
    pub name: String,

    /// Array of mappings between chars
    ///
    /// Each index maps to another index
    char_map: CharMapping,

    /// Array of reverse mappings to speed up reverse lookup
    reverse_char_map: CharMapping,

    /// Current position of the rotor
    pos: Letter,

    /// Positions at which turning over this rotor turns over
    /// the next rotor
    turnover_pos: Vec<Letter>,

    /// Whether this rotor is capable of performing a double-stepping operation
    can_double_step: bool,
}

impl Rotor {
    pub fn new(
        name: String,
        mappings: [(Letter, Letter); NUM_LETTERS],
        turnover_pos: Vec<Letter>,
        can_double_step: bool,
        pos: Letter,
    ) -> Rotor {
        // Build the mappings
        let char_map = CharMapping::from(mappings);
        let reverse_char_map = CharMapping::from_reverse_of(&char_map);

        Rotor {
            name,
            char_map,
            reverse_char_map,
            turnover_pos,
            can_double_step,
            pos,
        }
    }

    /// Convert a character sending it forwards through the system
    pub fn char_in(&self, c: Letter) -> Letter {
        let idx = c + self.pos;
        self.char_map[idx] - self.pos
    }

    pub fn char_out(&self, c: Letter) -> Letter {
        let idx = c + self.pos;
        self.reverse_char_map[idx] - self.pos
    }

    /// Move this rotor to the next step
    ///
    /// Returns whether the next rotor should perform a regular step (`true`)
    /// which happens if this rotor reaches its turnover position, or should
    /// perform a potential double step (`false`) which happens otherwise
    pub fn step(&mut self) -> bool {
        self.pos += 1;
        self.turnover_pos.contains(&self.pos)
    }

    /// Move this rotor to the previous step
    ///
    /// Returns whether the next rotor should also undo its step (`true`)
    /// which happens if this rotor was at its turnover position, or should
    /// potentially undo a double step (`false`) which happens otherwise
    pub fn unstep(&mut self) -> bool {
        let did_turnover = self.turnover_pos.contains(&self.pos);
        self.pos -= 1;
        did_turnover
    }


    /// Potentially perform a double step
    ///
    /// The rotor is stepped, iff it is configured to be a double-stepping
    /// rotor (ie not the first or last), and if it is currently at a turnover
    /// position
    ///
    /// Returns whether the next one should be a regular step (`true`), which
    /// happens if this rotor stepped, or a double step (`false`), which
    /// happens otherwise
    pub fn double_step(&mut self) -> bool {
        if self.can_double_step && self.turnover_pos.contains(&(self.pos + 1)) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    /// Potentially undo a double step
    ///
    /// The rotor is unstepped, iff it is configured to be a double-stepping
    /// rotor (ie not the first or last) and it will unstep to a turnover
    /// position
    ///
    /// Returns whether the next one should be a regular unstep (`true`), which
    /// happens if this rotor is unstepped, or a double step (`false`), which
    /// happens otherwise
    pub fn double_unstep(&mut self) -> bool {
        if self.can_double_step && self.turnover_pos.contains(&self.pos) {
            self.pos -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::super::data::RotorId;
    use crate::letter::Letter;

    #[test]
    fn inputs_are_symmetric() {
        let r = RotorId::I.make_rotor(Letter::A, false);

        for i in Letter::iter() {
            let encoded = r.char_in(i);
            let decoded = r.char_out(encoded);
            assert_eq!(decoded, i);
        }
    }

    #[test]
    fn step_unstep_simple() {
        let mut r = RotorId::I.make_rotor(Letter::A, false);

        let starting_pos = r.pos;

        r.step();
        r.unstep();

        assert_eq!(r.pos, starting_pos);
    }

    /// When we hit a turnover step, does unstepping tell us that it was a
    /// turnover
    #[test]
    fn step_unstep_turnover() {
        let mut r = RotorId::I.make_rotor(Letter::R, false);

        // It should have triggered the next one to step
        assert!(r.step());

        // And unstepping should also trigger it
        assert!(r.unstep());
    }

    #[test]
    fn double_step_double_unstep() {
        let mut r = RotorId::I.make_rotor(Letter::R, true);

        // The next one should double step
        assert!(r.double_step());

        // And unstepping should double unstep
        assert!(r.double_unstep());
    }
}
