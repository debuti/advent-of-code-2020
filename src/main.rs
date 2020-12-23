use std::convert::TryFrom;

const DEBUG: bool = true;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

#[derive(Debug)]
struct CircVec<T> {
    head: T,
    data: Vec<T>,
    capacity: usize,
    highestv: T,
}

// We need a trait which tells us the "one" value for a type
trait Increment {
    fn one() -> Self;
    fn zero() -> Self;
}

impl Increment for i32 {
    fn one() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
}

impl<
        T: Copy
            + PartialEq
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Rem<Output = T>
            + Increment
            + std::fmt::Debug,
    > CircVec<T>
{
    fn headpos(&self) -> usize {
        self.data.iter().position(|&x| x == self.head).unwrap()
    }

    fn pick3(&mut self) -> [T; 3] {
        let head = self.headpos() + 1;
        [
            if head < self.data.len() {
                self.data.remove(head)
            } else {
                self.data.remove(0)
            },
            if head < self.data.len() {
                self.data.remove(head)
            } else {
                self.data.remove(0)
            },
            if head < self.data.len() {
                self.data.remove(head)
            } else {
                self.data.remove(0)
            },
        ]
    }

fn destcup(&mut self) -> usize {
    let mut needle = self.head;
    loop {
        needle = ((needle - T::one()) + self.highestv) % (self.highestv);
        if needle == T::zero() {
            needle = self.highestv;
        }
        println!("blah: {:?}", needle);
        if self.data.contains(&needle) {
            println!("destination: {:?}", needle);
            return self.data.iter().position(|&x| x == needle).unwrap();
        }
    }
}

    fn reinsert3_and_move_head(&mut self, new: [T; 3]) {
        let pos = self.destcup();
        for item in new.iter().rev() {
            self.data.insert(pos + 1, *item);
        }
        self.head = self.data[(self.headpos() + 1) % self.data.len()];
    }
}

impl std::iter::FromIterator<i32> for CircVec<i32> {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut c = CircVec {
            data: Vec::new(),
            capacity: 0,
            head: 0,
            highestv: 0,
        };

        for i in iter {
            c.data.push(i);
        }
        c.capacity = c.data.len();
        c.highestv = *c.data.iter().max().unwrap();
        c.head = c.data[0];
        c
    }
}

fn main() {
    let _test = "389125467";
    let _data = "653427918";
    let mut data: CircVec<i32> = _data
        .chars()
        .map(|x| i32::try_from(x.to_digit(10).unwrap()).unwrap())
        .collect();

    for moveid in 1..101 {
        debugln!("-- move {} --", moveid);
        debugln!("cups: {:?}", data);
        let removed = data.pick3();
        debugln!("pick up: {:?}", removed);
        data.reinsert3_and_move_head(removed);
        debugln!("\n");
    }
    debugln!("-- final --");
    debugln!("cups: {:?}", data);
}
