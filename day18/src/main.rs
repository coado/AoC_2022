use std::{collections::HashSet, hash::Hash};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Default, Copy)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}

enum Dimensions {
    X,
    Y,
    Z,
}

impl Coord {
    fn get_neighbours(&self) -> Vec<Coord> {
        let mut neighbours: Vec<Coord> = Vec::new(); 
        
        for dimension in [Dimensions::X, Dimensions::Y, Dimensions::Z] {
            for direction in [-1, 1] {
                
                let mut neighbour = self.clone();
                
                match dimension {
                    Dimensions::X => neighbour.x += direction,
                    Dimensions::Y => neighbour.y += direction,
                    Dimensions::Z => neighbour.z += direction,
                }

                neighbours.push(neighbour);
            }
        }

        neighbours
    }   

    fn in_bounds(&self, bounds: &[Coord; 2]) -> bool {
        let [mins, maxs] = bounds;
        self.x >= mins.x - 1
            && self.x <= maxs.x + 1
            && self.y >= mins.y - 1
            && self.y <= maxs.y + 1
            && self.z >= mins.z - 1
            && self.z <= maxs.z + 1
    }
}

fn parse() -> HashSet<Coord> {
    let input = std::fs::read_to_string("./data.txt").unwrap();

    input
        .lines()
        .map(|line| {
            let mut values = line.split(',').map(|value| value.parse().unwrap());

            Coord {
                x: values.next().unwrap(),
                y: values.next().unwrap(),
                z: values.next().unwrap(),
            }

        })
        .collect()
}

fn part_1() -> i32 {
    let blocks = parse();

    let result = blocks
        .iter()
        .flat_map(|block| block.get_neighbours())
        .filter(|neighbour| !blocks.contains(neighbour))
        .count();

    result as i32
}

fn bounds(cubes: &HashSet<Coord>) -> [Coord; 2] {
    cubes.iter().fold(
        [Coord::default(), Coord::default()],
        |[mut mins, mut maxs], cube| {
            mins.x = mins.x.min(cube.x);
            mins.y = mins.y.min(cube.y);
            mins.z = mins.z.min(cube.z);
            maxs.x = maxs.x.max(cube.x);
            maxs.y = maxs.y.max(cube.y);
            maxs.z = maxs.z.max(cube.z);
            [mins, maxs]
        })
}


fn exposed(cubes: &HashSet<Coord>) -> HashSet<Coord> {
    let bounds = bounds(cubes);
    let mut exposed = HashSet::new();

    let start = Coord::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(start);

    while let Some(cube) = stack.pop() {
        for neighbour in cube.get_neighbours() {
            if cubes.contains(&neighbour) || !neighbour.in_bounds(&bounds) {
                continue;
            }

            if seen.insert(neighbour) {
                stack.push(neighbour);
                exposed.insert(neighbour);
            }
        }
    }
    exposed
}

fn part_2() -> i32 {
    let blocks = parse();
    let exposed = exposed(&blocks);

    let result = blocks
        .iter()
        .flat_map(|block| block.get_neighbours())
        .filter(|neighbour| exposed.contains(neighbour))
        .count();

    result as i32
}

fn main() {
    let result_1 = part_1();
    println!("{result_1}");

    let result_2 = part_2();
    println!("{result_2}");
}
