// const INPUTS: &str = r#"
// Player 1 starting position: 4
// Player 2 starting position: 8
// "#;
const INPUTS: &str = include_str!("./inputs/day21.txt");

pub fn run() {
    let (mut pos1, mut pos2) = parse_inputs(INPUTS.trim());
    println!("pos1: {}, pos2: {}", pos1, pos2);

    let mut dice = Dice100::new();
    let mut total_score1 = 0;
    let mut total_score2 = 0;
    loop {
        let r1 = dice.roll(3);
        let s1 = get_score_on_10_spaces(&pos1, &r1.iter().sum());
        total_score1 += s1;
        pos1 = s1;
        println!(
            "Player 1 rolls {:?} and move to space {} for a total score of {}",
            r1, s1, total_score1
        );

        if total_score1 >= 1000 {
            println!(
                "P1 wins! {} x {} = {}",
                total_score2,
                dice.rolled,
                total_score2 * dice.rolled
            );
            break;
        }

        let r2 = dice.roll(3);
        let s2 = get_score_on_10_spaces(&pos2, &r2.iter().sum());
        total_score2 += s2;
        pos2 = s2;
        println!(
            "Player 2 rolls {:?} and move to space {} for a total score of {}",
            r2, s2, total_score2
        );

        if total_score2 >= 1000 {
            println!(
                "P2 wins! {} x {} = {}",
                total_score1,
                dice.rolled,
                total_score1 * dice.rolled
            );
            break;
        }
    }
}

fn parse_inputs(inputs: &str) -> (u32, u32) {
    let (p1, p2) = inputs.split_once("\n").unwrap();
    let p1 = p1.trim_start_matches("Player 1 starting position: ");
    let p2 = p2.trim_start_matches("Player 2 starting position: ");

    let pos1 = p1.parse::<u32>().unwrap();
    let pos2 = p2.parse::<u32>().unwrap();

    (pos1, pos2)
}

struct Dice100 {
    count: u32,
    pub rolled: u32,
}
impl Dice100 {
    fn new() -> Self {
        Dice100 {
            count: 1,
            rolled: 0,
        }
    }

    fn roll(&mut self, times: u32) -> Vec<u32> {
        let mut result = vec![];
        for _ in 0..times {
            result.push(self.count);
            self.count += 1;

            if self.count > 100 {
                self.count = 1;
            }
        }

        self.rolled += times;
        result
    }
}

fn get_score_on_10_spaces(cur: &u32, m: &u32) -> u32 {
    let score = (cur + m) % 10;

    if score == 0 {
        return 10;
    }
    score
}
