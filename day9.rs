mod read;


fn move_ud(command:String,steps:u32,L:&mut [i32],H:&mut [i32]){
    let mut mut_h = H;
    let mut mut_t = L;
    if command == "R"{
        if (mut_h[0]-mut_t[0]).abs()==0{
            for i in 0..steps{
                mut_h[1]+=1;
                if (&mut_t[1]-&mut_h[1]).abs()!=0 ||  (&mut_t[1]-&mut_h[1]).abs()!=1{
                    mut_t[1]+=1;
                }
            }
        }else{
            if (mut_h[1]-mut_t[1]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
            }else{
                move_rl(command,steps,mut_t,mut_h);
            }
        }
    }else if command == "L"{
        if (mut_h[0]-mut_t[0]).abs()==0{
            for i in 0..steps{
                mut_h[1]-=1;
                if (&mut_t[1]-&mut_h[1]).abs()!=0 ||  (&mut_t[1]-&mut_h[1]).abs()!=1{
                    mut_t[1]-=1;
                }
            }
        }else{
            if (mut_h[1]-mut_t[1]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
            }else{
                move_rl(command,steps,mut_t,mut_h);
            }
        }
    }
}

fn move_rl(command:String,steps:u32,L:&mut [i32],H:&mut [i32]){
    let mut mut_h =  H;
    let mut mut_t =  L;
    if command == "U"{
        if (mut_h[1]-mut_t[1]).abs()==0{
            for i in 0..steps{
                mut_h[0]+=1;
                if (&mut_t[0]-&mut_h[0]).abs()!=0 ||  (&mut_t[0]-&mut_h[0]).abs()!=1{
                    mut_t[0]+=1;
                }
            }
        }else{
            if (mut_h[1]-mut_t[1]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
            }else{
                move_ud(command,steps,mut_t,mut_h);
            }
        }
    }else if command == "D"{
        if (mut_h[1]-mut_t[1]).abs()==0{
            for i in 0..steps{
                mut_h[0]-=1;
                if (&mut_t[0]-&mut_h[0]).abs()!=0 ||  (&mut_t[0]-&mut_h[0]).abs()!=1{
                    mut_t[0]-=1;
                }
            }
        }else{
            if (mut_h[1]-mut_t[1]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
            }else{
                move_ud(command,steps,mut_t,mut_h);
            }
        }
    }
}

fn main(){
    let mut L:[i32; 2] = [0,0];
    let mut H:[i32; 2] = [0,0];
    let mut spot : Vec<(i32,i32)>= Vec::new();
    if let Ok(lines) = read::read_lines("./Inp_9.txt"){
        for line in lines{
            println!("{:?}",&L);
            if let Ok(ip) = line{
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                let command = String::from(vec[0]);
                let steps = vec[1].parse::<u32>().unwrap();
                if command == "R" || command =="L"{
                    move_rl(command,steps,&mut L,&mut H);
                    println!("R");
                }else{
                    move_ud(command,steps,&mut L,&mut H);
                    println!("U");
                }
            }
        }
    }
}
