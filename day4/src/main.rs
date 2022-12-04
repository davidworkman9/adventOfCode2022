use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_pcs (range_str: &String) -> Vec<u32> {
    let numbers: Vec<String> = range_str.split('-').map(String::from).collect();
    if numbers.len() != 2 {
        panic!("Range with more than two numbers found.");
    }

    let start = numbers[0].parse::<u32>().unwrap();
    let end = numbers[1].parse::<u32>().unwrap();

    (start..end+1).collect()
}

fn main() {
    let file = File::open("input.txt").expect("Couldn't open input.txt");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();

    let mut full_overlap_count = 0;
    let mut partial_overlap_count = 0;
    for line_x in lines.iter() {
        let pcs: Vec<String> = line_x.split(',').map(String::from).collect();

        if pcs.len() != 2 {
            panic!("More than two elves found on one line.");
        }

        let range_1 = get_pcs(&pcs[0]);
        let range_2 = get_pcs(&pcs[1]);

        let mut ranges = vec![range_1, range_2];
        ranges.sort_by(|v, v2| v.len().partial_cmp(&v2.len()).unwrap());

        let mising_from_lg_range = ranges[0].iter().filter(|x| !ranges[1].contains(x));
        let num_missing = mising_from_lg_range.count();
        if num_missing > 0 {
            if num_missing < ranges[0].len() {
                partial_overlap_count += 1;
            }
        } else {
            full_overlap_count += 1;
            partial_overlap_count += 1;
        }
    }

    println!("num of full overlap {}", full_overlap_count); // part 1
    println!("num of partial overlap {}", partial_overlap_count); // part 2
}
