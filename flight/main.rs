use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn longest_path(
    flights: &HashMap<i16, Vec<i16>>,
    num_cities: i16,
) -> Result<Vec<i16>, &'static str> {
    // using algorithm found here: https://cs.stackexchange.com/q/22855
    // basicaly:
    // - do a BFS from any node remember the last node found (u)
    // - do a BFS from u remember the last node found (v)
    // - diam(G) = d(u, v)

    let mut current_node = 1;
    let mut current_path: Vec<i16> = Vec::new();
    let mut visited_cities: Vec<i16> = Vec::new();

    // first pass
    let mut next_visiting_first_pass: VecDeque<i16> = VecDeque::new(); // node_id
    next_visiting_first_pass.push_back(current_node);
    while next_visiting_first_pass.len() > 0 {
        current_node = next_visiting_first_pass.pop_front().unwrap();
        for n in flights[&current_node].iter() {
            if !visited_cities.contains(n) && !next_visiting_first_pass.contains(n) {
                next_visiting_first_pass.push_back(*n);
            }
        }
        visited_cities.push(current_node);
    }
    // second pass
    let mut next_visiting_second_pass: VecDeque<(i16, Vec<i16>)> = VecDeque::new(); // node_id, (path)
    let mut next_visiting_second_pass2: VecDeque<i16> = VecDeque::new(); // node_id
    let mut visited_cities: Vec<i16> = Vec::new();
    next_visiting_second_pass.push_back((current_node, Vec::new()));
    next_visiting_second_pass2.push_back(current_node);
    while next_visiting_second_pass.len() > 0 {
        let (current_node_, current_path_) = next_visiting_second_pass.pop_front().unwrap();
        current_node = current_node_;
        current_path = current_path_;
        // eprintln!("{:?}", current_path);
        for n in flights[&current_node].iter() {
            if !visited_cities.contains(n) && !next_visiting_second_pass2.contains(n) {
                let mut new_path = current_path.clone();
                new_path.push(current_node);
                next_visiting_second_pass.push_back((*n, new_path));
                next_visiting_second_pass2.push_back(*n);
            }
        }
        visited_cities.push(current_node);
    }
    if (visited_cities.len() as i16) < num_cities {
        // the graph is not connex, we assume the max value
        // TODO: move that somewhere else
        return Err("Graph not conex");
    }
    // current_path.push(current_node);
    // eprintln!("{}", visited_cities.len());
    // eprintln!("{:?}", current_path);
    // eprintln!("{:?}", current_path);
    Ok(current_path)
}

