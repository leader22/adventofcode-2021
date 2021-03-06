// const INPUTS: &str = r#"199
// 200
// 208
// 210
// 200
// 207
// 240
// 269
// 260
// 263"#;
const INPUTS: &str = include_str!("./inputs/day01.txt");

pub fn run() {
    let inputs = INPUTS.lines().map(|line| line.parse::<i32>().unwrap());

    let mut last = 0;
    let mut count = -1;
    for i in inputs {
        if last < i {
            count += 1;
        }

        last = i;
    }

    println!("{}", count);
}
