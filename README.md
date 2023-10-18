# Enigma

An implementation of an Enigma machine, built in Rust.

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
Jtdvt, zndgl! Jrvr cq ik ydkqk qmws Nxxxtx sylgzjn, kmfwdmfwcv gc Iqcx!
$ cargo run -q -- B -r V:X I:C II:B
Jtdvt, zndgl! Jrvr cq ik ydkqk qmws Nxxxtx sylgzjn, kmfwdmfwcv gc Iqcx!
Hello, world! This is my super cool Enigma machine, programmed in Rust!
```

## References

* [Franklin Heath - Paper Enigma](http://wiki.franklinheath.co.uk/index.php/Enigma/Paper_Enigma) for the excellent overview on how Enigma machines work
* [Ilmari Karonen - Stack Exchange](https://crypto.stackexchange.com/a/71395/112016) for the explanation of double stepping
* [Wikipedia - Enigma Rotor Details](https://en.wikipedia.org/wiki/Enigma_rotor_details) for details on the specific rotor configurations
* [101 Computing - Enigma Machine Emulator](https://www.101computing.net/enigma-machine-emulator/) which I used to validate my work somewhat
