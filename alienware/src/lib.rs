use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// The possible sources of the HDMI output port
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDMISource {
    Cable,
    Gpu,
    Unknown,
}

impl fmt::Display for HDMISource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HDMISource::Cable => {
                write!(f, "cable")
            }
            HDMISource::Gpu => {
                write!(f, "gpu")
            }
            _ => {
                write!(f, "unknown")
            }
        }
    }
}

/// The possible states of the Input HDMI port
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDMICableState {
    Connected,
    Unconnected,
    Unknown,
}

impl fmt::Display for HDMICableState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HDMICableState::Connected => {
                write!(f, "connected")
            }
            HDMICableState::Unconnected => {
                write!(f, "unconnected")
            }
            _ => {
                write!(f, "unknown")
            }
        }
    }
}

/// Enumeration of possible LEDs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Zone {
    Head,
    Left,
    Right,
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Zone::Head => {
                write!(f, "head")
            }
            Zone::Left => {
                write!(f, "left")
            }
            Zone::Right => {
                write!(f, "right")
            }
        }
    }
}

/// State of the HDMI ports
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDMI {
    pub source: HDMISource,
    pub cable_state: HDMICableState,
    pub exists: bool,
}

impl Default for HDMI {
    fn default() -> Self {
        Self {
            source: HDMISource::Unknown,
            cable_state: HDMICableState::Unknown,
            exists: false,
        }
    }
}

