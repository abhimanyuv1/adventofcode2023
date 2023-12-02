use std::fs::read_to_string;

#[derive(Debug)]
#[derive(PartialEq)]
struct GameGroup {
    red: u32,
    blue: u32,
    green: u32,
}

fn parse_input(input: &str) -> Vec<GameGroup> {
    let mut grp_vec = Vec::new();
    let groups: Vec<_> = input.split(':').last().unwrap().split(';').collect();
    for g in groups.clone() {
        let s: Vec<_> = g.split(',').collect();
        let mut grp: GameGroup = GameGroup {
            red: 0,
            blue: 0,
            green: 0,
        };
        for i in s {
            let c: Vec<_> = i.trim().split(' ').collect();
            //println!("{}, {}", c[0], c[1]);
            if c[1] == "red" {
                grp.red = c[0].parse().unwrap();
            }
            if c[1] == "green" {
                grp.green = c[0].parse().unwrap();
            }
            if c[1] == "blue" {
                grp.blue = c[0].parse().unwrap();
            }
        }
        grp_vec.push(grp);
    }
    //println!("{:?}", grp_vec);
    grp_vec
}

fn is_game_possible(group: Vec<GameGroup>) -> bool {
    let input_cubes = GameGroup {
        red: 12, green: 13, blue: 14,
    };

    for g in group {
        if g.red > input_cubes.red || g.green > input_cubes.green || g.blue > input_cubes.blue {
            return false
        }
    }

    true
}

fn min_set_cube(group: Vec<GameGroup>) -> u32 {
    let mut min_set = GameGroup {
        red: 0, blue: 0, green: 0
    };

    for g in group {
        if g.red > min_set.red {
            min_set.red = g.red;
        }
        if g.green > min_set.green {
            min_set.green = g.green;
        }
        if g.blue > min_set.blue {
            min_set.blue = g.blue;
        }
    }

    min_set.red * min_set.green * min_set.blue
}

fn main() {
    let mut sum = 0;

    let mut lineno = 1;
    for line in read_to_string("day2/input").unwrap().lines() {
        let groups = parse_input(line.to_string().as_str());
        if is_game_possible(groups) {
                sum += lineno;
        }
        lineno += 1;
    }

    println!("Sum: {}", sum);

    let mut power: u32 = 0;
    for line in read_to_string("day2/input").unwrap().lines() {
        let groups = parse_input(line.to_string().as_str());
        let set_power = min_set_cube(groups);
        power += set_power;
    }

    println!("Power: {}", power);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), [GameGroup { red: 4, blue: 3, green: 0 }, GameGroup { red: 1, blue: 6, green: 2 }, GameGroup { red: 0, blue: 0, green: 2 }]);
        assert_eq!(parse_input("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), [GameGroup { red: 0, blue: 1, green: 2 }, GameGroup { red: 1, blue: 4, green: 3 }, GameGroup { red: 0, blue: 1, green: 1 }]);
        assert_eq!(parse_input("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), [GameGroup { red: 20, blue: 6, green: 8 }, GameGroup { red: 4, blue: 5, green: 13 }, GameGroup { red: 1, blue: 0, green: 5 }]);
        assert_eq!(parse_input("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), [GameGroup { red: 3, blue: 6, green: 1 }, GameGroup { red: 6, blue: 0, green: 3 }, GameGroup { red: 14, blue: 15, green: 3 }]);
        assert_eq!(parse_input("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), [GameGroup { red: 6, blue: 1, green: 3 }, GameGroup { red: 1, blue: 2, green: 2 }]);
    }

    #[test]
    fn test_is_game_possible() {
        let group: Vec<_> = Vec::from([GameGroup { red: 4, blue: 3, green: 0 }, GameGroup { red: 1, blue: 6, green: 2 }, GameGroup { red: 0, blue: 0, green: 2 }]);
        assert_eq!(is_game_possible(group), true);
        let group: Vec<_> = Vec::from([GameGroup { red: 0, blue: 1, green: 2 }, GameGroup { red: 1, blue: 4, green: 3 }, GameGroup { red: 0, blue: 1, green: 1 }]);
        assert_eq!(is_game_possible(group), true);
        let group: Vec<_> = Vec::from([GameGroup { red: 20, blue: 6, green: 8 }, GameGroup { red: 4, blue: 5, green: 13 }, GameGroup { red: 1, blue: 0, green: 5 }]);
        assert_eq!(is_game_possible(group), false);
        let group: Vec<_> = Vec::from([GameGroup { red: 3, blue: 6, green: 1 }, GameGroup { red: 6, blue: 0, green: 3 }, GameGroup { red: 14, blue: 15, green: 3 }]);
        assert_eq!(is_game_possible(group), false);
        let group: Vec<_> = Vec::from([GameGroup { red: 6, blue: 1, green: 3 }, GameGroup { red: 1, blue: 2, green: 2 }]);
        assert_eq!(is_game_possible(group), true);
    }

    #[test]
    fn test_min_set_cube() {
        let group: Vec<_> = Vec::from([GameGroup { red: 4, blue: 3, green: 0 }, GameGroup { red: 1, blue: 6, green: 2 }, GameGroup { red: 0, blue: 0, green: 2 }]);
        assert_eq!(min_set_cube(group), 48);
        let group: Vec<_> = Vec::from([GameGroup { red: 0, blue: 1, green: 2 }, GameGroup { red: 1, blue: 4, green: 3 }, GameGroup { red: 0, blue: 1, green: 1 }]);
        assert_eq!(min_set_cube(group), 12);
        let group: Vec<_> = Vec::from([GameGroup { red: 20, blue: 6, green: 8 }, GameGroup { red: 4, blue: 5, green: 13 }, GameGroup { red: 1, blue: 0, green: 5 }]);
        assert_eq!(min_set_cube(group), 1560);
        let group: Vec<_> = Vec::from([GameGroup { red: 3, blue: 6, green: 1 }, GameGroup { red: 6, blue: 0, green: 3 }, GameGroup { red: 14, blue: 15, green: 3 }]);
        assert_eq!(min_set_cube(group), 630);
        let group: Vec<_> = Vec::from([GameGroup { red: 6, blue: 1, green: 3 }, GameGroup { red: 1, blue: 2, green: 2 }]);
        assert_eq!(min_set_cube(group), 36);
    }
}
