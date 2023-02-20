// Name: ingenuity-dl
// Author: Parker Lamb
// Date: 2.18.23
// Description: Main control file for ingenutiy-dl

use anyhow::Result;

mod input;
mod functions;

fn main() -> Result<()> {
    // 1. Get application CLI arguments
    let arguments = input::get_args();

    // 2. Retrieve a list of images from https://mars.nasa.gov/ for the given sol
    let image_list = functions::get_images(&arguments)?;

    // 3. Download images to a temporary directory
    let images = functions::download_images(image_list)?;

    // 4. Compile images into a GIF 
    functions::compile_gif(images, &arguments)?;

    Ok(())
}
