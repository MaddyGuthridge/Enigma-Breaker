mod char_mapping;
mod data;
mod plug_board;
mod reflector;
mod rotor;

pub use data::{ReflectorId, RotorId};
pub use plug_board::PlugBoard;
pub use reflector::Reflector;
pub use rotor::Rotor;

use crate::letter::Letter;

/// Overall representation of the enigma machine
#[derive(Debug)]
pub struct EnigmaMachine {
    /// Plug board of the machine
    plug_board: PlugBoard,

    /// Vector of rotors in the machine
    rotors: Vec<Rotor>,

    /// Reflector, placed at the end of the rotors
    reflector: Reflector,

    /// Number of steps that the machine has performed (positive for forwards,
    /// negative for backwards)
    steps: i32,
}

impl EnigmaMachine {
    /// Create a new instance of the enigma machine
    pub fn new(
        plug_board_config: &Vec<(char, char)>,
        rotor_ids: &[RotorId],
        rotor_starts: &[Letter],
        reflector_id: ReflectorId,
    ) -> EnigmaMachine {
        let double_step_rotors = 1..rotor_ids.len() - 1;
        EnigmaMachine {
            plug_board: PlugBoard::new(plug_board_config),
            rotors: rotor_ids
                .iter()
                .zip(rotor_starts)
                .enumerate()
                .map(|(i, (id, start))| id.make_rotor(*start, double_step_rotors.contains(&i)))
                .collect(),
            reflector: reflector_id.make_reflector(),
            steps: 0,
        }
    }

    /// Move the machine forward by a step
    pub fn step(&mut self) {
        let mut do_single_step = true;
        for rotor in self.rotors.iter_mut().rev() {
            if do_single_step {
                do_single_step = rotor.step();
            } else {
                do_single_step = rotor.double_step();
            }
        }
        self.steps += 1;
    }

    /// Move the machine backwards by a step
    pub fn unstep(&mut self) {
        let mut do_single_step = true;
        for rotor in self.rotors.iter_mut().rev() {
            if do_single_step {
                do_single_step = rotor.unstep();
            } else {
                do_single_step = rotor.double_unstep();
            }
        }
        self.steps -= 1;
    }

    fn encode_char(&mut self, c: char) -> char {
        if let Some((mut letter, capital)) = Letter::from_char(c) {
            // First, tick the rotors
            self.step();

            // Through plug board
            letter = self.plug_board.map_char(letter);

            // Then each rotor forwards
            for rotor in self.rotors.iter().rev() {
                letter = rotor.char_out(letter);
            }

            // Then through the reflector
            letter = self.reflector.reflect(letter);

            // Then back through the rotors (in reverse this time)
            for rotor in &self.rotors {
                letter = rotor.char_in(letter);
            }

            // Then finally back through the plug board
            letter = self.plug_board.map_char(letter);

            letter.to_char(capital)
        } else {
            c
        }
    }

    /// Move the machine forwards by a number of steps
    pub fn jump_forwards(&mut self, distance: usize) {
        for _ in 0..distance {
            self.step();
        }
    }

    /// Move the machine backwards by a number of steps
    pub fn jump_backwards(&mut self, distance: usize) {
        for _ in 0..distance {
            self.unstep();
        }
    }

    /// Reset the machine to its initial state (ie rotors in default positions)
    pub fn reset(&mut self) {
        if self.steps < 0 {
            self.jump_forwards((-self.steps) as usize)
        } else {
            self.jump_backwards(self.steps as usize)
        }
        self.steps = 0;
    }

    /// Encode a string, returning the result
    pub fn consume(&mut self, input: &str) -> String {
        input.chars().map(|c| self.encode_char(c)).collect()
    }

    /// Attempt to consume the given input, failing if it
    ///
    /// The machine is reset to its starting state if they don't match, but is
    /// not reset if the string was consumed successfully
    pub fn try_consume(&mut self, input: &str, expected_output: &str) -> bool {
        let start_steps = self.steps;

        // Optimisation - if input and expected output contain any letters that
        // are equal, the input is guaranteed not to encode to the output,
        // since enigma machines never encode a character to itself
        for (c_in, c_exp) in input.chars().zip(expected_output.chars()) {
            if c_in == c_exp && c_in.is_ascii_alphabetic() {
                return false;
            }
        }

        for (c_in, c_exp) in input.chars().zip(expected_output.chars()) {
            if self.encode_char(c_in) != c_exp {
                // Jump back to position before consuming
                self.jump_backwards((self.steps - start_steps) as usize);
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde::Deserialize;

    use crate::Letter;

    use super::EnigmaMachine;

    #[derive(Debug, Deserialize)]
    struct TestCase {
        /// ID of reflector to use
        reflector_id: String,

        /// Configs for each rotor
        rotors: Vec<(String, char)>,

        /// Plug board config
        plugs: Vec<(char, char)>,

        /// Input for the test case
        input: String,

        /// Expected output
        expect: String,
    }

    fn read_test_case(path: &str) -> TestCase {
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }

    fn run_test_case(path: &str) {
        let test_data = read_test_case(path);

        let (rotor_ids, rotor_starts): (Vec<_>, Vec<_>) = test_data.rotors.into_iter().unzip();

        let rotor_ids = rotor_ids
            .into_iter()
            .map(|id| id.as_str().try_into().unwrap())
            .collect::<Vec<_>>();

        let mut machine = EnigmaMachine::new(
            &test_data.plugs,
            &rotor_ids,
            &rotor_starts
                .iter()
                .map(|c| Letter::from_char(*c).unwrap().0)
                .collect::<Vec<_>>(),
            test_data.reflector_id.as_str().try_into().unwrap(),
        );

        let encoded = machine.consume(&test_data.input);

        assert_eq!(encoded, test_data.expect);
    }

    #[test]
    fn double_step() {
        run_test_case("tests/double_step.json");
    }

    #[test]
    fn simple() {
        run_test_case("tests/simple.json");
    }

    #[test]
    fn richards_favourite_word() {
        run_test_case("tests/richard.json");
    }

    #[test]
    fn test_jumps() {
        let test_data = read_test_case("tests/simple.json");

        let (rotor_ids, rotor_starts): (Vec<_>, Vec<_>) = test_data.rotors.into_iter().unzip();

        let rotor_ids = rotor_ids
            .into_iter()
            .map(|id| id.as_str().try_into().unwrap())
            .collect::<Vec<_>>();

        let mut machine = EnigmaMachine::new(
            &test_data.plugs,
            &rotor_ids,
            &rotor_starts
                .iter()
                .map(|c| Letter::from_char(*c).unwrap().0)
                .collect::<Vec<_>>(),
            test_data.reflector_id.as_str().try_into().unwrap(),
        );

        // Jump forwards then backwards
        machine.jump_forwards(1000);
        machine.jump_backwards(1000);

        let encoded = machine.consume(&test_data.input);

        assert_eq!(encoded, test_data.expect);
    }
}
