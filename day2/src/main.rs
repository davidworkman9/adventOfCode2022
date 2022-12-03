use std::fs::File;
use std::io::{BufRead, BufReader};



fn get_your_score_pt2 (opp: &str, you: &str) -> u32 {
    match you {
        // lose
        "X" => match opp {
            // Rock
            "A" => 3,
            // Paper
            "B" => 1,
            // Scissors
            "C" => 2,
            _ => 0,
        },
        // Draw
        "Y" => match opp {
            // Rock
            "A" => 3 + 1,
            // Paper
            "B" => 3 + 2,
            // Scissors
            "C" => 3 + 3,
            _ => 0,
        }
        // win
        "Z" => match opp {
            // Rock
            "A" => 6 + 2,
            // Paper
            "B" => 6 + 3,
            // Scissors
            "C" => 6 + 1,
            _ => 0,
        }
        _ => 0,
    }
}

fn get_your_score (opp: &str, you: &str) -> u32 {
    let usage_score: u32 = match you {
        // Rock
        "X" => 1,
        // Paper
        "Y" => 2,
        // Scissors
        "Z" => 3,
        _ => 0,
    };

    
    let gameplay_score: u32 = match you {
        // Rock
        "X" => match opp {
            // Rock
            "A" => 3,
            // Paper
            "B" => 0,
            // Scissors
            "C" => 6,
            _ => 0,
        },
        // Paper
        "Y" => match opp {
            // Rock
            "A" => 6,
            // Paper
            "B" => 3,
            // Scissors
            "C" => 0,
            _ => 0,
        }
        // Scissors
        "Z" => match opp {
            // Rock
            "A" => 0,
            // Paper
            "B" => 6,
            // Scissors
            "C" => 3,
            _ => 0,
        }
        _ => 0,
    };

    usage_score + gameplay_score
}

fn main() {

    let mut games = Vec::new();
    {
        let file = File::open("input.txt").expect("Couldn't open input.txt");
        let reader = BufReader::new(file);

        // Rust won't let me do this because it "moves" the reader?
        // games.reserve(reader.lines().flatten().count());
        
        for line_x in reader.lines().flatten() {

            
            let pcs = line_x.split(' ').map(String::from);

            let mut game = vec!["".to_string();2];
            for (i, str) in pcs.enumerate() {
                if i > 1 {
                    break;
                }

                game[i] = str;
            }
            
            games.push(game);
        }
    }

    let sum_of_all_games: u32 = games.iter()
        .map(|game| get_your_score(game[0].as_str(), game[1].as_str()))
        .sum();

    // part 1
    println!("Sum of all games: {}", sum_of_all_games);


    let sum_of_all_games_pt2: u32 = games.iter()
    .map(|game| get_your_score_pt2(game[0].as_str(), game[1].as_str()))
    .sum();
    
    // part 2
    println!("Sum of all games pt2: {}", sum_of_all_games_pt2);
}
