// const INPUTS: &str = r#"
// [({(<(())[]>[[{[]{<()<>>
// [(()[<>])]({[<{<<[]>>(
// {([(<{}[<>[]}>{[]{[(<()>
// (((({<>}<{<{<>}{[]{[]{}
// [[<[([]))<([[{}[[()]]]
// [{[{({}]{}}([{[{{{}}([]
// {<[[]]>}<{[{[{[]{()[[[]
// [<(<(<(<{}))><([]([]()
// <{([([[(<>()){}]>(<<{{
// <{([{{}}[<[[[<>{}]]]>[]]
// "#;
const INPUTS: &str = include_str!("./inputs/day10.txt");

pub fn run() {
    let answer: i32 = INPUTS
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .map(|chars| match find_corrupted(&chars) {
            Some(corrupted) => to_score(corrupted),
            None => 0,
        })
        .sum();

    println!("{}", answer);
}

fn find_corrupted(chars: &Vec<char>) -> Option<char> {
    let mut last_openers: Vec<char> = vec![];
    for c in chars {
        if is_opener(*c) {
            println!("OP: {:?}", c);
            last_openers.push(*c);
        } else if is_closer(*c) {
            println!("CL: {:?}", c);
            match last_openers.pop() {
                Some(opener) => {
                    if is_paired((opener, *c)) {
                        println!("  => Paired!");
                    } else {
                        println!("  => Corrupted!");
                        return Some(*c);
                    }
                }
                None => unreachable!(),
            }
        } else {
            unreachable!();
        }
    }

    None
}

fn is_opener(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}
fn is_closer(c: char) -> bool {
    match c {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false,
    }
}
fn is_paired(pair: (char, char)) -> bool {
    match pair {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}
fn to_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
