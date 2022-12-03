use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Elf {
    _calories: Vec<u32>,
    total: u32,
}

fn main() {
    let mut elves = Vec::new();
    {
        let file = File::open("input.txt").expect("Couldn't open input.txt");
        let reader = BufReader::new(file);
        let mut calories = Vec::new();
        for (i, line_x) in reader.lines().flatten().enumerate() {
            if line_x.trim() == "" {
                let total = calories.iter().sum();
                elves.push(Elf {
                    _calories: calories,
                    total
                });
                calories = vec![];
                continue;
            }

            let maybe_calorie = line_x.parse::<u32>();
            match maybe_calorie {
                Ok (calorie) => {
                    calories.push(calorie);
                }

                Err (_e) => {
                    println!("Skipping line {} not a valid integer.", i + 1);
                }
            }
        }
    }

    let maybe_max_cal = elves.iter().map(|x| x.total).max();

    match maybe_max_cal {
        Some (max_cal) => {
            println!("Complete, found {} elves. The elf with the most calories has {} total calories.", elves.len(), max_cal);
        }
        None => {
            println!("Was unable to find the elf with the most for some reason.");
        }
    }


    let num_elves = elves.len();

    let mut totals = vec![0; num_elves];
    for (i, elf) in elves.iter().enumerate() {
        totals[i] = elf.total;
    }

    totals.sort();
    let highest = totals[num_elves-1];
    let second_highest = totals[num_elves-2];
    let third_highest = totals[num_elves-3];
    let top_three = highest + second_highest + third_highest;

    println!("The top 3 elves have a total calorie count of {}.", top_three);

}