/// Setup of a particular LED
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RGBZone {
    pub zone: Zone,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Setup of all of the LEDs
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct RGBZones {
    pub zones: HashMap<Zone, RGBZone>,
    pub exists: bool,
}

/// Access to the settings for a Alienware server
pub struct Alienware {
    platform: String,
}

impl Default for Alienware {
    fn default() -> Self {
        Self::new()
    }
}

impl Alienware {
    /// Construct a new instance of Alienware
    pub fn new() -> Alienware {
        Alienware {
            platform: "/sys/devices/platform/alienware-wmi".to_string(),
        }
    }

    /// Construct a new instance of Alienware used for testing that can change the root of the sysfs files
    #[allow(dead_code)]
    fn test(platform: String) -> Alienware {
        Alienware { platform }
    }

    /// Check that this is an Alienware server (i.e. has the alienware platform settings in sysfs)
    pub fn is_alienware(&self) -> bool {
        Path::new(&self.platform).exists()
    }

    /// Get the state of the HDMI ports
    pub fn get_hdmi(&self) -> std::io::Result<HDMI> {
        let mut source = HDMISource::Unknown;
        let mut cable_state = HDMICableState::Unknown;
        let mut exists = false;
        if self.is_alienware() {
            exists = true;
            let mut path_buf = PathBuf::new();
            path_buf.push(&self.platform);
            path_buf.push("hdmi");

            if path_buf.exists() {
                source = self.parse_source()?;
                cable_state = self.parse_cable_state()?;
            }
        }
        Ok(HDMI {
            source,
            cable_state,
            exists,
        })
    }

    /// Parse the state of the HDMI Output source
    fn parse_source(&self) -> std::io::Result<HDMISource> {
        match self.parse_sys_file("hdmi/source") {
            Ok(Some(s)) => {
                if s.eq("cable") {
                    Ok(HDMISource::Cable)
                } else if s.eq("gpu") {
                    Ok(HDMISource::Gpu)
                } else {
                    Ok(HDMISource::Unknown)
                }
            }
            Ok(None) => Ok(HDMISource::Unknown),
            Err(x) => Err(x),
        }
    }

    /// Parse the state of the HDMI input cable
    fn parse_cable_state(&self) -> std::io::Result<HDMICableState> {
        match self.parse_sys_file("hdmi/cable") {
            Ok(Some(s)) => {
                if s.eq("connected") {
                    Ok(HDMICableState::Connected)
                } else if s.eq("unconnected") {
                    Ok(HDMICableState::Unconnected)
                } else {
                    Ok(HDMICableState::Unknown)
                }
            }
            Ok(None) => Ok(HDMICableState::Unknown),
            Err(x) => Err(x),
        }
    }

    /// Set the source for the HDMI Output port
    pub fn set_hdmi_source(self, source: HDMISource) -> std::io::Result<()> {
        self.write_sys_file(
            "hdmi/source",
            match source {
                HDMISource::Cable => "cable",
                HDMISource::Gpu => "gpu",
                HDMISource::Unknown => "unknown",
            },
        )?;
        Ok(())
    }

    /// Get the state of the various LEDs
    pub fn get_rgb_zones(&self) -> std::io::Result<RGBZones> {
        let mut zones = HashMap::new();
        let mut exists = false;
        if self.is_alienware() {
            exists = true;
            let mut path_buf = PathBuf::new();
            path_buf.push(&self.platform);
            path_buf.push("rgb_zones");
            if path_buf.exists() {
                path_buf.push("zone00");
                if path_buf.exists() {
                    zones.insert(
                        Zone::Head,
                        self.parse_rgb_zone(Zone::Head, "rgb_zones/zone00")?,
                    );
                }

                path_buf.pop();
                path_buf.push("zone01");
                if path_buf.exists() {
                    zones.insert(
                        Zone::Left,
                        self.parse_rgb_zone(Zone::Left, "rgb_zones/zone01")?,
                    );
                }

                path_buf.pop();
                path_buf.push("zone02");
                if path_buf.exists() {
                    zones.insert(
                        Zone::Right,
                        self.parse_rgb_zone(Zone::Right, "rgb_zones/zone02")?,
                    );
                }
            }
        }
        Ok(RGBZones { zones, exists })
    }

    /// Set an LED colour
    pub fn set_rgb_zone(&self, zone: Zone, red: u8, green: u8, blue: u8) -> std::io::Result<()> {
        let rgb = format!("{red:02x}{green:02x}{blue:02x}");
        self.write_sys_file(
            match zone {
                Zone::Head => "rgb_zones/zone00",
                Zone::Left => "rgb_zones/zone01",
                Zone::Right => "rgb_zones/zone02",
            },
            rgb.as_str(),
        )?;
        Ok(())
    }

    /// Parse the current colour of an LED
    fn parse_rgb_zone(&self, zone: Zone, file_name: &str) -> std::io::Result<RGBZone> {
        match self.parse_sys_rgb_file(file_name) {
            Ok((red, green, blue)) => Ok(RGBZone {
                zone,
                red,
                green,
                blue,
            }),
            Err(x) => Err(x),
        }
    }

    /// Checks whether the alienware HDMI setup is available
    pub fn has_hdmi(self) -> bool {
        if let Ok(hdmi) = self.get_hdmi() {
            hdmi.exists
        } else {
            false
        }
    }

    /// Checks whether the alienware LED setup is available
    pub fn has_rgb_zones(self) -> bool {
        if let Ok(rgb_zones) = self.get_rgb_zones() {
            rgb_zones.exists
        } else {
            false
        }
    }

    /// Parses a single setting sysfs file
    fn parse_sys_file(&self, file_name: &str) -> std::io::Result<Option<String>> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"\[([^)]+)\]").unwrap());
        let mut path_buf = PathBuf::new();
        path_buf.push(&self.platform);
        path_buf.push(file_name);
        let mut file = File::open(path_buf.as_path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let caps = re.captures(contents.as_str()).unwrap();
        match caps.len() > 0 {
            true => Ok(Some(caps[1].to_string())),
            false => Ok(None),
        }
    }

    /// Parses a sysfs file that holds an RGB setting
    fn parse_sys_rgb_file(&self, file_name: &str) -> std::io::Result<(u8, u8, u8)> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"^red: (\d+), green: (\d+), blue: (\d+)").unwrap());
        let mut path_buf = PathBuf::new();
        path_buf.push(&self.platform);
        path_buf.push(file_name);
        let mut file = File::open(path_buf)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        match re.captures(contents.as_str()) {
            Some(caps) if caps.len() == 4 => {
                let red = &caps[1];
                let green = &caps[2];
                let blue = &caps[3];
                Ok((
                    red.parse::<u8>().unwrap(),
                    green.parse::<u8>().unwrap(),
                    blue.parse::<u8>().unwrap(),
                ))
            }
            _ => Ok((0u8, 0u8, 0u8)),
        }
    }

    /// Write a value to a sysfs file
    fn write_sys_file(&self, file_name: &str, value: &str) -> std::io::Result<()> {
        let mut path_buf = PathBuf::new();
        path_buf.push(&self.platform);
        path_buf.push(file_name);
        let mut sys_file = File::create(path_buf)?;
        sys_file.write_all(value.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{HDMISource, Zone};
    use std::fs::{create_dir_all, metadata, remove_dir_all, File};
    use std::io::prelude::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn is_alienware() {
        let alienware = crate::Alienware::test(setup_aw("is_alienware"));
        let rtn = alienware.is_alienware();
        assert!(rtn);
    }

    #[test]
    fn is_not_alienware() {
        let alienware = crate::Alienware::test(setup_not_aw("is_not_alienware"));
        let rtn = alienware.is_alienware();
        assert!(!rtn);
    }

    #[test]
    fn has_rgb_zones() {
        let alienware = crate::Alienware::test(setup_aw("has_rgb_zones"));
        let rtn = alienware.has_rgb_zones();
        assert!(rtn);
    }

    #[test]
    fn get_rgb_zones() {
        let alienware = crate::Alienware::test(setup_aw("get_rgb_zones"));
        let rgbzone = alienware.get_rgb_zones();
        assert!(rgbzone.is_ok());
        if let Ok(rgbzone) = rgbzone {
            assert_eq!(rgbzone.zones.len(), 3);
            let head = rgbzone.zones.get(&crate::Zone::Head).unwrap();
            assert_eq!(head.zone, crate::Zone::Head);
            assert_eq!(head.red, 0u8);
            assert_eq!(head.green, 0u8);
            assert_eq!(head.blue, 15u8);
            let left = rgbzone.zones.get(&crate::Zone::Left).unwrap();
            assert_eq!(left.zone, crate::Zone::Left);
            assert_eq!(left.red, 0u8);
            assert_eq!(left.green, 15u8);
            assert_eq!(left.blue, 0u8);
            let right = rgbzone.zones.get(&crate::Zone::Right).unwrap();
            assert_eq!(right.zone, crate::Zone::Right);
            assert_eq!(right.red, 15u8);
            assert_eq!(right.green, 0u8);
            assert_eq!(right.blue, 0u8);
        }
    }

    #[test]
    fn set_rgb_zones() {
        let alienware = crate::Alienware::test(setup_aw("set_rgb_zones"));
        match alienware.set_rgb_zone(Zone::Left, 15, 7, 0) {
            Err(_) => {
                panic!("Failed to set the RGB Zone");
            }
            Ok(()) => {
                let path = Path::new(
                    "/tmp/alienware_wmi_test/set_rgb_zones/alienware-wmi/rgb_zones/zone01",
                );
                assert!(path.exists());
                let mut file = File::open(path).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                assert_eq!("0f0700", contents);
            }
        }
    }

    #[test]
    fn has_hdmi() {
        let alienware = crate::Alienware::test(setup_aw("has_hdmi"));
        let rtn = alienware.has_hdmi();
        assert!(rtn);
    }

    #[test]
    fn get_hdmi() {
        let alienware = crate::Alienware::test(setup_aw("get_hdmi"));
        let hdmi = alienware.get_hdmi();
        assert!(hdmi.is_ok());
        if let Ok(hdmi) = hdmi {
            assert!(hdmi.exists);
            assert_eq!(hdmi.source, crate::HDMISource::Gpu);
            assert_eq!(hdmi.cable_state, crate::HDMICableState::Connected);
        }
    }

    #[test]
    fn set_hdmi_source() {
        let alienware = crate::Alienware::test(setup_aw("set_hdmi_source"));
        match alienware.set_hdmi_source(HDMISource::Cable) {
            Err(_) => {
                panic!("Failed to set the HDMI Source");
            }
            Ok(()) => {
                let path = "/tmp/alienware_wmi_test/set_hdmi_source/hdmi/source";
                if metadata(path).is_ok() {
                    let mut file = File::open(path).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    assert_eq!("cable", contents);
                }
            }
        }
    }

    const TEST_PATH: &str = "/tmp/alienware_wmi_test";

    fn setup_not_aw(test: &str) -> String {
        let mut path_buf = PathBuf::new();
        path_buf.push(TEST_PATH);
        path_buf.push(test);
        if path_buf.exists() && remove_dir_all(path_buf.as_path()).is_err() {
            panic!("Failed to remove test path while setting up not_aw scenario")
        }
        if create_dir_all(path_buf.as_path()).is_err() {
            panic!("Failed to setup test path while setting up not_aw scenario")
        };

        path_buf.push("alienware-wmi");
        let platform = path_buf.as_os_str().to_str().unwrap().to_string();
        platform
    }

    fn setup_aw(test: &str) -> String {
        let mut path_buf = PathBuf::new();
        path_buf.push(TEST_PATH);
        path_buf.push(test);
        if metadata(path_buf.as_path()).is_ok() && remove_dir_all(path_buf.as_path()).is_err() {
            panic!("Failed to remove test path while setting up aw scenario")
        }
        path_buf.push("alienware-wmi");
        if create_dir_all(path_buf.as_path()).is_err() {
            panic!("Failed to setup test path while setting up aw scenario")
        };
        // hdmi mux
        path_buf.push("hdmi");
        if create_dir_all(path_buf.as_path()).is_err() {
            panic!("Failed to setup hdmi while setting up aw scenario")
        };

        // cable file
        path_buf.push("cable");
        let mut file = File::create(path_buf.as_path()).unwrap();
        file.write_all(b"unconnected [connected] unknown").unwrap();
        path_buf.pop();

        // source file
        path_buf.push("source");
        let mut file = File::create(path_buf.as_path()).unwrap();
        file.write_all(b"cable [gpu] unknown,").unwrap();
        path_buf.pop();

        path_buf.pop();
        // rgb_zones
        path_buf.push("rgb_zones");
        if create_dir_all(path_buf.as_path()).is_err() {
            panic!("Failed to setup rgb_zones while setting up aw scenario")
        };

        // zone00
        path_buf.push("zone00");
        let mut file = File::create(path_buf.as_path()).unwrap();
        file.write_all(b"red: 0, green: 0, blue: 15").unwrap();
        path_buf.pop();

        // zone01
        path_buf.push("zone01");
        let mut file = File::create(path_buf.as_path()).unwrap();
        file.write_all(b"red: 0, green: 15, blue: 0").unwrap();
        path_buf.pop();

        // zone02
        path_buf.push("zone02");
        let mut file = File::create(path_buf.as_path()).unwrap();
        file.write_all(b"red: 15, green: 0, blue: 0").unwrap();
        path_buf.pop();

        path_buf.pop();

        let platform = path_buf.as_os_str().to_str().unwrap().to_string();
        platform
    }
}
