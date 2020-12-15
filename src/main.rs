use std::collections::HashMap;
fn main() {
    let data = String::from_utf8_lossy(include_bytes!("test.txt"));
    let data: Vec<_> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", turns(data.clone(), 2020));
    println!("{:?}", turns(data.clone(), 30_000_000));
}

fn turns(mut data: Vec<u32>, until: usize) -> u32 {
    let mut counts: HashMap<u32, u32> = data.iter().map(|&x| (x, 1)).collect();
    while data.len() < until {
        let lastidx = data.len() - 1;
        let last = data[lastidx];
        data.push(if *counts.get(&last).unwrap() == 1 {
            0
        } else {
            (lastidx - data[..lastidx].iter().rposition(|&x| x == last).unwrap()) as u32
        });
        *counts.entry(*data.last().unwrap()).or_insert(0) += 1;
    }
    *data.last().unwrap()
}
