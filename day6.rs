use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn main(){
    if let Ok(lines) = read_lines("./Inp_6.txt") {
        for line in lines {
            if let Ok(ip) = line {
                for i in 0..ip.len(){
                    let part = &ip[i..i+14];
                    if unique(part) == None{
                        println!("{:?} {}",&ip[i..i+14],i+14);
                        break;
                    }
                }
            }
        }
    }
}