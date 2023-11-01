use std::{
    collections::{HashMap, HashSet, VecDeque, BTreeSet},
    fs,
};

#[derive(Debug)]
struct Valve<'a> {
    rate: u64,
    connecting: Vec<&'a str>,
}

fn parse(inp: &str) -> HashMap<&str, Valve> {
    let mut valves: HashMap<&str, Valve> = HashMap::new();

    let lines = inp.lines();

    for (_, line) in lines.enumerate() {
        let tokens = line.split(" ");
        let mut name: &str = "";
        let mut rate: u64 = 0;
        let mut connecting: Vec<&str> = Vec::new();
        for (index, token) in tokens.enumerate() {
            match index {
                1 => {
                    name = token;
                }
                4 => {
                    rate = token
                        .split("=")
                        .nth(1)
                        .unwrap()
                        .trim_end_matches(";")
                        .parse::<u64>()
                        .unwrap()
                }
                9.. => connecting.push(token.trim_end_matches(",")),

                _ => {}
            }
        }
        valves.insert(&name, Valve { rate, connecting });
    }
    valves
}

fn distance<'a>(start: &'a str, parsed: &HashMap<&'a str, Valve<'a>>) -> HashMap<&'a str, u64> {
    let mut not_visited: HashMap<&'a str, u64> =
        parsed.keys().map(|&value| (value, u64::MAX)).collect();
    not_visited.insert(start, 0);
    let mut visited: HashMap<&str, u64> = HashMap::new();
    let mut curr = start;
    visited.insert(curr, 0);
    while !not_visited.is_empty() {
        if let Some((next, _)) = not_visited.iter().min_by_key(|&(_, &dist)| dist) {
            curr = next;
        } //Get the smallest distance node

        let neighbours: HashSet<&str> =
            HashSet::from_iter(parsed.get(curr).unwrap().connecting.iter().copied()); //Create a
                                                                                      //hashset of neighbours from the connecting nodes of current
        for neighbour in not_visited //Visit those nodes that are not visited
            .keys()
            .cloned()
            .collect::<HashSet<&str>>()
            .intersection(&neighbours)
        {
            let alt = not_visited.get(curr).unwrap() + 1;

            if alt < *not_visited.get(neighbour).unwrap() {
                not_visited.insert(&neighbour, alt);
                visited.insert(&neighbour, *not_visited.get(neighbour).unwrap());
            }
        }
        not_visited.remove(curr); //Remove the current smallest node that is.
    }
    visited
}

struct State <'a>{
    curr : &'a str,
    opened : BTreeSet<& 'a str>,
    time_left : u64,
    pressure_released:u64,
}


fn wait_until_ending(
    time_left: u64,
    relieved: u64,
    opened: &BTreeSet<&str>,
    map: &HashMap<&str, Valve>)->u64{

    let relieved_per_min: u64 = opened.iter().map(|name| &map[name].rate).sum();
    relieved + (relieved_per_min * time_left)
}

fn part1_2(inp:&str){
    let mut q = VecDeque::new();
    
    let parsed = parse(inp);

    let mut distances:HashMap<(&str,&str),u64> = HashMap::new();

    for key in parsed.keys() {
        let map_from_key = distance(key, &parsed);
        for pair in &map_from_key{
            distances.insert((key,*pair.0),*pair.1 );
        
        }
    }
        
    dbg!(&distances);

    const START: &str = "AA";
    
    let mut seen = HashSet::new();

    let to_visit: HashSet<&str> = HashSet::from_iter(
        parsed
            .iter()
            .filter(|(_, valve)| valve.rate > 0)
            .map(|(name, _)| *name),
    );


    q.push_back(
        State{
            curr:START,
            opened : BTreeSet::new(),
            time_left:30,
            pressure_released: 0,
        }
    );
    
    seen.insert((BTreeSet::<&str>::new(),30,0));
    
    let mut max_relieved = 0;

    while let Some(
        State{
            curr,
            opened,
            time_left,
            pressure_released,
        }
    ) = q.pop_front() {
            if opened.len() == to_visit.len() || time_left<=0{
                //TODO
                let relieved_at_end = wait_until_ending( time_left, pressure_released, &opened, &parsed);
                max_relieved = max_relieved.max(relieved_at_end);
                continue;
            }

            let unopened = to_visit.iter().filter(|name| !opened.contains(*name));

            for dest in unopened{
                let cost = distances.get(&(curr, *dest)).unwrap() + 1;

                let new_time_left_p = time_left as i32 -cost as i32 ;

                if new_time_left_p<=0 {
                    let relieved_at_end = wait_until_ending( time_left, pressure_released, &opened, &parsed);
                    max_relieved = max_relieved.max(relieved_at_end);
                continue;
                }
                
                let new_time_left = time_left - cost; 
                
                let release_per_min:u64 = opened.iter().map(|&name|  parsed.get(name).unwrap().rate ).sum();
                let new_released = pressure_released+(release_per_min*cost);
                let mut new_opened = opened.clone();
                new_opened.insert(dest);
                
                if seen.insert((new_opened.clone(),new_time_left,new_released)) {
                    q.push_back(State {
                    opened: new_opened,
                    curr: dest,
                    time_left: new_time_left,
                    pressure_released : new_released,
                });
                } 

        }
        println!("{:?}",opened);
    }
    println!("Part 1 : {:?}",max_relieved);
    
}

