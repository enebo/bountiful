use std::fmt::{Display, Formatter};
use core::fmt;
use pathfinding::prelude::{absdiff, astar};

#[derive(Clone)]
pub struct Tile {
    id: char,
    weight: usize,
}

impl Tile {
    fn new(id: char, weight: usize) -> Tile {
        Tile {
            id,
            weight,
        }
    }
}

pub struct Map {
    width: usize,
    height: usize,
    map: Vec<Tile>
}

struct CoordIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
    index: usize,
}

impl<'a> CoordIterator<'a> {
    fn new(map: &'a Map, loc: usize) -> Self {
        let (x, y) = map.coords(loc);

        Self {
            map,
            x,
            y,
            index: 0,
        }
    }

    fn math_is_hard(base: usize, delta: i32) -> Option<usize> {
        let negative = delta < 0;

        if negative {
            base.checked_sub(delta.abs() as usize)
        } else {
            base.checked_add(delta as usize)
        }
    }
}

const POINTS: [(i32, i32); 8] = [
    (-1, -1),  // upper left
    (0, -1),   // up
    (1, -1),   // upper right
    (-1, 0),   // left
    (1, 0),    // right
    (-1, 1),   // lower left
    (0, 1),    // down
    (1, 1)     // lower right
];

impl<'a> Iterator for CoordIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < POINTS.len() {
            let (dx, dy) = POINTS[self.index];
            self.index += 1;

            // checked add to guarantee no negative values and at_xy still checks upper bounds of map.
            if let Some(nx) = Self::math_is_hard(self.x, dx) {
                if let Some(ny) = Self::math_is_hard(self.y, dy) {
                    if let Some(new_loc) = self.map.at_xy(nx, ny) {
                        let tile = self.map.at(new_loc);
                        return Some((new_loc, tile.weight))
                    }
                }
            }
        }

        None
    }
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            map: vec![Tile::new(' ', 1); width * height],
        }
    }

    /// Note: Assumes all index accesses will get an index from a method which will prepare
    /// a safe index.
    fn at(&self, index: usize) -> &Tile {
        &self.map[index]
    }

    fn set_at(&mut self, index: usize, tile: Tile) {
        self.map[index] = tile;
    }

    fn at_xy(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = self.at_xy_raw(x, y);

        if index > self.map.len() {
            return None;
        }

        Some(index)
    }

    fn at_xy_raw(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Note: Assumes all index accesses will get an index from a method which will prepare
    /// a safe index.
    fn coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn adjacent_ats(&self, index: usize) -> impl Iterator<Item=(usize, usize)> + '_ {
        CoordIterator::new(&self, index)
    }

    fn distance(&self, index1: usize, index2: usize) -> usize {
        // FIXME: decompose to simple math
        let (x1, y1) = self.coords(index1);
        let (x2, y2) = self.coords(index2);
        let x = absdiff(x1, x2);
        let y = absdiff(y1, y2);
        x + y
    }

    fn shortest_path(&self, start: usize, end: usize) -> Option<(Vec<usize>, usize)>{
        astar(&start, |i| self.adjacent_ats(*i).filter(|(i, _)| self.at(*i).id == '.'), |i| self.distance(*i, end), |i| i == &end)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let stream: Vec<char> = self.map.iter().map(|e| e.id).collect();
        let split = &stream.chunks(self.width).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>();
        for line in split {
            let _ = writeln!(f, "{}", line);
        }
        Ok(())
    }
}

fn generate_ascii_map(ascii_map: &str) -> Option<Map> {
    let rows: Vec<&str> = ascii_map.split_terminator('\n').collect();
    let height = rows.len();

    if height == 0 {
        return None;
    }

    let width = rows[0].len();

    // verify all lines are same length;
    if let Some(_) = rows.iter().find(|e| e.len() != width) {
        return None;
    }

    println!("Making map of size: {}x{}", width, height);
    let mut map = Map::new(width, height);

    for (y, row) in rows.iter().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            let tile = Tile::new(tile, 1);

            map.set_at(map.at_xy_raw(x, y), tile);
        }
    }

    Some(map)
}

