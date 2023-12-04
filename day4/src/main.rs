use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader};

fn parse_input(line: &str) -> u32 {
    let mut hash_map = HashMap::new();
    let groups: Vec<_> = line.split(':').last().unwrap().split('|').collect();
    let _:Vec<_> = groups.first().unwrap()
        .split(' ')
        .filter(|x| *x != "")
        .map(|x| hash_map.insert(x.parse::<u32>().unwrap(), "")).collect();

    let number_have: Vec<_> = groups.last().unwrap()
        .split(' ')
        .filter(|x| *x != "")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    //println!("map: {:?}", hash_map);
    //println!("num: {:?}", number_have);

    let mut winning_sum = 0;
    for n in number_have {
        if hash_map.contains_key(&n) {
            match winning_sum {
                0 => winning_sum = 1,
                1 => winning_sum = 2,
                _ => winning_sum *= 2
            }
        }
    }

    winning_sum
}

fn parse_input_v2(line: &str) -> u32 {
    let mut hash_map = HashMap::new();
    let groups: Vec<_> = line.split(':').last().unwrap().split('|').collect();
    let _:Vec<_> = groups.first().unwrap()
        .split(' ')
        .filter(|x| *x != "")
        .map(|x| hash_map.insert(x.parse::<u32>().unwrap(), "")).collect();

    let number_have: Vec<_> = groups.last().unwrap()
        .split(' ')
        .filter(|x| *x != "")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut matching_count = 0;
    for n in number_have {
        if hash_map.contains_key(&n) {
            matching_count += 1;
        }
    }

    matching_count
}


fn main() {
    let mut winning_count = Vec::new();
    for line in read_to_string("day4/input").unwrap().lines() {
        let winning_sum = parse_input(line);
        winning_count.push(winning_sum);
    }

    println!("{}", winning_count.iter().sum::<u32>());

    let input = File::open("day4/input").unwrap();
    let buffered = BufReader::new(input);
    let line_count = buffered.lines().count();

    let mut matching_counts: Vec<_> = vec![0; line_count];
    let mut line_no = 0;
    for line in read_to_string("day4/input").unwrap().lines() {
        let count = parse_input_v2(line);
        let copies_prev = matching_counts[line_no as usize];
        //println!("count: {}", count);
        matching_counts[line_no as usize] += 1;
        for i in line_no+1..line_no+count+1 {
            //println!("copied: {}",copies_prev);
            matching_counts[i as usize] += copies_prev + 1;
        }
        line_no += 1;
    }

    //println!("{:?}", matching_counts);
    println!("{}", matching_counts.iter().sum::<u32>());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_input() {
        let winning_sum = parse_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(winning_sum, 8);
        let winning_sum = parse_input("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(winning_sum, 2);
        let winning_sum = parse_input("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(winning_sum, 2);
        let winning_sum = parse_input("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(winning_sum, 1);
        let winning_sum = parse_input("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(winning_sum, 0);
    }

    #[test]
    fn test_parse_input_v2() {
        let winning_sum = parse_input_v2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(winning_sum, 4);
        let winning_sum = parse_input_v2("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(winning_sum, 2);
        let winning_sum = parse_input_v2("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(winning_sum, 2);
        let winning_sum = parse_input_v2("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(winning_sum, 1);
        let winning_sum = parse_input_v2("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(winning_sum, 0);
    }
}
