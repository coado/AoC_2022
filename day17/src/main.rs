

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
}

enum Rocks {
    Minus,
    Plus,
    L,
    Straight,
    Square,
}

impl Rocks {
    fn get_rocks_queue() -> Vec<Rocks> {
        let rocks_queue = vec![Rocks::Minus, Rocks::Plus, Rocks::L, Rocks::Straight, Rocks::Square];
        rocks_queue
    }

    fn get_rock_coords(&self, highest_point: i32) -> Vec<Node> {
        let y = highest_point + 4;

        let rock_coords = match self {
            Rocks::Minus => {
                let coords = vec![Node{x: 2, y}, Node{x: 3, y}, Node{x: 4, y}, Node{x: 5, y}];
                coords
            },
            Rocks::Plus => {
                let coords = vec![Node{x: 2, y: y+1}, Node{x: 3, y}, Node{x: 3, y: y+1}, Node{x: 3, y: y+2}, Node{x: 4, y: y+1}];
                coords
            },
            Rocks::L => {
                let coords = vec![Node{x: 2, y}, Node{x: 3, y}, Node{x: 4, y}, Node{x: 4, y: y+1}, Node{x: 4, y: y+2}];
                coords
            },
            Rocks::Straight => {
                let coords = vec![Node{x: 2, y}, Node{x: 2, y: y+1}, Node{x: 2, y: y+2}, Node{x: 2, y: y+3}];
                coords
            },
            Rocks::Square => {
                let coords = vec![Node{x: 2, y}, Node{x: 2, y: y+1}, Node{x: 3, y}, Node{x: 3, y: y+1}];
                coords
            }
        };

        rock_coords
    }
}

fn parse() -> Vec<char> {
    let input = std::fs::read_to_string("./data.txt").unwrap();
    let moves: Vec<_> = input.chars().collect();
    moves
}


fn part_1(rocks_to_fail: i32) -> i32 {
    let moves = parse();
    let mut surface = vec![0; 7];
    
    let mut failing_rock_index = 1;
    let mut current_rock_index = 0;
    let mut move_index = 0;

    let rocks_queue = Rocks::get_rocks_queue();

    loop {
        if failing_rock_index > rocks_to_fail {
            break;
        }

        let highest_point = *surface.iter().max().unwrap();
        let mut current_rock = rocks_queue[current_rock_index].get_rock_coords(highest_point);
        
        loop {
            let move_direction = moves[move_index];

            // MOVE ROCK LEFT/RIGHT
            if move_direction == '<' {
                if current_rock[0].x != 0 {
                    for node in &mut current_rock {
                        node.x -= 1;
                    }

                    println!("MOVING LEFT: {:?}", current_rock);
                }
            } else {
                if current_rock[current_rock.len() - 1].x != 6 {
                    for node in &mut current_rock {
                        node.x += 1;
                    }
                    println!("MOVING RIGHT: {:?}", current_rock);

                }
            }
            
            if move_index == moves.len() - 1 {
                move_index = 0;
            } else {
                move_index += 1;
            }


            // MOVE ROCK DOWN
            let move_down = current_rock.iter().all(|node| node.y - 1 > surface[node.x as usize]);

            if !move_down {
                break;
            }
            
            for node in &mut current_rock {
                node.y -= 1;
            }
        }


        // UPDATING SURFACE
        current_rock   
            .iter()
            .for_each(|node| {
                if node.y > surface[node.x as usize] {
                    surface[node.x as usize] = node.y;
                }

            });


        if current_rock_index == rocks_queue.len() - 1 {
            current_rock_index = 0;
        } else {
            current_rock_index += 1;
        }
        failing_rock_index += 1;

        println!("{:?}", surface);
    }

    return *surface.iter().max().unwrap();
}

fn main() {
    let heighest_point = part_1(5);
    println!("{heighest_point}");
}
