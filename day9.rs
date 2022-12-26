mod read;

#[derive(Debug)]
struct Ends{
    x : i32,
    y : i32,
}
fn move_head(command:&String,head:&mut Ends){
    let mut mut_head = head;
    match command.as_str(){
        "U"=> mut_head.y +=1,
        "D"=> mut_head.y -=1,
        "L"=> mut_head.x -=1,
        "R"=> mut_head.x +=1,
        _ => {}
    }
}

fn follow(head:&Ends,tail:&mut Ends,location:&mut Vec<(i32,i32)>){
    while (tail.x-head.x).abs()>1 || (tail.y-head.y).abs()>1{
        let dx = head.x-tail.x;
        let dy = head.y-tail.y;
        tail.x += dx.signum();
        tail.y += dy.signum();
        if location.contains(&(tail.x,tail.y)){

        }else{
            location.push((tail.x,tail.y));
        }
    }
}

fn main(){
    let mut h = Ends {x:0,y:0};
    let mut l = Ends {x:0,y:0};
    let mut spot : Vec<(i32,i32)>= Vec::new();
    if let Ok(lines) = read::read_lines("./Inp_9.txt"){
        for line in lines{
            println!("Head : {:?}",&h);
            println!("Tail : {:?}\n",&l);
            if let Ok(ip) = line{
                let split = ip.split(" ");
                let vec: Vec<&str> = split.collect();
                let command = String::from(vec[0]);
                let steps = vec[1].parse::<u32>().unwrap();
                for i in 0..steps{
                    move_head(&command,&mut h);
                    follow(&mut h,&mut l,&mut spot);
                }
            }
        }
    }
    println!("{:?}",spot.len()+1);
}
