fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt")).replace(" ", "");
    let data: Vec<&str> = data.split("\n").filter(|x| x.len() > 0).collect();

    let mut acc = 0;
    for eq in &data {
        let res = solve1(&eq);
        acc += res;
        println!("{} solves to {}", eq, res);
    }
    println!("Solution {}", acc);

    println!("");

    let mut acc = 0;
    for eq in &data {
        let res = solve2(&eq);
        acc += res;
        println!("{} solves to {}", eq, res);
    }
    println!("Solution {}", acc);
}

fn find_closing(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (idx, c) in s.chars().enumerate() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(idx);
                }
            }
            _ => (),
        }
    }
    None
}

fn find_1st_level_op(s: &str, needles: Vec<char>, rev: bool) -> Option<usize> {
    let mut depth = 0;
    let mut iter = s.chars().enumerate().collect::<Vec<(usize, char)>>();
    if rev {
        iter = iter
            .iter()
            .rev()
            .map(|&x| x)
            .collect::<Vec<(usize, char)>>();
    }
    for (idx, c) in iter {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            a if needles.contains(&a) => {
                if depth == 0 {
                    return Some(idx);
                }
            }
            _ => (),
        }
    }
    None
}

fn solve1(s: &str) -> i64 {
    if s.starts_with("(") {
        let closingidx = find_closing(s).unwrap();
        if closingidx == s.len() - 1 {
            return solve1(&s[1..s.len() - 1]);
        }
    }
    let opidx = find_1st_level_op(s, vec!['*', '+'], true);
    match opidx {
        Some(idx) => match s.chars().nth(idx).unwrap() {
            '*' => solve1(&s[0..idx]) * solve1(&s[idx + 1..]),
            '+' => solve1(&s[0..idx]) + solve1(&s[idx + 1..]),
            _ => unreachable!(),
        },
        None => s.parse::<i64>().unwrap(),
    }
}

fn solve2(s: &str) -> i64 {
    if s.starts_with("(") {
        let closingidx = find_closing(s).unwrap();
        if closingidx == s.len() - 1 {
            return solve2(&s[1..s.len() - 1]);
        }
    }
    let prodidx = find_1st_level_op(s, vec!['*'], false);
    match prodidx {
        Some(idx) => solve2(&s[0..idx]) * solve2(&s[idx + 1..]),
        None => {
            let sumidx = find_1st_level_op(s, vec!['+'], false);
            match sumidx {
                Some(idx) => solve2(&s[0..idx]) + solve2(&s[idx + 1..]),
                None => s.parse::<i64>().unwrap(),
            }
        }
    }
}
