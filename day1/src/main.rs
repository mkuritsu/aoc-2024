use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let buf = read_to_string("input.txt").unwrap();
    let lines = buf.lines();
    let mut vec1: Vec<i64> = vec![];
    let mut vec2: Vec<i64> = vec![];
    for line in lines {
        let col: Vec<&str> = line.trim().split_whitespace().collect();
        let fst = i64::from_str_radix(col[0], 10).unwrap();
        let snd = i64::from_str_radix(col[1], 10).unwrap();
        vec1.push(fst);
        vec2.push(snd);
    }
    vec1.sort();
    vec2.sort();
    part1(&vec1, &vec2);
    part2(&vec1, &vec2);
}

fn part1(vec1: &Vec<i64>, vec2: &Vec<i64>) {
    let mut dist = 0;
    for (a, b) in vec1.iter().zip(vec2.iter()) {
        println!("{} {}", a, b);
        dist += (b - a).abs();
    }
    println!("Part1 Solution: {dist}");
}

fn part2(vec1: &Vec<i64>, vec2: &Vec<i64>) {
    let mut map = HashMap::new();
    for n in vec2 {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }
    let mut result: i64 = 0;
    for n in vec1 {
        result += match map.get(n) {
            Some(a) => n * a,
            None => 0,
        }
    }
    println!("Part2 Solution: {result}");
}
