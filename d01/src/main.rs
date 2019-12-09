use file_argument;
use parse_input;

fn main() {
    let input_file = file_argument::only_file_arg("d01");
    let masses: Vec<i64> = parse_input::i64_file(&input_file).unwrap();
    println!("part one: {}", part_one(&masses));
    println!("part two: {}", part_two(&masses));
}

fn part_one(modules: &Vec<i64>) -> i64 {
    let mut fuel = 0;
    for m in modules {
        fuel += calculate_fuel(*m);
    }
    fuel
}

fn part_two(modules: &Vec<i64>) -> i64 {
    let mut fuel = 0;
    for m in modules {
        let f = calculate_fuel(*m);
        let mut ff = 0;
        let mut c = f;
        loop {
            let a = calculate_fuel(c);
            if a <= 0 {
                break;
            }
            ff += a;
            c = a;
        }
        fuel += f + ff;
    }
    fuel
}

fn calculate_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

#[test]
fn test_part_one() {
    let mut result = part_one(&vec![12]);
    assert_eq!(2, result);

    result = part_one(&vec![14]);
    assert_eq!(2, result);

    result = part_one(&vec![1969]);
    assert_eq!(654, result);

    result = part_one(&vec![100756]);
    assert_eq!(33583, result);

    result = part_one(&parse_input::i64_file("input").unwrap());
    assert_eq!(3342050, result);
}

#[test]
fn test_part_two() {
    let mut result = part_two(&vec![14]);
    assert_eq!(2, result);

    result = part_two(&vec![1969]);
    assert_eq!(966, result);

    result = part_two(&vec![100756]);
    assert_eq!(50346, result);

    result = part_two(&parse_input::i64_file("input").unwrap());
    assert_eq!(5010211, result);
}
