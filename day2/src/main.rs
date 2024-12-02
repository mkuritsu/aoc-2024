use std::fs::read_to_string;

fn main() {
    let buf = read_to_string("input.txt").unwrap();
    let lines = buf.lines();
    let mut numb_safe = 0;
    'line_loop: for line in lines {
        let numbs: Vec<i32> = line
            .split_whitespace()
            .map(|x| i32::from_str_radix(x, 10).unwrap())
            .collect();
        let mut dif = numbs[1] - numbs[0];
        let mut prev = numbs[1];
        if dif.abs() < 1 || dif.abs() > 3 {
            continue;
        }
        for n in numbs.iter().skip(2) {
            let prev_dif = n - prev;
            let dist = prev_dif.abs();
            prev = *n;
            if dist < 1 || dist > 3 || prev_dif * dif < 0 {
                continue 'line_loop;
            }
            dif = prev_dif;
        }
        numb_safe += 1;
    }
    println!("Part1 solution: {numb_safe}");
}
