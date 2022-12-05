use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    part_one();
    part_two();
 
}

fn part_one() {

    let mut total_priority = 0u32;

    if let Ok(lines) = read_lines("./data/input.txt") {

        for rucksack in lines.flatten() {

            let nr_of_items_per_comp = rucksack.len() / 2;

            let comp_1 = &rucksack[..nr_of_items_per_comp];
            let comp_2 = &rucksack[nr_of_items_per_comp..];

            for item in comp_1.chars() {
                if comp_2.contains(item) {
                    total_priority += get_prio(item);
                    break;
                }
            }
        }
    }

    // Part 1
    println!("Part 1");
    println!("The total score is {}.", total_priority);

}

fn part_two() {

    let mut total_priority = 0u32;
    let mut group_counter  = 1u32;
    let mut group_rucksack: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./data/input.txt") {

        for rucksack in lines.flatten() {

            if group_counter < 3 {
                group_rucksack.push(rucksack);
                group_counter += 1;
                continue;
            }

            group_rucksack.push(rucksack);

            for item in group_rucksack[0].chars() {
                if group_rucksack[1].contains(item) && group_rucksack[2].contains(item) {
                    total_priority += get_prio(item);
                    break;    
                }
            }

            group_counter = 1;
            group_rucksack.clear();

        }
    }

    // Part 2
    println!("Part 2");
    println!("The total score is {}.", total_priority);

}

fn get_prio(item: char) -> u32 {

    let item_raw_id = item as u32;
    if (65..=90).contains(&item_raw_id) {
        item_raw_id - 38
    } else if (97..=122).contains(&item_raw_id) {
        item_raw_id - 96
    } else {
        panic!("Unknown item: {}", item);
    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


