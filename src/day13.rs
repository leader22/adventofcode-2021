// const INPUTS: &str = r#"
// 6,10
// 0,14
// 9,10
// 0,3
// 10,4
// 4,11
// 6,0
// 6,12
// 4,1
// 0,13
// 10,12
// 3,4
// 3,0
// 8,4
// 1,10
// 2,14
// 8,10
// 9,0
//
// fold along y=7
// fold along x=5
// "#;
const INPUTS: &str = include_str!("./inputs/day13.txt");

use std::collections::BTreeMap;

pub fn run() {
    let ((mut dots_map, max_pos), folds) = parse_inputs(INPUTS);
    println!("dots_map: {:?}", dots_map);
    println!("max_pos: {:?}", max_pos);
    println!("folds: {:?}", folds);

    let mut paper = init_paper(&mut dots_map, max_pos);
    print_paper(&paper);

    for fold in folds {
        println!("=========== fold: {:?}", fold);
        paper = fold_paper(&paper, &fold);
        print_paper(&paper);
        println!("count: {}", count_marked(&paper));

        // only the 1st fold
        break;
    }
}

type Pos = (i16, i16);
type DotsMap = BTreeMap<Pos, bool>;
type Fold = (&'static str, usize);
type Folds = Vec<Fold>;
fn parse_inputs(inputs: &'static str) -> ((DotsMap, Pos), Folds) {
    let (dots, folds) = inputs.trim().split_once("\n\n").unwrap();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut dots_map = BTreeMap::new();
    dots.trim().lines().for_each(|line| {
        let (x, y) = line.split_once(",").unwrap();
        let x = x.parse::<i16>().unwrap();
        let y = y.parse::<i16>().unwrap();
        if max_x < x {
            max_x = x;
        }
        if max_y < y {
            max_y = y;
        }
        dots_map.insert((x, y), true);
    });

    let folds = folds
        .trim()
        .lines()
        .map(|line| {
            let (_, pos) = line.split_once("along ").unwrap();
            let (dir, unit) = pos.split_once("=").unwrap();
            let unit = unit.parse::<usize>().unwrap();
            (dir, unit)
        })
        .collect::<Vec<_>>();

    ((dots_map, (max_x, max_y)), folds)
}

type Paper = Vec<Vec<char>>;
fn init_paper(dots_map: &DotsMap, (max_x, max_y): Pos) -> Paper {
    let mut paper = vec![];
    for y in 0..=max_y {
        let mut row = vec![];
        for x in 0..=max_x {
            if dots_map.contains_key(&(x, y)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        paper.push(row);
    }

    paper
}

fn fold_paper(paper: &Paper, fold: &Fold) -> Paper {
    let (dir, unit) = fold;

    let row_size = paper.len();
    let col_size = paper[0].len();
    let mut next_paper: Paper = vec![];
    match dir {
        &"x" => {
            for y in 0..row_size {
                next_paper.push(vec!['?'; *unit]);

                for x in 0..*unit {
                    next_paper[y][x] = paper[y][x];
                }
                for x in *unit + 1..col_size {
                    // 0-indexed
                    let next_x = x - *unit - 1;

                    let v = paper[y][col_size - next_x - 1];
                    // overlap if not marked
                    if next_paper[y][next_x] != '#' {
                        next_paper[y][next_x] = v;
                    }
                }
            }
        }
        &"y" => {
            for y in 0..*unit {
                next_paper.push(vec!['?'; col_size]);

                for x in 0..col_size {
                    next_paper[y][x] = paper[y][x];
                }
            }

            for y in *unit + 1..row_size {
                // 0-indexed
                let next_y = y - *unit - 1;

                for x in 0..col_size {
                    let v = paper[row_size - next_y - 1][x];
                    // overlap if not marked
                    if next_paper[next_y][x] != '#' {
                        next_paper[next_y][x] = v;
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    next_paper
}

fn print_paper(paper: &Paper) {
    for row in paper {
        for d in row {
            print!("{}", d);
        }
        println!("");
    }
}

fn count_marked(paper: &Paper) -> usize {
    paper
        .iter()
        .map(|row| row.iter().filter(|d| *d == &'#').count())
        .sum()
}
