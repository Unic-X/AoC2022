use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate regex;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn remove_first<T>(vec: &mut Vec<T>) -> Option<T> {
    if vec.is_empty() {
        return None;
    }
    Some(vec.remove(0))
}

fn main(){
    //let mut vec: Vec<Vec<char>> = Vec::with_capacity(9 * 8);

    let mut vec = vec![Vec::with_capacity(8);9];
    //let mut r_vec = vec![vec![char]];
    if let Ok(lines) = read_lines("./Inp_5.txt") {
        let mut l_i = 0;
        let mut t_m = 0;                                //no of items to move
        let mut c_n = 0;                                //which index to move
        let mut t_n = 0;                                // to where index
        for line in lines {
            let mut lis_n = 0;                          //set index of the vector to update
            if let Ok(ip) = line {
                if l_i<8{                               //first part to parse the given matrix
                    for i in (1..ip.len()).step_by(4){
                        let emp = ip.chars().nth(i).unwrap().is_whitespace();
                        if emp == false{
                            vec[lis_n].push(ip.chars().nth(i).unwrap());
                        }
                        lis_n+=1;
                    };
                }else if l_i>9{
                   let re = Regex::new(r"move ([\s \S]*) from ([\s \S]*) to ([\s \S]*)").unwrap();
                   let caps = re.captures(&ip).unwrap();
                   t_m = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                   c_n = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
                   t_n = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
                   for i in (1..t_m+1).rev(){
                    let item = vec[(c_n-1) as usize][(i-1) as usize];
                    println!("{}",item);
                    vec[(c_n-1) as usize].remove((i-1) as usize);
                    vec[(t_n-1) as usize].insert(0,item);
                    
                   }
                   //let item = vec[c_n]
            }
            l_i+=1;
        }
    }
    vec.iter().for_each(|it| {
        println!("{:?}", it);})
}
}


