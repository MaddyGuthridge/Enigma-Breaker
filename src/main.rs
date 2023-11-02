mod consts;
mod force;
mod letter;
mod machine;

use clap::Parser;
use letter::Letter;
use machine::{EnigmaMachine, RotorId};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ID of reflector to use, eg `"B"`
    reflector_id: String,

    /// IDs of the rotors to use
    ///
    /// Each rotor should be specified in the format `id` or `id:start`, where
    /// `id` is the rotor ID (in roman numerals), and `start` is the starting
    /// position of said rotor. `start` defaults to 0
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    rotor_ids: Vec<String>,

    /// Sets of plugs to use in the plug board
    ///
    /// Each connection should specify two letters to swap, for example `AB`.
    #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
    plug_map: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    // Parse the rotor options
    let rotors: Vec<(RotorId, Letter)> = args
        .rotor_ids
        .into_iter()
        .map(|r| match r.split_once(':') {
            None => (
                r.as_str()
                    .try_into()
                    .unwrap_or_else(|_| panic!("Invalid rotor ID {r:?}")),
                Letter::A,
            ),
            Some((id, start)) => {
                let parsed = start.chars().next().unwrap();
                (
                    id.try_into()
                        .unwrap_or_else(|_| panic!("Invalid rotor ID {r:?}")),
                    Letter::from_char(parsed).unwrap().0,
                )
            }
        })
        .collect();

    let (rotor_ids, rotor_starts): (Vec<_>, Vec<_>) = rotors.into_iter().unzip();

    // And also parse the plug maps
    let plugs: Vec<(char, char)> = args
        .plug_map
        .into_iter()
        .map(|c| {
            assert_eq!(c.len(), 2);
            (c.chars().next().unwrap(), c.chars().nth(1).unwrap())
        })
        .collect();

    // Now configure the machine
    let mut machine = EnigmaMachine::new(
        &plugs,
        &rotor_ids,
        &rotor_starts,
        args.reflector_id
            .as_str()
            .try_into()
            .expect("Invalid reflector ID"),
    );

    for line in std::io::stdin().lines() {
        println!("{}", machine.consume(&line.unwrap()));
    }
}
