use crate::utils::read_file_by_line_as_vec_u32;

pub fn run() {
    let inputs = read_file_by_line_as_vec_u32("./src/inputs/day01.txt");
    // let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let mut last = 0;
    let mut count = -1;
    for i in inputs {
        if last < i {
            count += 1;
        }

        last = i;
    }

    println!("{}", count);
}
