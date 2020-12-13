#[derive(Debug, Clone)]
struct Status(f64, f64, f64);

impl std::ops::AddAssign for Status {
    fn add_assign(&mut self, other: Self) {
        *self = Self(
            self.0 + other.0,
            self.1 + other.1,
            (self.2 + other.2) % 360.0,
        )
    }
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<_> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.split_at(1))
        .map(|x| (x.0.chars().nth(0).unwrap(), x.1.parse::<f64>().unwrap()))
        .collect();
    //println!("{:#?}", data);

    method1(&data);
    method2(&data);
}

fn method1(data: &Vec<(char, f64)>) {
    let mut status = Status(0.0, 0.0, 0.0);
    for (order, value) in data {
        match order {
            'N' => status += Status(0.0, *value, 0.0),
            'S' => status += Status(0.0, *value * -1.0, 0.0),
            'E' => status += Status(*value, 0.0, 0.0),
            'W' => status += Status(*value * -1.0, 0.0, 0.0),
            'R' => status += Status(0.0, 0.0, value.to_radians() * -1.0),
            'L' => status += Status(0.0, 0.0, value.to_radians() * 1.0),
            'F' => {
                status.0 += *value * status.2.cos();
                status.1 += *value * status.2.sin();
            }
            _ => unreachable!(),
        }
    }
    println!("{:?}", status);
}

fn method2(data: &Vec<(char, f64)>) {
    fn rotate_waypoint(ship: &Status, waypoint: &Status, angle: f64) -> Status {
        let (o0, o1) = (waypoint.0 - ship.0, waypoint.1 - ship.1);
        let (r, mut ang) = ((o0.powi(2) + o1.powi(2)).sqrt(), o1.atan2(o0));
        ang += angle;
        Status(ship.0 + (r * ang.cos()), ship.1 + (r * ang.sin()), 0.0)
    }
    let mut ship = Status(0.0, 0.0, 0.0);
    let mut waypoint = Status(10.0, 1.0, 0.0);
    for (order, value) in data {
        match order {
            'N' => waypoint += Status(0.0, *value, 0.0),
            'S' => waypoint += Status(0.0, *value * -1.0, 0.0),
            'E' => waypoint += Status(*value, 0.0, 0.0),
            'W' => waypoint += Status(*value * -1.0, 0.0, 0.0),
            'R' => waypoint = rotate_waypoint(&ship, &waypoint, -1.0 * value.to_radians()),
            'L' => waypoint = rotate_waypoint(&ship, &waypoint, 1.0 * value.to_radians()),
            'F' => {
                let dpos = Status(
                    value * (waypoint.0 - ship.0),
                    value * (waypoint.1 - ship.1),
                    0.0,
                );
                ship += dpos.clone(); // Im lazy and i dont want to impl the add trait only for this
                waypoint += dpos;
            }
            _ => unreachable!(),
        }
    }
    println!("{:?}", ship);
}
