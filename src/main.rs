use std::collections::HashSet;

fn main() {
    step1();
    step2();
}

fn step1() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<Vec<(i32, i32, i32)>> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .enumerate()
        .map(|(idx, x)| {
            x.chars()
                .enumerate()
                .filter(|&(_, y)| y == '#')
                .map(|(idy, _)| (idy as i32, idx as i32, 0i32))
                .collect()
        })
        .collect();
    let mut data: HashSet<_> = data.iter().flatten().cloned().collect::<HashSet<_>>();

    for _ in 0..6 {
        let mut cloneddata = data.clone();
        let mut dimensions = (
            (std::i32::MAX, std::i32::MIN),
            (std::i32::MAX, std::i32::MIN),
            (std::i32::MAX, std::i32::MIN),
        );
        //If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
        for active in &data {
            // Retrieve min dimensions
            {
                if active.0 < dimensions.0 .0 {
                    dimensions.0 .0 = active.0;
                }
                if active.0 > dimensions.0 .1 {
                    dimensions.0 .1 = active.0;
                }
                if active.1 < dimensions.1 .0 {
                    dimensions.1 .0 = active.1;
                }
                if active.1 > dimensions.1 .1 {
                    dimensions.1 .1 = active.1;
                }
                if active.2 < dimensions.2 .0 {
                    dimensions.2 .0 = active.2;
                }
                if active.2 > dimensions.2 .1 {
                    dimensions.2 .1 = active.2;
                }
            }
            let activeneighbours = neighbours3d(&data, *active);
            if activeneighbours < 2 || 3 < activeneighbours {
                cloneddata.remove(active);
            }
        }

// for z in (dimensions.2 .0)..(dimensions.2 .1 + 1) {
//     for y in (dimensions.1 .0)..(dimensions.1 .1 + 1) {
//         for x in (dimensions.0 .0)..(dimensions.0 .1 + 1) {
//             if data.contains(&(x, y, z)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
//     println!("");
// }


        //If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
        for x in (dimensions.0 .0 - 1)..(dimensions.0 .1 + 2) {
            for y in (dimensions.1 .0 - 1)..(dimensions.1 .1 + 2) {
                for z in (dimensions.2 .0 - 1)..(dimensions.2 .1 + 2) {
                    if !data.contains(&(x, y, z)) {
                        if neighbours3d(&data, (x, y, z)) == 3 {
                            cloneddata.insert((x, y, z));
                        }
                    }
                }
            }
        }
        data = cloneddata;
    }
    println!("Step 1 answer: {:?}", data.len());
}

fn neighbours3d(data: &HashSet<(i32, i32, i32)>, coord: (i32, i32, i32)) -> u32 {
    let mut count = 0;
    for x in (coord.0 - 1)..(coord.0 + 2) {
        for y in (coord.1 - 1)..(coord.1 + 2) {
            for z in (coord.2 - 1)..(coord.2 + 2) {
                //println!("{:?}", (x, y, z));
                if (x, y, z) != coord && data.contains(&(x, y, z)) {
                    //println!("Found {:?} as neighbours of {:?}", (x, y, z), coord);
                    count += 1;
                }
            }
        }
    }
    count
}

fn step2() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<Vec<(i32, i32, i32, i32)>> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .enumerate()
        .map(|(idx, x)| {
            x.chars()
                .enumerate()
                .filter(|&(_, y)| y == '#')
                .map(|(idy, _)| (idy as i32, idx as i32, 0i32, 0i32))
                .collect()
        })
        .collect();
    let mut data: HashSet<_> = data.iter().flatten().cloned().collect::<HashSet<_>>();

    for _ in 0..6 {
        let mut cloneddata = data.clone();
        let mut dimensions = (
            (std::i32::MAX, std::i32::MIN),
            (std::i32::MAX, std::i32::MIN),
            (std::i32::MAX, std::i32::MIN),
            (std::i32::MAX, std::i32::MIN),
        );
        //If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
        for active in &data {
            // Retrieve min dimensions
            {
                if active.0 < dimensions.0 .0 {
                    dimensions.0 .0 = active.0;
                }
                if active.0 > dimensions.0 .1 {
                    dimensions.0 .1 = active.0;
                }
                if active.1 < dimensions.1 .0 {
                    dimensions.1 .0 = active.1;
                }
                if active.1 > dimensions.1 .1 {
                    dimensions.1 .1 = active.1;
                }
                if active.2 < dimensions.2 .0 {
                    dimensions.2 .0 = active.2;
                }
                if active.2 > dimensions.2 .1 {
                    dimensions.2 .1 = active.2;
                }
                if active.3 < dimensions.3 .0 {
                    dimensions.3 .0 = active.3;
                }
                if active.3 > dimensions.3 .1 {
                    dimensions.3 .1 = active.3;
                }
            }
            let activeneighbours = neighbours4d(&data, *active);
            if activeneighbours < 2 || 3 < activeneighbours {
                cloneddata.remove(active);
            }
        }

        //If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
        for x in (dimensions.0 .0 - 1)..(dimensions.0 .1 + 2) {
            for y in (dimensions.1 .0 - 1)..(dimensions.1 .1 + 2) {
                for z in (dimensions.2 .0 - 1)..(dimensions.2 .1 + 2) {
                    for w in (dimensions.3 .0 - 1)..(dimensions.3 .1 + 2) {
                        if !data.contains(&(x, y, z, w)) {
                            if neighbours4d(&data, (x, y, z, w)) == 3 {
                                cloneddata.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
        data = cloneddata;
    }
    println!("Step 2 answer: {:?}", data.len());
}

fn neighbours4d(data: &HashSet<(i32, i32, i32, i32)>, coord: (i32, i32, i32, i32)) -> u32 {
    let mut count = 0;
    for x in (coord.0 - 1)..(coord.0 + 2) {
        for y in (coord.1 - 1)..(coord.1 + 2) {
            for z in (coord.2 - 1)..(coord.2 + 2) {
                for w in (coord.3 - 1)..(coord.3 + 2) {
                    if (x, y, z, w) != coord && data.contains(&(x, y, z, w)) {
                        //println!("Found {:?} as neighbours of {:?}", (x, y, z), coord);
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
