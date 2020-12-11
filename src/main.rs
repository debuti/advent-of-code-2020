/*

*/
#[derive(PartialEq, Clone)]
enum LayoutPos {
    Floor,
    Empty,
    Occupied,
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<_> = data.split("\n").filter(|x| x.len() > 0).collect();

    //Layout is a vec of rows each being a vec holding columns
    let mut layout: Vec<Vec<LayoutPos>> = Vec::new();
    for line in data {
        let mut temp: Vec<LayoutPos> = Vec::new();
        for char in line.chars() {
            match char {
                '.' => temp.push(LayoutPos::Floor),
                'L' => temp.push(LayoutPos::Empty),
                '#' => temp.push(LayoutPos::Occupied),
                _ => unreachable!(),
            }
        }
        layout.push(temp);
    }
    //debuglayout(&layout);

    loop {
        if process(&mut layout) == 0 {
            break;
        }
        //debuglayout(&layout);
    }
    debuglayout(&layout);

    println!("The count of occupied seats is {}", countoccupied(&layout));
}
fn countoccupied(layout: &Vec<Vec<LayoutPos>>) -> u32 {
    let mut count = 0;
    for ridx in 0..layout.len() {
        for cidx in 0..layout[ridx].len() {
            if LayoutPos::Occupied == layout[ridx][cidx] {
                count += 1;
            }
        }
    }
    count
}

fn process(layout: &mut Vec<Vec<LayoutPos>>) -> u32 {
    let mut worklayout = layout.clone();
    let mut changes = 0;
    for ridx in 0..layout.len() {
        for cidx in 0..layout[ridx].len() {
            if layout[ridx][cidx] == LayoutPos::Empty && adjcount(&layout, ridx, cidx) == 0 {
                worklayout[ridx][cidx] = LayoutPos::Occupied;
                changes += 1;
            }
            if layout[ridx][cidx] == LayoutPos::Occupied && adjcount(&layout, ridx, cidx) >= 4 {
                worklayout[ridx][cidx] = LayoutPos::Empty;
                changes += 1;
            }
        }
    }

    for ridx in 0..layout.len() {
        for cidx in 0..layout[ridx].len() {
            layout[ridx][cidx] = worklayout[ridx][cidx].clone();
        }
    }
    changes
}

fn adjcount(layout: &Vec<Vec<LayoutPos>>, y: usize, x: usize) -> u32 {
    let mut count = 0;
    let limitrowlow = if y == 0 { 0 } else { y - 1 };
    let limitrowhigh = if y == layout.len() - 1 {
        layout.len() - 1
    } else {
        y + 1
    };
    let limitcollow = if x == 0 { 0 } else { x - 1 };
    let limitcolhigh = if x == layout[0].len() - 1 {
        layout[0].len() - 1
    } else {
        x + 1
    };
    for row in limitrowlow..limitrowhigh + 1 {
        for column in limitcollow..limitcolhigh + 1 {
            if !(row == y && column == x) {
                if LayoutPos::Occupied == layout[row][column] {
                    count += 1;
                }
            }
        }
    }
    count
}

impl std::fmt::Debug for LayoutPos {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                LayoutPos::Floor => '.',
                LayoutPos::Empty => 'L',
                LayoutPos::Occupied => '#',
            }
        )
    }
}

fn debuglayout(layout: &Vec<Vec<LayoutPos>>) {
    for ridx in 0..layout.len() {
        for cidx in 0..layout[ridx].len() {
            print!("{:?}", layout[ridx][cidx]);
        }
        println!();
    }
    println!();
}
