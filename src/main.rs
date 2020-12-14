use regex::Regex;
use std::collections::HashMap;

struct Mask(u64);
impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:#036b}", self.0)
    }
}

#[derive(Debug)]
enum Instr {
    Masks(
        Mask, //andmask
        Mask, //ormask
    ),
    Mem(
        u64, //addr
        u64, //value
    ),
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<&str> = data.split('\n').filter(|x| x.len() > 0).collect();

    step1(&data);
    step2(&data);
}

fn step1(data: &Vec<&str>) {
    let data: Vec<Instr> = data
        .iter()
        .map(|x| match Regex::new(r"mask = (.*)").unwrap().captures(x) {
            Some(x) => {
                let v = x.get(1).unwrap().as_str();
                Instr::Masks(
                    v.chars()
                        .map(|x| if x == '0' { 0u64 } else { 1u64 })
                        .fold(Mask(0), |acc, elem| Mask(acc.0 * 2 + elem)),
                    v.chars()
                        .map(|x| if x == '1' { 1u64 } else { 0u64 })
                        .fold(Mask(0), |acc, elem| Mask(acc.0 * 2 + elem)),
                )
            }
            None => match Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap().captures(x) {
                Some(x) => {
                    let a = x.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let v = x.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    Instr::Mem(a, v)
                }
                None => unreachable!(),
            },
        })
        .collect();

    let mut candmask = Mask(0b111111111111111111111111111111111111);
    let mut cormask = Mask(0b000000000000000000000000000000000000);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for instr in data {
        match instr {
            Instr::Masks(and, or) => {
                candmask = and;
                cormask = or;
            }
            Instr::Mem(addr, value) => {
                *memory.entry(addr).or_insert(0u64) = value & candmask.0 | cormask.0
            }
        }
    }
    println!("The 1st step sum is {:?}", memory.values().sum::<u64>());
}

#[derive(Debug)]
enum Instr2 {
    Mask(String),
    Mem(
        u64, //addr
        u64, //value
    ),
}

fn step2(data: &Vec<&str>) {
    let data: Vec<Instr2> = data
        .iter()
        .map(|x| match Regex::new(r"mask = (.*)").unwrap().captures(x) {
            Some(x) => Instr2::Mask(x.get(1).unwrap().as_str().to_string()),
            None => match Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap().captures(x) {
                Some(x) => Instr2::Mem(
                    x.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    x.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                ),
                None => unreachable!(),
            },
        })
        .collect();

    let mut cmask = "000000000000000000000000000000000000".to_owned();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for instr in data {
        match instr {
            Instr2::Mask(s) => cmask = s,
            Instr2::Mem(addr, value) => {
                let mask: String = cmask
                    .chars()
                    .zip(format!("{:036b}", addr).chars())
                    .map(|x| {
                        if x.0 == 'X' {
                            x.0
                        } else if x.0 == '0' {
                            x.1
                        } else {
                            '1'
                        }
                    })
                    .collect();
                let w: u32 = mask.chars().filter(|&x| x == 'X').count() as u32;
                for i in 0..2u32.pow(w) {
                    let mut addr = mask.clone();
                    for c in (0..w).rev() {
                        let stuff = format!("{:#b}", i >> c);
                        addr = addr.replacen("X", &stuff.chars().last().unwrap().to_string(), 1);
                    }
                    *memory
                        .entry(u64::from_str_radix(&addr, 2).unwrap())
                        .or_insert(0u64) = value;
                }
            }
        }
    }
    println!("The 2nd step sum is {:?}", memory.values().sum::<u64>());
}
