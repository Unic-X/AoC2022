use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_max(arr:&Vec<Vec<u32>>){
    /*
    1st part ---->
    let mut visible_trees  = 392;
    for v in 1..(arr.len()-1){
        let mut p = 0;
        for j in  1..(arr.len()-1){
            let mut right = 1;
            //check right 
            for k in j+1..99{
                if arr[v][j]>arr[v][k]{
                    right*=1;
                }else{
                    right*=0;
                }
            }
            if right == 1{
                p+=1;
            }
            let mut left = 1;

            for b in 0..j{
                if arr[v][j]>arr[v][b]{
                    left *= 1;
                }else{
                    left*=0;
                }
            }
            if left == 1{
                p+=1
            }
            let mut top = 1;

            for z in 0..v{
                if arr[v][j]>arr[z][j]{
                    top *= 1;
                }else{
                    top*=0;
                }
            }
            if top == 1{
                p+=1
            }
            let mut bottom = 1;
            for d in v+1..99{
                if arr[v][j] > arr[d][j]{
                    bottom *= 1;
                }else{
                    bottom*=0;
                }
            }
            if bottom ==1{
                p+=1
            }
            if p > 0{
                visible_trees = visible_trees + 1;
                p = 0;
            }
            }
        }
        println!("{}",visible_trees)
        
        2nd Part---->
        */
    
        let mut highest_score  = 0;
    for v in 1..(arr.len()-1){
        for j in  1..(arr.len()-1){
            let mut score = 1;
            let mut right = 0;
            //check right 
            for k in j+1..99{
                if arr[v][j]<=arr[v][k]{
                    right+=1;
                    break;
                }else{
                    right+=1;
                }
            }
            println!("Right :{}",right);
            let mut left = 0;

            for b in (0..j).rev(){
                if arr[v][j]<=arr[v][b]{
                    left += 1;
                    break;
                }else{
                    left+=1;
                }
            }
            println!("Left {}",left);

            let mut top = 0;

            for z in (0..v).rev(){
                if arr[v][j]<=arr[z][j]{
                    top += 1;
                    break;
                }else{
                    top+=1;
                }
            }
            println!("Top {}",top);

            let mut bottom = 0;
            for d in v+1..99{
                if arr[v][j] <= arr[d][j]{
                    bottom += 1;
                    break;
                }else{
                    bottom+=1;
                }
            }
            println!("Bottom {} Element:{}",bottom,arr[v][j]);

            score = bottom*top*left*right;
            if score>highest_score{
                highest_score = score;
            }
            }
        }
        println!("{}",highest_score)
    
    

}

fn main(){
    let mut h_arr = vec![Vec::with_capacity(99);99];
    let mut lis_m = 0;
    if let Ok(lines) = read_lines("./Inp_8.txt") {
        for line in lines {
            if let Ok(ip) = line{
                for i in 0..ip.len(){
                    h_arr[lis_m].push(ip.chars().nth(i).unwrap().to_digit(10).unwrap());
                }lis_m+=1;
            }
        } 
    }is_max(&h_arr);
}
