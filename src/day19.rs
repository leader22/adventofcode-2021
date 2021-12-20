// const INPUTS: &str = r#"
// --- scanner 0 ---
// 404,-588,-901
// 528,-643,409
// -838,591,734
// 390,-675,-793
// -537,-823,-458
// -485,-357,347
// -345,-311,381
// -661,-816,-575
// -876,649,763
// -618,-824,-621
// 553,345,-567
// 474,580,667
// -447,-329,318
// -584,868,-557
// 544,-627,-890
// 564,392,-477
// 455,729,728
// -892,524,684
// -689,845,-530
// 423,-701,434
// 7,-33,-71
// 630,319,-379
// 443,580,662
// -789,900,-551
// 459,-707,401
//
// --- scanner 1 ---
// 686,422,578
// 605,423,415
// 515,917,-361
// -336,658,858
// 95,138,22
// -476,619,847
// -340,-569,-846
// 567,-361,727
// -460,603,-452
// 669,-402,600
// 729,430,532
// -500,-761,534
// -322,571,750
// -466,-666,-811
// -429,-592,574
// -355,545,-477
// 703,-491,-529
// -328,-685,520
// 413,935,-424
// -391,539,-444
// 586,-435,557
// -364,-763,-893
// 807,-499,-711
// 755,-354,-619
// 553,889,-390
//
// --- scanner 2 ---
// 649,640,665
// 682,-795,504
// -784,533,-524
// -644,584,-595
// -588,-843,648
// -30,6,44
// -674,560,763
// 500,723,-460
// 609,671,-379
// -555,-800,653
// -675,-892,-343
// 697,-426,-610
// 578,704,681
// 493,664,-388
// -671,-858,530
// -667,343,800
// 571,-461,-707
// -138,-166,112
// -889,563,-600
// 646,-828,498
// 640,759,510
// -630,509,768
// -681,-892,-333
// 673,-379,-804
// -742,-814,-386
// 577,-820,562
//
// --- scanner 3 ---
// -589,542,597
// 605,-692,669
// -500,565,-823
// -660,373,557
// -458,-679,-417
// -488,449,543
// -626,468,-788
// 338,-750,-386
// 528,-832,-391
// 562,-778,733
// -938,-730,414
// 543,643,-506
// -524,371,-870
// 407,773,750
// -104,29,83
// 378,-903,-323
// -778,-728,485
// 426,699,580
// -438,-605,-362
// -469,-447,-387
// 509,732,623
// 647,635,-688
// -868,-804,481
// 614,-800,639
// 595,780,-596
//
// --- scanner 4 ---
// 727,592,562
// -293,-554,779
// 441,611,-461
// -714,465,-776
// -743,427,-804
// -660,-479,-426
// 832,-632,460
// 927,-485,-438
// 408,393,-506
// 466,436,-512
// 110,16,151
// -258,-428,682
// -393,719,612
// -211,-452,876
// 808,-476,-593
// -575,615,604
// -485,667,467
// -680,325,-822
// -627,-443,-432
// 872,-547,-609
// 833,512,582
// 807,604,487
// 839,-516,451
// 891,-625,532
// -652,-548,-490
// 30,-46,-14
// "#;
const INPUTS: &str = include_str!("./inputs/day19.txt");

use std::collections::{HashMap, HashSet};

pub fn run() {
    let mut scanners = parse_inputs(INPUTS.trim());
    println!("{:?}", scanners);
    println!("----- {:?} scanners -----", scanners.len());

    let answer = count_beacons(&mut scanners);
    println!("answer: {:?}", answer);
}

type Point = (i64, i64, i64);
type Distance = Point;
type Scanner = HashSet<Point>;
const COMMON_BEACONS_COUNT: usize = 12;

