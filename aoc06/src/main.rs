use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<MapTile>>,
}

impl Map {
    fn rows(&self) -> i32 {
        self.map.len() as i32
    }
    fn cols(&self) -> i32 {
        // self.map[0].len()  // may panic
        if self.map.is_empty() {
            0
        } else {
            self.map[0].len() as i32 // Assume a rectangular map
        }
    }
    fn on_map(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && row < self.rows() && col < self.cols()
    }
    // Returns the map tile, or None if we have left the map
    fn get(&self, row: i32, col: i32) -> Option<MapTile> {
        if self.on_map(row, col) {
            Some(self.map[row as usize][col as usize].clone())
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

struct MapState {
    map: Map,
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

        MapState {
            map: Map { map },
            guard,
        }
    }
    fn print(&self) {
        for (row, line) in self.map.map.iter().enumerate() {
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
        }
        self.guard.advance();
    }
}

fn input_lines() -> Vec<String> {
    std::io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let mut map_state = MapState::parse(&input_lines());
    // Part 1:
    let mut visited = HashSet::new();
    while map_state.on_map() {
        visited.insert((map_state.guard.row, map_state.guard.col));
        map_state.advance();
    }
    println!("visited: {}", visited.len());
}
