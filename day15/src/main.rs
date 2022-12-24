use std::ops::RangeInclusive;
use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn manhattan(self, other: &Self) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
 }


 fn beacon_row_range(sensor: &Coord, beacon: &Coord, row: i32) -> Option<RangeInclusive<i32>> {
    let radius = sensor.manhattan(beacon);
    let offset = radius - (sensor.row - row).abs();
    if offset < 0 {
        None
    } else {
        Some(sensor.col - offset..=sensor.col + offset)
    }
}

fn parse() -> Option<Vec<[Coord; 2]>> {
    let input = std::fs::read_to_string("./data.txt").unwrap();

    let mut pairs = Vec::new();
    for line in input.lines() {
        let (sensor, beacon) = line.split_once(": ")?;
        let sensor = sensor.strip_prefix("Sensor at ")?;
        let beacon = beacon.strip_prefix("closest beacon is at ")?;
        let (sx, sy) = sensor.split_once(", ")?;
        let (bx, by) = beacon.split_once(", ")?;
        let sx = sx.strip_prefix("x=")?;
        let sy = sy.strip_prefix("y=")?;
        let bx = bx.strip_prefix("x=")?;
        let by = by.strip_prefix("y=")?;
        
        let pair = [
            Coord {
                col: sx.parse().ok()?,
                row: sy.parse().ok()?,
            },
            Coord {
                col: bx.parse().ok()?,
                row: by.parse().ok()?,
            }
        ];

        pairs.push(pair);
    }

    Some(pairs)

}



fn row_ranges(row: i32, pairs: &[[Coord; 2]]) -> Vec<RangeInclusive<i32>> {
    let mut ranges: Vec<_> = pairs
        .iter()
        .flat_map(|pair| beacon_row_range(&pair[0], &pair[1], row))
        .collect();

   
    ranges.sort_unstable_by_key(|range| *range.start());

    let mut merged_ranges = vec![ranges[0].clone()];
    for next in &ranges[1..] {
        let last_idx = merged_ranges.len() - 1;
        let last = &merged_ranges[last_idx];
        // check if ranges overlap
        if next.start() <= last.end() || last.end() + 1 == *next.start() {
            // replace with bigger range
            if next.end() > last.end() {
                // let old = &merged_ranges[last_idx];
                let new = *last.start()..=*next.end();
                merged_ranges[last_idx] = new;
            }
        } else {
            merged_ranges.push(next.clone());
        }
    }

    merged_ranges
}



fn part_1() -> usize {
    let input = parse().unwrap();
    let row = 2_000_000;

    let covered = row_ranges(row, &input)
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i32>() as usize;
    
    let beacons = input
        .into_iter()
        .map(|pair| pair[1])
        .filter(|beacon| beacon.row == row)
        .map(|beacon| beacon.col)
        .dedup()
        .count();

    covered - beacons
}

fn part_2() -> i64 {
    let input = parse().unwrap();
    let size = 4_000_000;

    let (row, col_ranges) = (0..=size)
        .rev()
        .map(|row| (row, row_ranges(row, &input)))
        .find(|(_, ranges)| ranges.len() > 1)
        .unwrap();

    let col = col_ranges.first().unwrap().end() + 1;

    i64::from(col) * 4_000_000 + i64::from(row)
}
 
fn main() {
    let res_1 = part_1();
    println!("{res_1}");

    let res_2 = part_2();
    println!("{res_2}");

}   
