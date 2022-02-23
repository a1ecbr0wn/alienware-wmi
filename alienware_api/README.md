# alienware-api

[![Build Status](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml/badge.svg)](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml)

The `alienware-api` crate provides a Rust API access to the Linux sysfs platform
api for control of the lights on an Alienware Alpha desktop machine.  The API
is based on the `alienware_wmi_control.sh` script that used to come with the SteamOS
distribution of Linux for Alienware machines.

You might also want to check out a python project to control the same lights
[`AlienFX`](https://github.com/trackmastersteve/alienfx).

## Install/Use

To use `alienware-api`, first add this to your `Cargo.toml`:

```toml
[dependencies]
alienware_api = "0.1.0"
```

Then, add this to your crate root:

```rust
use alienware_api;
```
