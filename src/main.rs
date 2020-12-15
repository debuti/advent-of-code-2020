use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<_> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let fnts: Vec<fn(Vec<u32>, usize) -> (u32, Duration)> = vec![turnsv2, turnsv1, turnsv0];
    for until in &[2020, 30_000_000] {
        for fnt in &fnts {
            println!("{:?}", fnt(data.clone(), *until as usize));
        }
    }
}

fn turnsv0(mut data: Vec<u32>, until: usize) -> (u32, Duration) {
    let start = Instant::now();
    while data.len() < until {
        let lastidx = data.len() - 1;
        let last = data[lastidx];
        data.push(if data.iter().filter(|&x| *x == last).count() == 1 {
            0
        } else {
            (lastidx - data[..lastidx].iter().rposition(|&x| x == last).unwrap()) as u32
        });
    }
    (*data.last().unwrap(), start.elapsed())
}

fn turnsv1(mut data: Vec<u32>, until: usize) -> (u32, Duration) {
    let start = Instant::now();
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
    (*data.last().unwrap(), start.elapsed())
}

fn turnsv2(data: Vec<u32>, until: usize) -> (u32, Duration) {
    let start = Instant::now();
    let mut counts: HashMap<u32, u32> = data.iter().map(|&x| (x, 1)).collect();
    let mut lasts: HashMap<u32, usize> =
        data.iter().enumerate().map(|(idx, &x)| (x, idx)).collect();
    let mut counter = data.len();
    let mut last = *data.last().unwrap();
    while counter < until {
        let it = if *counts.get(&last).unwrap() == 1 {
            0
        } else {
            (counter - 1 - lasts.get(&(last as u32)).unwrap()) as u32
        };
        *lasts.entry(last).or_insert(0) = counter - 1;
        *counts.entry(it).or_insert(0) += 1;
        last = it;
        counter += 1;
    }
    (last, start.elapsed())
}
