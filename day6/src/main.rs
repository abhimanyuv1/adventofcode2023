fn main() {
    // Part 1
    let boat_timing = Vec::from([41, 96, 88, 94]);
    let boat_distance = Vec::from([214, 1789, 1127, 1055]);

    let mut num_of_way = 1;
    for i in 0..boat_timing.len() {
        let time = boat_timing[i];
        let distance = boat_distance[i];

        let mut count = 0;
        for t in 1..time+1 {
            let distance_travelled = (time-t) * t;
            if distance_travelled > distance {
                count += 1;
            }
        }
        num_of_way *= count;
    }

    println!("{}", num_of_way);

    // Part 2
    let time: u128 = 41968894;
    let distance: u128 = 214178911271055;

    let mut count: u128 = 0;
    for t in 14..time+1 {
        let distance_travelled = (time - t) * t;
        if distance_travelled > distance {
            count += 1;
        }
    }

    println!("{}", count);
}
