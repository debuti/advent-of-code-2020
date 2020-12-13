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
    let mut originallayout: Vec<Vec<LayoutPos>> = Vec::new();
    // Parsing
    {
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
            originallayout.push(temp);
        }
        //debuglayout(&layout);
    }

    // First question
    {
        let mut layout = originallayout.clone();
        loop {
            let (changes, newlayout) = process1(layout);
            layout = newlayout;
            if changes == 0 {
                break;
            }
        }
        debuglayout(&layout);
        println!(
            "The count of occupied seats is {}",
            layout
                .into_iter()
                .flatten()
                .filter(|x| *x == LayoutPos::Occupied)
                .count()
        );
    }

    // Second question
    {
        let mut layout = originallayout.clone();
        loop {
            let (changes, newlayout) = process2(layout);
            layout = newlayout;
            if changes == 0 {
                break;
            }
        }
        debuglayout(&layout);
        println!(
            "The count of occupied seats is now {}",
            layout
                .into_iter()
                .flatten()
                .filter(|x| *x == LayoutPos::Occupied)
                .count()
        );
    }
}

fn process1(layout: Vec<Vec<LayoutPos>>) -> (u32, Vec<Vec<LayoutPos>>) {
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
    (changes, worklayout)
}

fn process2(layout: Vec<Vec<LayoutPos>>) -> (u32, Vec<Vec<LayoutPos>>) {
    let mut worklayout = layout.clone();
    let mut changes = 0;
    for ridx in 0..layout.len() {
        for cidx in 0..layout[ridx].len() {
            if layout[ridx][cidx] == LayoutPos::Empty && visiblecount(&layout, ridx, cidx) == 0 {
                worklayout[ridx][cidx] = LayoutPos::Occupied;
                changes += 1;
            }
            if layout[ridx][cidx] == LayoutPos::Occupied && visiblecount(&layout, ridx, cidx) >= 5 {
                worklayout[ridx][cidx] = LayoutPos::Empty;
                changes += 1;
            }
        }
    }
    (changes, worklayout)
}

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn inlimits(layout: &Vec<Vec<LayoutPos>>, y: i8, x: i8) -> bool {
    (y >= 0) && ((layout.len() - 1) as i8 >= y) && (x >= 0) && ((layout[0].len() - 1) as i8 >= x)
}

fn adjcount(layout: &Vec<Vec<LayoutPos>>, y: usize, x: usize) -> u32 {
    let mut count = 0;

    for direction in &DIRECTIONS {
        let mut crow: i8 = y as i8;
        let mut ccol: i8 = x as i8;
        crow += direction.0;
        ccol += direction.1;
        if inlimits(layout, crow, ccol) {
            match layout[crow as usize][ccol as usize] {
                LayoutPos::Occupied => {
                    count += 1;
                }
                _ => {}
            }
        }
    }
    count
}

fn visiblecount(layout: &Vec<Vec<LayoutPos>>, y: usize, x: usize) -> u32 {
    let mut count = 0;

    for direction in &DIRECTIONS {
        let mut crow: i8 = y as i8;
        let mut ccol: i8 = x as i8;
        loop {
            crow += direction.0;
            ccol += direction.1;
            if inlimits(layout, crow, ccol) {
                match layout[crow as usize][ccol as usize] {
                    LayoutPos::Occupied => {
                        count += 1;
                        break;
                    }
                    LayoutPos::Empty => {
                        break;
                    }
                    LayoutPos::Floor => {}
                }
            } else {
                break;
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
