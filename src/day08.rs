// const INPUTS: &str = r#"
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
// "#;
const INPUTS: &str = include_str!("./inputs/day08.txt");

pub fn run() {
    let inputs: i32 = INPUTS
        .lines()
        .flat_map(|line| line.trim().split_once(" | "))
        .map(|(l, r)| {
            (
                l.split_whitespace().collect::<Vec<_>>(),
                r.split_whitespace().collect::<Vec<_>>(),
            )
        })
        .map(|(l, r)| (count_use_unique_numbers(&l), count_use_unique_numbers(&r)))
        .map(|(_, r)| r)
        .sum();

    println!("{}", inputs);
}

// 0 use: abc efg = 6 chars
// 1 use:   c  f  = 2 chars
// 2 use: a cde g = 5 chars
// 3 use: a cd fg = 5 chars
// 4 use:  bcd f  = 4 chars
// 5 use: ab d fg = 5 chars
// 6 use: ac defg = 6 chars
// 7 use: a c  f  = 3 chars
// 8 use: abcdefg = 7 chars
// 9 use: abcd fg = 6 chars
fn count_use_unique_numbers(digits: &Vec<&str>) -> i32 {
    digits
        .iter()
        .filter(|digit| match digit.len() {
            2 => true, // 1
            4 => true, // 4
            3 => true, // 7
            7 => true, // 8
            _ => false,
        })
        .count() as i32
}
