use std::cmp::min;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Mappings {
    source_start: u64,
    source_end: u64,
    dest_start: u64,
    dest_end: u64,
}

fn parse_input(filename: &str) -> u64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fettilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temp = Vec::new();
    let mut temp_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    loop {
        let input = lines.next();
        if input.is_none() {
            break
        }
        let line = input.unwrap().unwrap();

        let mut data = line.split(':');
        let map_name = data.next().unwrap().trim();

        println!("{}", map_name);

        if map_name == "seeds" {
            let _: Vec<_> = data.last().unwrap()
                .split(' ')
                .filter(|x| *x != "")
                .map(|x| seeds.push(x.parse::<u64>().unwrap())).collect();
            println!("{:?}", seeds);
        }

        if map_name == "seed-to-soil map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);

                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                seed_to_soil.push(m)
            }
            //println!("{:?}", seed_to_soil);
        }

        if map_name == "soil-to-fertilizer map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                soil_to_fettilizer.push(m);
            }
            //println!("{:?}", soil_to_fettilizer);
        }

        if map_name == "fertilizer-to-water map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                fertilizer_to_water.push(m);
            }
            //println!("{:?}", fertilizer_to_water);
        }

        if map_name == "water-to-light map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                water_to_light.push(m);
            }
            //println!("{:?}", water_to_light);
        }

        if map_name == "light-to-temperature map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                light_to_temp.push(m);
            }
            //println!("{:?}", light_to_temp);
        }

        if map_name == "temperature-to-humidity map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                temp_to_humidity.push(m);
            }
            //println!("{:?}", temp_to_humidity);
        }

        if map_name == "humidity-to-location map" {
            loop {
                let input = lines.next();
                if input.is_none() {
                    break;
                }
                let line = input.unwrap().unwrap();
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                humidity_to_location.push(m);
            }
            //println!("{:?}", humidity_to_location);
        }
    }


    let mut min_location = u64::MAX;
    for s in seeds {
        let mut soil = s;
        for m in &seed_to_soil {
            if s >= m.source_start && s <= m.source_end {
                let diff = s - m.source_start;
                soil = m.dest_start + diff;
            }
        }
        //println!("soil: {}", soil);

        let mut fertilizer = soil;
        for m in &soil_to_fettilizer {
            if soil >= m.source_start && soil <= m.source_end {
                let diff = soil - m.source_start;
                fertilizer = m.dest_start + diff;
            }
        }
        //println!("fertilizer: {}", fertilizer);

        let mut water = fertilizer;
        for m in &fertilizer_to_water {
            if fertilizer >= m.source_start && fertilizer <= m.source_end {
                let diff = fertilizer - m.source_start;
                water = m.dest_start + diff;
            }
        }
        //println!("water: {}", water);

        let mut light = water;
        for m in &water_to_light {
            if water >= m.source_start && water <= m.source_end {
                let diff = water - m.source_start;
                light = m.dest_start + diff;
            }
        }
        //println!("light: {}", light);

        let mut temperature = light;
        for m in &light_to_temp {
            if light >= m.source_start && light <= m.source_end {
                let diff = light - m.source_start;
                temperature = m.dest_start + diff;
            }
        }
        //println!("temperature: {}", temperature);

        let mut humidity = temperature;
        for m in &temp_to_humidity {
            if temperature >= m.source_start && temperature <= m.source_end {
                let diff = temperature - m.source_start;
                humidity = m.dest_start + diff;
            }
        }
        //println!("humidity: {}", humidity);

        let mut location = humidity;
        for m in &humidity_to_location {
            if humidity >= m.source_start && humidity <= m.source_end {
                let diff = humidity - m.source_start;
                location = m.dest_start + diff;
            }
        }
        //println!("location: {}", location);

        min_location = min(min_location, location);
    }

    min_location
}