fn main() {
    let map = Map::new(80, 24);

    println!("Map {}", map);
}

#[cfg(test)]
mod tests {
    use crate::{Map, Tile, generate_ascii_map};

    #[test]
    fn test_at_xy() {
        let width = 5;
        let map = Map::new(width, 10);

        assert_eq!(map.at_xy(0, 0), Some(0));
        assert_eq!(map.at_xy(1, 0), Some(1));
        assert_eq!(map.at_xy(0, 1), Some(5));
        assert_eq!(map.at_xy(6, 0), None);
        assert_eq!(map.at_xy(0, 10), None);
    }

    #[test]
    fn test_coords() {
        let width = 5;
        let map = Map::new(width, 10);

        assert_eq!(map.coords(0), (0, 0));
        assert_eq!(map.coords(1), (1, 0));
        assert_eq!(map.coords(5), (0, 1));
    }

    #[test]
    fn test_at_and_set_at() {
        let width = 5;
        let mut map = Map::new(width, 10);

        assert_eq!(map.at(map.at_xy(0, 0).unwrap()).id, ' ');
        map.set_at(map.at_xy(0, 0).unwrap(), Tile::new('.', 1));
        assert_eq!(map.at(map.at_xy(0, 0).unwrap()).id, '.');
    }

    #[test]
    fn test_adjacent_ats() {
        let width = 5;
        let map = Map::new(width, 10);

        //  +--
        //  |xo
        //  |oo
        let ats = map.adjacent_ats(map.at_xy(0, 0).unwrap());
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| map.coords(i)).collect();
        assert_eq!(ats, vec![(1, 0), (0, 1), (1, 1)]);

        //  +---
        //  |oxo
        //  |ooo
        let ats = map.adjacent_ats(map.at_xy(1, 0).unwrap());
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| map.coords(i)).collect();
        assert_eq!(ats, vec![(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);

        //  +---
        //  |ooo
        //  |oxo
        //  |ooo
        let ats = map.adjacent_ats(map.at_xy(1, 1).unwrap());
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| map.coords(i)).collect();
        assert_eq!(ats, vec![(0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2)]);

        // --+
        // ox|
        // oo|
        let ats = map.adjacent_ats(map.at_xy(4, 0).unwrap());
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| map.coords(i)).collect();
        assert_eq!(ats, vec![(3, 0), (3, 1), (4, 1)]);

        // oo|
        // ox|
        // --+
        let ats = map.adjacent_ats(map.at_xy(4, 9).unwrap());
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| map.coords(i)).collect();
        assert_eq!(ats, vec![(3, 8), (4, 8), (3, 9)]);
    }

    #[test]
    fn test_generate_ascii_map() {
        let map_string = "##############\n\
                                #..#......#..#\n\
                                #...##.#.....#\n\
                                #..##...#.#..#\n\
                                #..######.#..#\n\
                                #............#\n\
                                ##############";


        let mut map = generate_ascii_map(map_string).unwrap();
/*        assert_eq!(map.width, 14);
        assert_eq!(map.height, 4);
        assert_eq!(map.at(map.at_xy(0, 0).unwrap()), TileType::Wall);
        assert_eq!(map.at(map.at_xy(1, 1).unwrap()), TileType::Floor);*/
        println!("{}", map);

        let path = map.shortest_path(map.at_xy_raw(1,1), map.at_xy_raw(12, 1));
        if let Some(path) = path {
            let (path, distance) = path;
            println!("distance {}", distance);
            let route: Vec<_> = path.iter().map(|i| map.coords(*i)).collect();
            println!("Path {:?}", route);
            for i in path {
                map.set_at(i, Tile::new('x', 1));
            }
            println!("{}", map);
        }

    }
}

