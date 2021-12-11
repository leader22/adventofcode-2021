// const INPUTS: &str = r#"
// 5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526
// "#;
const INPUTS: &str = include_str!("./inputs/day11.txt");

pub fn run() {
    let mut grid = parse_grid(INPUTS);
    let mut flash_count = 0;

    let row_size = grid.len();
    let col_size = grid[0].len();

    print_grid(0, &grid);
    for step in 1..=100 {
        for r in 0..row_size {
            for c in 0..col_size {
                increment_and_flash(&mut grid, r, c);
            }
        }

        for r in 0..row_size {
            for c in 0..col_size {
                if grid[r][c] < 0 {
                    grid[r][c] = 0;
                    flash_count += 1;
                }
            }
        }

        print_grid(step, &grid);
    }

    println!("count: {}", flash_count);
}

type Grid = Vec<Vec<i8>>;
fn parse_grid(inputs: &str) -> Grid {
    inputs
        .trim()
        .lines()
        .map(|row| {
            row.trim()
                .split("")
                .flat_map(|x| x.trim().parse::<i8>())
                .collect()
        })
        .collect()
}

fn print_grid(step: i32, grid: &Grid) {
    if step == 0 {
        println!("Before any steps:");
    } else {
        println!("After step {}:", step);
    }

    for row in grid {
        println!("{:?}", row);
    }
}

fn flash(grid: &mut Grid, r: usize, c: usize) {
    // mark as flashed
    grid[r][c] = -1;

    let has_top = r > 0;
    let has_left = c > 0;
    let has_right = c < grid[r].len() - 1;
    let has_bottom = r < grid.len() - 1;

    // ↖️
    if has_top && has_left {
        increment_and_flash(grid, r - 1, c - 1);
    }

    // ⬆️
    if has_top {
        increment_and_flash(grid, r - 1, c);
    }

    // ↗️
    if has_top && has_right {
        increment_and_flash(grid, r - 1, c + 1);
    }

    // ⬅️
    if has_left {
        increment_and_flash(grid, r, c - 1);
    }

    // ➡️
    if has_right {
        increment_and_flash(grid, r, c + 1);
    }

    // ↙️
    if has_bottom && has_left {
        increment_and_flash(grid, r + 1, c - 1);
    }

    // ⬇️
    if has_bottom {
        increment_and_flash(grid, r + 1, c);
    }

    // ↘️
    if has_bottom && has_right {
        increment_and_flash(grid, r + 1, c + 1);
    }
}

fn increment_and_flash(grid: &mut Grid, r: usize, c: usize) {
    if grid[r][c] < 0 {
        return;
    }

    grid[r][c] += 1;
    if grid[r][c] > 9 {
        flash(grid, r, c);
    }
}
