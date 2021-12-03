pub fn run() {
    // let inputs = vec![
    //     "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
    //     "00010", "01010",
    // ];
    let inputs: Vec<&str> = include_str!("./inputs/day03.txt").lines().collect();

    let bits_len = &inputs[0].len();
    let mut bit_counter = vec![0; *bits_len];
    for line in inputs {
        let bits = line.chars();

        for (idx, bit) in bits.enumerate() {
            match bit {
                '0' => bit_counter[idx] -= 1,
                '1' => bit_counter[idx] += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();
    for i in bit_counter {
        if i > 0 {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }
    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_str, 2).unwrap();

    println!("g: {:?} => {}", &gamma_str, gamma);
    println!("e: {:?} => {}", &epsilon_str, epsilon);

    println!("{}", gamma * epsilon);
}
