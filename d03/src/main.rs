#![allow(dead_code)]

use file_argument;
use parse_input;

fn main() {
    let input_file = file_argument::only_file_arg("d03");
    let lines = parse_input::file(&input_file).unwrap();

    println!("parsing course one");
    let course_one = parse_course(&lines[0]);

    println!("parsing course two");
    let course_two = parse_course(&lines[1]);

    let mut wire_one = Wire::new();
    println!("plotting wire one");
    wire_one.plot(&course_one);
    println!("wire one has {} coordinates", wire_one.path.len());

    let mut wire_two = Wire::new();
    println!("plotting wire two");
    wire_two.plot(&course_two);
    println!("wire two has {} coordinates", wire_two.path.len());

    println!(
        "finding intersections with {} comparisons",
        wire_one.path.len() * wire_two.path.len()
    );
    let i = intersections(&wire_one, &wire_two);
    println!("found {} intersections", i.len());

    println!("finding min manhattan distance of intersections");
    let md = min_manhattan_distance(&i);

    let wd = min_wire_distance(&i);

    println!("part_one: {}", md);
    println!("part two: {}", wd);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Move {
    direction: Direction,
    distance: u32,
}

impl Move {
    fn new(dir: Direction, dist: u32) -> Move {
        Move {
            direction: dir,
            distance: dist,
        }
    }
}

type Course = Vec<Move>;

fn parse_course(line: &str) -> Course {
    let mut path: Course = Vec::new();
    for elem in line.split(",") {
        let n = Move::new(
            match elem.chars().collect::<Vec<char>>().first().unwrap() {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("bad direction: '{}' for element {}", &elem[..0], elem),
            },
            elem[1..].parse::<u32>().unwrap(),
        );
        path.push(n);
    }
    path
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate::with_position(0, 0)
    }

    fn with_position(x: i32, y: i32) -> Coordinate {
        Coordinate { x: x, y: y }
    }

    fn tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}:{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Wire {
    position: Coordinate,
    path: Vec<Coordinate>,
}

impl Wire {
    fn new() -> Wire {
        Wire {
            position: Coordinate::new(),
            path: vec![Coordinate::with_position(0, 0)],
        }
    }

    fn mv(&mut self, m: &Move) {
        for _ in 0..m.distance {
            let c = match m.direction {
                Direction::Up => Coordinate::with_position(self.position.x, self.position.y + 1),
                Direction::Down => Coordinate::with_position(self.position.x, self.position.y - 1),
                Direction::Left => Coordinate::with_position(self.position.x - 1, self.position.y),
                Direction::Right => Coordinate::with_position(self.position.x + 1, self.position.y),
            };
            self.position = c;
            self.path.push(c);
        }
    }

    fn plot(&mut self, c: &Course) {
        for m in c {
            self.mv(m);
        }
    }

    fn crossed(&self, point: &Coordinate) -> Option<usize> {
        for i in 0..self.path.len() {
            if self.path[i].tuple() == point.tuple() {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Intersection {
    position: Coordinate,
    moves: usize,
}

impl Intersection {
    fn new(c: Coordinate, m: usize) -> Intersection {
        Intersection {
            position: c,
            moves: m,
        }
    }
}

fn intersections(a: &Wire, b: &Wire) -> Vec<Intersection> {
    let mut intersections = Vec::new();
    for i in 0..a.path.len() {
        if a.path[i].x == 0 && a.path[i].y == 0 {
            continue;
        }
        if let Some(intsc) = b.crossed(&a.path[i]) {
            intersections.push(Intersection::new(a.path[i].clone(), i + intsc));
        }
    }
    intersections
}

fn manhattan_distance(c: &Intersection) -> usize {
    c.position.x.abs() as usize + c.position.y.abs() as usize
}

fn min_manhattan_distance(v: &Vec<Intersection>) -> usize {
    if v.len() == 0 {
        return 0;
    }
    let mut min = manhattan_distance(&v[0]);
    for c in v {
        let d = manhattan_distance(&c);
        if d < min {
            min = d;
        }
    }
    min
}

fn min_wire_distance(v: &Vec<Intersection>) -> usize {
    if v.len() == 0 {
        return 0;
    }
    let mut min = v[0].moves;
    for intsc in v {
        if intsc.moves < min {
            min = intsc.moves;
        }
    }
    min
}

#[test]
fn test_parse_course() {
    let tests: Vec<(&str, Course)> = vec![
        (
            "R1",
            vec![Move {
                direction: Direction::Right,
                distance: 1,
            }],
        ),
        (
            "R1,L2",
            vec![
                Move {
                    direction: Direction::Right,
                    distance: 1,
                },
                Move {
                    direction: Direction::Left,
                    distance: 2,
                },
            ],
        ),
        (
            "R1,L2,U234,D23423",
            vec![
                Move {
                    direction: Direction::Right,
                    distance: 1,
                },
                Move {
                    direction: Direction::Left,
                    distance: 2,
                },
                Move {
                    direction: Direction::Up,
                    distance: 234,
                },
                Move {
                    direction: Direction::Down,
                    distance: 23423,
                },
            ],
        ),
    ];

    for test in tests {
        assert_eq!(test.1, parse_course(&test.0));
    }
}

#[test]
fn test_plot() {
    let c = parse_course("U1,R2,D1,L2");
    let mut w = Wire::new();
    w.plot(&c);
    assert_eq!(7, w.path.len());
    assert_eq!(
        Coordinate::with_position(0, 0),
        w.path.last().cloned().unwrap(),
    );
    assert_eq!(3, w.crossed(&Coordinate::with_position(2, 1)).unwrap());
}

#[test]
fn test_intersections() {
    let c1 = parse_course("R8,U5,L5,D3");
    let c2 = parse_course("U7,R6,D4,L4");
    let mut w1 = Wire::new();
    w1.plot(&c1);
    let mut w2 = Wire::new();
    w2.plot(&c2);
    let intersections = intersections(&w1, &w2);
    assert_eq!(2, intersections.len());
    assert_eq!(
        Intersection::new(Coordinate::with_position(6, 5), 30),
        intersections[0]
    );
    assert_eq!(
        Intersection::new(Coordinate::with_position(3, 3), 40),
        intersections[1]
    );
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(
        6,
        manhattan_distance(&Intersection::new(Coordinate::with_position(3, 3), 0))
    );
    assert_eq!(
        6,
        manhattan_distance(&Intersection::new(Coordinate::with_position(3, -3), 0))
    );
    assert_eq!(
        6,
        manhattan_distance(&Intersection::new(Coordinate::with_position(-3, -3), 0))
    );
}

#[test]
fn test_min_manhattan_distance() {
    assert_eq!(
        6,
        min_manhattan_distance(&vec![
            Intersection::new(Coordinate::with_position(6, 5), 0),
            Intersection::new(Coordinate::with_position(3, 3), 0),
        ])
    );
}

#[test]
fn test_part_one_examples() {
    let tests: Vec<(&str, &str, usize)> = vec![
        (
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            159,
        ),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
        ),
    ];

    for test in tests {
        let mut w1 = Wire::new();
        w1.plot(&parse_course(test.0));
        let mut w2 = Wire::new();
        w2.plot(&parse_course(test.1));
        assert_eq!(test.2, min_manhattan_distance(&intersections(&w1, &w2)));
    }
}

#[test]
fn test_part_two_examples() {
    let tests: Vec<(&str, &str, usize)> = vec![
        (
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            610,
        ),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            410,
        ),
    ];

    for test in tests {
        let mut w1 = Wire::new();
        w1.plot(&parse_course(test.0));
        let mut w2 = Wire::new();
        w2.plot(&parse_course(test.1));
        let i = intersections(&w1, &w2);
        assert_eq!(test.2, min_wire_distance(&i));
    }
}
