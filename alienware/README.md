# alienware

[![Crates.io](https://img.shields.io/crates/l/alienware)](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/alienware)](https://crates.io/crates/alienware) [![Build Status](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml/badge.svg)](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml) [![docs.rs](https://img.shields.io/docsrs/alienware)](https://docs.rs/alienware)

The `alienware` crate provides a Rust API access to the Linux sysfs platform api for control of the lights on an
Alienware Alpha desktop machine.  The API is based on the `alienware_wmi_control.sh` script that used to come with the
SteamOS distribution of Linux for Alienware machines.

You might also want to check out a python project to control the same lights
[`AlienFX`](https://github.com/trackmastersteve/alienfx).

## Install/Use

To use the `alienware` api, first add this to your `Cargo.toml`:

```toml
[dependencies]
alienware = "1.0.1"
```

and then, add this to your crate root:

```rust
use alienware;
```

## Disclaimer and License

If you use this software, you use it AT YOUR OWN RISK.

This software is licenced under the [Apache-2.0](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE) licence.
