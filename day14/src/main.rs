use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn neighbours(&self) -> [Coord; 3] {
        let down = Coord {
            x: self.x,
            y: self.y + 1
        };
        let down_left = Coord {
            x: self.x - 1,
            y: self.y + 1
        };
        let down_right = Coord {
            x: self.x + 1,
            y: self.y + 1,
        };

        [down, down_left, down_right]
    }

    fn next_p1(&self, cave: &[Vec<Tile>]) -> Option<Coord> {
        self.neighbours()
            .into_iter()
            .find(|coord| cave[coord.y as usize][coord.x as usize] == Tile::Air)
    }

    fn next_p2(&self, cave: &[Vec<Tile>], floor_y: i32) -> Option<Coord> {
        if (self.y + 1) == floor_y {
            // hit floor
            return None;
        }
        // first available position in neighbours (down, left-down, right-down)
        self.neighbours()
            .into_iter()
            .find(|p| cave[p.y as usize][p.x as usize] == Tile::Air)
    }
        
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]

enum Tile {
    Rock,
    Sand,
    Air,
}

fn rocks_in_cave(rock_lines: Vec<Vec<Coord>>) -> HashSet<Coord> {
    rock_lines
        .iter()
        .flat_map(|path| {
            path.iter().tuple_windows().flat_map(|(start, end)| {
                let diff_x = end.x - start.x;
                let diff_y = end.y - start.y;
                let direction = Coord {
                    x: diff_x.signum(),
                    y: diff_y.signum()
                };

                // one of two differences is always 0 because rock lines are vertical or horizontal
                let amount = diff_x.abs().max(diff_y.abs()) as usize;

                (0..=amount).map(move |amount| {
                    let diff_x = amount as i32 * direction.x;
                    let diff_y = amount as i32 * direction.y;
                    
                    Coord {
                        x: start.x + diff_x,
                        y: start.y + diff_y
                    }
                })

            }) 
        })
        .collect()
}

fn parse() -> Vec<Vec<Coord>> {
    let input = std::fs::read_to_string("./data.txt").unwrap();
    
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (x, y) = coords.split_once(',').unwrap();
                    let x = x.parse::<i32>().unwrap();
                    let y = y.parse::<i32>().unwrap();
                    Coord { x, y }
                })
                .collect()
        })
        .collect()
}


fn part_1() -> usize {
    let rock_lines = parse();
    let rocks = rocks_in_cave(rock_lines);

    let start = Coord { x: 500, y: 0 };
    let max_y = rocks.iter().map(|p| p.y).max().unwrap();

    let width = 500 + max_y + 2;

    let mut cave = vec![vec![Tile::Air; width as usize]; (max_y + 2) as usize];

    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    for i in 0.. {
        let mut sand = start;

        while let Some(next_air_coord) = sand.next_p1(&cave) {
            sand = next_air_coord;
            if sand.y > max_y {
                return i;
            }
        }

        cave[sand.y as usize][sand.x as usize] = Tile::Sand;
    }

    unreachable!()
}


fn part_2() -> usize {
    let rock_lines = parse();
    let rocks = rocks_in_cave(rock_lines);

    let start = Coord { x: 500, y: 0 };
    let max_y = rocks.iter().map(|p| p.y).max().unwrap();

    let width = 500 + max_y + 2;
    let floor_y = max_y + 2;

    let mut cave = vec![vec![Tile::Air; width as usize]; (max_y + 2) as usize];

    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    for i in 0.. {
        let mut sand = start;

        while let Some(next_air_coord) = sand.next_p2(&cave, floor_y) {
            sand = next_air_coord;
        }

        cave[sand.y as usize][sand.x as usize] = Tile::Sand;

        if sand == start {
            return i + 1;
        }
    }

    unreachable!()
}

fn main() {
    let result_1 = part_1();
    println!("Part 1: {result_1}");

    let result_2 = part_2();
    println!("Part 2: {result_2}");

}
