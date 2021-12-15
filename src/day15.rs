// const INPUTS: &str = r#"
// 1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581
// "#;
const INPUTS: &str = include_str!("./inputs/day15.txt");

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn run() {
    let inputs = parse_inputs(INPUTS);
    print(&inputs);
    let answer = find_risk(&inputs);
    println!("answer: {}", answer);
}

type Map = Vec<Vec<i32>>;
fn parse_inputs(inputs: &str) -> Map {
    inputs
        .trim()
        .lines()
        .map(|line| {
            line.split("")
                .flat_map(|i| i.trim().parse::<i32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn print(map: &Map) {
    for row in map {
        println!("{:?}", row);
    }
    println!("");
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
struct Entry {
    value: usize,
    pos: (usize, usize),
}

fn find_risk(input: &Map) -> usize {
    let mut prev = HashMap::new();
    prev.insert((0, 0), 0);

    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Entry {
        value: 0,
        pos: (0, 0),
    }));

    let last = (input[0].len() - 1, input.len() - 1);
    loop {
        let min = queue.pop().unwrap().0;
        if min.pos == last {
            break;
        }

        for neighbor in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (x, y) = (
                (min.pos.0 as i32 + neighbor.0) as usize,
                (min.pos.1 as i32 + neighbor.1) as usize,
            );
            if prev.get(&(x, y)).is_some() {
                continue;
            }

            let value = match input.get(y).and_then(|p| p.get(x)) {
                Some(v) => *v as usize,
                None => continue,
            };
            let entry = Entry {
                pos: (x, y),
                value: min.value + value,
            };
            queue.push(Reverse(entry));

            prev.entry((x, y)).or_insert(min.value + value);
        }
    }

    *prev.get(&last).unwrap()
}
