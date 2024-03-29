# alienware-cli

 [![Crates.io](https://img.shields.io/crates/l/alienware_cli)](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/alienware_cli)](https://crates.io/crates/alienware_cli) [![Build Status](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml/badge.svg)](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml)  [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/alienware-wmi/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/alienware-wmi) [![snapcraft.io](https://snapcraft.io/alienware-cli/badge.svg)](https://snapcraft.io/alienware-cli)

The `alienware-cli` application controls the LEDs and the HDMI in and out ports on an Alienware Alpha desktop machine.
The API is based on the `alienware_wmi_control.sh` script that used to come with the SteamOS distribution of Linux for
Alienware machines.

You might also want to check out a python project to control the same lights
[`AlienFX`](https://github.com/trackmastersteve/alienfx).

## Install

First method: if cargo is available to your root user with a correct toolchain:

```bash
sudo cargo install alienware_cli --root /usr/local
```

Second method: install for yourself using cargo and then copy to a location that is in all users path:

```bash
cargo install alienware-cli
sudo cp ~/.cargo/bin/alienware-cli /usr/local/bin
```

Third method: grab the latest version from the [releases](https://github.com/a1ecbr0wn/alienware-wmi/releases) page:

```bash
curl -OL https://github.com/a1ecbr0wn/alienware-wmi/releases/download/latest/alienware-cli && chmod 775 alienware-cli
```

## Use

To see a description of the command line parameters use the `-h` parameter:

``` bash
$ alienware-cli -h
Command Line app to control the lights on an Alienware Alpha R1/R2

Usage: alienware-cli [OPTIONS]

Options:
  -c, --connector      State of the HDMI ports
  -l, --led-state      State of the LEDs
  -H, --head <HEAD>    Set the LED state of the head button
  -L, --left <LEFT>    Set the LED state of the left LEDs
  -R, --right <RIGHT>  Set the LED state of the right LEDs
  -j, --json           Output in JSON format for machine readability (combined with -c or -l)
  -V, --version        Print version information
  -h, --help           Print help
```

The `-c` and `-l` parameters show information about the hdmi connections and LEDs respectively, this two parameters can
be used together or separately and can also be used with the `-j` parameter which formats the response in json format
for machine readability:

```bash
$ alienware-cli -lc
HDMI passthrough state: present
    Input HDMI is unconnected
    Output HDMI is connected to gpu

LED state: present
    head:
        red: 15
        green: 0
        blue: 15
    left:
        red: 0
        green: 15
        blue: 15
```

```bash
$ alienware-cli -jlc
{"hdmi":{"hdmi":{"exists":true,"input":"unconnected","output":"gpu"}},"leds":{"exists":true,"left":{"red":0,"green":15,"blue":15},"head":{"red":15,"green":0,"blue":15}}}
```

The colour of the LEDs can be set for different LED clusters with the `-H`, `-L`, and `-R` parameters for the head, left
and right clusters respectively.  The LED colours can be set either with the name of the colour or the RGB value where
the value of each colour has a 0-15 scale.  Since these commands are changing the values of files in sysfs, the command
should be run with root permissions:

Both of the following examples sets the head button to cyan:

```bash
sudo alienware-cli -H cyan
```

```bash
sudo alienware-cli -H "0 15 15"
```

## Disclaimer and License

If you use this software, you use it AT YOUR OWN RISK.

This software is licenced under the [Apache-2.0](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE) licence.
