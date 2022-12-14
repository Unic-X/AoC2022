extern crate regex;
use regex::Regex;
mod read;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
struct TreeNode {
  size: Option<u32>,
  name:String,
  children: Vec<Rc<RefCell<TreeNode>>>,
  parent: Option<Rc<RefCell<TreeNode>>>,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Commands{
    Dir,
    Cd,
    Ls,
    File,
}

fn command(ip:String)-> Commands/*return tuple returns command and file size and dir name*/{
    let mut comm = Commands::File;
    if ip.starts_with("$ "){
        let re = Regex::new(r"\$ ([\s \S]*)").unwrap();
        let caps = re.captures(&ip).unwrap();
        let command = caps.get(1).unwrap().as_str();
        if command.starts_with("ls"){
            comm = Commands::Ls;
        }else if command.starts_with("cd"){
            comm = Commands::Cd;
        }
    }
    if ip.starts_with("dir"){
        comm = Commands::Dir;
    }
    if ip.chars().next().unwrap().is_numeric(){
        comm = Commands::File;
    }
    return comm;
}
    


impl TreeNode {
    fn new() -> TreeNode {
        return TreeNode {
        size: Some(0),
        name:String::from(""),
        children: vec![],
        parent: None,
        };
    }
    fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }
}

fn init_tree(s: String) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&root);
    let child = Rc::new(RefCell::new(TreeNode::new()));
    return root;
}

fn main(){
    if let Ok(lines) = read::read_lines("./Inp_7.txt"){
        for line in lines{
            if let Ok(ip) = line{
                let mut command = command(ip);
                println!("{:?}",command==Commands::Cd);
            } 
        }
    }
}