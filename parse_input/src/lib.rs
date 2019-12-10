use std::fs::File;
use std::io::prelude::*;

pub fn file(p: &str) -> std::io::Result<Vec<String>> {
    let mut input = String::new();
    let mut f = File::open(p)?;
    f.read_to_string(&mut input)?;

    let r: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    Ok(r)
}

pub fn i64_file(p: &str) -> std::io::Result<Vec<i64>> {
    let strings = file(p)?;
    let mut nums: Vec<i64> = Vec::new();
    for s in strings {
        match s.parse::<i64>() {
            Ok(i) => nums.push(i),
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
    Ok(nums)
}

pub fn i64_csv(p: &str) -> std::io::Result<Vec<i64>> {
    let lines = file(p)?;
    let mut nums: Vec<i64> = Vec::new();
    for n in lines[0].split(",") {
        match n.parse::<i64>() {
            Ok(v) => nums.push(v),
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
    Ok(nums)
}

#[test]
fn test_file() {
    let output = file("./tests/strings.txt").unwrap();
    assert_eq!(output, ["one", "two"]);
}

#[test]
fn test_i64_file() {
    let output = i64_file("./tests/i64.txt").unwrap();
    assert_eq!(output, [104042, 112116, 57758, 139018, 105580]);
}

#[test]
fn test_i64_csv() {
    let output = i64_csv("./tests/i64.csv").unwrap();
    assert_eq!(output, [104042, 112116, 57758, 139018, 105580]);
}
