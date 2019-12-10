use file_argument;
use parse_input;

fn main() {
    let input_file = file_argument::only_file_arg("d02");
    let initial_state = parse_input::i64_csv(&input_file).unwrap();

    let mut program = initial_state.clone();
    program[1] = 12;
    program[2] = 2;
    let result = execute(&program);
    println!("part one: {}", result[0]);

    let nv = brute_force_result(&initial_state, 19690720);
    println!("part two: {}", 100 * nv.0 + nv.1);
}

fn execute(prog: &Vec<i64>) -> Vec<i64> {
    let mut p = prog.clone();
    let mut instp = 0;
    loop {
        match p[instp] {
            1 => {
                let lpos = p[instp + 1] as usize;
                let rpos = p[instp + 2] as usize;
                let res = p[instp + 3] as usize;
                p[res] = p[lpos] + p[rpos];
                instp += 4;
            }
            2 => {
                let lpos = p[instp + 1] as usize;
                let rpos = p[instp + 2] as usize;
                let res = p[instp + 3] as usize;
                p[res] = p[lpos] * p[rpos];
                instp += 4;
            }
            99 => break,
            _ => {
                panic!("invalid opcode {} -- aborting", p[instp]);
            }
        }
    }
    p
}

fn brute_force_result(prog: &Vec<i64>, target: i64) -> (i64, i64) {
    for i in 0..100 {
        for j in 0..100 {
            let mut p = prog.clone();
            p[1] = i;
            p[2] = j;
            if execute(&p)[0] == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}

#[test]
fn test_part_one_examples() {
    let tests: Vec<(Vec<i64>, Vec<i64>)> = vec![
        (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
        (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
        (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
        (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
    ];

    for test in tests {
        assert_eq!(test.1, execute(&test.0), "left=expected, right=actual");
    }
}

#[test]
fn test_part_one() {
    let mut program = parse_input::i64_csv("input").unwrap();
    program[1] = 12;
    program[2] = 2;
    assert_eq!(5866663, execute(&program)[0]);
}

#[test]
fn test_part_two() {
    let program = parse_input::i64_csv("input").unwrap();
    let nv = brute_force_result(&program, 19690720);
    assert_eq!(4259, 100 * nv.0 + nv.1);
}
