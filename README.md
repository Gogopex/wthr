# wthr
A simple Rust command line that displays weather information for a given city. 

## About
A small project to mess around with Rust, Serde, and StructOpt. It doesn't do much, and isn't very well made.


## Installation
I'm not sure this deserves to be uploaded as a crate, so for now you can:
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

## Help
![Output of wthr --help](https://i.imgur.com/OzmW6kI.png)
