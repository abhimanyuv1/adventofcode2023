use std::collections::BTreeMap;
use std::fs::read_to_string;

pub fn parse_input(input: &str) -> u32 {
    let first = input.chars()
                     .into_iter()
                     .find(|&x| x.is_digit(10)).unwrap()
                     .to_digit(10).unwrap();
    let second = input.chars()
        .into_iter().rev()
        .find(|&x| x.is_digit(10)).unwrap()
        .to_digit(10).unwrap();

    first*10+second
}

pub fn parse_input_v2(input: &str) -> u32 {
    let num_val = vec!["1","2","3","4","5","6","7","8","9"];
    let num_str = vec!["one","two","three","four","five","six","seven","eight","nine"];
    let mut hash_value = BTreeMap::new();

    for num in &num_val {
        for (idx, matched) in input.match_indices(*num) {
            hash_value.insert(idx, matched);
        }
    }

    for num in &num_str {
        for (idx, matched) in input.match_indices(*num) {
            hash_value.insert(idx, matched);
        }
    }

    let first = hash_value.values().next().unwrap();
    let last = hash_value.values().last().unwrap();

    let first_value;
    let second_value;

    if num_val.contains(first) {
        first_value = num_val.iter().position(|x| x == first).unwrap() + 1;
    } else {
        first_value = num_str.iter().position(|x| x == first).unwrap() + 1;
    }

    if num_val.contains(last) {
        second_value = num_val.iter().position(|x| x == last).unwrap() + 1;
    } else {
        second_value = num_str.iter().position(|x| x == last).unwrap() + 1;
    }

    //println!("{}, {}, {:?}", first_value, second_value, hash_value);

    (first_value*10+second_value) as u32
}

fn main() {
    let mut sum: u64 = 0;
    for line in read_to_string("/home/abhi/RustroverProjects/adventofcode/day1/src/input").unwrap().lines() {
        let calibration = parse_input(line.to_string().as_str());
        println!("Cal: {}", calibration);
        sum += calibration as u64;
    }

    println!("Sum: {}", sum);

    sum = 0;
    for line in read_to_string("/home/abhi/RustroverProjects/adventofcode/day1/src/input").unwrap().lines() {
        println!("line: {}", line);
        let calibration = parse_input_v2(line.to_string().as_str());
        sum += calibration as u64;
    }

    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("q9"), 99);
        assert_eq!(parse_input("1abc2"), 12);
        assert_eq!(parse_input("pqr3stu8vwx"), 38);
        assert_eq!(parse_input("a1b2c3d4e5f"), 15);
        assert_eq!(parse_input("treb7uchet"), 77);
        assert_eq!(parse_input("seven5639"), 59);
    }

    #[test]
    fn test_parse_input_v2() {
        assert_eq!(parse_input_v2("qgrgqjlszpcnpq82"), 82);
        assert_eq!(parse_input_v2("abcone2threexyz"), 13);
        assert_eq!(parse_input_v2("abc4one2threexyz"), 43);
        assert_eq!(parse_input_v2("abc4one2three5xyz"), 45);
        assert_eq!(parse_input_v2("abcone2three5xyz"), 15);
        assert_eq!(parse_input_v2("two1nine"), 29);
        assert_eq!(parse_input_v2("eightwothree"), 83);
        assert_eq!(parse_input_v2("abcone2threexyz"), 13);
        assert_eq!(parse_input_v2("xtwone3four"), 24);
        assert_eq!(parse_input_v2("4nineeightseven2"), 42);
        assert_eq!(parse_input_v2("zoneight234"), 14);
        assert_eq!(parse_input_v2("7pqrstsixteen"), 76);
    }
}
