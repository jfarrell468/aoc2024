use std::collections::HashSet;

trait Map {
    fn rows(&self) -> i32;
    fn cols(&self) -> i32;
    fn on_map(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && row < self.rows() && col < self.cols()
    }
    fn get(&self, row: i32, col: i32) -> Option<MapTile>;
}

impl Map for Vec<Vec<MapTile>> {
    fn rows(&self) -> i32 {
        self.len() as i32
    }

    fn cols(&self) -> i32 {
        if self.is_empty() {
            0
        } else {
            self[0].len() as i32 // Assume a rectangular map
        }
    }

    fn get(&self, row: i32, col: i32) -> Option<MapTile> {
        if self.on_map(row, col) {
            Some(self[row as usize][col as usize].clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
enum MapTile {
    Empty,
    Blocked,
}

impl MapTile {
    fn from_char(c: char) -> MapTile {
        match c {
            '.' => Self::Empty,
            '#' => Self::Blocked,
            _ => panic!(),
        }
    }
    fn as_char(&self) -> char {
        match &self {
            MapTile::Empty => '.',
            MapTile::Blocked => '#',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    row: i32,
    col: i32,
    orientation: Orientation,
}

impl Guard {
    fn next(&self) -> (i32, i32) {
        match self.orientation {
            Orientation::Up => (self.row - 1, self.col),
            Orientation::Right => (self.row, self.col + 1),
            Orientation::Down => (self.row + 1, self.col),
            Orientation::Left => (self.row, self.col - 1),
        }
    }
    fn advance(&mut self) {
        let (row, col) = self.next();
        self.row = row;
        self.col = col;
    }
    fn right90(&mut self) {
        self.orientation = self.orientation.right90();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn right90(&self) -> Orientation {
        match &self {
            Orientation::Up => Self::Right,
            Orientation::Right => Self::Down,
            Orientation::Down => Self::Left,
            Orientation::Left => Self::Up,
        }
    }
    fn from_char(c: char) -> Orientation {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!(),
        }
    }
    fn as_char(&self) -> char {
        match &self {
            Orientation::Up => '^',
            Orientation::Right => '>',
            Orientation::Down => 'v',
            Orientation::Left => '<',
        }
    }
}

#[derive(Debug, Clone)]
struct MapState {
    map: Vec<Vec<MapTile>>,
    guard: Guard,
}

impl MapState {
    fn parse(lines: &Vec<String>) -> MapState {
        let mut map = Vec::new();
        let mut guard = Guard {
            row: 0,
            col: 0,
            orientation: Orientation::Up,
        };
        for (row, line) in lines.iter().enumerate() {
            let mut map_line = Vec::new();
            for (col, tile) in line.chars().enumerate() {
                match tile {
                    '^' | '>' | 'v' | '<' => {
                        map_line.push(MapTile::Empty);
                        guard.row = row as i32;
                        guard.col = col as i32;
                        guard.orientation = Orientation::from_char(tile);
                    }
                    _ => map_line.push(MapTile::from_char(tile)),
                }
            }
            map.push(map_line);
        }

        MapState { map, guard }
    }
    fn print(&self) {
        for (row, line) in self.map.iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                if self.guard.row == row as i32 && self.guard.col == col as i32 {
                    print!("{}", self.guard.orientation.as_char());
                } else {
                    print!("{}", tile.as_char());
                }
            }
            println!("");
        }
    }
    fn on_map(&self) -> bool {
        self.map.on_map(self.guard.row, self.guard.col)
    }
    fn blocked(&self) -> bool {
        let (nextrow, nextcol) = self.guard.next();
        self.map
            .get(nextrow, nextcol)
            .is_some_and(|tile| matches!(tile, MapTile::Blocked))
    }
    fn advance(&mut self) {
        if self.blocked() {
            self.guard.right90();
        } else {
            self.guard.advance();
        }
    }
    fn has_loop(&mut self) -> bool {
        let mut visited = HashSet::new();
        while self.on_map() {
            if !visited.insert(self.guard.clone()) {
                return true;
            }
            if self.blocked() {
                self.guard.right90();
            } else {
                self.guard.advance();
            }
        }
        false
    }
}

fn input_lines() -> Vec<String> {
    std::io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let fresh_map = MapState::parse(&input_lines());

    // Part 1:
    {
        let mut map_state = fresh_map.clone();
        let mut visited = HashSet::new();
        while map_state.on_map() {
            visited.insert((map_state.guard.row, map_state.guard.col));
            map_state.advance();
        }
        println!("Part 1: {}", visited.len());
    }

    // Part 2:
    let mut loops = 0;
    for (row, line) in fresh_map.map.iter().enumerate() {
        println!("{}/{}", row, fresh_map.map.rows());
        for (col, tile) in line.iter().enumerate() {
            if matches!(tile, MapTile::Blocked)
                || (fresh_map.guard.row == row as i32 && fresh_map.guard.col == col as i32)
            {
                continue;
            }
            let mut map_state = fresh_map.clone();
            map_state.map[row][col] = MapTile::Blocked;
            if map_state.has_loop() {
                loops += 1;
            }
        }
    }
    println!("Part 2: {}", loops);
}
