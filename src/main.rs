use std::collections::HashMap;

const DEBUG: bool = false;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

macro_rules! prof {
    ($something:expr; $($otherthings:expr;)*) => {
        {
            let start = Instant::now();
            $something;
            $(
                $otherthings;
            )*
            start.elapsed()
        }
    };
}

#[derive(Clone)]
struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
    size: usize,
    flip: bool,
    angle: u16,
}

impl Tile {
    fn build(id: usize, raw: Vec<&str>) -> Tile {
        let mut data = Vec::new();
        for line in raw {
            data.push(line.chars().map(|c| c == '#').collect());
        }
        Tile {
            id: id,
            size: data.len(),
            data: data,
            flip: false,
            angle: 0,
        }
    }
    fn edges(&self) -> [i32; 4] {
        [
            self.data[0]
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + (*x as i32 * 2i32.pow(i as u32))),
            self.data
                .iter()
                .map(|x| *x.last().unwrap())
                .collect::<Vec<bool>>()
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + (*x as i32 * 2i32.pow(i as u32))),
            self.data[self.size - 1]
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + (*x as i32 * 2i32.pow(i as u32))),
            self.data
                .iter()
                .map(|x| *x.first().unwrap())
                .collect::<Vec<bool>>()
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + (*x as i32 * 2i32.pow(i as u32))),
        ]
    }
    /// Flips the tile upside-down
    fn flip(&mut self) -> &Self {
        self.data = flip(&self.data);
        self.flip = !self.flip;
        self
    }
    /// Rotates 90 degrees clockwise
    fn rot(&mut self) -> &Self {
        self.data = rot(&self.data);
        self.angle = (self.angle + 90) % 360;
        self
    }
    fn _rotn(&mut self, mut n: usize) -> &Self {
        while n > 0 {
            self.rot();
            n -= 1;
        }
        self
    }
    fn trim(&self) -> Vec<Vec<bool>> {
        let mut result: Vec<Vec<bool>> = self.data.clone();
        result.remove(result.len() - 1);
        result.remove(0);
        result
            .into_iter()
            .map(|mut x| {
                x.remove(x.len() - 1);
                x.remove(0);
                x
            })
            .collect()
    }
}

fn rot(data: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();
    for i in 0..data.len() {
        result.push(
            data.iter()
                .map(|x| *x.iter().nth(i).unwrap())
                .rev()
                .collect::<Vec<bool>>(),
        );
    }
    result
}
fn flip(data: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    data.clone().into_iter().rev().collect()
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Tile {{ ID {} Size {}x{} Flip {} Angle {}\n{}",
            self.id,
            self.size,
            self.size,
            self.flip,
            self.angle,
            printimage(&self.data)
        )?;
        write!(fmt, "}}\n")?;
        Result::Ok(())
    }
}

fn printimage(data: &Vec<Vec<bool>>) -> String {
    let mut result = String::new();
    for row in data {
        for it in row {
            result.push_str(&format!("{}", if *it { "#" } else { "." }));
        }
        result.push_str(&format!("\n"));
    }
    result
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let mut tiles: HashMap<usize, Tile> = data
        .split("\n\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            let lines: Vec<&str> = x.split("\n").filter(|x| x.len() > 0).collect();
            let id = lines
                .first()
                .unwrap()
                .replace("Tile ", "")
                .replace(":", "")
                .parse::<usize>()
                .unwrap();
            (id, Tile::build(id, (&lines[1..]).to_vec()))
        })
        .collect();

    //println!("Monster\n{:#?}", monster);
    //println!("Number of tiles: {}", tiles.len());

    let mut image: HashMap<(isize, isize), Tile> = HashMap::new();
    let originkey = tiles.iter().next().unwrap().0.clone();
    let origin = tiles.remove(&originkey).unwrap();
    image.insert((0, 0), origin);

    while tiles.len() > 0 {
        search(&mut tiles, &mut image);
        debugln!("Tiles left to process: {}", tiles.len());
    }
    //println!("Image: {:?}", image);

    let xmin = image.iter().map(|(&k, _)| k.0).min().unwrap();
    let xmax = image.iter().map(|(&k, _)| k.0).max().unwrap();
    let ymin = image.iter().map(|(&k, _)| k.1).min().unwrap();
    let ymax = image.iter().map(|(&k, _)| k.1).max().unwrap();
    println!(
        "Part 1 result: {}",
        image.get(&(xmin, ymin)).unwrap().id
            * image.get(&(xmin, ymax)).unwrap().id
            * image.get(&(xmax, ymin)).unwrap().id
            * image.get(&(xmax, ymax)).unwrap().id
    );

    let mut image = makeimage(image, (xmin, ymin, xmax, ymax));
    debugln!("{}", printimage(&image));

    let monster: Vec<Vec<bool>> = String::from_utf8_lossy(include_bytes!("monster.txt"))
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().map(|c| c == '#').collect())
        .collect();

    for pos in 0..8 {
        let monsterscount = countmonsters(&image, &monster);
        if monsterscount > 0 {
            debugln!("{} monsters detected! ", monsterscount);
            println!(
                "Part 2 result: {}",
                image
                    .iter()
                    .map(|r| r.iter().filter(|&q| *q).count())
                    .sum::<usize>()
                    - monsterscount
                        * monster
                            .iter()
                            .map(|r| r.iter().filter(|&q| *q).count())
                            .sum::<usize>()
            );
            return;
        }
        image = rot(&image);
        if pos == 3 {
            image = flip(&image);
        }
    }
}

