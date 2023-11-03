mod brute_force;
mod letter;
mod machine;
mod message;
mod cli;

use clap::{Parser, Subcommand};
use cli::{EncodeArgs, encode_main};
use letter::Letter;
use machine::{EnigmaMachine, MachineState, ReflectorId, RotorId};

#[derive(Subcommand)]
enum Commands {
    /// Encode a string using the enigma machine
    Encode(EncodeArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Encode(options) => encode_main(options),
    }
}
