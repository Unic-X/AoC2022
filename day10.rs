mod read;
fn addx(cycle:&mut u32,cycsum:&mut i32,value:i32,x:&mut i32){
    for i in 0..2{
        *cycle+=1;
        if *cycle == 20{
            *cycsum += 20*(*x);
            println!("cycle 20 value of x {}",x);
        }else if *cycle == 60{
            *cycsum += 60*(*x);
            println!("cycle 60 value of x {}",x);
        }else if *cycle == 99{
            *cycsum += 100*(*x);
            println!("cycle 100 value of x {}",x);
        }else if *cycle == 140{
            *cycsum += 140*(*x);
            println!("cycle 140 value of x {}",x);
        }else if *cycle == 180{
            *cycsum += 180*(*x);
            println!("cycle 180 value of x {}",x);
        }else if *cycle == 220{
            *cycsum += 220*(*x);
            println!("cycle 220 value of x {}",x);

    }
    }*x+=value;
}
fn main(){
    let mut x = 1;
    let mut cycle=0;
    let mut cycsum = 0;
    if let Ok(lines) = read::read_lines("./Inp_10.txt"){
        for line in lines{
            if let Ok(ip) = line{
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                if vec[0] == "addx"{
                    addx(&mut cycle,&mut cycsum,vec[1].parse::<i32>().unwrap(),&mut x);
                }else if vec[0] == "noop"{
                    cycle+=1;
                }
                }
            
            }
        }
        println!("{} {}",x,cycsum)
}
