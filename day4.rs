use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::cmp;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main(){
    if let Ok(lines) = read_lines("./Inp_4.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(ip) = line {
                    let mut e_0: [u32;2] = [0,0]; 
                    let mut e_1: [u32;2] = [0,0];
                    let p = ip.to_string();
                    let split = p.split(",");
                    let mut index = 0;
                    for i in split{
                        let x = i.to_string();
                        let vec: Vec<&str> = x.split("-").collect();
                        if index == 0{
                            e_0[0] = vec[0].parse::<u32>().unwrap();
                            e_0[1] = vec[1].parse::<u32>().unwrap();
                        }else{
                            e_1[0] = vec[0].parse::<u32>().unwrap();
                            e_1[1] = vec[1].parse::<u32>().unwrap();
                        }
                        
                        index = 1;
                    }
                    //(e_0[0]<=e_1[0] && e_1[1]<=e_0[1])||(e_1[0]<=e_0[0] && e_0[1]<=e_1[1])
                    if (e_0[1]<e_1[0])||(e_1[1]<e_0[0]) {
                        
                    }else{
                        println!("L1: {:?}    L2:{:?}",e_0,e_1);
                        count+=1;
                    }

                }
            

        }println!("{}",count);
    }
}