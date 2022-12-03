use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./Inp_2.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                if vec[0]=="A" /*rock*/{
                    if vec[1]=="X"{println!("scissor");sum+=3;} 
                    if vec[1]=="Y"{println!{"rock"};sum+=4;}
                    if vec[1]=="Z"{println!("win");sum+=8;}
                }else if vec[0]=="B" /*paper*/{
                    if vec[1]=="X"{println!("rock");sum+=1;}
                    if vec[1]=="Y"{println!("Draw");sum+=5;}
                    if vec[1]=="Z"{println!{"WIN"};sum+=9;}
                }else if vec[0]=="C" /*Scissors*/{
                    if vec[1]=="X"{println!{"paper"};sum+=2;}
                    if vec[1]=="Y"{println!("scissor");sum+=6;}
                    if vec[1]=="Z"{println!("rock");sum+=7}
            }
        }
    }
}
    println!("{} ",sum)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

