use std::collections::{HashSet, HashMap, BinaryHeap, BTreeSet, VecDeque};
use std::cmp::Ordering;
use itertools::Itertools;

struct Valve<'a> {
    flow: u32,
    neighbours: HashSet<&'a str>
}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32,
    curr: &'a str,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State<'a> {
    opened: BTreeSet<&'a str>,
    curr: &'a str,
    elapsed: u32,
    relieved: u32,
}

fn min_cost(from: &str, to: &str, map: &HashMap<&str, Valve>) -> u32 {
    // dijkstra
    let mut pq = BinaryHeap::new();
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: 0,
        curr: from,
    });

    seen.insert(from);

    while let Some(Node { cost, curr }) = pq.pop() {
        if curr == to {
            return cost;
        }

        for neighbour in map[curr].neighbours.iter() {
            if seen.insert(neighbour) {
                pq.push(Node {
                    cost: cost + 1,
                    curr: neighbour
                });
            }
        }
    }

    u32::MAX
}

fn min_distances<'a>(map: &'a HashMap<&str, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            acc.entry(("AA", name1))
                .or_insert_with(|| min_cost("AA", name1, map));
            acc.entry(("AA", name2))
                .or_insert_with(|| min_cost("AA", name2, map));
                
            let dist = min_cost(name1, name2, map);

            acc.insert((name1, name2), dist);
            acc.insert((name2, name1), dist);
            acc
        })
        
}

fn parse(input: &str) -> HashMap<&str, Valve> {
    input
        .lines()
        .map(|line| {
            let (valve, neighbours) = line.split_once("; ").unwrap();
            let valve = valve.strip_prefix("Valve ").unwrap();
            let (name, flow) = valve.split_once(" has flow rate=").unwrap();
            let flow = flow.parse::<u32>().unwrap();
            let neighbours = neighbours
                .strip_prefix("tunnels lead to valves ")
                .or_else(|| neighbours.strip_prefix("tunnel leads to valve "))
                .unwrap();
            let neighbours = neighbours.split_terminator(", ").collect();

            (name, Valve { flow, neighbours })
        })
        .collect()
}

fn wait_until_ending<'a>(
    max_time: u32,
    elapsed: u32,
    relieved: u32,
    opened: &'a BTreeSet<&'a str>,
    map: &'a HashMap<&'a str, Valve>,
) -> u32 {
    let time_left = max_time - elapsed;
    let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow).sum();
    relieved + (relieved_per_min * time_left)
}


pub fn part_1(input: &str) -> u32 {
    let map = parse(input);
    let dist_map = min_distances(&map); // key: (from, to), value: move_cost
    let flowing: HashSet<_> = map
        .iter()
        .filter(|(_, valve)| valve.flow > 0)
        .map(|(&name, _)| name)
        .collect();

    let mut max_relieved = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_back(State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });
    // current position doesn't matter for seen
    seen.insert((BTreeSet::new(), 0, 0));

    while let Some(State {
        opened,
        curr,
        elapsed,
        relieved,
    }) = q.pop_front()
    {
        // if all flowing valves are opened, wait until the end
        if opened.len() == flowing.len() || elapsed >= 30 {
            let relieved_at_end = wait_until_ending(30, elapsed, relieved, &opened, &map);
            max_relieved = max_relieved.max(relieved_at_end);
            continue;
        }
        // for every unopened valve, run simulation
        let unopened = flowing.iter().filter(|name| !opened.contains(*name));

        for dest in unopened {
            // how long would moving to dest take? +1 to open the valve
            let cost = dist_map[&(curr, *dest)] + 1;
            let new_elapsed = elapsed + cost;
            // if opening the dest valve would exceed the time limit, wait until the end
            if new_elapsed >= 30 {
                let relieved_at_end = wait_until_ending(30, elapsed, relieved, &opened, &map);
                max_relieved = max_relieved.max(relieved_at_end);
                continue;
            }

            // relieve pressure of opened valves while we move to dest and open it
            let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow).sum();
            let new_relieved = relieved + (relieved_per_min * cost);
            // add opened valve to opened valves
            let mut new_opened = opened.clone();
            new_opened.insert(dest);

            if seen.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                q.push_back(State {
                    opened: new_opened,
                    curr: dest,
                    elapsed: new_elapsed,
                    relieved: new_relieved,
                });
            }
        }
    }

    max_relieved
}

fn main() {
    let input = std::fs::read_to_string("./data.txt").unwrap();
    let res_1 = part_1(&input);
    println!("{res_1}");
}
