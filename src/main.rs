use std::env;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match &day[..] {
        "1" => day01::run(),
        "2" => day02::run(),
        "3" => day03::run(),
        "4" => day04::run(),
        // "5" => day05::run(),
        _ => {
            println!("Specify target day or Not yet implemented!");
        }
    }
}
