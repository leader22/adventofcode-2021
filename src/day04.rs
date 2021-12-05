// const INPUTS: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//
// 22 13 17 11  0
//  8  2 23  4 24
// 21  9 14 16  7
//  6 10  3 18  5
//  1 12 20 15 19
//
//  3 15  0  2 22
//  9 18 13 17  5
// 19  8  7 25 23
// 20 11 10 24  4
// 14 21 16 12  6
//
// 14 21 17 24  4
// 10 16 15  9 19
// 18  8 23 26 20
// 22 11 13  6  5
//  2  0 12  3  7"#;
const INPUTS: &str = include_str!("./inputs/day04.txt");

pub fn run() {
    let (numbers_str, bingos_str) = INPUTS.split_once("\n\n").unwrap();

    let numbers = numbers_str
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("{:?}", numbers);

    let mut bingos = bingos_str
        .split("\n\n")
        .map(|b| Bingo::from_str(b))
        .collect::<Vec<Bingo>>();

    for b in &bingos {
        println!("{:?}", b);
        println!("");
    }

    'outer: for n in numbers {
        println!("{:?}", n);
        for bingo in &mut bingos {
            bingo.mark(&n);
            println!("{:?}", bingo);

            if bingo.is_bingo() {
                let answer = bingo.calculate(&n);
                println!("{}", answer);
                break 'outer;
            }
        }
    }
}

struct Bingo {
    items: Vec<Vec<(i32, bool)>>,
}

impl Bingo {
    fn from_str(str: &str) -> Bingo {
        let items = str
            .trim()
            .split("\n")
            .map(|l| {
                l.split_whitespace()
                    .map(|x| (x.parse::<i32>().unwrap(), false))
                    .collect::<Vec<(i32, bool)>>()
            })
            .collect::<Vec<Vec<(i32, bool)>>>();

        Bingo { items }
    }

    fn mark(&mut self, n: &i32) {
        for row in &mut self.items {
            for e in row {
                if e.0 == *n {
                    e.1 = true;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        for row in &self.items {
            if row.iter().all(|&e| e.1 == true) {
                return true;
            }
        }

        let size = self.items.len();
        for i in 0..size {
            if self.items.iter().map(|x| x[i]).all(|e| e.1 == true) {
                return true;
            }
        }

        false
    }

    fn calculate(&self, i: &i32) -> i32 {
        let mut sum = 0;
        for row in &self.items {
            for e in row {
                if e.1 == false {
                    sum += e.0;
                }
            }
        }

        sum * *i
    }
}

use std::fmt;
impl fmt::Debug for Bingo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.items {
            if let Err(err) = writeln!(f, "{:?}", row) {
                println!("Writing error: {}", err.to_string());
            }
        }
        write!(f, "")
    }
}
