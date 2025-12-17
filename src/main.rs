mod parser;
mod solutions;

macro_rules! solution_fn {
    ($day:literal, $problem:literal) => {
        solutions::day$day::solution$problem();
    };
}

fn main() {
    println!("Hello, world!");
}
