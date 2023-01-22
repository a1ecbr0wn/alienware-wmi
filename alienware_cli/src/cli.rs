use clap::Parser;

/// Struct containing the parsed command line arguments
#[derive(Parser)]
#[command(name = "alienware-cli")]
#[command(bin_name = "awc")]
#[command(author, version, about, long_about = None, arg_required_else_help(true), disable_version_flag(true))]
pub struct Options {
    /// State of the HDMI ports
    #[arg(short, long, value_parser, default_value_t = false)]
    pub connector: bool,

    /// State of the LEDs
    #[arg(short, long, value_parser, default_value_t = false)]
    pub led_state: bool,

    /// Set the LED state of the head button
    #[arg(short = 'H', long, value_parser)]
    pub head: Option<String>,

    /// Set the LED state of the left LEDs
    #[arg(short = 'L', long, value_parser)]
    pub left: Option<String>,

    /// Set the LED state of the right LEDs
    #[arg(short = 'R', long, value_parser)]
    pub right: Option<String>,

    /// Output in JSON format for machine readability (combined with -c or -l)
    #[arg(short, long, value_parser, default_value_t = false)]
    pub json: bool,

    /// Print version information
    #[arg(short = 'V', long, value_parser, display_order(9))]
    pub version: bool,
}
