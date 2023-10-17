# Enigma

A somewhat-broken implementation of an Enigma machine, built in Rust.

It is able to encode and decode its own inputs, but fails inputs from other
sources. I don't have the energy to debug it, but it's still neat, I think.

## Usage

`cargo run -- <reflector ID> -r [rotor IDs] -p [plug maps]`

For example, to use reflector `B`, with rotors `III`, `IV` and `I`, you can run

`cargo run -- B -r III IV I`

For more details, use `cargo run -- --help`
