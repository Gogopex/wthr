![Rust](https://github.com/Gogopex/wthr/workflows/Rust/badge.svg?branch=master&event=push)

# wthr
A simple Rust command line that displays weather information for a given city. 

## Installation
I'm not sure this deserves to be uploaded as a crate or as a package somewhere, so for now you can:
```
git clone https://github.com/Gogopex/wthr.git
cd wthr
# wthr requires cargo/rustc
cargo build --release
```
This will generate a bin file in `target/release/build`

## Usage
If you've just ran `cargo build --release`, replace `wthr` by `./target/release/wthr` in the following examples.
```
# use wthr <city> to print the weather of a city
$ wthr "new york"
$ Clear Sky - 😎  -- Current temperature is 29°C. Feels like 33°C!

# you can specify the units with --unit
$ wthr philadelphia -u=imperial
$ Broken Clouds - 🌥  -- Current temperature is 83°F. Feels like 90°F!
```

## Demo
![Usage of wthr](https://i.imgur.com/lTntAkb.gif)
