use std::env;

mod day01;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match &day[..] {
        "1" => {
            day01::run();
        }
        _ => {
            println!("Specify target day!");
        }
    }
}
