use std::collections::HashMap;

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
        self.data = self.data.clone().into_iter().rev().collect();
        self.flip = !self.flip;
        self
    }
    /// Rotates 90 degrees clockwise
    fn rot(&mut self) -> &Self {
        let mut result: Vec<Vec<bool>> = Vec::new();
        for i in 0..self.size {
            result.push(
                self.data
                    .iter()
                    .map(|x| *x.iter().nth(i).unwrap())
                    .rev()
                    .collect::<Vec<bool>>(),
            );
        }
        self.data = result;
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
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Tile {{ ID {} Size {}x{} Flip {} Angle {}\n",
            self.id, self.size, self.size, self.flip, self.angle
        );
        for row in &self.data {
            for it in row {
                write!(fmt, "{}", if *it { "#" } else { "." });
            }
            write!(fmt, "\n");
        }
        write!(fmt, "}}\n");
        Result::Ok(())
    }
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

    println!("Number of tiles: {}", tiles.len());

    let mut image: HashMap<(isize, isize), Tile> = HashMap::new();
    let originkey = tiles.iter().next().unwrap().0.clone();
    let origin = tiles.remove(&originkey).unwrap();
    image.insert((0, 0), origin);

    while tiles.len() > 0 {
        search(&mut tiles, &mut image);
        println!("Tiles left to process: {}", tiles.len());
    }
    println!("Image: {:?}", image);

    let xmin = image.iter().map(|(&k, _)| k.0).min().unwrap();
    let xmax = image.iter().map(|(&k, _)| k.0).max().unwrap();
    let ymin = image.iter().map(|(&k, _)| k.1).min().unwrap();
    let ymax = image.iter().map(|(&k, _)| k.1).max().unwrap();
    println!(
        "Result {}",
        image.get(&(xmin, ymin)).unwrap().id
            * image.get(&(xmin, ymax)).unwrap().id
            * image.get(&(xmax, ymin)).unwrap().id
            * image.get(&(xmax, ymax)).unwrap().id
    );
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
                        //println!(                            "Found a match for tile {}/edge {}: {:?}",                           subimage.id, edgeidx, tile                        );
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
