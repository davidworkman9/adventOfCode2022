use regex::Regex;

fn main() {
    let col_num_regex = Regex::new(r"^(\s?+[0-9]\s+)+$").unwrap();
    let move_regex = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    let str = include_str!("../input.txt");
    let lines: Vec<String> = str.split('\n').map(String::from).collect();
    

    let mut columns: Vec<Vec<String>> = Vec::new();
    let mut past_start = false;
    let mut move_cmds: Vec<(usize, usize, usize)> = Vec::new();
    for (line_idx, line) in lines.iter().enumerate() {
        if line.trim() == "" {
            past_start = true;
            continue;
        }
        if col_num_regex.is_match(line) {
            past_start = true;
            continue;
        }

        if !past_start {
            let chars: Vec<char> = line.chars().collect();

            let chunks = chars.chunks(4);
            let num_columns = chunks.len();
            if line_idx == 0 {
                columns = vec![Vec::new();num_columns];
            }
            for (idx, group) in chunks.enumerate() {
                let letter = group.get(1).unwrap().to_string().trim().to_string();
                if !letter.is_empty() {
                    columns[idx].push(letter);
                }
            }
        } else {
            let mut move_cmd_tuple = (usize::MAX, usize::MAX, usize::MAX);
            if !move_regex.is_match(line) {
                panic!("Line does not match regex. Line: {}", line);
            }
            for cap in move_regex.captures_iter(line) {
                move_cmd_tuple.0 = cap[1].parse::<usize>().unwrap();
                move_cmd_tuple.1 = cap[2].parse::<usize>().unwrap();
                move_cmd_tuple.2 = cap[3].parse::<usize>().unwrap();
            }
            

            if move_cmd_tuple.0 == usize::MAX || move_cmd_tuple.1 == usize::MAX || move_cmd_tuple.2 == usize::MAX {
                panic!("Invalid move command parsed, did not have 3 integers.");
            }
            move_cmds.push(move_cmd_tuple);
        }
    }

    for col in columns.iter_mut() {
        col.reverse();
    }
    let mut columnsPt2 = columns.clone();

     for move_cmd in move_cmds.iter() {
        for _ in 0..move_cmd.0 {
            let to_move = columns[move_cmd.1-1].pop().unwrap().clone();
            columns[move_cmd.2-1].push(to_move);
        }
    }

    let mut str: String = String::new();
    for col in columns {
        str += &col[col.len()-1].to_string();
    }
    println!("Part 1: {}", str);


    for move_cmd in move_cmds.iter() {
        let mut to_move: Vec<String> = Vec::new();
        for _ in 0..move_cmd.0 {
            let str_to_move = columnsPt2[move_cmd.1-1].pop().unwrap().clone();
            to_move.push(str_to_move);
        }
        to_move.reverse();

        for str in to_move {
            columnsPt2[move_cmd.2-1].push(str);
        }
    }

    let mut str: String = String::new();
    for col in columnsPt2 {
        str += &col[col.len()-1].to_string();
    }
    println!("Part 2: {}", str);
}
