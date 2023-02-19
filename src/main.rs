// Name: ingenuity-dl
// Author: Parker Lamb
// Date: 2.18.23
// Description: Main control file for ingenutiy-dl

extern crate reqwest;
use std::process::ExitCode;
use anyhow::Result;

mod input;
mod functions;

fn main() -> Result<()> {
    // 1. Get application CLI arguments
    let arguments = input::get_args();

    // 2. Retrieve a list of images from the net for the given Sol
    let json = functions::get_json(&arguments)?;

    // 3. Download images to a temporary directory

    // 4. Compile images into video 

    // 5. Delete images afterwards

    Ok(())
}
