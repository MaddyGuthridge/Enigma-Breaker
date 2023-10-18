use crate::{data::{get_rotor_config, get_reflector_config}, plug_board::PlugBoard, reflector::Reflector, rotor::Rotor, util::{letter_to_index, index_to_letter}};

#[derive(Debug)]
pub struct EnigmaMachine {
    plug_board: PlugBoard,
    rotors: Vec<Rotor>,
    reflector: Reflector,
}

impl EnigmaMachine {
    pub fn new(
        plug_board_config: &Vec<(char, char)>,
        rotor_ids: &[(String, char)],
        reflector_id: &str,
    ) -> EnigmaMachine {
        let double_step_rotors = 1..rotor_ids.len() - 1;
        EnigmaMachine {
            plug_board: PlugBoard::new(plug_board_config),
            rotors: rotor_ids
                .iter()
                .enumerate()
                .map(|(i, (id, start))| {
                    let (turnover_pos, mappings) = get_rotor_config(id);
                    Rotor::new(
                        id.clone(),
                        mappings,
                        turnover_pos,
                        double_step_rotors.contains(&i),
                        letter_to_index(*start).0,
                    )
                })
                .collect(),
            reflector: Reflector::new(get_reflector_config(reflector_id)),
        }
    }

    fn tick(&mut self) {
        let mut do_single_step = true;
        for rotor in self.rotors.iter_mut().rev() {
            if do_single_step {
                do_single_step = rotor.step();
            } else {
                do_single_step = rotor.double_step();
            }
        }
    }

    fn encode_char(&mut self, c: char) -> char {
        if c.is_ascii_alphabetic() {
            let (mut i, capital) = letter_to_index(c);

            // First, tick the rotors
            self.tick();

            // Through plug board
            i = self.plug_board.map_char(i);

            // Then each rotor forwards
            for rotor in self.rotors.iter().rev() {
                i = rotor.char_out(i);
            }

            // Then through the reflector
            i = self.reflector.reflect(i);

            // Then back through the rotors (in reverse this time)
            for rotor in &self.rotors {
                i = rotor.char_in(i);
            }

            // Then finally back through the plug board
            i = self.plug_board.map_char(i);

            index_to_letter(i, capital)
        } else {
            c
        }
    }

    pub fn consume(&mut self, input: String) -> String {
        input
            .chars()
            .map(|c| self.encode_char(c))
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use std::fs;

    use serde::Deserialize;

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
        serde_json::from_str(
            &fs::read_to_string(path).unwrap()
        ).unwrap()
    }

    fn run_test_case(path: &str) {
        let test_data = read_test_case(path);

        let mut machine = EnigmaMachine::new(
            &test_data.plugs,
            &test_data.rotors,
            &test_data.reflector_id,
        );

        let encoded = machine.consume(test_data.input);

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
}