fn center_of_tree(
    flights: &HashMap<i16, Vec<i16>>,
    num_cities: i16,
    start: i16,
) -> Result<i16, &'static str> {
    // based on https://stackoverflow.com/a/5056755
    let mut visited: Vec<i16> = Vec::new();
    let mut to_visit: Vec<i16> = Vec::new();
    let mut prune_fifo: VecDeque<i16> = VecDeque::new();
    to_visit.push(start);
    while to_visit.len() > 0 {
        let current = to_visit.pop().unwrap();
        for n in flights[&current].iter() {
            if !visited.contains(n) && !to_visit.contains(n) {
                to_visit.push(*n);
            }
        }
        visited.push(current);

        // eprintln!("{:?}", flights[&current]);
        if flights[&current].len() <= 1 {
            prune_fifo.push_back(current);
        }
    }

    let mut connexity: HashMap<i16, i16> = flights.iter().map(|(k, v)| (*k, v.len() as i16)).collect();
    let mut pruned: Vec<i16> = Vec::new();

    // eprintln!("{:?}", prune_fifo);
    while prune_fifo.len() > 1 {
        let leaf = prune_fifo.pop_front().unwrap();
        for n in flights[&leaf].iter() {
            if !pruned.contains(n) {
                let new_con = connexity[n] - 1;
                connexity.insert(*n, new_con);
                if connexity[n] <= 1 {
                    prune_fifo.push_back(*n);
                }
            }
        }
        pruned.push(leaf);
    }

    match prune_fifo.pop_front() {
        Some(n) => Ok(n),
        None => Err("bug"),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut flights: HashMap<i16, Vec<i16>> = HashMap::new();
    let mut flights_vec: Vec<(i16, i16)> = Vec::new();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let num_cities: i16 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    for line in lines {
        let nums: Vec<i16> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        flights
            .entry(nums[0])
            .or_insert_with(Vec::new)
            .push(nums[1]);
        flights
            .entry(nums[1])
            .or_insert_with(Vec::new)
            .push(nums[0]);
        flights_vec.push((nums[0], nums[1]));
    }

    let mut min_diam = num_cities;
    let mut best_removed_flight: (i16, i16) = (0, 0);
    let mut best_added_flight: (i16, i16) = (0, 0);
    let longest = longest_path(&flights, num_cities).unwrap();
    for i in (0..(longest.len() - 1)){
        let removed_flight = &(longest[i], longest[i + 1]);
        let (former_start, former_end) = (&longest[i], &longest[i + 1]);
        // eprintln!("removed: {:?}", removed_flight);
        // edit the flight
        let former_end_id = flights[former_start]
            .iter()
            .position(|l| l == former_end)
            .unwrap();
        let former_start_id = flights[former_end]
            .iter()
            .position(|l| l == former_start)
            .unwrap();
        flights
            .entry(*former_start)
            .or_insert_with(Vec::new)
            .remove(former_end_id);
        flights
            .entry(*former_end)
            .or_insert_with(Vec::new)
            .remove(former_start_id);


        let left_cent = center_of_tree(&flights, num_cities, *former_start).unwrap();
        let right_cent = center_of_tree(&flights, num_cities, *former_end).unwrap();


        let added_flight: (i16, i16) = (left_cent, right_cent);
        let (start_city, end_city) = (left_cent, right_cent);
        flights
            .entry(start_city)
            .or_insert_with(Vec::new)
            .push(end_city);
        flights
            .entry(end_city)
            .or_insert_with(Vec::new)
            .push(start_city);

        let diam = match longest_path(&flights, num_cities) {
            Ok(val) => val.len() as i16,
            Err(_) => num_cities,
        };
        if diam < min_diam {
            best_removed_flight = *removed_flight;
            best_added_flight = added_flight;
            min_diam = diam;
        }
        //eprintln!("with: {:?}, diam: {}", added_flight, diam);

        flights.entry(start_city).or_insert_with(Vec::new).pop();
        flights.entry(end_city).or_insert_with(Vec::new).pop();

        // eprintln!("with: {:?}, diam: {}", added_flight, diam);

        // Fid centroid of right (end)

        // let mut visited_left: Vec<i16> = Vec::new();
        // let mut to_visit_left: Vec<i16> = Vec::new();
        // to_visit_left.push(*former_start);
        // while to_visit_left.len() > 0 {
        //     let current_left = to_visit_left.pop().unwrap();
        //     let mut visited_right: Vec<i16> = Vec::new();
        //     let mut to_visit_right: Vec<i16> = Vec::new();
        //     to_visit_right.push(*former_end);
        //     while to_visit_right.len() > 0 {
        //         let current_right = to_visit_right.pop().unwrap();
        //         // HERE we do the action
        //         let added_flight: (i16, i16) = (current_left, current_right);
        //         let (start_city, end_city) = (current_left, current_right);
        //         flights
        //             .entry(start_city)
        //             .or_insert_with(Vec::new)
        //             .push(end_city);
        //         flights
        //             .entry(end_city)
        //             .or_insert_with(Vec::new)
        //             .push(start_city);

        //         let diam = match longest_path(&flights, num_cities) {
        //             Ok(val) => val.len() as i16,
        //             Err(_) => num_cities,
        //         };
        //         if diam < min_diam {
        //             best_removed_flight = *removed_flight;
        //             best_added_flight = added_flight;
        //             min_diam = diam;
        //         }
        //         //eprintln!("with: {:?}, diam: {}", added_flight, diam);

        //         flights.entry(start_city).or_insert_with(Vec::new).pop();
        //         flights.entry(end_city).or_insert_with(Vec::new).pop();
        //         
        //         for n in flights[&current_right].iter() {
        //             if !visited_right.contains(n) && !to_visit_right.contains(n) {
        //                 to_visit_right.push(*n);
        //             }
        //         }
        //         visited_right.push(current_right);
        //     }
        //     for n in flights[&current_left].iter() {
        //         if !visited_left.contains(n) && !to_visit_left.contains(n) {
        //             to_visit_left.push(*n);
        //         }
        //     }
        //     visited_left.push(current_left);
        // }


        // for start_city in 1..num_cities {
        //     for end_city in (start_city + 1)..(num_cities + 1) {
        //         let added_flight: (i16, i16) = (start_city, end_city);
        //         match flights[&start_city].iter().any(|&x| x == end_city) {
        //             true => continue,
        //             false => {}
        //         }
        //         flights
        //             .entry(start_city)
        //             .or_insert_with(Vec::new)
        //             .push(end_city);
        //         flights
        //             .entry(end_city)
        //             .or_insert_with(Vec::new)
        //             .push(start_city);

        //         let diam = match longest_path(&flights, num_cities) {
        //             Ok(val) => val.len() as i16,
        //             Err(_) => num_cities,
        //         };
        //         if diam < min_diam {
        //             best_removed_flight = *removed_flight;
        //             best_added_flight = added_flight;
        //             min_diam = diam;
        //         }
        //         //eprintln!("with: {:?}, diam: {}", added_flight, diam);

        //         flights.entry(start_city).or_insert_with(Vec::new).pop();
        //         flights.entry(end_city).or_insert_with(Vec::new).pop();
        //     }
        // }

        // rebuilt the flight
        flights
            .entry(*former_start)
            .or_insert_with(Vec::new)
            .push(*former_end);
        flights
            .entry(*former_end)
            .or_insert_with(Vec::new)
            .push(*former_start);
    }
    println!("{}", min_diam);
    let (former_start, former_end) = best_removed_flight;
    let (new_start, new_end) = best_added_flight;
    println!("{} {}", former_start, former_end);
    println!("{} {}", new_start, new_end);
}
