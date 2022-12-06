use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::collections::VecDeque;


fn get_input(filename: impl AsRef<Path>) -> String {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    
    buf.lines().nth(0).unwrap().unwrap()
}


fn main() {
    let data = get_input("data/input.txt");

    // Part 1: Dirty way to find first four non-matching chars
    let mut x0 = ' ';
    let mut x1 = ' ';
    let mut x2 = ' ';
    let mut x3 = ' ';
    let mut count = 0;
    for x in data.chars()
    {
        count += 1;
        x0 = x1;
        x1 = x2;
        x2 = x3;
        x3 = x;

        if x0 != x1 && x0 != x2 && x0 != x3 && x1 != x2 && x1 != x3 && x2 != x3 && x0 != ' '
        {
            println!("Found 4-digit start code after {} chars", count);
            break;
        }
    }

    // Part 2: Find first 14 non-matching chars
    let mut char_count = vec![0; 256];
    let mut non_distinct_count = 0;
    let mut ringbuffer = VecDeque::new();
    let mut count = 0;
    for x in data.chars()
    {
        count += 1;

        let u = x as usize;
        ringbuffer.push_back(x);
        char_count[u] += 1;
        if char_count[u] == 2 { non_distinct_count += 1; }
        if count > 14
        {
            let x = ringbuffer.pop_front().unwrap();
            let u = x as usize;
            if char_count[u] == 2 { non_distinct_count -= 1; }
            char_count[u] -= 1;

            if non_distinct_count == 0
            {
                println!("Found 14-digit start code after {} chars", count);
                break;
            }
        }
    }
}
