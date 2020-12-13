use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<&str> = data.split("\n").filter(|x| x.len() > 0).collect();

    // First part
    let deptime: u64 = data[0].parse().unwrap();
    let sched: Vec<u64> = data[1]
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    let rem: Vec<u64> = sched.iter().map(|x| x - deptime % x).collect();
    let idx = rem
        .iter()
        .position(|s| s == rem.iter().min().unwrap())
        .unwrap();
    println!(
        "The answer to the first question is {}",
        sched[idx] * rem[idx]
    );

    // Second part
    let sched: Vec<u64> = data[1]
        .split(',')
        .map(|x| if x != "x" { x.parse().unwrap() } else { 1 })
        .collect();
    // It has to happen between zero and sched.product(). Take advantage of the hint in the question
    let range = (100_000_000_000_000u64, sched.iter().product::<u64>());
    let found = Arc::new(AtomicBool::new(false));
    let width: u64 = (range.1 - range.0) / num_cpus::get() as u64;
    let mut threads = Vec::new();
    for tid in 1..num_cpus::get() + 1 {
        let sched = sched.clone();
        let found = found.clone();
        threads.push(thread::spawn(move || {
            let from = range.0 + width * (tid as u64 - 1);
            let to = range.0 + width * tid as u64;
            //println!("tid: {} start: {} end: {}", tid, from, to);
            let mut idx: u64 = from - from % sched[0];
            'outer: loop {
                idx += sched[0];
                if (idx / sched[0]) % 1000000 == 0 && found.load(Ordering::Relaxed) {
                    return;
                }
                for ptr in 1..sched.len() {
                    if sched[ptr] > 1 // Little optimization to save some time on the anytime buses
                     && (idx + ptr as u64) % sched[ptr] != 0
                    {
                        continue 'outer;
                    }
                }
                println!("It happened at {:?}", idx);
                found.swap(true, Ordering::Relaxed);
                return;
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
