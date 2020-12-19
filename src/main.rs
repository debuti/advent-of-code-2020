use regex::Regex;
use std::collections::HashMap;

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt")).replace("\"", "");
    let data: Vec<Vec<&str>> = data
        .split("\n\n")
        .map(|x| x.split("\n").filter(|x| x.len() > 0).collect())
        .collect();
    assert_eq!(2, data.len());
    let rules: HashMap<usize, &str> = data[0]
        .iter()
        .map(|x| {
            (
                x.split(": ").next().unwrap().parse::<usize>().unwrap(),
                x.split(": ").nth(1).unwrap(),
            )
        })
        .collect();
    let data = &data[1];

    println!("{:?}", rules);
    println!("{:?}", data);

    let regexes: HashMap<usize, String> =
        rules.iter().map(|(k, v)| (*k, solve(&rules, v))).collect();
    println!("{:?}", regexes);
    println!(
        "Data lines matching rule 0: {}",
        data.iter()
            .filter(|x| {
                match Regex::new(&format!("^{}$", regexes.get(&0).unwrap()))
                    .unwrap()
                    .captures(x)
                {
                    Some(_) => true,
                    None => false,
                }
            },)
            .count()
    );
}

fn solve(rules: &HashMap<usize, &str>, rule: &str) -> String {
    rule.split(" ")
        .map(|x| {
            if x.parse::<usize>().is_ok() {
                format!(
                    "({})",
                    solve(rules, rules.get(&x.parse::<usize>().unwrap()).unwrap())
                )
            } else {
                x.to_string()
            }
        })
        .collect::<String>()
}
