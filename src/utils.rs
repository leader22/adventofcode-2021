use std::fs;

pub fn read_file_by_line_as_vec_u32(path: &str) -> Vec<u32> {
    let inputs = fs::read_to_string(path).expect("Read input file failed!");
    inputs
        .trim()
        .split('\n')
        .map(|x| x.parse::<u32>().expect("Parse line to u32 failed!"))
        .collect()
}
