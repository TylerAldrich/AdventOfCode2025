mod parser;
mod solutions;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of solution to run
    #[arg(short, long)]
    day: u8,

    /// Problem number [1 or 2]
    #[arg(short, long, default_value_t = 1)]
    problem: u8,

    /// Should this use the test input?
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let solution = match (args.day, args.problem) {
        (1, 1) => solutions::day1::solution1(args.test),
        (1, 2) => solutions::day1::solution2(args.test),
        (2, 1) => solutions::day2::solution1(args.test),
        (2, 2) => solutions::day2::solution2(args.test),
        (3, 1) => solutions::day3::solution1(args.test),
        (3, 2) => solutions::day3::solution2(args.test),
        (4, 1) => solutions::day4::solution1(args.test),
        (4, 2) => solutions::day4::solution2(args.test),
        (_, _) => panic!("Invalid solution or day")
    };

    println!("{}", solution);
}
