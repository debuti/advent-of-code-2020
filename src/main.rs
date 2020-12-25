fn main() {
    let _test = vec![5764801, 17807724];
    let _data = vec![5290733, 15231938];
    println!("Answer: {}", transf(_data[0], cracktransf(7, _data[1])));
}

fn transf(subject: u128, loops: usize) -> u128 {
    let mut v = 1;
    let mut r = 0;
    while r != loops {
        r += 1;
        v *= subject;
        v %= 20201227;
    }
    return v;
}

fn cracktransf(subject: u128, target: u128) -> usize {
    let mut v = 1;
    let mut r = 0;
    loop {
        r += 1;
        v *= subject;
        v %= 20201227;
        if v == target {
            return r;
        }
    }
}
