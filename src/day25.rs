// const INPUTS: &str = r#"
// v...>>.vv>
// .vv>>.vv..
// >>.>v>...v
// >>v>>.>.v.
// v>v.vv.v..
// >.>>..v...
// .vv..>.>v.
// v.v..>>v.v
// ....v..v.>
// "#;
const INPUTS: &str = include_str!("./inputs/day25.txt");

pub fn run() {
    let mut grid = parse_inputs(INPUTS.trim());
    println!("Initial state:");
    print_grid(&grid);

    for i in 1..i32::MAX {
        println!("After {} step:", i);

        let result_right = move_right(&grid);
        grid = result_right.0;
        let result_down = move_down(&grid);
        grid = result_down.0;

        print_grid(&grid);

        if result_right.1 == false && result_down.1 == false {
            break;
        }
    }
}

type Grid = Vec<Vec<char>>;
fn parse_inputs(s: &str) -> Grid {
    s.lines().map(|x| x.trim().chars().collect()).collect()
}

fn print_grid(grid: &Grid) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}

fn move_right(grid: &Grid) -> (Grid, bool) {
    let row_size = grid.len();
    let col_size = grid[0].len();
    let mut num_of_move = 0;
    let mut next_grid = grid.clone();
    // let mut next_grid = vec![vec!['.'; col_size]; row_size];
    // for y in 0..row_size {
    //     for x in 0..col_size {
    //         next_grid[y][x] = grid[y][x];
    //     }
    // }

    // >
    for y in 0..row_size {
        for x in 0..col_size {
            if grid[y][x] != '>' {
                continue;
            }

            let dest_x = if x == col_size - 1 { 0 } else { x + 1 };
            if grid[y][dest_x] != '.' {
                continue;
            }

            next_grid[y][x] = '.';
            next_grid[y][dest_x] = '>';
            num_of_move += 1;
        }
    }

    (next_grid, num_of_move > 0)
}

fn move_down(grid: &Grid) -> (Grid, bool) {
    let row_size = grid.len();
    let col_size = grid[0].len();
    let mut num_of_move = 0;
    // let mut next_grid = vec![vec!['.'; col_size]; row_size];
    // for y in 0..row_size {
    //     for x in 0..col_size {
    //         next_grid[y][x] = grid[y][x];
    //     }
    // }
    let mut next_grid = grid.clone();

    // v
    for y in 0..row_size {
        for x in 0..col_size {
            if grid[y][x] != 'v' {
                continue;
            }

            let dest_y = if y == row_size - 1 { 0 } else { y + 1 };
            if grid[dest_y][x] != '.' {
                continue;
            }

            next_grid[y][x] = '.';
            next_grid[dest_y][x] = 'v';
            num_of_move += 1;
        }
    }

    (next_grid, num_of_move > 0)
}
