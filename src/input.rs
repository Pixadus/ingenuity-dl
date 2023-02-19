// Description: functions to deal with file inputs and CLI arguments.

use clap::{App, Arg, ArgMatches, SubCommand};

// Get all CLI arguments, if they exist. 
pub fn get_args() -> ArgMatches<'static> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about(env!("CARGO_PKG_DESCRIPTION"))
                .args_from_usage(
                    "-d, --sol=[SOL]     'Retrieve images from a specific Sol (defaults to latest)'
                    -s, --save           'Save images instead of deleting them'")
                .get_matches();
    matches
}