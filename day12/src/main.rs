use std::{fs, collections::{BinaryHeap, HashSet}};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut result = Vec::new();

        // up
        if self.y > 0 {
            result.push(Self {
                x: self.x,
                y: self.y - 1
            });
        }

        // down
        if self.y < rows - 1 {
            result.push(Self {
                x: self.x,
                y: self.y + 1
            });
        }

        // left
        if self.x > 0 {
            result.push(Self {
                x: self.x - 1,
                y: self.y
            });
        }

        // right
        if self.x < cols - 1 {
            result.push(Self {
                x: self.x + 1,
                y: self.y
            });
        }

        result
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: u32,
    coord: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse() -> (Coord, Coord, Vec<Vec<u8>>, usize, usize) {
    let input = fs::read_to_string("./data.txt").unwrap();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut map = vec![vec![0; cols]; rows];
    let mut start = Coord { x: 0, y: 0};
    let mut end = Coord { x: 0, y: 0 };

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let letter = match c {
                'S' => {
                    start.x = col;
                    start.y = row;
                    'a'
                }
                'E' => {
                    end.x = col;
                    end.y = row;
                    'z'
                }
                'a'..='z' => c,
                _ => panic!("Invalid input"),
            };

            let val = letter as u8 - b'a';
            map[row][col] = val;
        }
    }

    (start, end, map, rows, cols)
}


fn part_1() -> u32 {
    let (start, end, map, rows, cols) = parse();
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(Node {
        cost: 0,
        coord: start,
    });

    visited.insert(start);

    while let Some(Node { coord, cost }) = pq.pop() {
        if coord == end {
            return cost;
        }

        let curr_height = map[coord.y][coord.x];
        let neighbours = coord.neighbours(rows, cols);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|coord| {
                let height = map[coord.y][coord.x];
                height <= curr_height || height == curr_height + 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    coord: *candidate
                })
            }
        }
        }

        panic!("No path found!");
}


pub fn part_2() -> u32 {
    let (start, end, map, rows, cols) = parse();
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Node {
        cost: 0,
        coord: end,
    });
    visited.insert(start);

    while let Some(Node { coord, cost }) = pq.pop() {
        let curr_height = map[coord.y][coord.x];

        if curr_height == 0 {
            return cost;
        }

        let neighbours = coord.neighbours(rows, cols);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|coord| {
                let height = map[coord.y][coord.x];
                height >= curr_height || height == curr_height - 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    coord: *candidate,
                })
            }
        }
    }

    panic!("No path found");
}


fn main() {
    let path_1_length = part_1();
    println!("{path_1_length}");

    let path_2_length = part_2();
    println!("{path_2_length}");
}
