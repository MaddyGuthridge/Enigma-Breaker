# Enigma

A somewhat-broken implementation of an Enigma machine, built in Rust.

It is able to encode and decode its own inputs, but fails inputs from other
sources. I don't have the energy to debug it, but it's still neat, I think.

## Usage

The program behaves similarly to `cat`.

`cargo run -- <reflector ID> -r [rotor IDs] -p [plug maps]`

For example, to use reflector `B`, with rotors `III`, `IV` and `I`, you can run

`cargo run -- B -r III IV I`

For more details, use `cargo run -- --help`

## Example

```txt
$ cargo run -q -- B -r V:X I:C II:B
Hello, world! This is my super cool Enigma machine, programmed in Rust!
Vrqsp, zhjfx! Sash tu fu mbkje rnlj Voompj jzksoqh, vkezntgglg ai Xgdf!
$ cargo run -q -- B -r V:X I:C II:B
Vfohs, qggtk! Vbsu nv ih wirih bvve Nkhcay ymfnelm, rehsdkhquv cg Owls!
Hello, world! This is my super cool Enigma machine, programmed in Rust!
```
