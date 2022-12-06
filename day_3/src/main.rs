use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;


fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn priority(value: char) -> u32
{
    let c8 = value as u8;
    (if c8 >= b'a' && c8 <= b'z' { c8 - b'a' + 1 } else { c8 - b'A' + 27 }) as u32
}


fn main() {
    let path = "data/input.txt";
    let lines = read_lines(path);

    // Part 1: Search for duplicate items in each compartment in the rucksack
    let mut priority_sum = 0;
    for line in lines.iter()
    {
        let len = line.len();
        let half_len = len / 2;
        let compartment_1 = &line[0..half_len];
        let compartment_2 = &line[half_len..len];
    
        for c in compartment_2.chars()
        {
            if compartment_1.contains(c)
            {
                priority_sum += priority(c);
                break;
            }
        }
    }
    println!("Sum of duplicate item priorities: {}", priority_sum);

    // Part 2: Find the badge for each set of three rucksacks
    let mut priority_sum = 0;
    let line_count = lines.len();
    for i in (0 .. line_count).step_by(3)
    {
        let mut rucksack1: HashSet<char> = HashSet::from_iter(lines[i].chars());
        let rucksack2: HashSet<char> = HashSet::from_iter(lines[i+1].chars());
        let rucksack3: HashSet<char> = HashSet::from_iter(lines[i+2].chars());

        // Intersection
        rucksack1.retain(|x| rucksack2.contains(x));
        rucksack1.retain(|x| rucksack3.contains(x));

        // Rucksack 1 should now contain just one entry
        assert_eq!(rucksack1.len(), 1);
        let c = rucksack1.iter().next().unwrap().clone();
        priority_sum += priority(c);
    }
    println!("Sum of badge priorities: {}", priority_sum);

}
