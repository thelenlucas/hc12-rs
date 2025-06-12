# `hc12-rs`

A builder-syntax programmer for `hc01` `HC-12` radio modules.

## Motivation

The `HC-12` is a fantastic small radio module used for embedded applications,
typically used in hobbyist applications, as it does not require licensing in
most jurisdictions. However, poor documentation and an unconventional parameter
and settings system make it difficult to work with on occasion. This crate provides
methods for programming and reconfiguring the module, without needing to touch serial
devices manually every time.

The best-translated English manual for the module can be found [here](https://github.com/robert-rozee/HC-12-user-manual---reformatted/blob/master/HC-12%20v2.3C.pdf).

## Features

- Re/Program `HC-12` modules dynamically with error handling
- Statically typed builder prevents invalid states
- Use any [embedded-hal](https://crates.io/crates/embedded-hal)/[embedded-io](https://crates.io/crates/embedded-io)-capable board
- `no-std` and `no-alloc` capable

## Usage

```rust

let mut hc12 = HC12::new(serial, programming_pin, &mut delay)
  .unwrap()
  .channel(Channel::new(15).unwrap())
  .power(Power::P8)
  .b4800
  .fu3()
  .program(&mut timer_two)
  .unwrap()
  .into_transparent_mode()
  .unwrap();

hc12.write_all("Hello world!".as_bytes()).ok();

let mut hc12_low_power = hc12.into_programming_mode()
  .fu1()
  .unwrap()
  .into_transparent_mode()
  .unwrap();

hc12_low_power.write_all(b"Hello from the low power mode!").ok();
```

## To-Dos

- Interrogation of underlying module parameters
- Unsafe no-assumptions interface
- Async feature
