// const INPUTS: &str = r#"forward 5
// down 5
// forward 8
// up 3
// down 8
// forward 2"#;
const INPUTS: &str = include_str!("./inputs/day02.txt");

pub fn run() {
    let inputs = INPUTS
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>());

    let mut pos = (0, 0);
    for words in inputs {
        let x = words[1].parse::<i32>().unwrap();
        match words[0] {
            "forward" => pos.0 += x,
            "down" => pos.1 += x,
            "up" => pos.1 -= x,
            _ => {}
        }
    }

    println!("{:?} => {}", pos, pos.0 * pos.1);
}
