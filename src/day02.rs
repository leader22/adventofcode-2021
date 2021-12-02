pub fn run() {
    // let inputs = vec![
    //     vec!["forward", "5"],
    //     vec!["down", "5"],
    //     vec!["forward", "8"],
    //     vec!["up", "3"],
    //     vec!["down", "8"],
    //     vec!["forward", "2"],
    // ];
    let inputs = include_str!("./inputs/day02.txt")
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
