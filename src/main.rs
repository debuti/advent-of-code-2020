use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Op {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
struct CompoundRule {
    name: String,
    first: SimpleRule,
    op: Op,
    second: SimpleRule,
}
impl CompoundRule {
    fn check(&self, v: u32) -> bool {
        match self.op {
            Op::Or => self.first.check(v) || self.second.check(v),
            Op::And => self.first.check(v) && self.second.check(v),
        }
    }
}

#[derive(Debug, PartialEq)]
struct SimpleRule {
    lower: u32,
    higher: u32,
}
impl SimpleRule {
    fn check(&self, v: u32) -> bool {
        self.lower <= v && v <= self.higher
    }
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<_> = data.split("\n\n").filter(|x| x.len() > 0).collect();
    let mut rules: Vec<CompoundRule> = data[0]
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            match Regex::new(r"(.*): (\d+)-(\d+) (.*+) (\d+)-(\d+)")
                .unwrap()
                .captures(&x)
            {
                Some(x) => CompoundRule {
                    name: x.get(1).unwrap().as_str().to_string(),
                    first: SimpleRule {
                        lower: x.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                        higher: x.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                    },
                    op: match x.get(4).unwrap().as_str() {
                        "or" => Op::Or,
                        "and" => Op::And,
                        _ => unreachable!(),
                    },
                    second: SimpleRule {
                        lower: x.get(5).unwrap().as_str().parse::<u32>().unwrap(),
                        higher: x.get(6).unwrap().as_str().parse::<u32>().unwrap(),
                    },
                },
                None => unreachable!(),
            }
        })
        .collect();
    let my: Vec<u32> = data[1]
        .split("\n")
        .skip(1)
        .filter(|x| x.len() > 0)
        .map(|x| x.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
        .pop()
        .unwrap();
    let nearby: Vec<Vec<u32>> = data[2]
        .split("\n")
        .skip(1)
        .filter(|x| x.len() > 0)
        .map(|x| x.split(",").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    // Step 1
    let mut values_not_valid: Vec<u32> = Vec::new();
    let mut valid_nearby: Vec<Vec<u32>> = Vec::new();
    for ticket in &nearby {
        let mut valid = true;
        for num in ticket {
            let mut count = 0;
            for rule in &rules {
                if !rule.check(*num) {
                    count += 1;
                }
            }
            if count == rules.len() {
                values_not_valid.push(*num);
                valid = false;
            }
        }
        if valid {
            valid_nearby.push(ticket.to_vec());
        }
    }

    println!(
        "Answer to first question {:#?}",
        values_not_valid.into_iter().sum::<u32>()
    );

    // Step 2
    let fields = transpose(valid_nearby);
    let mut result: HashMap<String, usize> = HashMap::new();
    while rules.len() > 0 {
        for (fieldidx, field) in fields.iter().enumerate() {
            let mut df = rules
                .iter()
                .map(|rule| {
                    if field
                        .iter()
                        .map(|&x| rule.check(x))
                        .filter(|&x| x == true)
                        .count()
                        == field.len()
                    {
                        return Some(rule);
                    }
                    None
                })
                .filter(|&x| if let Some(_) = x { true } else { false })
                .collect::<Vec<Option<&CompoundRule>>>();
            // If only one rule matches the field
            if df.len() == 1 {
                let selectedrule = df[0].unwrap();
                result.insert(selectedrule.name.clone(), fieldidx);
                rules.remove(rules.iter().position(|x| x == selectedrule).unwrap());
            }
        }
    }
    println!(
        "Answer to second question {:?}",
        result
            .iter()
            .filter(|&(k, _)| k.starts_with("departure"))
            .map(|(_, v)| my[*v] as u64)
            .product::<u64>()
    );
}

/// Took from https://stackoverflow.com/a/64499219/219355
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
