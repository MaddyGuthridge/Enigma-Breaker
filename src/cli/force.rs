use clap::Args;

use crate::{EnigmaMachine, Letter, MachineState, RotorId};

#[derive(Args)]
pub struct EncodeArgs {
    /// ID of reflector to use, eg `"B"`. Use `"?"` if unknown
    reflector_id: Option<String>,

    /// IDs of the rotors to use
    ///
    /// Each rotor should be specified in the format `id` or `id:start`, where
    /// `id` is the rotor ID (in roman numerals), and `start` is the starting
    /// position of said rotor. `start` defaults to unknown.
    ///
    /// Unknown values should be set as `"?"`. For example, if the rotor ID is
    /// unknown, use `"?"`, if the rotor id is unknown but the position is
    /// known to be A, use "?:A", etc.
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    rotor_ids: Vec<String>,

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
}
