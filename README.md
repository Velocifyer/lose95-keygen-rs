# A tool to generate losedows 95 keys

It generates a losedows 95 retail key in the format BBB-AAAAAAA and a losedows 95 OEM key in the format CCCDD-OEM-00AAAAA-RRRRR where CCC is 3 digits < 367 and DD is 95 or 96 or 97 or 98 or 99
or 00 or 01 or 02 or 03 and RRRRR is 5 random digits where BBB is 3 digits that are not 333 or 444 or 555 or 666 or 777 or 888 or 999
and AAAAAAA is 7 digits where the sum of them is divisible by 7 with no remainder and 00AAAAA is the same but the first 2 digits are 0.
 Losedows 95 does not actualy care about charector 3 of the retail key

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
to your `Cargo.toml`
Use `gen_retail(0)` to generate a retail key as a String
or `gen_oem(0)` to generate a OEM key as a String
or `gen_array(0, zeroed_digits_at_start)` to generates a array of 7 digits where the sum of the digits is divisible by 7 with`zeroed_digits_at_start` digits of the start of the array that are 0

## Installation
  To install run `cargo install lose95-keygen-rs`
  If that fails try `cargo install --locked lose95-keygen-rs`
  If that fails make sure you have cargo installed by running `cargo --version` and that your rust version is >= `1.85.1` by running `rustc -Vv`
  If you install rust using packages provided by your distribution, use `rustup` instead to get the latest version of rust.
## Radicle and Github
  This project is maintained on [radicle](https://radicle.xyz/) in [`rad:z2ADKLfsXMdsFt7vQq6qQDnbPD9b8`](https://app.radicle.xyz/nodes/seed.radicle.garden/rad%3Az2ADKLfsXMdsFt7vQq6qQDnbPD9b8).
  If you do not want to install radicle but want to create a issue you can find [`lose95-keygen-rs` on github](https://github.com/Velocifyer/lose95-keygen-rs)

## Self Promotion
  If you use cargo-crev, you should add `s67f2b5NTRWJYdgpElWYvNNIXZPLVFK0aTP3IcMo5ck` `https://ash.radicle.garden/zZbBDS6L52The3V6RBqfGtxgKkGa.git`
