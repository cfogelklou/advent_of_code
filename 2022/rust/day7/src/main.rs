
use std::{io::{self}, collections::{VecDeque, HashMap}, cmp, iter::Map};
use itertools::Itertools;
use std::rc::Rc;
use std::cell::RefCell;
mod utils;


/*
    We will use a key, value map (or hashmap) for each item.
    The value will be a struct with 
    typedef struct DirectoryNode 
    {
        name: String;
        size: int; // Zero if it is a directory
        children: Map<String, DirectoryNode>;
    }

*/

type Link = Option<Box<MyHashEntry>>;

// Note...
// https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust?noredirect=1&lq=1

#[derive(Clone)]
struct MyHashEntry {
    name: String,
    size: usize,
    parent: Link,
    kids: HashMap<String, Box<MyHashEntry>>,
}

impl MyHashEntry {
    pub fn new(name_in:&str) -> Self {
        MyHashEntry { name: name_in.to_string(), size: 0, parent: None, kids: HashMap::new() }
    }
    pub fn set_size(&mut self, sz:usize){
        self.size = sz;        
    }
    pub fn add_kid(&mut self, elem:MyHashEntry){
        self.kids.insert(elem.name.to_string(), Box::new(MyHashEntry {
            name:elem.name.to_string(),
            size: elem.size,
            parent: elem.parent,
            kids: elem.kids,
        }));
    }
    pub fn get_name(&self)->String{
        return self.name.to_string();
    }
    

}

#[derive(Debug)]
struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data: data, children: vec![] }
    }

    fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }
}

#[derive(Debug)]
struct NavigableNode<'a, T: 'a> {
    node: &'a Node<T>,
    parent: Option<&'a NavigableNode<'a, T>>,
}

impl<'a, T> NavigableNode<'a, T> {
    fn child(&self, index: usize) -> NavigableNode<T> {
        NavigableNode {
            node: &self.node.children[index],
            parent: Some(self)
        }
    }
}

impl<T> Node<T> {
    fn navigate<'a>(&'a self) -> NavigableNode<T> {
        NavigableNode { node: self, parent: None }
    }
}

#[cfg(test)]
mod tests {
    

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn crates_check_0() {
        
        let raw_string: String = "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k".to_string();
            
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());

        let mut line_cnt = 0;
        let mut pwd: String= "".to_string();
        let mut paths: VecDeque<String> = VecDeque::new();
        paths.push_back(pwd.to_string());

        let all_files_and_paths: Vec<(&str, usize)> = Vec::new();


        for next_line in v {
            line_cnt += 1;
            let words:Vec<&str> = next_line.split_whitespace().collect();
            if words.len() == 0 { break; }
            if 0 == words[0].chars().count() { break; }
            if "$" == words[0]{                
                if "cd" == words[1]{
                    println!("line {}: cd {}", line_cnt, words[2]);
                    if ".." == words[2]{
                        let up_path = paths.pop_back().unwrap();
                        let up_len = up_path.len();                        
                        let new_pwd = &pwd.to_string()[0..(pwd.len()-up_len)];
                        pwd = new_pwd.to_string();
                        println!("line {}: Current path is {}", line_cnt, pwd );
                    }                    
                    else {
                        if words[2] == "/" {
                            paths.clear();
                            pwd = String::from("/");
                        }
                        else {
                            let mut new_path: String = String::from("/");
                            new_path.push_str(words[2].clone());                            
                            paths.push_back(new_path.clone());
                            pwd.push_str(&new_path.clone());
                        }
                        println!("line {}: Current path is {}", line_cnt, pwd );
                    }
                }
                else {
                    println!("line {}: command {}", line_cnt, words[1]);
                }
            } else {
                if "dir" == words[0]{
                    println!("line {}: directory {}", line_cnt, words[1]);
                }
                else {
                    let filesize = words[0].parse::<i32>().unwrap();
                    if 0 != filesize {
                        println!("line {}: filesize {}", line_cnt, filesize);
                    }
                }
            }
        }

    }

    

}



fn main()  -> io::Result<()> {
    use std::io::BufRead;

    let filename = std::env::args().nth(1).expect("Expected filename");
    let file = std::io::BufReader::new(
        std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(
            &filename,
        ))
        .unwrap(),
    );
    let mut v:Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {    
        let l:String = line.unwrap();
        v.push(l);
    }        
    
 

    Ok(())
}