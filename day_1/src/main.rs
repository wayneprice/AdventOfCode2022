use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


struct Elf
{
    calories: Vec<u32>
}

impl Elf
{
    fn new() -> Elf
    {
        return Elf { calories: Vec::new() };
    }

    fn total_calories(&self) -> u32
    {
        let mut total = 0;
        for calories in &self.calories
        {
            total += calories
        }
        return total;
    }
}


fn main() {
    let path = "data/input.txt";
    let file = match File::open(&path)
    {
        Err(why) => panic!("can't open {}: {}", path, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut elves : Vec<Elf> = Vec::new();
    let mut elf = Elf::new();

    for line in reader.lines()
    {
        let line = line.unwrap();
        if line.is_empty()
        {
            elves.push(elf);
            elf = Elf::new();
        }
        else
        {
            let num : u32 = line.trim().parse().expect("Should be an integer");
            elf.calories.push(num);
        }
    }

    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
    println!("Maximum calories for single elf: {}", elves[0].total_calories());

    println!("Total calories for top three elves: {}", elves[0].total_calories() + elves[1].total_calories() + elves[2].total_calories());
}
