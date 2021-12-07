// const INPUTS: &str = r#"3,4,3,1,2"#;
const INPUTS: &str = include_str!("./inputs/day06.txt");

pub fn run() {
    let timers = INPUTS.split(',').flat_map(|d| d.trim().parse::<usize>());
    let mut days = [0; 9];

    for timer in timers {
        days[timer] += 1;
    }

    for _ in 0..80 {
        let num_zero = days[0];
        days.rotate_left(1);
        days[6] += num_zero;
        days[8] = num_zero;
    }

    println!("{}", days.iter().sum::<usize>());
}