fn countmonsters(image: &Vec<Vec<bool>>, monster: &Vec<Vec<bool>>) -> usize {
    fn placemonster(
        monster: &Vec<Vec<bool>>,
        coord: (usize, usize),
        bounds: ((usize, usize), (usize, usize)),
    ) -> Vec<Vec<bool>> {
        let imagewidth = bounds.0 .0;
        let imageheight = bounds.0 .1;
        let monsterwidth = bounds.1 .0;
        let monsterheight = bounds.1 .1;
        let mut result = vec![vec![false; imagewidth]; imageheight];
        for y in coord.1..coord.1 + monsterheight {
            for x in coord.0..coord.0 + monsterwidth {
                result[y][x] = monster[y - coord.1][x - coord.0];
            }
        }
        result
    }
    fn or(bg: &Vec<Vec<bool>>, fg: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        bg.iter()
            .zip(fg.iter())
            .map(|(bgrow, fgrow)| {
                bgrow
                    .iter()
                    .zip(fgrow.iter())
                    .map(|(bgitem, fgitem)| bgitem | fgitem)
                    .collect()
            })
            .collect()
    }
    let mut count = 0;
    let imagewidth = image.first().unwrap().len();
    let imageheight = image.len();
    let monsterwidth = monster.first().unwrap().len();
    let monsterheight = monster.len();
    for y in 0..imageheight - monsterheight + 1 {
        for x in 0..imagewidth - monsterwidth + 1 {
            let monster_in_place = placemonster(
                monster,
                (x, y),
                ((imagewidth, imageheight), (monsterwidth, monsterheight)),
            );
            debugln!("{}", printimage(&or(image, &monster_in_place)));
            if &or(image, &monster_in_place) == image {
                count += 1;
            }
        }
    }
    count
}

fn makeimage(
    image: HashMap<(isize, isize), Tile>,
    boundaries: (isize, isize, isize, isize),
) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();
    for y in boundaries.1..boundaries.3 + 1 {
        let mut row: Option<Vec<Vec<bool>>> = None;
        for x in boundaries.0..boundaries.2 + 1 {
            let data: Vec<Vec<bool>> = image.get(&(x, y)).unwrap().trim();
            if row.is_none() {
                row = Some(data);
            } else {
                let tmp = row.unwrap();
                row = Some(
                    tmp.into_iter()
                        .zip(data.into_iter())
                        .map(|(mut x, mut y)| {
                            x.append(&mut y);
                            x
                        })
                        .collect(),
                );
            }
        }
        for line in row.unwrap() {
            result.push(line);
        }
    }
    result
}

fn search(tiles: &mut HashMap<usize, Tile>, image: &mut HashMap<(isize, isize), Tile>) {
    fn matchingedge(i: usize) -> usize {
        match i {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => unreachable!(),
        }
    }
    fn getnewcoord(coord: (isize, isize), i: usize) -> (isize, isize) {
        match i {
            0 => (coord.0, coord.1 - 1),
            1 => (coord.0 + 1, coord.1),
            2 => (coord.0, coord.1 + 1),
            3 => (coord.0 - 1, coord.1),
            _ => unreachable!(),
        }
    }

    let mut matchinfo: Option<(usize, (isize, isize), Tile)> = None;
    'outer: for (coord, subimage) in image.iter() {
        for (edgeidx, edge) in subimage.edges().iter().enumerate() {
            let newcoord = getnewcoord(*coord, edgeidx);
            if image.contains_key(&newcoord) {
                continue;
            }
            for (tileid, tile) in &mut *tiles {
                let mut tile = tile.clone();
                for pos in 0..8 {
                    if tile.edges()[matchingedge(edgeidx)] == *edge {
                        matchinfo = Some((*tileid, newcoord, tile));
                        break 'outer;
                    }
                    tile.rot();
                    if pos == 3 {
                        tile.flip();
                    }
                }
            }
        }
    }
    if matchinfo.is_some() {
        let (deleteid, newcoord, tile) = matchinfo.unwrap();
        //Delete tile from tiles
        tiles.remove(&deleteid);
        //Add tile to the proper place in image
        image.insert(newcoord, tile);
    }
}
