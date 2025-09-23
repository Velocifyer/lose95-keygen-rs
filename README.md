# A tool to generate losedows 95 keys

It generates a losedows 95 retail key in the format BBB-AAAAAAA and a losedows 95 OEM key in the format CCCDD-OEM-00AAAAA-RRRRR where CCC is 3 digits < 367 and DD is 95 or 96 or 97 or 98 or 99
or 00 or 01 or 02 or 03 and RRRRR is 5 random digits where BBB is 3 digits that are not 333 or 444 or 555 or 666 or 777 or 888 or 999
and AAAAAAA is 7 digits where the sum of them is divisible by 7 with no remainder and 00AAAAA is the same but the first 2 digits are 0.
 Losedows 95 does not actualy care about charector 3 of the retail key

> [!TIP]
> If you use this library or contribute to it you should subscribe to the [lose95-keygen-rs-announce mailing list](https://lists.sr.ht/~velocifyer/lose95-keygen-rs-announce) to get important updates!

## Usage as executable
Can be ran as a command in the syntax
`lose95-keygen-rs version keys_printed`
Version currently can only be `0` but in the future will be able to controll what format arguments will be taken in and what format stuff will be outputted in.
Keys_printed defaults to 255(`all`) witch outputs all keys but if set to 0(`retail`) it only outputs the retail key and if it is 1(`OEM`) (case insensitive) it only outputs the oem key

## Usage as library
Add
```toml
[dependencies]
lose95-keygen-rs = "1.1.1"
```
to your `Cargo.toml`.
Use `gen_retail(0)` to generate a retail key as a String
or `gen_oem(0)` to generate a OEM key as a String
or `gen_array(0, zeroed_digits_at_start)` to generates a array of 7 digits where the sum of the digits is divisible by 7 with`zeroed_digits_at_start` digits of the start of the array that are 0

## Installation
  To install run `cargo install lose95-keygen-rs`.
  If that fails try `cargo install --locked lose95-keygen-rs`.
  If that fails make sure you have cargo installed by running `cargo --version` and that your rust version is ≥ `1.85.1` by running `rustc -Vv`.

## Issues and patches
  Issues can be submited on [codeberg](https://codeberg.org/Velocifyer/lose95-keygen-rs/issues) or [github](https://github.com/Velocifyer/lose95-keygen-rs/issues). Bug reports can also be submitted on [the lose95-keygen-rs-bugs mailing list](https://lists.sr.ht/~velocifyer/lose95-keygen-rs-bugs) Using Codeberg is preferred over github. Patches/Merge requests can be submitted on [codeberg](https://codeberg.org/Velocifyer/lose95-keygen-rs/pulls) or [github](https://github.com/Velocifyer/lose95-keygen-rs/pulls) or using [git-send-email](https://git-send-email.io/) on [the lose95-keygen-rs-devel mailing list](https://lists.sr.ht/~velocifyer/lose95-keygen-rs-devel).
## Codeberg and Github
  This project was maintained on [codeberg](https://codeberg.org/Velocifyer/lose95-keygen-rs). A mirror is available on [github](https://github.com/Velocifyer/lose95-keygen-rs) but it is not as updated as codeberg. A diffrent mirror is avalible on [sourcehut](https://sr.ht/~velocifyer/lose95-keygen-rs/).

## Self Promotion
  If you use cargo-crev, you should add `s67f2b5NTRWJYdgpElWYvNNIXZPLVFK0aTP3IcMo5ck` `https://ash.radicle.garden/zZbBDS6L52The3V6RBqfGtxgKkGa.git`
