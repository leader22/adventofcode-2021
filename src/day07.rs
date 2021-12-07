// const INPUTS: &str = r#"16,1,2,0,4,2,7,1,2,14"#;
const INPUTS: &str = include_str!("./inputs/day07.txt");

type Positions = Vec<i32>;

pub fn run() {
    let positions = INPUTS
        .split(",")
        .flat_map(|x| x.trim().parse::<i32>())
        .collect::<Vec<_>>();

    println!("{:?}", positions);

    let mut min = i32::MAX;
    for n in &positions {
        let fuel = calc_fuel(&positions, *n);
        println!("{} = {}", n, fuel);

        if min > fuel {
            min = fuel;
        }
    }

    println!("min is {}", min);
}

fn calc_fuel(positions: &Positions, dest: i32) -> i32 {
    positions.iter().map(|n| (dest - n).abs()).sum()
}