fn parse_inputs(inputs: &str) -> Vec<Scanner> {
    let mut scanners = vec![];

    for scanner_lines in inputs.split("\n\n") {
        let mut scanner = Scanner::new();

        for line in scanner_lines.lines().skip(1) {
            let points = line
                .split(",")
                .flat_map(|x| x.parse::<i64>())
                .collect::<Vec<_>>();

            assert_eq!(3, points.len());
            scanner.insert((points[0], points[1], points[2]));
        }

        scanners.push(scanner);
    }

    return scanners;
}

fn count_beacons(scanners: &mut Vec<Scanner>) -> i64 {
    while scanners.len() > 1 {
        match try_merge_scanners(&scanners) {
            Some((base_scanner_index, merged_scanner_index, _, merged_scanner)) => {
                scanners.remove(merged_scanner_index);
                *scanners.get_mut(base_scanner_index).unwrap() = merged_scanner;
            }
            None => unreachable!(),
        }
    }

    assert_eq!(1, scanners.len());
    scanners[0].len() as i64
}

fn try_merge_scanners(scanners: &Vec<Scanner>) -> Option<(usize, usize, Distance, Scanner)> {
    let mut merge_with = None;
    for (base_scanner_index, base_scanner) in scanners.iter().enumerate() {
        for (scanner2_index, scanner2) in scanners.iter().enumerate().skip(base_scanner_index + 1) {
            for scanner2 in rotate_scanner(&scanner2) {
                match try_match_points(base_scanner, &scanner2) {
                    Some(distance) => {
                        let mut moved_scanner = move_scanner(&scanner2, distance);
                        for point in base_scanner {
                            moved_scanner.insert(*point);
                        }
                        merge_with =
                            Some((base_scanner_index, scanner2_index, distance, moved_scanner));
                        break;
                    }
                    None => continue,
                }
            }

            if merge_with.is_some() {
                break;
            }
        }

        if merge_with.is_some() {
            break;
        }
    }
    merge_with
}

fn move_scanner(scanner: &Scanner, distance: Distance) -> Scanner {
    scanner
        .iter()
        .map(|(x, y, z)| (x + distance.0, y + distance.1, z + distance.2))
        .collect()
}

fn try_match_points(scanner1: &Scanner, scanner2: &Scanner) -> Option<Distance> {
    let mut distances_point: HashMap<Distance, Scanner> = HashMap::new();
    for point2 in scanner2.iter().copied() {
        for point1 in scanner1.iter().copied() {
            let diff = (
                point1.0 - point2.0,
                point1.1 - point2.1,
                point1.2 - point2.2,
            );
            distances_point.entry(diff).or_default().insert(point2);
        }
    }

    for (distance, points) in distances_point {
        if points.len() >= COMMON_BEACONS_COUNT {
            return Some(distance);
        }
    }
    None
}

fn rotate_scanner(scanner: &Scanner) -> Vec<Scanner> {
    let rotations_axis = [
        |(x, y, z): Point| (x, y, z),
        |(x, y, z): Point| (x, z, y),
        |(x, y, z): Point| (y, x, z),
        |(x, y, z): Point| (y, z, x),
        |(x, y, z): Point| (z, x, y),
        |(x, y, z): Point| (z, y, x),
    ];
    let rotations_rotate = [
        |(x, y, z): Point| (x, y, z),
        |(x, y, z): Point| (-x, -y, z),
        |(x, y, z): Point| (x, -y, -z),
        |(x, y, z): Point| (-x, y, -z),
        |(x, y, z): Point| (-x, -y, -z),
        |(x, y, z): Point| (-x, y, z),
        |(x, y, z): Point| (x, -y, z),
        |(x, y, z): Point| (x, y, -z),
    ];

    let mut rotations = vec![];
    for axis_rotation in rotations_axis {
        for rotate_rotation in rotations_rotate {
            let rotated = scanner
                .iter()
                .copied()
                .map(|point| axis_rotation(rotate_rotation(point)))
                .collect::<Scanner>();
            rotations.push(rotated);
        }
    }
    rotations
}
