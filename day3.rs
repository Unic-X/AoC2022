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
    let mut index =1; 
    //let mut arr = vec!["GGVGlqWFgVfFqqVZGFlblJPMsDbbMrDMpDsJRn", "LwzHtwdLHHwDrzPZzzsJbJ", "wdLTBvSvHvZVGCjhfN"];
    let mut arr: [String; 3] =["GGVGlqWFgVfFqqVZGFlblJPMsDbbMrDMpDsJRn".to_string(),"LwzHtwdLHHwDrzPZzzsJbJ".to_string(),"wdLTBvSvHvZVGCjhfN".to_string()];
    if let Ok(lines) = read_lines("./Inp_3.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let blank = ip.trim().is_empty();
                if blank == true{
                    for c in arr[0].chars(){
                        if arr[1].contains(c) && arr[2].contains(c){
                            println!("{:?} {} {}",arr,c,c as u32);
                            if c.is_uppercase(){
                                let n = c as u32;
                                value+=n-38;
                                break;
                                }else{
                                let n = c as u32;
                                value+=n-96;
                                break;
                            }
                        }
                    }
                }else{
                    if index%3==1{
                        arr[0]=ip;
                        index+=1;
                    }else if index%3==2{
                        arr[1]=ip;
                        index+=1;
                    }else{
                        arr[2]=ip;
                        index=1;
                    }

                }
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
    println!("{}",value+33);
}
