mod challenges;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];
    match day.as_str() {
        "1" => challenges::day01::run(),
        "2" => challenges::day02::run(),
        "3" => challenges::day03::run(),
        "4" => challenges::day04::run(),
        _ => eprintln!("Day {} not implemented", day),
    }
}
