// Description: public functions

use clap::ArgMatches;
use serde_json;
use console::style;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use anyhow::{anyhow, Result};
use std::io::Write;
use bytes::Bytes;
use std::fs::File;
use std::io::Cursor;
use image::{DynamicImage, GenericImageView};
use image::io::Reader;
use imgref::{Img, ImgVec};
use rgb::RGBA8;
use std::thread;

/// Downloads the JSON document for a given sol
pub fn get_images(arguments: &ArgMatches) -> Result<Vec<String>>{
    let mut sol: i32 = arguments.value_of("sol").unwrap_or("-1").trim().parse()?;

    if sol == -1 {
        print!(
            "{} Retrieving latest sol ... ",
            style("[1/4]").bold().dim()
        );
        std::io::stdout().flush().unwrap();

        // Retrieve latest sol
        let latest = reqwest::blocking::get(
            "https://mars.nasa.gov/rss/api/?feed=raw_images&category=ingenuity&feedtype=json&num=1"
        )?.text()?;
        let json: serde_json::Value = serde_json::from_str(latest.as_str())?;
        let latest = &json["images"][0]["sol"];

        sol = latest.as_i64().unwrap() as i32;
        println!(
            "{}",
            style(sol).cyan()
        );
    }
    else {
        print!(
            "{} Checking sol {} ... ",
            style("[1/4]").bold().dim(),
            style(sol).cyan()
        );
        std::io::stdout().flush().unwrap();
        
        // Check if sol exists
        let latest = reqwest::blocking::get(
            format!("https://mars.nasa.gov/rss/api/?feed=raw_images&category=ingenuity&feedtype=json&sol={}",
            sol
        ))?.text()?;
        let json: serde_json::Value = serde_json::from_str(latest.as_str())?;
        if &json["num_images"] == &serde_json::json!(null) {
            return Err(anyhow!(
                "No data from sol {}. For a list of sols that Ingenuity flew on, check https://en.wikipedia.org/wiki/List_of_Ingenuity_flights.", 
                style(sol).cyan()
            ));
        }
        else {
            println!(
                "{}",
                style("found!").green()
            );
        }
    }

    println!("{} Retrieving image list for sol {}",
                style("[2/4]").bold().dim(),
                style(sol).cyan()
            );
    let body = reqwest::blocking::get(format!("https://mars.nasa.gov/rss/api/?feed=raw_images&category=ingenuity&feedtype=json&sol={}", sol))?.text()?;
    let json: serde_json::Value = serde_json::from_str(body.as_str())?;

    let num_images_ref = &json["num_images"];
    let num_images: i32 = num_images_ref.as_i64().unwrap() as i32;

    let mut images: Vec<String> = Vec::new();
    for i in 0..num_images {
        let image_file_ref = &json["images"][i as usize]["image_files"]["full_res"];
        let image_file: String = String::from(image_file_ref.as_str().unwrap());
        images.push(image_file);
    }

    Ok(images)
}

pub fn download_images(image_list: Vec<String>) -> Result<Vec<Bytes>> {
    println!(
        "{} Downloading images ...",
        style("[3/4]").bold().dim()
    );

    let pb = ProgressBar::new(image_list.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let mut image_data: Vec<Bytes> = Vec::new();

    for image in image_list.iter() {
        let img_bytes = reqwest::blocking::get(image)?.bytes()?;
        image_data.push(img_bytes);
        pb.inc(1);
    }

    Ok(image_data)
}

/// Compile a GIF from the provided vec of bytes with a given framerate
pub fn compile_gif(image_data: Vec<Bytes>, arguments: &ArgMatches) -> Result<()> {
    let outfile = arguments.value_of("output").unwrap_or("output.gif");
    let fps: i32 = arguments.value_of("fps").unwrap_or("3").trim().parse()?;

    println!(
        "{} Compiling GIF to {} with FPS {}",
        style("[4/4]").bold().dim(),
        style(outfile).cyan(),
        style(fps).cyan()
    );

    // Set up our PB for the compilation
    let pb = ProgressBar::new(image_data.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    // Set up our GIF instance
    let (collector, writer) = gifski::new(
        gifski::Settings { 
            width: Some(640), 
            height: Some(480), 
            quality: (100), 
            fast: (false), 
            repeat: (gifski::Repeat::Infinite)
        }
    )?;

    let collector_thread = thread::spawn(move || {
        for (i, data) in image_data.iter().enumerate() {
            let img = Reader::new(
                Cursor::new(data.as_ref()))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            collector
                .add_frame_rgba(
                    i, 
                    get_img_vec(img), 
                    i as f64 / fps as f64)
                .expect("Failed to collect a frame"
            );
            pb.inc(1);
        }
    });

    let buffer = File::create(outfile)?;

    writer.write(
        buffer,
        &mut gifski::progress::NoProgress {}
    )?;

    collector_thread
        .join()
        .unwrap();
    
    Ok(())
}

// get_img_vec function from https://github.com/orhun/menyoki/blob/a45137e135c0370026fe15bb224eb4919e3d658a/src/image/mod.rs
fn get_img_vec(data: DynamicImage) -> ImgVec<RGBA8> {
    Img::new(
        data
            .pixels()
            .fold(Vec::<RGBA8>::new(), |mut rgba8, rgba| {
                let alpha = 255;
                rgba8.extend(vec![RGBA8 {
                    r: rgba.2[0],
                    g: rgba.2[1],
                    b: rgba.2[3],
                    a: alpha,
                }]);
                rgba8
            }),
        640,
        480,
    )
}