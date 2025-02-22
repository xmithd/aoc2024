use std::time::Instant;
use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day23;

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) {
        println!("Please enter a day");
        return;
    }
    let day = args[1].parse::<u8>().unwrap();
    let start = Instant::now();
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        5 => day5::day5(),
        6 => day6::day6(),
        7 => day7::day7(),
        8 => day8::day8(),
        9 => day9::day9(),
        10 => day10::day10(),
        11 => day11::day11(),
        12 => day12::day12(),
        13 => day13::day13(),
        14 => day14::day14(),
        15 => day15::day15(),
        16 => day16::day16(),
        17 => day17::day17(),
        18 => day18::day18(),
        19 => day19::day19(),
        20 => day20::day20(),
        21 => day21::day21(),
        22 => println!("Not implemented"),
        23 => day23::day23(),
        24 => println!("Not implemented"),
        25 => println!("Not implemented"),
        _ => println!("Invalid day"),
    }

    println!("Time taken: {:?}", start.elapsed());
}
