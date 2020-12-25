// use piston_window::*;
use std::collections::HashSet;

const DEBUG: bool = true;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

fn main() {
    let data: String = String::from_utf8_lossy(include_bytes!("data.txt"))
        .replace("se", "m")
        .replace("sw", "b")
        .replace("ne", "i")
        .replace("nw", "y")
        .to_string();
    let data: Vec<&str> = data.split("\n").filter(|&x| x.len() > 0).collect();

    part1(&data);
    part2(&data);
}

fn part2(data: &Vec<&str>) {
    fn adj(coord: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        vec![
            (coord.0 - 1, coord.1 + 1, coord.2 + 0),
            (coord.0 + 0, coord.1 + 1, coord.2 - 1),
            (coord.0 + 1, coord.1 + 0, coord.2 - 1),
            (coord.0 + 1, coord.1 - 1, coord.2 + 0),
            (coord.0 + 0, coord.1 - 1, coord.2 + 1),
            (coord.0 - 1, coord.1 + 0, coord.2 + 1),
        ]
    }
    fn adjc(flipped: &HashSet<(i32, i32, i32)>, coord: &(i32, i32, i32)) -> usize {
        adj(coord).iter().filter(|x| flipped.contains(x)).count()
    }

    let mut flipped: HashSet<(i32, i32, i32)> = HashSet::new();
    for tile in data {
        let mut coord = (0, 0, 0);
        for c in tile.chars() {
            coord = match c {
                'w' => (coord.0 - 1, coord.1 + 1, coord.2 + 0),
                /*nw*/ 'y' => (coord.0 + 0, coord.1 + 1, coord.2 - 1),
                /*ne*/ 'i' => (coord.0 + 1, coord.1 + 0, coord.2 - 1),
                'e' => (coord.0 + 1, coord.1 - 1, coord.2 + 0),
                /*se*/ 'm' => (coord.0 + 0, coord.1 - 1, coord.2 + 1),
                /*sw*/ 'b' => (coord.0 - 1, coord.1 + 0, coord.2 + 1),
                _ => unreachable!(),
            };
        }
        if flipped.contains(&coord) {
            flipped.remove(&coord);
        } else {
            flipped.insert(coord);
        }
    }
    debugln!("Flipped {:?}", flipped.len());

    for day in 1..101 {
        let lastday = flipped.clone();
        for black in &lastday {
            // Check blacks rule
            {
                let adjc = adjc(&lastday, &black);
                if adjc == 0 || adjc > 2 {
                    flipped.remove(&black);
                }
            }
            // Check whites rule for every adj tile
            {
                for adj in adj(&black) {
                    if !lastday.contains(&adj) && adjc(&lastday, &adj) == 2 {
                        flipped.insert(adj);
                    }
                }
            }
        }
        debugln!("Day {:?}: {}", day, flipped.len());
    }
    println!("Step 2 answer: {:?}", flipped.len());
    //TODO: Use piston to render the board dynamically
    // let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap();
    // while let Some(e) = window.next() {
    //     window.draw_2d(&e, |c, g, _device| {
    //         clear([1.0; 4], g);
    //         rectangle(
    //             [1.0, 0.0, 0.0, 1.0], // red
    //             [0.0, 0.0, 100.0, 100.0],
    //             c.transform,
    //             g,
    //         );
    //     });
    // }
}

fn part1(data: &Vec<&str>) {
    let mut flipped: HashSet<(i32, i32)> = HashSet::new();
    for tile in data {
        let mut coord = (0, 0);
        for c in tile.chars() {
            coord = match c {
                'w' => (coord.0 - 2, coord.1 + 0),
                'y' => (coord.0 - 1, coord.1 + 3),
                'i' => (coord.0 + 1, coord.1 + 3),
                'e' => (coord.0 + 2, coord.1 + 0),
                'm' => (coord.0 + 1, coord.1 - 3),
                'b' => (coord.0 - 1, coord.1 - 3),
                _ => unreachable!(),
            }
        }
        if flipped.contains(&coord) {
            flipped.remove(&coord);
        } else {
            flipped.insert(coord);
        }
    }
    println!("Step 1 answer: {:?}", flipped.len());
}
