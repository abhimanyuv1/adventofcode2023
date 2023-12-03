use std::collections::BTreeMap;
use std::fs::read_to_string;

fn get_file_as_vec(filename: &str) -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = vec![];
    for line in read_to_string(filename).unwrap().lines() {
        let buf = line.chars().collect();
        data.push(buf);
    }

    data
}

fn parse_input(input: Vec<Vec<char>>) -> u32 {
    let row = input.len();
    let col = input[0].len();
    let mut parts = Vec::new();

    for i in 0..row {
        let mut num: Vec<_> = Vec::new();
        let mut adjacent_to_symbol = false;
        for j in 0..col {
            if input[i][j].is_digit(10) {
                num.push(input[i][j]);
            }

            if !input[i][j].is_digit(10) {
                let d: String = num.iter().collect();
                if adjacent_to_symbol {
                    if !d.is_empty() {
                        let num = d.parse::<u32>().unwrap();
                        parts.push(num);
                    }
                }
                adjacent_to_symbol = false;
                num.clear();
                continue
            }

            // (0,-1)
            if j.saturating_sub(1) > 0 && !input[i][j-1].is_digit(10) && input[i][j-1] != '.' {
                adjacent_to_symbol = true;
            }
            // (0,1)
            if (j+1) < col && !input[i][j+1].is_digit(10) && input[i][j+1] != '.' {
                adjacent_to_symbol = true;
            }
            // (-1,0)
            if i.saturating_sub(1) > 0 && !input[i-1][j].is_digit(10) && input[i-1][j] != '.' {
                adjacent_to_symbol = true;
            }
            // (1,0)
            if (i+1) < row && !input[i+1][j].is_digit(10) && input[i+1][j] != '.' {
                adjacent_to_symbol = true;
            }
            // (-1,-1)
            if i.saturating_sub(1) > 0 && j.saturating_sub(1) > 0 && !input[i-1][j-1].is_digit(10) && input[i-1][j-1] != '.' {
                adjacent_to_symbol = true;
            }
            // (1,1)
            if (i+1) < row && (j+1) < col && !input[i+1][j+1].is_digit(10) && input[i+1][j+1] != '.' {
                adjacent_to_symbol = true;
            }
            // (-1,1)
            if i.saturating_sub(1) > 0 && (j+1) < col && !input[i-1][j+1].is_digit(10) && input[i-1][j+1] != '.' {
                adjacent_to_symbol = true;
            }
            // (1,-1)
            if j.saturating_sub(1) > 0 && (i+1) < row && !input[i+1][j-1].is_digit(10) && input[i+1][j-1] != '.' {
                adjacent_to_symbol = true;
            }
        }
        let d: String = num.iter().collect();
        if adjacent_to_symbol {
            if !d.is_empty() {
                let num = d.parse::<u32>().unwrap();
                parts.push(num);
            }
        }
    }

    parts.iter().sum::<u32>()
}

fn parse_input_v2(input: Vec<Vec<char>>) -> u32 {
    let row = input.len();
    let col = input[0].len();
    let mut parts = Vec::new();
    let mut gears = BTreeMap::new();

    for i in 0..row {
        let mut num: Vec<_> = Vec::new();
        let mut adjacent_to_star = false;
        let mut key: String = String::new();

        for j in 0..col {
            if input[i][j].is_digit(10) {
                num.push(input[i][j]);
            }

            if !input[i][j].is_digit(10) {
                let d: String = num.iter().collect();
                if adjacent_to_star {
                    if !d.is_empty() && !key.is_empty() {
                        //println!("key: {}", key);
                        let num = d.parse::<u32>().unwrap();
                        if gears.contains_key(key.as_str()) {
                            let mul = gears.get(key.as_str()).unwrap() * num;
                            parts.push(mul);
                        } else {
                            gears.insert(key.clone(), num);
                        }
                    }
                }
                adjacent_to_star = false;
                num.clear();
                key.clear();
                continue
            }

            // (0,-1)
            if j.saturating_sub(1) > 0 && !input[i][j-1].is_digit(10) && input[i][j-1] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i, j - 1);
            }
            // (0,1)
            if (j+1) < col && !input[i][j+1].is_digit(10) && input[i][j+1] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i, j + 1);
            }
            // (-1,0)
            if i.saturating_sub(1) > 0 && !input[i-1][j].is_digit(10) && input[i-1][j] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i-1, j);
            }
            // (1,0)
            if (i+1) < row && !input[i+1][j].is_digit(10) && input[i+1][j] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i+1, j);
            }
            // (-1,-1)
            if i.saturating_sub(1) > 0 && j.saturating_sub(1) > 0 && !input[i-1][j-1].is_digit(10) && input[i-1][j-1] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i-1, j-1);
            }
            // (1,1)
            if (i+1) < row && (j+1) < col && !input[i+1][j+1].is_digit(10) && input[i+1][j+1] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i+1, j+1);
            }
            // (-1,1)
            if i.saturating_sub(1) > 0 && (j+1) < col && !input[i-1][j+1].is_digit(10) && input[i-1][j+1] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i-1, j+1);
            }
            // (1,-1)
            if j.saturating_sub(1) > 0 && (i+1) < row && !input[i+1][j-1].is_digit(10) && input[i+1][j-1] == '*' {
                adjacent_to_star = true;
                key = format!("{}+{}", i+1, j-1);
            }
        }
        let d: String = num.iter().collect();
        if adjacent_to_star {
            if !d.is_empty() {
                let num = d.parse::<u32>().unwrap();
                if gears.contains_key(key.as_str()) {
                    let mul = gears.get(key.as_str()).unwrap() * num;
                    parts.push(mul);
                } else {
                    gears.insert(key.clone(), num);
                }
            }
        }
    }

    parts.iter().sum::<u32>()
}

fn main() {
    let data = get_file_as_vec("day3/input");
    let parts_sum = parse_input(data.clone());
    println!("Parts sum: {}", parts_sum);

    let gear_sum = parse_input_v2(data);
    println!("Gear sum: {}", gear_sum);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_input() {
        let data = get_file_as_vec("/home/abhi/RustroverProjects/adventofcode/day3/test");
        let parts = parse_input(data);
        assert_eq!(parts, 4361);
    }

    #[test]
    fn test_parse_input_v2() {
        let data = get_file_as_vec("/home/abhi/RustroverProjects/adventofcode/day3/test");
        let gear_sum = parse_input_v2(data);
        assert_eq!(gear_sum, 467835);
    }
}
