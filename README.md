# alienware-wmi

[![Crates.io](https://img.shields.io/crates/l/alienware)](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/alienware)](https://crates.io/crates/alienware) [![Build Status](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml/badge.svg)](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml) [![docs.rs](https://img.shields.io/docsrs/alienware)](https://docs.rs/alienware)

Repository for tools that approximate to the `alienware_wmi_control.sh` script that used to come with the SteamOS
distribution of Linux for Alienware Alpha desktop machine.  The primary crate is the
[`alienware`](https://github.com/a1ecbr0wn/alienware-wmi/tree/main/alienware) crate which provides the API access to the alienware sysfs access to the LED lights and the HDMI input/output control. The
[`alienware-cli`](https://github.com/a1ecbr0wn/alienware-wmi/tree/main/alienware_cli) crate uses the API to provide
command line access to some of the API functionality,

You might also want to check out a python project to control the same lights
[`AlienFX`](https://github.com/trackmastersteve/alienfx).

## Disclaimer and License

If you use this software, you use it AT YOUR OWN RISK.

This software is licenced under the [Apache-2.0](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE) licence.
