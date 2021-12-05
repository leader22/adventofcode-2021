// const INPUTS: &str = r#"0,9 -> 5,9
// 8,0 -> 0,8
// 9,4 -> 3,4
// 2,2 -> 2,1
// 7,0 -> 7,4
// 6,4 -> 2,0
// 0,9 -> 2,9
// 3,4 -> 1,4
// 0,0 -> 8,8
// 5,5 -> 8,2"#;

const INPUTS: &str = include_str!("./inputs/day05.txt");

pub fn run() {
    let mut max_size = 1;
    let mut vents: Vec<Vent> = vec![];
    for line in INPUTS.lines() {
        let (from_pos, to_pos) = line.split_once(" -> ").unwrap();
        let (x1_str, y1_str) = from_pos.split_once(",").unwrap();
        let (x2_str, y2_str) = to_pos.split_once(",").unwrap();

        let x1 = x1_str.parse::<i32>().unwrap();
        let x2 = x2_str.parse::<i32>().unwrap();
        let y1 = y1_str.parse::<i32>().unwrap();
        let y2 = y2_str.parse::<i32>().unwrap();

        let nums = vec![x1, x2, y1, y1];
        let max = nums.iter().max().unwrap();
        if max_size < *max {
            max_size = *max;
        }

        vents.push((x1 as usize, y1 as usize, x2 as usize, y2 as usize));
    }
    println!("{:?}", vents);

    let mut diagram: Diagram = vec![];
    max_size += 1; // as usize
    for _ in 0..max_size {
        diagram.push(vec![0; max_size as usize]);
    }
    println!("size = {}", max_size);
    print_diagram(&diagram);

    use std::cmp;
    for (x1, y1, x2, y2) in vents {
        println!("[{:?},{:?}] > [{:?},{:?}]", x1, y1, x2, y2);

        if x1 == x2 {
            for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                diagram[y][x1] += 1;
            }
        }

        if y1 == y2 {
            for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                diagram[y1][x] += 1;
            }
        }
        // print_diagram(&diagram);
    }

    let answer = count_over(&diagram, 2);
    println!("answer = {}", answer);
}

type Diagram = Vec<Vec<i32>>;
type Vent = (usize, usize, usize, usize);

fn print_diagram(d: &Diagram) {
    for row in d {
        for c in row {
            if *c == 0 {
                print!(".");
            } else {
                print!("{:?}", c);
            }
        }
        println!("");
    }
    println!("");
}

fn count_over(d: &Diagram, i: i32) -> i32 {
    let mut count = 0;
    for row in d {
        for c in row {
            if *c >= i {
                count += 1;
            }
        }
    }
    count
}
