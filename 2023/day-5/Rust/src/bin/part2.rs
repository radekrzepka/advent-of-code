use std::fs;
use std::io::{self};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    start: i64,
    length: i64,
    has_changed: bool,
}

impl SeedRange {
    fn new(start: i64, length: i64, has_changed: bool) -> Self {
        SeedRange { start, length, has_changed }
    }

    fn end(&self) -> i64 {
        self.start + self.length
    }

    fn intersection(&self, other: &SeedRange) -> Option<SeedRange> {
        let start_max = std::cmp::max(self.start, other.start);
        let end_min = std::cmp::min(self.end(), other.end());

        if start_max < end_min {
            Some(SeedRange::new(start_max, end_min - start_max, true))
        } else {
            None
        }
    }

    fn non_intersecting_parts(&self, other: &SeedRange, offset: i64) -> Vec<SeedRange> {
        let intersection = self.intersection(other);
        match intersection {
            Some(intersect) => {
                let mut parts = Vec::new();
                if self.start < intersect.start {
                    parts.push(SeedRange::new(self.start, intersect.start - self.start, self.has_changed));
                }
                if intersect.end() < self.end() {
                    parts.push(SeedRange::new(intersect.end(), self.end() - intersect.end(), self.has_changed));
                }
                parts.push(SeedRange::new(intersect.start + offset, intersect.length, true));

                parts
            },
            None => vec![SeedRange::new(self.start, self.length, self.has_changed)],
        }
    }
}

pub fn main() -> io::Result<()> {
    let start = Instant::now();

    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let mut input = file.split("\r\n\r\n");
    let values: Vec<i64> = input.next().unwrap().split(":").last().unwrap().split(" ").filter_map(|v| v.parse::<i64>().ok()).collect();
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    let mut updated_seed_ranges: Vec<SeedRange> = Vec::new();

    for i in (0..values.len()).step_by(2) {
        let seed_start = values[i];
        let seed_length = values[i+1];

        let seed_range = SeedRange::new(seed_start, seed_length, false);
        seed_ranges.push(seed_range);
    } 

    for map_str in input.by_ref() {
        let seed_data = map_str.split(":").last().unwrap().split("\r\n");
        for line in seed_data {
            if line == "" {
                seed_ranges = seed_ranges.iter().map(|seed| SeedRange::new(seed.start, seed.length, false)).collect();
                continue;
            }
            
            let mut record = line.split(" ");

            let destination_start = record.next().unwrap().parse::<i64>().unwrap();
            let source_start = record.next().unwrap().parse::<i64>().unwrap();
            let range_length = record.next().unwrap().parse::<i64>().unwrap();

            let new_seed_range = SeedRange::new(source_start, range_length, false);

            for seed_range in seed_ranges.iter() {
                if seed_range.has_changed {
                    updated_seed_ranges.push(*seed_range);
                    continue;
                }
    
                if let Some(intersection) = seed_range.intersection(&new_seed_range) {
                    updated_seed_ranges.extend(seed_range.non_intersecting_parts(&new_seed_range, destination_start - source_start));
                } else {
                    updated_seed_ranges.push(*seed_range);
                }
            }

            seed_ranges.clear();
            seed_ranges.extend(updated_seed_ranges.drain(..));
            updated_seed_ranges.clear();
        }
    }

    let duration = start.elapsed();

    println!("{:?}", seed_ranges.iter().map(|v| v.start).min());
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}