use std::{collections::VecDeque};

struct State {
    // [ore, clay, obisidian, geode]
    inventory: [u16; 4],
    // [ore_robots, clay_robots, obisidian_robots, geode_robots]
    bots: [u16; 4],
    // time
    elapsed: u16
}

fn max_geodes(blueprint: &[[u16; 4]; 4]) -> u16 {
    let max_time = 24;
    let mut max_geodes = 0;

    let mut q = VecDeque::new();
    q.push_back(State {
        inventory: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(State {
        inventory,
        bots,
        elapsed,
    }) = q.pop_front()
    {
        println!("RUNNGING");
        // for every bot cost, run simulation
        for i in 0..blueprint.len() {
            let costs = &blueprint[i];
            // Find the limiting resource type for the costs.
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        // state has enough of current resource in inventory to cover that part of the target bot cost. 0 wait time
                        cost if cost <= inventory[idx] => 0,
                        // no target bot type made yet
                        // we can't build it (it takes more than max_time to build it).
                        _ if bots[idx] == 0 => max_time + 1,
                        _ => (costs[idx] - inventory[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            // if that choice would cause the time limit be to exceeded, skip
            // the + 1 is so the built bot has the chance to do something, it merely being built is not enough
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            // gather ores with previously available bots
            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            // increase bot type for the bot we just built
            let mut new_bots = bots;
            new_bots[i] += 1;

            q.push_back(State {
                inventory: new_inventory,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }

        let geodes = inventory[3] + bots[3] * (max_time - elapsed);
        max_geodes = geodes.max(max_geodes);
    }

    max_geodes
}


fn parse() -> Vec<[[u16; 4]; 4]> {
    let input = std::fs::read_to_string("data.txt").unwrap();
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        // ore bots cost ore
        let ore_bot_costs = [iter.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        // clay bots cost ore
        let clay_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, 0, 0];
        // obsidian bots cost ore and clay
        let obsidian_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            iter.nth(2).unwrap().parse().unwrap(),
            0,
            0,
        ];
        // geode bots cost ore and obsidian
        let geode_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            0,
            iter.nth(2).unwrap().parse().unwrap(),
            0,
        ];

        let blueprint = [
            ore_bot_costs,
            clay_bot_costs,
            obsidian_bot_costs,
            geode_bot_costs,
        ];
        blueprints.push(blueprint);
    }
    
    blueprints
}


fn part_1() -> usize {
    let blueprints = parse();

    blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint))
        .enumerate()
        .map(|(idx, geodes)| (idx + 1) * usize::from(geodes))
        .sum()
}

fn main() {
    let result_1 = part_1();
    println!("result_1: {result_1}");
    // println!("Hello, world!");
}
