mod read;

fn t_l(vector:&mut Vec<(i32,i32)>,tuple:(i32,i32)){
    if vector.contains(&tuple){
        
    }else{
        vector.push(tuple);
    }
}
fn move_ud(command:String,steps:u32,L:&mut [i32],H:&mut [i32],V:&mut Vec<(i32,i32)>){
    let mut mut_h = H;
    let mut mut_t = L;
    let mut mut_vec = V;
    if command == "U"{
        if (mut_h[0]-mut_t[0]).abs()==0{
            for i in 0..steps{
                mut_h[1]+=1;
                if (&mut_t[1]-&mut_h[1]).abs()>1{
                    mut_t[1]+=1;
                    t_l(mut_vec,(mut_t[0],mut_t[1]));

                }
            }
        }else{
            if (mut_h[0]-mut_t[0]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
                t_l(mut_vec,(mut_t[0],mut_t[1]));
                move_ud(command,steps,mut_t,mut_h,mut_vec);
            }else if (mut_h[0]-mut_t[0]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==0{
                mut_h[1]+=1;
                move_ud(command,steps-1,mut_t,mut_h,mut_vec);
        
                }    
            }
    }else if command == "D"{
        if (mut_h[0]-mut_t[0]).abs()==0{
            for i in 0..steps{
                mut_h[1]-=1;
                if (&mut_t[1]-&mut_h[1]).abs()>1{
                    mut_t[1]-=1;
                    t_l(mut_vec,(mut_t[0],mut_t[1]));
                }
            }
        }else{
            if (mut_h[0]-mut_t[0]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
                t_l(mut_vec,(mut_t[0],mut_t[1]));
                move_ud(command,steps,mut_t,mut_h,mut_vec);
            }else if (mut_h[0]-mut_t[0]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==0{
                mut_h[1]-=1;
                move_ud(command,steps-1,mut_t,mut_h,mut_vec);
                }  
        }
    }
}

fn move_rl(command:String,steps:u32,L:&mut [i32],H:&mut [i32],V:&mut Vec<(i32,i32)>){
    let mut mut_h =  H;
    let mut mut_t =  L;
    let mut mut_vec = V;
    if command == "R"{
        if (mut_h[1]-mut_t[1]).abs()==0{
            for i in 0..steps{
                mut_h[0]+=1;
                if (&mut_t[0]-&mut_h[0]).abs()>1{
                    mut_t[0]+=1;
                    t_l(mut_vec,(mut_t[0],mut_t[1]));
                }
            }
        }else{
            if (mut_h[0]-mut_t[0]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
                t_l(mut_vec,(mut_t[0],mut_t[1]));
                move_rl(command,steps,mut_t,mut_h,mut_vec);
            }else if (mut_h[1]-mut_t[1]).abs()==1 && (mut_h[0]-mut_t[0]).abs()==0{
                mut_h[0]+=1;
                move_rl(command,steps-1,mut_t,mut_h,mut_vec);
                }  
        }
    }else if command == "L"{
        if (mut_h[1]-mut_t[1]).abs()==0{
            for i in 0..steps{
                mut_h[0]-=1;
                if (&mut_t[0]-&mut_h[0]).abs()>1{
                    mut_t[0]-=1;
                    t_l(mut_vec,(mut_t[0],mut_t[1]));
                }
            }
        }else{
            if (mut_h[0]-mut_t[0]).abs()==1 && (mut_h[1]-mut_t[1]).abs()==1{
                mut_t[1] = mut_h[1];
                mut_t[0] = mut_h[0];
                t_l(mut_vec,(mut_t[0],mut_t[1]));
                move_rl(command,steps,mut_t,mut_h,mut_vec);
            }else if (mut_h[1]-mut_t[1]).abs()==1 && (mut_h[0]-mut_t[0]).abs()==0{
                mut_h[0]-=1;
                move_rl(command,steps-1,mut_t,mut_h,mut_vec);
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
            println!("Head : {:?}",&H);
            println!("Tail : {:?}\n",&L);
            if let Ok(ip) = line{
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                let command = String::from(vec[0]);
                let steps = vec[1].parse::<u32>().unwrap();
                if command == "R" || command =="L"{
                    move_rl(command,steps,&mut L,&mut H,&mut spot);
                }else if command =="U" || command=="D"{
                    move_ud(command,steps,&mut L,&mut H,&mut spot);
                }
            }
        }
    }
    println!("{}",spot.len());
}
