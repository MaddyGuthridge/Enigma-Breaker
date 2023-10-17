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
        rotor_ids: &[(String, usize)],
        reflector_id: &str,
    ) -> EnigmaMachine {
        EnigmaMachine {
            plug_board: PlugBoard::new(plug_board_config),
            rotors: rotor_ids
                .iter()
                .map(|(id, start)| {
                    let (turnover_pos, mappings) = get_rotor_config(id);
                    Rotor::new(mappings, turnover_pos, *start)
                })
                .collect(),
            reflector: Reflector::new(get_reflector_config(reflector_id)),
        }
    }

    fn tick(&mut self) {
        for rotor in &mut self.rotors {
            if !rotor.tick() {
                break;
            }
        }
    }

    fn encode_char(&mut self, c: char) -> char {
        if c.is_ascii_alphabetic() {
            let (mut i, capital) = letter_to_index(c);

            // Through plug board
            i = self.plug_board.map_char(i);

            // Then each rotor forwards
            for rotor in &self.rotors {
                i = rotor.char_in(i);
            }

            // Then through the reflector
            i = self.reflector.reflect(i);

            // Then back through the rotors (in reverse this time)
            for rotor in self.rotors.iter().rev() {
                i = rotor.char_out(i);
            }

            // Then finally back through the plug board
            i = self.plug_board.map_char(i);

            // Now tick the rotors
            self.tick();

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
