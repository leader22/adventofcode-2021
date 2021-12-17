// const INPUTS: &str = "target area: x=20..30, y=-10..-5";
const INPUTS: &str = include_str!("./inputs/day17.txt");

pub fn run() {
    let target_area = parse_inputs(INPUTS.trim());
    println!("target: {:?}", target_area);

    let mut answer = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            match fire_probe(x, y, &target_area) {
                Some(res) => {
                    answer = res;
                }
                None => {}
            }
        }
    }

    println!("answer: {}", answer);
}

type MinMax = (i32, i32);
type TargetArea = (MinMax, MinMax);

fn parse_inputs(inputs: &str) -> TargetArea {
    let area = inputs.trim_start_matches("target area: ");
    let (x, y) = area.split_once(", ").unwrap();

    (parse_area(x), parse_area(y))
}

fn parse_area(s: &str) -> MinMax {
    let (_, range) = s.split_once("=").unwrap();
    let (min, max) = range.split_once("..").unwrap();
    let min = min.parse::<i32>().unwrap();
    let max = max.parse::<i32>().unwrap();
    (min, max)
}

fn fire_probe(x: i32, y: i32, ((min_x, max_x), (min_y, max_y)): &TargetArea) -> Option<i32> {
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut vel_x = x;
    let mut vel_y = y;
    let mut highest = 0;
    while cur_x <= *max_x && cur_y >= *min_y {
        cur_x += vel_x;
        cur_y += vel_y;
        vel_x -= vel_x.signum();
        vel_y -= 1;

        highest = std::cmp::max(cur_y, highest);

        if cur_x >= *min_x && cur_x <= *max_x && cur_y >= *min_y && cur_y <= *max_y {
            return Some(highest);
        }
    }

    None
}
