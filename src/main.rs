use std::collections::HashMap;

use clap::Parser;
use colored::Colorize;
use std::time::Instant;

macro_rules! timeit {
    ($e:expr) => {{
        let now = Instant::now();
        let val = $e;
        (val, now.elapsed())
    }};
}

#[derive(Debug, Parser)]
struct Args {
    /// The URL to scrape for phone numbers
    url: String,

    /// Show debug information
    #[clap(long)]
    debug: bool,
}

fn report_time(name: &str, time: std::time::Duration) {
    let time = time.as_millis();
    let time = format!("{:.2} ms", time).green();
    println!("{} took {}", name.purple(), time);
}

fn main() -> anyhow::Result<()> {
    let Args { url, debug } = Args::parse();
    println!("Scraping {url} for phone numbers...");

    let (content, fetch_time) = timeit!(get(&url)?);
    report_time("Content Fetch", fetch_time);

    if debug {
        println!("Content: {content}");
    }

    let (finds, parse_time) = timeit!(find_phone_numbers(&content));
    report_time("Regex Search", parse_time);

    if finds.is_empty() {
        println!("No phone numbers found.");
        return Ok(());
    }

    for (number, hits) in finds {
        println!("{}: {} hits", number.blue(), hits.to_string().green());
    }

    Ok(())
}

fn get(url: &String) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()?;
    let resp = resp.text()?;
    Ok(resp)
}

fn find_phone_numbers(content: &str) -> HashMap<String, i32> {
    let re =
        regex::Regex::new(r"\(?\d{3}\)?[ -]\d{3}[ -]\d{4}").expect("You fucked up your regex!");

    re.find_iter(content)
        .map(|m| m.as_str().to_string())
        .fold(HashMap::new(), |mut acc, number| {
            let count = acc.entry(number).or_insert(0);
            *count += 1;
            acc
        })
}
