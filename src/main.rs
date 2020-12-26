use piston_window::*;
use std::collections::HashSet;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    const factor: f64 = 5.0;
    const R: f64 = factor * 1.0; // Hexagon circumradius
    const r: f64 = factor * 0.86602540378443864676372317075293618347140262690519031402790; // Hexagon inradius ((R * 3.0f64.sqrt()) / 2.0 solved for R=1 as Rust doesnt allow these const expressions)
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
    fn renderhexagon(coord: &(i32, i32, i32), offset: &[f64; 2]) -> [[f64; 2]; 6] {
        let center: (f64, f64) = (
            r * (2.0f64 * f64::from(coord.0) + f64::from(coord.2)),
            3.0f64 * R * f64::from(coord.2) / 2.0f64,
        );
        [
            [offset[0] + center.0 + 0.0, offset[1] + center.1 - R],
            [offset[0] + center.0 + r, offset[1] + center.1 - (R / 2.0)],
            [offset[0] + center.0 + r, offset[1] + center.1 + (R / 2.0)],
            [offset[0] + center.0 + 0.0, offset[1] + center.1 + R],
            [offset[0] + center.0 - r, offset[1] + center.1 + (R / 2.0)],
            [offset[0] + center.0 - r, offset[1] + center.1 - (R / 2.0)],
        ]
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

    let mut window: PistonWindow = WindowSettings::new("Advent of Rust 2020", [640 * 2, 480 * 2])
        .exit_on_esc(true)
        .graphics_api(OpenGL::V3_2)
        .build()
        .unwrap();
    let offset = [640.0, 480.0];

    let mut day = 0;
    while let Some(e) = window.next() {
        if day == 100 {
            break;
        }
        day += 1;

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

        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            for black in &flipped {
                polygon(
                    color::BLACK,
                    &renderhexagon(&black, &offset),
                    c.transform,
                    g,
                );
            }
        });
        thread::sleep(Duration::from_millis(100));
    }
    println!("Step 2 answer: {:?}", flipped.len());
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
