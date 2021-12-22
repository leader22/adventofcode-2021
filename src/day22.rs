// const INPUTS: &str = r#"
// on x=-20..26,y=-36..17,z=-47..7
// on x=-20..33,y=-21..23,z=-26..28
// on x=-22..28,y=-29..23,z=-38..16
// on x=-46..7,y=-6..46,z=-50..-1
// on x=-49..1,y=-3..46,z=-24..28
// on x=2..47,y=-22..22,z=-23..27
// on x=-27..23,y=-28..26,z=-21..29
// on x=-39..5,y=-6..47,z=-3..44
// on x=-30..21,y=-8..43,z=-13..34
// on x=-22..26,y=-27..20,z=-29..19
// off x=-48..-32,y=26..41,z=-47..-37
// on x=-12..35,y=6..50,z=-50..-2
// off x=-48..-32,y=-32..-16,z=-15..-5
// on x=-18..26,y=-33..15,z=-7..46
// off x=-40..-22,y=-38..-28,z=23..41
// on x=-16..35,y=-41..10,z=-47..6
// off x=-32..-23,y=11..30,z=-14..3
// on x=-49..-5,y=-3..45,z=-29..18
// off x=18..30,y=-20..-8,z=-3..13
// on x=-41..9,y=-7..43,z=-33..15
// on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
// on x=967..23432,y=45373..81175,z=27513..53682
// "#;
const INPUTS: &str = include_str!("./inputs/day22.txt");

use std::collections::HashMap;

pub fn run() {
    let range = (-50, 50);
    let inputs = parse_inputs(INPUTS.trim(), &range);
    // println!("{:?}", inputs);

    let mut cubes: HashMap<(i32, i32, i32), bool> = HashMap::new();

    for (state, (rx, ry, rz)) in inputs {
        for x in rx.0..=rx.1 {
            for y in ry.0..=ry.1 {
                for z in rz.0..=rz.1 {
                    cubes.insert((x, y, z), state);
                }
            }
        }
    }

    let answer = cubes.values().filter(|x| **x).count();
    println!("answer: {}", answer);
}
type Range = (i32, i32);

fn parse_inputs(s: &str, range: &Range) -> Vec<(bool, (Range, Range, Range))> {
    let mut parsed = vec![];
    for line in s.lines() {
        let (state, ranges) = line.trim().split_once(" ").unwrap();
        let ranges_xyz = ranges
            .split(",")
            .map(|r| {
                let (_, range) = r.split_once("=").unwrap();
                let (from, to) = range.split_once("..").unwrap();
                (from.parse::<i32>().unwrap(), to.parse::<i32>().unwrap())
            })
            .collect::<Vec<_>>();

        if ranges_xyz
            .iter()
            .all(|(from, to)| &range.0 <= from && to <= &range.1)
        {
            parsed.push((state == "on", (ranges_xyz[0], ranges_xyz[1], ranges_xyz[2])))
        }
    }
    parsed
}
