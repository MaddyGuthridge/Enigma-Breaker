mod util;
mod machine;

use clap::Parser;
use machine::EnigmaMachine;

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
    plug_map: Vec<String>
}

fn main() {
    let args = Cli::parse();

    // Parse the rotor options
    let rotors: Vec<(String, char)> = args.rotor_ids
        .into_iter()
        .map(|r| {
            match r.split_once(':') {
                None => (r, 'A'),
                Some((id, start)) => {
                    let parsed = start.chars().next().unwrap();
                    (id.to_owned(), parsed)
                },
            }
        })
        .collect();

    // And also parse the plug maps
    let plugs: Vec<(char, char)> = args.plug_map
        .into_iter()
        .map(|c| {
            assert_eq!(c.len(), 2);
            (c.chars().next().unwrap(), c.chars().nth(1).unwrap())
        })
        .collect();

    // Now configure the machine
    let mut machine = EnigmaMachine::new(
        &plugs,
        &rotors,
        &args.reflector_id,
    );

    for line in std::io::stdin().lines() {
        println!("{}", machine.consume(line.unwrap()));
    }
}
