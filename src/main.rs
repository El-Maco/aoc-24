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
        "5" => challenges::day05::run(),
        "6" => challenges::day06::run(),
        "7" => challenges::day07::run(),
        "8" => challenges::day08::run(),
        "9" => challenges::day09::run(),
        "10" => challenges::day10::run(),
        _ => eprintln!("Day {} not implemented", day),
    }
}
