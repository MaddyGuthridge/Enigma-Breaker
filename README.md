# Enigma Breaker

An enigma machine emulation and brute-force breaker, written in Rust.

## Usage

### Enciphering

The program behaves similarly to `cat`.

`cargo run -- <reflector ID> -r [rotor IDs] -p [plug maps]`

* Reflector ID is specified as a char (eg `A`)

* Rotors are given as roman numerals (eg `IV`). To specify a starting position,
  a colon is used, followed by the letter at which it should start. (eg `II:T`
  would use rotor `II` starting at position `T`). If not provided, the starting
  letter is assumed to be `A`.

* Plug maps are given as a set of letters to swap (eg `AC` would swap letters
  `A` and `C`).

For example, to use reflector `B`, with rotors `III`, `IV` and `I`, you can run

`cargo run -- encipher B -r III IV I`

For more details, use `cargo run -- --help`

#### Encoding example

```txt
$ cargo run -q -- encipher B -r V:X I:C II:B
Hello, world! This is my super cool Enigma machine, programmed in Rust!
Jtdvt, zndgl! Jrvr cq ik ydkqk qmws Nxxxtx sylgzjn, kmfwdmfwcv gc Iqcx!
$ cargo run -q -- encipher B -r V:X I:C II:B
Jtdvt, zndgl! Jrvr cq ik ydkqk qmws Nxxxtx sylgzjn, kmfwdmfwcv gc Iqcx!
Hello, world! This is my super cool Enigma machine, programmed in Rust!
```

### Brute force deciphering

The program behaves similarly to when enciphering. Any unknown values are
indicated using an `!` character.

* For an unknown reflector or rotor, you can simply use `!`.

* For an known rotor with an unknown starting position, you can just specify
  the rotor. For example, to specify rotor `V` with an unknown position, you
  can use `V` or `V:!`.

* For an unknown rotor with a known starting position (say `C`), you can use
  `!:C`.

For the plug board, you should specify a number of plugs that was used. For
example, to specify 10 plugs, you could use `10`. Ranges are also supported,
such as `4..7` for 4, 5 or 6 plugs, or `3..=5` for 3, 4 or 5 plugs.

Please note that as the number of plugs increases, the number of potential
combinations increases to absurd degrees. Using 10 plugs, and three unknown
rotors would result in 67 bits of work, which would take the software about
30000 years to solve from my calculations.

#### Message constraints

The program has no knowledge of what messages do/don't make sense, and so in
order to discard combinations, it requires constraints on the output message.

Currently, the available ways of narrowing this are by specifying any
combination of a known "message start", "message end", or "message contains".

* `--msg-start`: the message must start with the given string to be considered
  a match.

* `--msg-end`: the message must finish with the given string to be considered
  a match.

* `--msg-contains`: the message must contain the given string at any location
  to be considered a match.

#### Brute force example

Demonstrates enciphering, then brute-forcing a message. The only given
knowledge of the message is that it starts with the word "Hello".

```txt
$ cargo run -q -- encipher B -r V:X I:C II:B
Hello, world! This is my super cool Enigma machine, programmed in Rust!
Jtdvt, zndgl! Jrvr cq ik ydkqk qmws Nxxxtx sylgzjn, kmfwdmfwcv gc Iqcx!
$ cargo run -q -- force ! -r ! ! ! --msg-start Hello
Jtdvt, zndgl! Jrvr cq ik ydkqk qmws Nxxxtx sylgzjn, kmfwdmfwcv gc Iqcx!
Done! Found 2 matches
1 :: A --rotor-ids III:I V:R III:D
Hello, tipwh! Tned lc wv noegz tfyo Apdlih djpmfgf, hylfkogiqw wn Mcbj!

2 :: B --rotor-ids V:X I:C II:B
Hello, world! This is my super cool Enigma machine, programmed in Rust!
```

Observe that given our constraints (message starts with `Hello`), it was able
to find two matches. Since the program has no concept of words, it is unaware
that the first result is nonsensical.

## Todo

* [X] Implement an enigma machine
* [X] Support controlling the machine to try combinations
* [X] Write code for handling known/unknown properties when brute-forcing
* [X] Write algorithm for brute-forcing the output
* [X] Implement optimisation where letters cannot encode to themselves
* [X] Add benchmarks to track program performance
* [ ] Error when no known information is given about the text, since that would
      result in all combinations being valid
* [ ] Support brute-forcing from a list of possible configurations
* [ ] Support looking for dictionary words in the output
* [ ] Display progress while running brute-force algorithms
* [ ] Use threading to speed up brute-forcing
* [ ] Implement an estimated time remaining
* [ ] Make it faster by preventing unnecessary copying of data

## References

* [Franklin Heath - Paper Enigma](http://wiki.franklinheath.co.uk/index.php/Enigma/Paper_Enigma) for the excellent overview on how Enigma machines work
* [Ilmari Karonen - Stack Exchange](https://crypto.stackexchange.com/a/71395/112016) for the explanation of double stepping
* [Wikipedia - Enigma Rotor Details](https://en.wikipedia.org/wiki/Enigma_rotor_details) for details on the specific rotor configurations
* [101 Computing - Enigma Machine Emulator](https://www.101computing.net/enigma-machine-emulator/) which I used to validate my work somewhat
