use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
enum FileType {
    Dir { size: u64, path: PathBuf },
}

fn part1(input: &str) -> Vec<u64> {
    let mut files: HashMap<PathBuf, FileType> = HashMap::new();
    let mut curr_dir: PathBuf = "/".into();

    let result = input.split("\n").map(|line| {
        let mut tokens = line.split(" ");
        let c1 = tokens.next(); // Possible $/dir/ls/u64 commands

        let c2 = tokens.next(); //Possible c2 is name of the file

        let c3: &str;
        if c1.unwrap().trim() == "$" {
            if c2.unwrap() == "cd" {
                c3 = tokens.next().unwrap();
                match c3 {
                    ".." => {
                        curr_dir.pop();
                    }
                    r"/" => {
                        curr_dir.push(r"~");
                        let new_dir: FileType = FileType::Dir {
                            size: 0,
                            path: curr_dir.clone(),
                        };

                        files.insert(curr_dir.clone(), new_dir);
                    }
                    _ => {
                        curr_dir.push(c3);
                        let new_dir: FileType = FileType::Dir {
                            size: 0,
                            path: curr_dir.clone(),
                        };

                        files.insert(curr_dir.clone(), new_dir);
                    }
                }
            }
        } else {
            let _ = match c1.unwrap().parse::<u64>() {
                Ok(ok) => {
                    let mut parent_dir = curr_dir.clone();
                    while let Some(dir) = files.get_mut(&mut parent_dir) {
                        match dir {
                            FileType::Dir { size, .. } => {
                                *size += ok;
                            }
                        }
                        parent_dir.pop();
                    }
                }
                Err(_) => {}
            };
        }
    });
    for _ in result {}
    curr_dir = "/".into();
    curr_dir.push("~");
    println!("{:?}", files.get(&curr_dir));
    let sizes: Vec<u64> = files
        .iter()
        .filter_map(|(_, file_type)| {
            if let FileType::Dir { size, .. } = file_type {
                Some(*size)
            } else {
                None
            }
        })
        .collect();
    sizes
}

fn part2(input: &str) {
    const TOTAL: u64 = 70000000;
    const UNUSED: u64 = 30000000;
    let p1 = part1(input);
    let more_req = UNUSED - 27441688;
    let mut unsorted: Vec<u64> = p1
        .into_iter()
        .filter(|&elem| elem >= more_req)
        .collect::<Vec<_>>();
    unsorted.sort();
    println!("{:?}", unsorted)
}

fn main() {
    let input = fs::read_to_string("./input.txt");
    match input {
        Ok(a) => {
            part2(&a);
            //let sizes = part1(&a);
            //let sa:u64 = sizes.into_iter().filter(|size| size<&100_000).into_iter().sum();
        }
        Err(e) => println!("aloo {}", e),
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn tree_test() {}
}
