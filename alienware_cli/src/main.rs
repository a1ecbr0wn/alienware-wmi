use alienware::{Alienware, Zone};
use argparse::{ArgumentParser, Store, StoreTrue};
use json::object;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::ErrorKind;
use std::process::exit;

/// Struct containing the parsed command line arguments
struct Options {
    show_version: bool,
    json: bool,
    hdmi_connector: bool,
    led_state: bool,
    head: String,
    left: String,
    right: String,
}

// Parse the command line arguments
fn parse_arguments(description: &str, options: &mut Options) {
    let mut parser = ArgumentParser::new();
    parser.set_description(description);
    parser.refer(&mut options.show_version).add_option(
        &["-v", "--version"],
        StoreTrue,
        "Get version info",
    );
    parser.refer(&mut options.json).add_option(
        &["-j", "--json"],
        StoreTrue,
        "Output in JSON format for machine readability",
    );
    parser.refer(&mut options.hdmi_connector).add_option(
        &["-c", "--connector"],
        StoreTrue,
        "State of the HDMI ports",
    );
    parser.refer(&mut options.led_state).add_option(
        &["-l", "--led-state"],
        StoreTrue,
        "State of the LEDs",
    );
    parser.refer(&mut options.head).add_option(
        &["-H", "--head"],
        Store,
        "Set the LED state of the head button",
    );
    parser.refer(&mut options.left).add_option(
        &["-L", "--left"],
        Store,
        "Set the LED state of the left LEDs",
    );
    parser.refer(&mut options.right).add_option(
        &["-R", "--right"],
        Store,
        "Set the LED state of the right LEDs",
    );
    match parser.parse_args() {
        Ok(()) => {}
        Err(x) => {
            exit(x);
        }
    }
}

fn main() {
    let mut options = Options {
        show_version: false,
        json: false,
        hdmi_connector: false,
        led_state: false,
        head: "".to_string(),
        left: "".to_string(),
        right: "".to_string(),
    };
    lazy_static! {
        static ref DESCRIPTION: String = format!(
            "awc v{}: Command Line app to control the lights on an Alienware Alpha R1/R2",
            env!("CARGO_PKG_VERSION")
        );
    }

    parse_arguments(DESCRIPTION.as_str(), &mut options);

    if options.show_version {
        println!("{}", DESCRIPTION.as_str());
        exit(0);
    }

    let aw = Alienware::new();
    let mut json_data = object! {};

    if options.hdmi_connector {
        let hdmi = aw.get_hdmi();
        if options.json {
            let hdmi_data = object! {
                "hdmi": {
                    "exists": hdmi.exists,
                    "input": format!( "{}", hdmi.cable_state ),
                    "output": format!( "{}", hdmi.source ),
                }
            };
            json_data.insert("hdmi", hdmi_data).unwrap();
        } else {
            print!("HDMI passthrough state: ");
            if hdmi.exists {
                println!("present");
                println!("    Input HDMI is {}", hdmi.cable_state);
                println!("    Output HDMI is connected to {}", hdmi.source);
            } else {
                println!("not present");
            }
            println!();
        }
    }

    if options.led_state {
        let leds = aw.get_rgb_zones();
        if options.json {
            let mut leds_data = object! {
                "exists": leds.exists,
            };
            for zone in leds.zones.values() {
                let zone_data = object! {
                    "red": zone.red,
                    "green": zone.green,
                    "blue": zone.blue,
                };
                leds_data
                    .insert(format!("{}", zone.zone).as_str(), zone_data)
                    .unwrap();
            }
            json_data.insert("leds", leds_data).unwrap();
        } else {
            print!("LED state: ");
            if leds.exists {
                println!("present");
                for zone in leds.zones.values() {
                    println!("    {}:", zone.zone);
                    println!("        red: {}", zone.red);
                    println!("        green: {}", zone.green);
                    println!("        blue: {}", zone.blue);
                }
            } else {
                println!("not present");
            }
            println!();
        }
    }

    if options.json {
        println!("{}", json_data.dump());
    }

    if !options.head.is_empty() {
        set_led_zone_rgb(&aw, Zone::Head, options.head);
    }

    if !options.left.is_empty() {
        set_led_zone_rgb(&aw, Zone::Left, options.left);
    }

    if !options.right.is_empty() {
        set_led_zone_rgb(&aw, Zone::Right, options.right);
    }
}

/// Set the chosen Zone to the specified RGB
fn set_led_zone_rgb(aw: &Alienware, zone: Zone, input: String) {
    let leds = aw.get_rgb_zones();
    if leds.exists {
        if leds.zones.contains_key(&zone) {
            let (r, g, b) = parse_rgb_string(input.as_str());
            match aw.set_rgb_zone(zone, r, g, b) {
                Ok(_) => {}
                Err(error) => {
                    match error.kind() {
                        ErrorKind::PermissionDenied => {
                            println!("You do not have permission to run this command (do you need sudo?)")
                        }
                        _ => {
                            println!("Problem setting RGB value {:?} ", error.kind())
                        }
                    }
                }
            };
        } else {
            println!("There are no {} LEDs", zone);
        }
    } else {
        println!("There is no alienware LED unit on this machine");
    }
}

/// Parse the RGB value of the input string, either a named colour, or an rgb value
fn parse_rgb_string(input: &str) -> (u8, u8, u8) {
    let input = input.to_lowercase();
    match input.as_str() {
        "black" => (0u8, 0u8, 0u8),
        "white" => (15u8, 15u8, 15u8),
        "red" => (15u8, 0u8, 0u8),
        "yellow" => (15u8, 15u8, 0u8),
        "green" => (0u8, 15u8, 0u8),
        "cyan" => (0u8, 15u8, 15u8),
        "blue" => (0u8, 0u8, 15u8),
        "magenta" => (15u8, 0u8, 15u8),
        _ => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
            }
            match RE.captures(input.as_str()) {
                Some(caps) => {
                    if caps.len() == 4 {
                        let red = caps.get(1).unwrap().as_str().to_string();
                        let green = caps.get(2).unwrap().as_str().to_string();
                        let blue = caps.get(3).unwrap().as_str().to_string();
                        (
                            red.parse::<u8>().unwrap(),
                            green.parse::<u8>().unwrap(),
                            blue.parse::<u8>().unwrap(),
                        )
                    } else {
                        (0u8, 0u8, 15u8) // setting blue as the default
                    }
                }
                None => (0u8, 0u8, 15u8), // setting blue as the default
            }
        }
    }
}
