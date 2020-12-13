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
    // It is pretty slow so it will need to be ran in threads
    let mut idx: u64 = 100_000_000_000_000 - 100_000_000_000_000 % sched[0];
    'outer: loop {
        idx += sched[0];
        for ptr in 1..sched.len() {
            if sched[ptr] > 1 // Little optimization to save some time on the anytime buses
             && (idx + ptr as u64) % sched[ptr] != 0
            {
                continue 'outer;
            }
        }
        println!("It happened at {:?}", idx);
        break 'outer;
    }
}
