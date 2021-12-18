// const INPUTS: &str = r#"
// [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
// [[[5,[2,8]],4],[5,[[9,9],0]]]
// [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
// [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
// [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
// [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
// [[[[5,4],[7,7]],8],[[8,3],8]]
// [[9,3],[[9,9],[6,[4,9]]]]
// [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
// [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
// "#;
const INPUTS: &str = include_str!("./inputs/day18.txt");

use std::iter;

pub fn run() {
    let tokens: SnailfishTokens = INPUTS
        .trim()
        .lines()
        .map(|line| parse(line.as_ref()))
        .reduce(|x, y| add(x, y))
        .unwrap();

    let answer = magnitude(&mut tokens.into_iter()).unwrap();
    println!("answer: {}", answer);
}

#[derive(Clone, Copy)]
enum SnailfishToken {
    Open,
    Close,
    Value(u32),
}

type SnailfishTokens = Vec<SnailfishToken>;

fn parse(line: &str) -> SnailfishTokens {
    line.chars()
        .flat_map(|c| match c {
            '[' => Some(SnailfishToken::Open),
            ']' => Some(SnailfishToken::Close),
            _ => c.to_digit(10).map(SnailfishToken::Value),
        })
        .collect()
}

fn add(x: SnailfishTokens, y: SnailfishTokens) -> SnailfishTokens {
    let mut vec: SnailfishTokens = iter::once(SnailfishToken::Open)
        .chain(x)
        .chain(y)
        .chain(iter::once(SnailfishToken::Close))
        .collect();

    'outer: loop {
        let mut depth = 0;
        for (idx, window) in vec[..].windows(4).enumerate() {
            // explode
            if depth > 3 {
                if let [SnailfishToken::Open, SnailfishToken::Value(x), SnailfishToken::Value(y), SnailfishToken::Close] =
                    *window
                {
                    vec.splice(idx..idx + 4, iter::once(SnailfishToken::Value(0)));
                    if let Some(t) = vec.iter_mut().take(idx).rev().find_map(|t| match t {
                        SnailfishToken::Value(t) => Some(t),
                        _ => None,
                    }) {
                        *t += x;
                    }
                    if let Some(t) = vec.iter_mut().skip(idx + 1).find_map(|t| match t {
                        SnailfishToken::Value(t) => Some(t),
                        _ => None,
                    }) {
                        *t += y;
                    }
                    continue 'outer;
                }
            }

            match window[0] {
                SnailfishToken::Open => depth += 1,
                SnailfishToken::Close => depth -= 1,
                _ => {}
            }
        }

        // split
        if let Some((idx, t)) = vec.iter_mut().enumerate().find_map(|(idx, t)| match t {
            SnailfishToken::Value(t) => Some((idx, *t)).filter(|(_, t)| *t > 9),
            _ => None,
        }) {
            vec.splice(
                idx..=idx,
                [
                    SnailfishToken::Open,
                    SnailfishToken::Value(t / 2),
                    SnailfishToken::Value((t + 1) / 2),
                    SnailfishToken::Close,
                ],
            );
        } else {
            break vec;
        }
    }
}

fn magnitude(tokens: &mut dyn Iterator<Item = SnailfishToken>) -> Option<u32> {
    match tokens.next()? {
        SnailfishToken::Open => {
            let lhs = magnitude(tokens)?;
            let rhs = magnitude(tokens)?;
            match tokens.next()? {
                SnailfishToken::Close => Some(3 * lhs + 2 * rhs),
                _ => None,
            }
        }
        SnailfishToken::Close => None,
        SnailfishToken::Value(t) => Some(t),
    }
}
