mod cli;

use alienware::{Alienware, Zone};
use clap::Parser;
use jzon::object;
use regex::Regex;
use snapcraft::in_snap;
use std::io::ErrorKind;
use std::process::{exit, Command};
use std::sync::OnceLock;

fn main() {
    static DESCRIPTION: OnceLock<String> = OnceLock::new();
    let desc = DESCRIPTION.get_or_init(|| {
        format!(
            "alienware-cli v{}: Command Line app to control the lights on an Alienware Alpha R1/R2",
            env!("CARGO_PKG_VERSION")
        )
    });

    let options = cli::Options::parse();

    if options.version {
        println!("{}", desc.as_str());
        exit(0);
    }

    let aw = Alienware::new();
    let mut json_data = object! {};

    if options.connector {
        let hdmi = aw.get_hdmi();
        if let Ok(hdmi) = hdmi {
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
        } else if let Err(x) = hdmi {
            match x.kind() {
                ErrorKind::PermissionDenied => {
                    println!("You do not have permission to run this command (do you need sudo?)");
                    check_snap();
                }
                _ => {
                    println!("Problem getting HDMI state {:?} ", x.kind());
                }
            }
        }
    }

    if options.led_state {
        let leds = aw.get_rgb_zones();
        if let Ok(leds) = leds {
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
        } else if let Err(x) = leds {
            match x.kind() {
                ErrorKind::PermissionDenied => {
                    println!("You do not have permission to run this command (do you need sudo?)");
                    check_snap();
                }
                _ => {
                    println!("Problem getting LED state {:?} ", x.kind());
                }
            }
        }
    }

    if options.json {
        println!("{}", json_data.dump());
    }

    if let Some(head) = options.head {
        set_led_zone_rgb(&aw, Zone::Head, head);
    }

    if let Some(left) = options.left {
        set_led_zone_rgb(&aw, Zone::Left, left);
    }

    if let Some(right) = options.right {
        set_led_zone_rgb(&aw, Zone::Right, right);
    }
}

/// Set the chosen Zone to the specified RGB
fn set_led_zone_rgb(aw: &Alienware, zone: Zone, input: String) {
    let leds = aw.get_rgb_zones();
    if let Ok(leds) = leds {
        if leds.exists {
            if leds.zones.contains_key(&zone) {
                let (r, g, b) = parse_rgb_string(input.as_str());
                match aw.set_rgb_zone(zone, r, g, b) {
                    Ok(_) => {}
                    Err(x) => match x.kind() {
                        ErrorKind::PermissionDenied => {
                            println!("You do not have permission to run this command (do you need sudo?)");
                            check_snap();
                        }
                        _ => {
                            println!("Problem setting RGB value {:?} ", x.kind());
                        }
                    },
                };
            } else {
                println!("There are no {zone} LEDs");
            }
        } else {
            println!("There is no alienware LED unit on this machine");
        }
    } else if let Err(x) = leds {
        match x.kind() {
            ErrorKind::PermissionDenied => {
                println!("You do not have permission to run this command (do you need sudo?)");
                check_snap();
            }
            _ => {
                println!("Problem setting RGB value {:?} ", x.kind());
            }
        }
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
            static RE: OnceLock<Regex> = OnceLock::new();
            let re = RE.get_or_init(|| Regex::new(r"(\d+) (\d+) (\d+)").unwrap());
            match re.captures(input.as_str()) {
                Some(caps) => {
                    if caps.len() == 4 {
                        let red = &caps[1];
                        let green = &caps[2];
                        let blue = &caps[3];
                        (
                            red.parse::<u8>().unwrap(),
                            green.parse::<u8>().unwrap(),
                            blue.parse::<u8>().unwrap(),
                        )
                    } else {
                        (0u8, 0u8, 15u8) // setting blue as the default
                    }
                }
                _ => (0u8, 0u8, 15u8), // setting blue as the default
            }
        }
    }
}

fn check_snap() {
    if in_snap() {
        let is_snap_connected = {
            let snap_connected_status = Command::new("snapctl")
                .arg("is-connected")
                .arg("alienware")
                .status()
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to check whether snap is able to read /sys/devices/platform/alienware-wmi"
                    )
                });
            snap_connected_status.success()
        };
        if is_snap_connected {
            println!("This may be because you have installed alienware-cli from snap, which prevents automatic setup.\n");
            print!("The snap container initially blocks access to the alienware device setup that is needed to carry out this action.  ");
            println!("The following command can be run to unblock access to the alienware device and then you can try again:\n");
            println!("    sudo snap connect alienware-wmi:alienware\n \n",);
        }
    }
}
