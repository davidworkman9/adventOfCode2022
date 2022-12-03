use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_to_priority (char: &str) -> u8 {
    let array = char.as_bytes();
    let code = array[0];

    if code > 64 && code < 91 {   // capital 27-52
        code - 38
    } else if code > 96 && code < 123 { // lower case 1-26
        code - 96
    } else {
        0
    }
}

fn main() {
    let file = File::open("input.txt").expect("Couldn't open input.txt");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();

    let mut sum: u32 = 0;
    for line_x in lines.iter() {
        let (comp_1, comp_2) = line_x.split_at(line_x.len() / 2);
        
        
        let mut matching_char: char = '&';
        let chars: Vec<char> = comp_1.chars().collect();
        for char in chars {
            if comp_2.contains(char) {
                matching_char = char;
                break;
            }
        }

        let priority = char_to_priority(matching_char.to_string().as_str()) as u32;
        sum += priority;
    }

    println!("The sum of all priorities is {}", sum);
    assert!(sum == 8349);   // part 1


    let mut groups:Vec<Vec<&String>> = Vec::new();
    let mut curr_group = Vec::new();
    for line_x in lines.iter() {
        curr_group.push(line_x);
        if curr_group.len() == 3 {
            groups.push(curr_group);
            curr_group = Vec::new();
        }
    }

    let mut sum2 = 0;
    for group in groups {
        let elf1 = group[0];
        let elf2 = group[1];
        let elf3 = group[2];

        let mut matching_char: char = '&';
        let chars: Vec<char> = elf1.chars().collect();
        for char in chars {
            if elf2.contains(char) && elf3.contains(char) {
                matching_char = char;
                break;
            }
        }

        let priority = char_to_priority(matching_char.to_string().as_str()) as u32;
        sum2 += priority;
    }

    println!("The sum of the priority of each group is {}", sum2);
    assert!(sum2 == 2681); // part 2
}
