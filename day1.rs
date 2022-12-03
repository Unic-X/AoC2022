use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn main() {
//    let mut len = 252;

    let mut arr : [i32; 252] = [0;252];
    let mut index = 0;
    let mut sum = 0;
    if let Ok(lines) = read_lines("./Inp_1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let blank = ip.trim().is_empty();
                if blank == false{
                    let my_int = ip.parse::<i32>().unwrap();
                    sum+=my_int;
                }else{
                    arr[index]=sum;
                    index+=1;
                    sum=0;
                }
            }
        }
        let mut max = 0;
        for i in 0..252 {
            max = cmp::max(max, arr[i]);
        }
        let mut vector = arr.to_vec();
        sort(&mut vector);
        println!("{:?}",vector);
    }
}

fn sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
      for j in 0..array.len() - i - 1 {
        if array[j + 1] < array[j] {
          array.swap(j, j + 1);
        }
      }
    }
  }

// The output is wrapped in a Result to allow matching on errors
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}