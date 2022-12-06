use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
//use std::collections::HashSet;
use regex::Regex;


fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}


fn fully_overlapping_ranges(x0: i32, x1: i32, y0: i32, y1: i32) -> bool
{
    (x0 >= y0 && x1 <= y1) || (y0 >= x0 && y1 <= x1)
}


fn partial_overlapping_ranges(x0: i32, x1: i32, y0: i32, y1: i32) -> bool
{
    (x0 >= y0 && x0 <= y1) || (x1 >= y0 && x1 <= y1) ||
    (y0 >= x0 && y0 <= x1) || (y1 >= x0 && y1 <= x1)
}



fn main() {

    let re = Regex::new(r"^(\d*)-(\d*),(\d*)-(\d*)$").unwrap();

    let mut fully_overlapping_count = 0;

    let lines = read_lines("data/input.txt");
    for line in lines.iter()
    {
        let cap = re.captures(&line).unwrap();
        if fully_overlapping_ranges(cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap(),
                                    cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap())
        {
            fully_overlapping_count += 1;
        }
    }

    println!("Fully overlapping ranges: {}", fully_overlapping_count);


    let mut partial_overlaps = 0;

    for line in lines.iter()
    {
        let cap = re.captures(&line).unwrap();
        if partial_overlapping_ranges(cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap(),
                                      cap[3].parse::<i32>().unwrap(), cap[4].parse::<i32>().unwrap())
        {
            partial_overlaps += 1;
        }
    }

    println!("Overlapping ranges: {}", partial_overlaps);

}
