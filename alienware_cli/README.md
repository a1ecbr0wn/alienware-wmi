# alienware-wmi

[![Build Status](https://github.com/a1ecbr0wn/alienware-wmi/workflows/CI/badge.svg)](https://github.com/a1ecbr0wn/alienware-wmi/actions)

The `alienware-cli` crate uses the `alienware-api` Rust API crate to provide CLI access
to the Linux sysfs platform api for control of the lights on an Alienware Alpha desktop
machine.  The API is based on the `alienware_wmi_control.sh` script that used to come
with the SteamOS distribution of Linux for Alienware machines.

You might also want to check out a python project to control the same lights
[`AlienFX`](https://github.com/trackmastersteve/alienfx).

## Install/Use

To use `alienware-cli`, first add this to your `Cargo.toml`:

```bash
cargo install alienware-cli
```
