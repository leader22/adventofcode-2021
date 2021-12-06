// const INPUTS: &str = r#"3,4,3,1,2"#;
const INPUTS: &str = include_str!("./inputs/day06.txt");

pub fn run() {
    let mut map = INPUTS
        .split(",")
        .flat_map(|x| x.parse::<usize>())
        .fold([0; 9], |mut map, n| {
            map[n] += 1;
            map
        });

    for day in 1..80 {
        map[(day + 7) % 9] += map[day % 9];
    }

    println!("{}", map.iter().sum::<usize>());
}
