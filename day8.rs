use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_max(arr:&Vec<u32>,ind:usize,n:&mut u32){
    let vec1 = &arr[..ind];
    let vec2 = &arr[ind..];
    let max_value1 = vec1.iter().max();
    let max_value2 = vec2.iter().max();
    match max_value1 {
        Some(max) if *max == *vec1.last().unwrap() => {
                println!("{:?} has max as:{}",vec1,max);
                *n+=1;
            },
        Some(_) => {},
        None => {},
    }
    match max_value2{
        Some(max) if *max == vec2[0] =>{
            println!("{:?} has max as :{}",vec2,max);
            *n+=1;
        },
        Some(_) => {},
        None => {},
    }
}

fn main(){
    let mut v_arr = vec![Vec::with_capacity(5);5];
    let mut h_arr = vec![Vec::with_capacity(5);5];
    let mut lis_m = 0;
    let mut n = 0;
    if let Ok(lines) = read_lines("./Inp_8.txt") {
        for line in lines {
            let mut lis_n = 0;
            if let Ok(ip) = line{
                for i in 0..ip.len(){
                    v_arr[lis_n].push(ip.chars().nth(i).unwrap().to_digit(10).unwrap());
                    h_arr[lis_m].push(ip.chars().nth(i).unwrap().to_digit(10).unwrap());
                    lis_n+=1;
                }lis_m+=1;
            }
        }
        println!("Starting for vertical array\n");
        for v in &v_arr[1..4]{
            for i in 2..(v.len()-1){
                is_max(&v,i,&mut n);
                println!("{}",n)
            }
        };
        println!("Starting for horizontal array\n");
        for v in &h_arr[1..4]{
            for i in 2..(v.len()-1){
                is_max(&v,i,&mut n);
                println!("{}",n)
            }
        };

        v_arr.iter().for_each(|it| {
        println!("{:?}", it);})
    }
}
