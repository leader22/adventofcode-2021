// const INPUTS: &str = r#"
// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678
// "#;
const INPUTS: &str = include_str!("./inputs/day09.txt");

pub fn run() {
    let inputs = INPUTS
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split("")
                .flat_map(|x| x.trim().parse::<i32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    for row in &inputs {
        println!("{:?}", row);
    }

    let row_size = &inputs.len();
    let col_size = &inputs[0].len();
    println!("{}x{} matrix", col_size, row_size);

    let mut risk_total = 0;
    for (ridx, row) in inputs.iter().enumerate() {
        for (cidx, i) in row.iter().enumerate() {
            let mut check_pos = vec![];
            // ↑
            if ridx > 0 {
                check_pos.push((ridx - 1, cidx));
            }
            // ↓
            if ridx < row_size - 1 {
                check_pos.push((ridx + 1, cidx));
            }
            // →
            if cidx > 0 {
                check_pos.push((ridx, cidx - 1))
            }
            // ←
            if cidx < col_size - 1 {
                check_pos.push((ridx, cidx + 1))
            }

            let low_point = check_pos.iter().all(|(r, c)| inputs[*r][*c] > *i);
            println!(
                "[{}, {}] = {} check {:?}, low ? {}",
                ridx, cidx, i, check_pos, low_point
            );
            if low_point {
                risk_total += i + 1;
            }
        }
    }

    println!("Risk total = {}", risk_total);
}
