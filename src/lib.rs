use std::fmt::{Display, Formatter};
use core::fmt;
use nalgebra::Point2;
use pathfinding::prelude::astar;
use pathfinding::utils::absdiff;

pub type Point = Point2<usize>;

#[derive(Debug)]
pub struct MyError {}

#[derive(Clone)]
pub struct Tile {
    id: char,
    weight: usize,
}

impl Tile {
    pub fn new(id: char, weight: usize) -> Tile {
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

// FIXME: I had wanted loc to be reference but life time woes once I hit calling astar in shortest path.
struct CoordIterator<'a> {
    map: &'a Map,
    loc: Point,
    // Current index in POINTS
    index: usize,
}

impl<'a> CoordIterator<'a> {
    fn new(map: &'a Map, loc: Point) -> Self {
        Self {
            map,
            loc,
            index: 0,
        }
    }

    // This is only for apply POINTS offset values.  We do not care about
    // overflowing because no map will have a dimension of biggest usize.
    fn math_is_hard(base: usize, delta: isize) -> Option<usize> {
        let result = base as isize + delta;

        if result.is_negative() {
            None
        } else {
            Some(result as usize)
        }
    }
}

const POINTS: [(isize, isize); 8] = [
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
    type Item = (Point, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < POINTS.len() {
            let (dx, dy) = POINTS[self.index];
            self.index += 1;

            // checked add to guarantee no negative values and at_xy still checks upper bounds of map.
            if let Some(nx) = Self::math_is_hard(self.loc.x, dx) {
                if let Some(ny) = Self::math_is_hard(self.loc.y, dy) {
                    let new_loc = Point::new(nx, ny);
                    if let Some(tile) = self.map.at(&new_loc) {
                        return Some((new_loc, tile.weight))
                    }
                }
            }
        }

        None
    }
}

impl Map {
    pub fn new(width: usize, height: usize, default_char: char, default_weight: usize) -> Self {
        Self {
            width,
            height,
            map: vec![Tile::new(default_char, default_weight); width * height],
        }
    }

    /// Note: Assumes all index accesses will get an index from a method which will prepare
    /// a safe index.
    fn at(&self, loc: &Point) -> Option<&Tile> {
        if let Some(index) = self.is_valid_loc(loc) {
            return Some(&self.map[index]);
        }

        None
    }

    fn is_valid_loc(&self, loc: &Point) -> Option<usize> {
        if loc.x >= self.width || loc.y >= self.height {
            None
        } else {
            Some(self.at_xy_raw(loc))
        }
    }

    pub fn set_at(&mut self, loc: &Point, tile: Tile) -> Result<(), MyError>{
        if let Some(index) = self.is_valid_loc(loc) {
            self.map[index] = tile;
            Ok(())
        } else {
            Err(MyError{})
        }
    }

    fn at_xy_raw(&self, loc: &Point) -> usize {
        loc.y * self.width + loc.x
    }

    /// Note: Assumes all index accesses will get an index from a method which will prepare
    /// a safe index.
    fn coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    // Assumes valid point
    fn adjacent_ats<'a>(&'a self, loc: Point) -> impl Iterator<Item=(Point, usize)> + 'a {
        CoordIterator::new(self, loc)
    }

    fn distance(p1: &Point, p2: &Point) -> usize {
        absdiff(p1.x, p2.x) + absdiff(p1.y, p2.y)
    }

    pub fn shortest_path(&self, start: &Point, end: &Point) -> Option<(Vec<Point>, usize)> {
        astar(&start,
              |i| self.adjacent_ats(i.clone()).filter(|(i, _)| self.at(i).unwrap().id == '.'),
              |i| Self::distance(i, end),
              |i| i == end)
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
    let mut map = Map::new(width, height, '.', 1);

    for (y, row) in rows.iter().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            // FIXME: All tiles will be immutable so share them all.
            let tile = Tile::new(tile, 1);
            let point = Point::new(x, y);

            map.set_at(&point, tile).unwrap();
        }
    }

    Some(map)
}

fn main() {
    let map = Map::new(80, 24, '.', 1);

    println!("Map {}", map);
}

#[cfg(test)]
mod tests {
    use crate::{Map, Point, Tile, generate_ascii_map};

    #[test]
    fn test_is_valid_loc() {
        let width = 5;
        let map = Map::new(width, 10, '.', 1);

        assert_eq!(map.is_valid_loc(&Point::new(0, 0)), Some(0));
        assert_eq!(map.is_valid_loc(&Point::new(1, 0)), Some(1));
        assert_eq!(map.is_valid_loc(&Point::new(0, 1)), Some(5));
        assert_eq!(map.is_valid_loc(&Point::new(6, 0)), None);
        assert_eq!(map.is_valid_loc(&Point::new(0, 10)), None);
    }

    #[test]
    fn test_coords() {
        let width = 5;
        let map = Map::new(width, 10, '.', 1);

        assert_eq!(map.coords(0), (0, 0));
        assert_eq!(map.coords(1), (1, 0));
        assert_eq!(map.coords(5), (0, 1));
    }

    #[test]
    fn test_at_and_set_at() {
        let width = 5;
        let mut map = Map::new(width, 10, '.', 1);

        let point = &Point::new(0, 0);
        assert_eq!(map.at(point).unwrap().id, '.');
        map.set_at(point, Tile::new('=', 1)).unwrap();
        assert_eq!(map.at(point).unwrap().id, '=');
    }

    #[test]
    fn test_adjacent_ats() {
        let width = 5;
        let map = Map::new(width, 10, '.', 1);

        //  +--
        //  |xo
        //  |oo
        let ats = map.adjacent_ats(Point::new(0, 0));
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| (i.x, i.y)).collect();
        assert_eq!(ats, vec![(1, 0), (0, 1), (1, 1)]);

        //  +---
        //  |oxo
        //  |ooo
        let ats = map.adjacent_ats(Point::new(1, 0));
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| (i.x, i.y)).collect();
        assert_eq!(ats, vec![(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);

        //  +---
        //  |ooo
        //  |oxo
        //  |ooo
        let ats = map.adjacent_ats(Point::new(1, 1));
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| (i.x, i.y)).collect();
        assert_eq!(ats, vec![(0, 0), (1, 0), (2, 0), (0, 1), (2, 1), (0, 2), (1, 2), (2, 2)]);

        // --+
        // ox|
        // oo|
        let ats = map.adjacent_ats(Point::new(4, 0));
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| (i.x, i.y)).collect();
        assert_eq!(ats, vec![(3, 0), (3, 1), (4, 1)]);

        // oo|
        // ox|
        // --+
        let ats = map.adjacent_ats(Point::new(4, 9));
        let ats: Vec<(usize, usize)> = ats.map(|(i, _)| (i.x, i.y)).collect();
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

        let start = Point::new(1, 1);
        let end = Point::new(12, 1);
        let path = map.shortest_path(&start, &end);
        if let Some(path) = path {
            let (path, distance) = path;
            println!("distance {}", distance);
            let route: Vec<_> = path.iter().collect();
            println!("Path {:?}", route);
            for i in &path {
                let tile = Tile::new('x', 1);
                map.set_at(i, tile).unwrap();
            }
            println!("{}", map);
        }

    }
}