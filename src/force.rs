use std::{io, process::exit};

use clap::Args;
use itertools::Itertools;

use lib_enigma::{Letter, Unknown, RotorId, ReflectorId, PlugboardOptions, Message, force_combinations, EnigmaMachine};

#[derive(Args)]
pub struct ForceArgs {
    /// Sets of plugs to use in the plug board
    ///
    /// Each connection should specify two letters to swap, for example `AB`.
    ///
    /// If a number of plugs is unknown, a number or range can be given to
    /// represent the number of plugs in use. For example, you could use
    /// `"10"` to use 10 plugs (connecting 20 letters). If you were using up to
    /// 10 plugs, you could specify `0..11`.
    #[clap(short, long, value_parser, num_args = 0.., value_delimiter = ' ')]
    plug_map: Vec<String>,

    /// IDs of the rotors to use
    ///
    /// Each rotor should be specified in the format `id` or `id:start`, where
    /// `id` is the rotor ID (in roman numerals), and `start` is the starting
    /// position of said rotor. `start` defaults to unknown.
    ///
    /// Unknown values should be set as `"!"`. For example, if the rotor ID is
    /// unknown, use `"!"`, if the rotor id is unknown but the position is
    /// known to be A, use "!:A", etc.
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    rotor_ids: Vec<String>,

    /// ID of reflector to use, eg `"B"`. Use `"!"` if unknown
    reflector_id: Option<String>,

    /// String known to be located at the start of the enciphered message
    #[clap(long)]
    msg_start: Option<String>,

    /// String known to be located at the end of the enciphered message
    #[clap(long)]
    msg_end: Option<String>,

    /// String known to be contained at some location in the enciphered message
    #[clap(long)]
    msg_contains: Option<String>,
}

pub fn force_main(args: ForceArgs) {
    // Check that at least one of `msg-start`, `msg-end` and `msg-contains`
    // is specified
    let mut found_one = false;
    for prop in [&args.msg_start, &args.msg_end, &args.msg_contains] {
        if prop.is_some() {
            found_one = true;
            break;
        }
    }
    if !found_one {
        eprintln!("At least one of --msg-start, --msg-end and --msg-contains must be given");
        eprintln!("Otherwise, the program cannot rule out any combinations");
        exit(1);
    }

    // Parse the rotor options
    // If the options are specified, parse them
    let rotors: Option<Vec<(Unknown<RotorId>, Unknown<Letter>)>> = if !args.rotor_ids.is_empty() {
        Some(
            args.rotor_ids
                .into_iter()
                .map(|r| match r.split_once(':') {
                    // Only rotor ID
                    None => {
                        if r == "!" {
                            (Unknown::Unknown, Unknown::Unknown)
                        } else {
                            (
                                Unknown::Known(
                                    r.as_str()
                                        .try_into()
                                        .unwrap_or_else(|_| panic!("Invalid rotor ID {r:?}")),
                                ),
                                Unknown::Unknown,
                            )
                        }
                    }
                    // Rotor ID and starting position
                    Some((id, start)) => {
                        assert_eq!(start.len(), 1);
                        let start_letter = start.chars().next().unwrap();

                        let rotor_unknown: Unknown<RotorId> = if id == "!" {
                            Unknown::Unknown
                        } else {
                            Unknown::Known(
                                id.try_into()
                                    .unwrap_or_else(|_| panic!("Invalid rotor ID {r:?}")),
                            )
                        };

                        let start_unknown = if start_letter == '!' {
                            Unknown::Unknown
                        } else {
                            Unknown::Known(Letter::from_char(start_letter).unwrap().0)
                        };

                        (rotor_unknown, start_unknown)
                    }
                })
                .collect(),
        )
    } else {
        // Otherwise, no options specified, so default to 3 rotors
        None
    };

    // Also handle the reflector
    let reflector: Unknown<ReflectorId> = if let Some(id) = args.reflector_id {
        if id == "!" {
            Unknown::Unknown
        } else {
            Unknown::Known(id.as_str().try_into().expect("Invalid reflector ID"))
        }
    } else {
        Unknown::Unknown
    };

    // Parse the plug maps
    // If there's one arg and it has a `..` or is a number, there's a number of
    // plug boards
    let plug_options = if args.plug_map.len() == 1
        && (args.plug_map[0].contains("..") || args.plug_map[0].parse::<usize>().is_ok())
    {
        // If there's a `..`, create a range
        if let Some((a, b)) = args.plug_map[0].split_once("..") {
            let start_num: usize = a.parse().expect("Invalid num plugs starting value");
            // Check for inclusive range
            let (b, inclusive_range) = if b.starts_with('=') {
                (b.replacen('=', "", 1), true)
            } else {
                (b.to_owned(), false)
            };
            let end_num: usize = b.parse().expect("Invalid num plugs ending value");

            if inclusive_range {
                PlugboardOptions::NumberInRangeInclusive(start_num..=end_num)
            } else {
                PlugboardOptions::NumberInRange(start_num..end_num)
            }
        } else {
            // Single digit -> inclusive range from `num..=num`
            let num = args.plug_map[0]
                .parse::<usize>()
                .expect("Invalid number of plugs");
            PlugboardOptions::NumberInRangeInclusive(num..=num)
        }
    } else {
        // Otherwise, the plugs are known, so parse them normally
        PlugboardOptions::KnownConnections(
            args.plug_map
                .into_iter()
                .map(|c| {
                    assert_eq!(c.len(), 2);
                    (
                        Letter::from_char(c.chars().next().unwrap()).unwrap().0,
                        Letter::from_char(c.chars().nth(1).unwrap()).unwrap().0,
                    )
                })
                .collect(),
        )
    };

    let input = io::stdin().lines().map(|line| line.unwrap()).join("\n");

    let message = Message::from(input);

    let matches = force_combinations(
        plug_options,
        rotors,
        reflector,
        &message,
        &args.msg_start.map(|m| m.into()),
        &args.msg_end.map(|m| m.into()),
        &args.msg_contains.map(|m| m.into()),
    );

    println!("Done! Found {} matches", matches.len());

    for (i, result) in matches.iter().enumerate() {
        println!("{} :: {}", i + 1, result);
        let decoded = EnigmaMachine::from(result.clone())
            .consume(&message)
            .to_string();
        println!("{decoded}");
        println!();
    }
}
