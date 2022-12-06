use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn initialize_crates(data : &Vec<String>) -> Vec<Vec<char>>
{
    // Build stacks for the data. The last line of the input will contains just
    // integers to represent the number of stacks
    let stack_re = Regex::new(r"\d+").unwrap();
    let stack_count = stack_re.find_iter(&data.last().unwrap()).count();

    // Setup stacks
    const EMPTY_STACK : Vec<char> = Vec::new();
    let mut stacks = vec!(EMPTY_STACK; stack_count);

    // Initialize with data
    let mut idx = data.len()-1;
    while idx > 0
    {
        let chars = data[idx-1].as_bytes();
        for i in 0..stack_count
        {
            if chars[i*4] == b'[' && chars[i*4+2] == b']'
            {
                stacks[i].push(chars[i*4+1] as char);
            }
        }
        idx -= 1;
    }

    return stacks;
}



fn main() {
    // Read all lines from the file
    let lines = read_lines("data/input.txt");

    // Find the first blank line; this will be the end of the initial stack state
    let mut blank_index = 0;
    for i in 1..lines.len()
    {
        if lines[i].is_empty()
        {
            blank_index = i;
        }
    }

    // Part 1
    {
        // Initialize the stacks
        let mut stacks = initialize_crates(&lines[0..blank_index].to_vec());

        // Process all rules
        let rules_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for rule_index in blank_index+1 .. lines.len()
        {
            let decode_rule = rules_re.captures(&lines[rule_index]).unwrap();
            let crate_count = decode_rule[1].parse::<i32>().unwrap();
            let move_from = (decode_rule[2].parse::<i32>().unwrap() - 1) as usize;
            let move_to = (decode_rule[3].parse::<i32>().unwrap() - 1) as usize;

            for _i in 0 .. crate_count
            {
                let x = stacks[move_from].pop().unwrap();
                stacks[move_to].push(x);
            }
        }

        let mut result = String::new();
        for stack in stacks
        {
            result.push(stack.last().unwrap().clone());
        }
        println!("Top of stacks: {}", result);        
    }

    // Part 2
    {
    // Initialize the stacks
    let mut stacks = initialize_crates(&lines[0..blank_index].to_vec());

    // Process all rules
    let rules_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for rule_index in blank_index+1 .. lines.len()
    {
        let decode_rule = rules_re.captures(&lines[rule_index]).unwrap();
        let crate_count = decode_rule[1].parse::<i32>().unwrap();
        let move_from = (decode_rule[2].parse::<i32>().unwrap() - 1) as usize;
        let move_to = (decode_rule[3].parse::<i32>().unwrap() - 1) as usize;

        let mut crates_in_motion: Vec<char> = Vec::new();
        for _i in 0 .. crate_count
        {
            let x = stacks[move_from].pop().unwrap();
            crates_in_motion.push(x);
        }
        crates_in_motion.reverse();
        stacks[move_to].append(&mut crates_in_motion);
    }

    let mut result = String::new();
    for stack in stacks
    {
        result.push(stack.last().unwrap().clone());
    }
    println!("Top of stacks: {}", result);
    }

}
