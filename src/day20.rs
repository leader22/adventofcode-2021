// const INPUTS: &str = r#"
// ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
// #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
// .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
// .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
// .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
// ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
// ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
//
// #..#.
// #....
// ##..#
// ..#..
// ..###
// "#;
const INPUTS: &str = include_str!("./inputs/day20.txt");

pub fn run() {
    let (iea, mut pixels) = parse_inputs(INPUTS.trim());
    println!("{}", iea.len());

    // extend to use original edge pixels

    for step in 1..=2 {
        println!("step: {}", step);
        pixels = extend_pixels(&pixels);
        pixels = extend_pixels(&pixels);
        pixels = update_pixels(&pixels, &iea);
        print_pixels(&pixels);
        println!("");
    }

    let answer = count_lights(&pixels);
    println!("answer: {}", answer);
}

type IEA = Vec<char>;
type Pixels = Vec<Vec<char>>;

fn parse_inputs(inputs: &str) -> (IEA, Pixels) {
    let (a, b) = inputs.split_once("\n\n").unwrap();

    let iea = a.chars().filter(|c| *c == '.' || *c == '#').collect();
    let pixels = b
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    (iea, pixels)
}

fn print_pixels(pixels: &Pixels) {
    for row in pixels {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn extend_pixels(pixels: &Pixels) -> Pixels {
    let mut next_pixels = vec![];
    next_pixels.push(vec!['.'; 1 + pixels[0].len() + 1]);
    for row in pixels {
        let mut next_row = vec![];
        next_row.push('.');
        next_row.extend(row.clone());
        next_row.push('.');
        next_pixels.push(next_row);
    }
    next_pixels.push(vec!['.'; 1 + pixels[0].len() + 1]);

    next_pixels
}

fn update_pixels(pixels: &Pixels, iea: &IEA) -> Pixels {
    let row_size = pixels.len();
    let col_size = pixels[0].len();
    println!("row: {} x col: {}", row_size, col_size);

    let mut next_pixels = pixels.clone();
    // // ignore top and bottom edge
    for y in 1..row_size - 1 {
        // // ignore left and right edge
        for x in 1..col_size - 1 {
            let nines = [
                pixels[y - 1][x - 1], // ↖️
                pixels[y - 1][x],     // ⬆️
                pixels[y - 1][x + 1], // ↗️
                pixels[y][x - 1],     // ⬅️
                pixels[y][x],         // ✅
                pixels[y][x + 1],     // ➡️
                pixels[y + 1][x - 1], // ↙️
                pixels[y + 1][x],     // ⬇️
                pixels[y + 1][x + 1], // ↘️
            ]
            .map(|c| match c {
                '#' => '1',
                '.' => '0',
                _ => unreachable!(),
            })
            .iter()
            .collect::<String>();
            let iea_idx = usize::from_str_radix(&nines, 2).unwrap();
            let next_pixel = iea[iea_idx];

            // println!(
            //     "pixels[{}][{}] = `{}` w/ {:?} = {} >> `{}`",
            //     y, x, pixels[y][x], nines, iea_idx, next_pixel
            // );
            next_pixels[y][x] = next_pixel;
        }
    }

    next_pixels
}

fn count_lights(pixels: &Pixels) -> u32 {
    let mut count = 0;
    for row in pixels {
        for c in row {
            if *c == '#' {
                count += 1
            }
        }
    }
    count
}
