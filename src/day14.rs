// const INPUTS: &str = r#"
// NNCB
//
// CH -> B
// HH -> N
// CB -> H
// NH -> C
// HB -> C
// HC -> B
// HN -> C
// NN -> C
// BH -> H
// NC -> B
// NB -> B
// BN -> B
// BB -> N
// BC -> B
// CC -> N
// CN -> C
// "#;
const INPUTS: &str = include_str!("./inputs/day14.txt");

use std::collections::HashMap;

pub fn run() {
    let (mut template, insertions) = parse_inputs(INPUTS);
    println!("template: {:?}", template);
    println!("insertions: {:?}", insertions);

    let mut char_counts = HashMap::new();
    for c in &template {
        char_counts.entry(*c).and_modify(|e| *e += 1).or_insert(1);
    }
    println!("char_counts: {:?}", char_counts);

    for step in 1..=10 {
        perform_insert(&mut template, &insertions, &mut char_counts);
        print_template(step, &template);
    }
    println!("char_counts: {:?}", char_counts);

    let mut counts = char_counts.iter().map(|(_, v)| *v).collect::<Vec<i32>>();
    counts.sort();
    println!("counts: {:?}", counts);

    let at_least = counts.first().unwrap();
    let at_most = counts.last().unwrap();
    println!("answer: {}", at_most - at_least);
}

type Template = Vec<char>;
type Insertions = HashMap<String, char>;
type CharCounts = HashMap<char, i32>;
fn parse_inputs(inputs: &str) -> (Template, Insertions) {
    let (template, insertions_) = inputs.trim().split_once("\n\n").unwrap();
    let template = template.trim().chars().collect::<Vec<_>>();

    let mut insertions = HashMap::new();
    for line in insertions_.trim().lines() {
        let (pair, el) = line.trim().split_once(" -> ").unwrap();
        insertions.insert(pair.to_string(), el.chars().next().unwrap());
    }

    (template, insertions)
}

fn print_template(step: i8, template: &Template) {
    println!("After step {}:", step);
    for c in template {
        print!("{}", c);
    }
    println!("");
}

fn perform_insert(template: &mut Template, insertions: &Insertions, char_counts: &mut CharCounts) {
    let mut i = 0;
    while i < template.len() {
        let c1 = template.get(i).unwrap();
        let c2 = template.get(i + 1).unwrap_or(&'_');
        if c2 == &'_' {
            break;
        }

        let pair = format!("{}{}", c1, c2);
        i += 1;

        let el = insertions.get(&pair.to_string());
        let el = match el {
            Some(el) => el,
            None => continue,
        };

        template.insert(i, *el);
        i += 1;

        char_counts.entry(*el).and_modify(|e| *e += 1).or_insert(1);
    }
}
