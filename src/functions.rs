// Description: public functions

use clap::ArgMatches;
use serde_json;
use console::style;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use anyhow::{anyhow, Result};
use std::io::Write;

/// Downloads the JSON document for a given sol
pub fn get_json(arguments: &ArgMatches) -> Result<serde_json::Value>{
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

    println!("{} Retrieving image data for sol {}",
                style("[2/4]").bold().dim(),
                style(sol).cyan()
            );
    let body = reqwest::blocking::get(format!("https://mars.nasa.gov/rss/api/?feed=raw_images&category=ingenuity&feedtype=json&sol={}", sol))?.text()?;
    let json: serde_json::Value = serde_json::from_str(body.as_str())?;

    Ok(json)
}