use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main(){
    let mut vec = vec![vec!['#'; 8]; 9];
    if let Ok(lines) = read_lines("./Inp_5.txt") {
        let mut l_i = 0;
        for line in lines {
            let mut lis_n = 0;                          //set index of the vector to update
            if let Ok(ip) = line {
                if l_i<8{                               //first part to parse the given matrix
                    for i in (1..ip.len()).step_by(4){
                        let emp = ip.chars().nth(i).unwrap().is_whitespace();
                        if emp == false{
                            println!("{}{}{}",ip.chars().nth(i).unwrap(),lis_n,l_i);
                            vec[lis_n][l_i] = ip.chars().nth(i).unwrap();
                        }
                        lis_n+=1
                    };
                }else if l_i>9{
                    
                   // r" move (?<after>\w+)";
                   // r" (?<before>\w+) to (?<after>\w+)" ;           //regex capture to int
                }
            }
            l_i+=1;
        }
    }
    
    vec.iter().for_each(|it| {
        println!("{:?}", it);})
}

