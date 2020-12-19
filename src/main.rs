use regex::Regex;
use std::collections::HashMap;

fn main() {
    // Switch between data.txt and datab.txt to get the results of the first and second steps
    let data = String::from_utf8_lossy(include_bytes!("datab.txt")).replace("\"", "");
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

    let regexes: HashMap<usize, String> = rules
        .iter()
        .map(|(k, v)| (*k, solve(&rules, v).replace("(a)", "a").replace("(b)", "b")))
        .collect();
    let regex0 = Regex::new(&format!("^{}$", regexes.get(&0).unwrap())).unwrap();
    println!(
        "Data lines matching rule 0: {}",
        data.iter().filter(|x| regex0.is_match(x)).count()
    );
}

static mut COUNT8: i32 = 0;
static mut COUNT11: i32 = 0;

fn solve(rules: &HashMap<usize, &str>, rule: &str) -> String {
    rule.split(" ")
        .map(|x| {
            if x.parse::<usize>().is_ok() {
                let idx = x.parse::<usize>().unwrap();
                unsafe {
                    if idx == 8 {
                        if COUNT8 == 5 {
                            COUNT8 = 0;
                            return "".to_string();
                        } else {
                            COUNT8 += 1;
                        }
                    }
                    if idx == 11 {
                        if COUNT11 == 5 {
                            COUNT11 = 0;
                            return "".to_string();
                        } else {
                            COUNT11 += 1;
                        }
                    }
                }
                format!("({})", solve(rules, rules.get(&idx).unwrap()))
            } else {
                x.to_string()
            }
        })
        .collect::<String>()
}