/*
fn parse_input_v2(filename: &str) -> u64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fettilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temp = Vec::new();
    let mut temp_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    loop {
        let input = lines.next();
        if input.is_none() {
            break
        }
        let line = input.unwrap().unwrap();

        let mut data = line.split(':');
        let map_name = data.next().unwrap().trim();

        println!("{}", map_name);

        if map_name == "seeds" {
            let _: Vec<_> = data.last().unwrap()
                .split(' ')
                .filter(|x| *x != "")
                .map(|x| seeds.push(x.parse::<u64>().unwrap())).collect();
            println!("{:?}", seeds);
        }

        if map_name == "seed-to-soil map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);

                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                seed_to_soil.push(m)
            }
            //println!("{:?}", seed_to_soil);
        }

        if map_name == "soil-to-fertilizer map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                soil_to_fettilizer.push(m);
            }
            //println!("{:?}", soil_to_fettilizer);
        }

        if map_name == "fertilizer-to-water map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                fertilizer_to_water.push(m);
            }
            //println!("{:?}", fertilizer_to_water);
        }

        if map_name == "water-to-light map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                water_to_light.push(m);
            }
            //println!("{:?}", water_to_light);
        }

        if map_name == "light-to-temperature map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                light_to_temp.push(m);
            }
            //println!("{:?}", light_to_temp);
        }

        if map_name == "temperature-to-humidity map" {
            loop {
                let line = lines.next().unwrap().unwrap();
                if line.is_empty() {
                    break
                }
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                temp_to_humidity.push(m);
            }
            //println!("{:?}", temp_to_humidity);
        }

        if map_name == "humidity-to-location map" {
            loop {
                let input = lines.next();
                if input.is_none() {
                    break;
                }
                let line = input.unwrap().unwrap();
                let data: Vec<_> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                let (dest, source, range) = (data[0], data[1], data[2]);
                let m = Mappings {
                    source_start: source,
                    source_end: source + range,
                    dest_start: dest,
                    dest_end: dest + range,
                };
                humidity_to_location.push(m);
            }
            //println!("{:?}", humidity_to_location);
        }
    }


    let mut min_location = u64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        println!("{}", i);
        for s in (seeds[i]..seeds[i + 1]) {
            println!("{}", s);
            let mut soil = s;
            for m in &seed_to_soil {
                if s >= m.source_start && s <= m.source_end {
                    let diff = s - m.source_start;
                    soil = m.dest_start + diff;
                }
            }
            //println!("soil: {}", soil);

            let mut fertilizer = soil;
            for m in &soil_to_fettilizer {
                if soil >= m.source_start && soil <= m.source_end {
                    let diff = soil - m.source_start;
                    fertilizer = m.dest_start + diff;
                }
            }
            //println!("fertilizer: {}", fertilizer);

            let mut water = fertilizer;
            for m in &fertilizer_to_water {
                if fertilizer >= m.source_start && fertilizer <= m.source_end {
                    let diff = fertilizer - m.source_start;
                    water = m.dest_start + diff;
                }
            }
            //println!("water: {}", water);

            let mut light = water;
            for m in &water_to_light {
                if water >= m.source_start && water <= m.source_end {
                    let diff = water - m.source_start;
                    light = m.dest_start + diff;
                }
            }
            //println!("light: {}", light);

            let mut temperature = light;
            for m in &light_to_temp {
                if light >= m.source_start && light <= m.source_end {
                    let diff = light - m.source_start;
                    temperature = m.dest_start + diff;
                }
            }
            //println!("temperature: {}", temperature);

            let mut humidity = temperature;
            for m in &temp_to_humidity {
                if temperature >= m.source_start && temperature <= m.source_end {
                    let diff = temperature - m.source_start;
                    humidity = m.dest_start + diff;
                }
            }
            //println!("humidity: {}", humidity);

            let mut location = humidity;
            for m in &humidity_to_location {
                if humidity >= m.source_start && humidity <= m.source_end {
                    let diff = humidity - m.source_start;
                    location = m.dest_start + diff;
                }
            }
            //println!("location: {}", location);

            min_location = min(min_location, location);
        }
    }

    min_location
}
*/

fn main() {
    //let min_location = parse_input("day5/test");
    //println!("Min Location: {}", min_location);

    let min_location = parse_input("day5/input");
    println!("Min Location: {}", min_location);

    //let min_location = parse_input_v2("day5/input");
    //println!("Min Location: {}", min_location);
}
