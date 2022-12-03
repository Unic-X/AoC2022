use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut value=0;
    //let mut a: u32=0;
    if let Ok(lines) = read_lines("./Inp_3.txt") {
        for line in lines {
            let mut arr: Vec<String> = Vec::new();
            if let Ok(ip) = line {
            arr[value]=ip;
            value+=1;
            if value>=3{
                for c in arr[0].chars(){
                    if arr[1].contains(c) && arr[2].contains(c){
                        print("Yes");
                    }
                }
                arr[0]=
            }
            //let r1_size = ip.len()/2;
            //let r1 = &ip[..r1_size];
            //let r2 = &ip[r1_size..];
            //for c in r1.chars(){
            //    if r2.contains(c){
            //        println!("{}",c);
            //        if c.is_uppercase(){
            //            let n = c as u32;
            //            value+=n-38;
            //            break;
            //        }else{
            //            let n = c as u32;
            //            value+=n-96;
            //            break;
            //        }
            //    }
            //}
            }
    } 
    }
    println!("{}",value);
}