/*fn part1(inp: &str) {
    let parsed = parse(inp);

    let mut pressure_released = 0;

    const START: &str = "AA";

    let mut time_left = 30;

    let mut to_visit: HashSet<&str> = HashSet::from_iter(
        parsed
            .iter()
            .filter(|(_, valve)| valve.rate > 0)
            .map(|(name, _)| *name),
    );


    let mut curr = START;
    let mut distances = distance(curr, &parsed);
    
    let mut pressure_per_min = 0;

    while time_left > 0 {
        if to_visit.is_empty() {
            pressure_released += pressure_per_min;
            time_left -= 1;
        } else {
            let next_node = distances
                .iter()
                .filter(|(&name, _)| to_visit.contains(name))
                .max_by(|(&name1, &dist1), (&name2, &dist2)| {
                    let a = (parsed.get(name1).unwrap().rate as f32) / (dist1 as f32);
                    let b = (parsed.get(name2).unwrap().rate as f32) / (dist2 as f32);
                    a.partial_cmp(&b).unwrap()
                })
                .unwrap()
                .0;
            let cost = distances.get(*next_node).unwrap()+1;
            // Get travelling cost + 1 (time to open valve)
            
            if (time_left as i32 - cost as i32)<0 {
                println!("hello ji");
                time_left-=1;
                continue;
            }

            time_left -= cost;
            // Time left - time of travelling + time to open valve

            let relived_till_opening = (pressure_per_min)*cost;
            // Already open valves ka pressure * cost

            //Open Valve

            curr = next_node;

            pressure_per_min += parsed.get(curr).unwrap().rate;
            distances = distance(next_node, &parsed);
            to_visit.remove(curr);
            pressure_released += relived_till_opening;
        }
       dbg!(pressure_per_min);
    }
    
    println!("sahi baat hai bhai {:?} {:?}", pressure_released,time_left);
}

FAILED SOLUN
*/


fn part2(inp:&str){
  let mut q = VecDeque::new();
    
    let parsed = parse(inp);

    let mut distances:HashMap<(&str,&str),u64> = HashMap::new();



    for key in parsed.keys() {
        let map_from_key = distance(key, &parsed);
        for pair in &map_from_key{
            distances.insert((key,*pair.0),*pair.1 );
        
        }
    }
        

    const START: &str = "AA";
    
    let mut seen = HashSet::new();

    let to_visit: HashSet<&str> = HashSet::from_iter(
        parsed
            .iter()
            .filter(|(_, valve)| valve.rate > 0)
            .map(|(name, _)| *name),
    );

    

    q.push_back(
        State{
            curr:START,
            opened : BTreeSet::new(),
            time_left:26,
            pressure_released: 0,
        }
    );
    
    seen.insert((BTreeSet::<&str>::new(),26,0));
    
    let mut max_relieved = 0;

    while let Some(
        State{
            curr,
            opened,
            time_left,
            pressure_released,
        }
    ) = q.pop_front() {
            if opened.len() == to_visit.len() || time_left<=0{
                //TODO
                let relieved_at_end = wait_until_ending( time_left, pressure_released, &opened, &parsed);
                max_relieved = max_relieved.max(relieved_at_end);
                continue;
            }

            let unopened = to_visit.iter().filter(|name| !opened.contains(*name));

            for dest in unopened{
                let cost = distances.get(&(curr, *dest)).unwrap() + 1;

                let new_time_left_p = time_left as i32 -cost as i32 ;

                if new_time_left_p<=0 {
                    let relieved_at_end = wait_until_ending( time_left, pressure_released, &opened, &parsed);
                    max_relieved = max_relieved.max(relieved_at_end);
                continue;
                }
                
                let new_time_left = time_left - cost; 
                
                let release_per_min:u64 = opened.iter().map(|&name|  parsed.get(name).unwrap().rate ).sum();
                let new_released = pressure_released+(release_per_min*cost);
                let mut new_opened = opened.clone();
                new_opened.insert(dest);
                
                if seen.insert((new_opened.clone(),new_time_left,new_released)) {
                    q.push_back(State {
                    opened: new_opened,
                    curr: dest,
                    time_left: new_time_left,
                    pressure_released : new_released,
                });
                } 

        }
    }
    println!("Part 2 : {:?}",max_relieved);

    

}

fn main() {
    let examp = fs::read_to_string("input/day_16.txt");
    match examp {
        Ok(ok) => {
            part1_2(&ok);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
