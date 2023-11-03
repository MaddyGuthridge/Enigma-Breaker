mod brute_force;
mod letter;
mod machine;
mod message;
mod cli;

use clap::{Parser, Subcommand};
use cli::{EncipherArgs, encipher_main, ForceArgs, force_main};
use letter::Letter;
use machine::{EnigmaMachine, MachineState, ReflectorId, RotorId};

#[derive(Subcommand)]
enum Commands {
    /// Encipher a string using the enigma machine
    Encipher(EncipherArgs),

    /// Use brute-force to decipher a message enciphered using an enigma machine
    Force(ForceArgs),
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
        Commands::Encipher(args) => encipher_main(args),
        Commands::Force(args) => force_main(args),
    }
}
