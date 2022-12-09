
use std::{io::{self}, collections::{VecDeque, HashMap}};
mod utils;




// Note...
// https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust?noredirect=1&lq=1
#[allow(dead_code)]
fn parse_directories(v: Vec<String>)->HashMap<String, usize> {
    let mut line_cnt = 0;
    let mut pwd: String = String::from("");
    let mut paths: VecDeque<String> = VecDeque::new();
    let mut top_dir_sizes:HashMap<String, usize> = HashMap::new();


    for next_line in v {
        line_cnt += 1;
        println!("line {}: {}", line_cnt, next_line);
        let words:Vec<&str> = next_line.split_whitespace().collect();
        if words.len() == 0 { break; }
        if 0 == words[0].chars().count() { break; }
        if "$" == words[0]{                
            if "cd" == words[1]{
                println!("\tcd {}", words[2]);
                if ".." == words[2]{
                    let up_path = paths.pop_back().unwrap();
                    let up_len = up_path.len();                        
                    let new_pwd = &pwd.to_string()[0..(pwd.len()-up_len)];
                    pwd = new_pwd.to_string();
                    println!("\tCurrent path is {}", pwd );
                }                    
                else {
                    if words[2] == "/" {
                        // Went to root
                        paths.clear();
                        pwd = String::from("");
                    }
                    else {
                        // Went into a path
                        let mut new_path: String = String::from("/");
                        new_path.push_str(words[2].clone());                            
                        paths.push_back(new_path.clone());
                        pwd.push_str(&new_path.clone());
                    }
                    println!("\tCurrent path is {}", pwd );
                }
            }
            else {
                println!("\tTODO: command {}", words[1]);
            }
        } else {
            if "dir" == words[0]{
                println!("\tTODO: directory listing for {}", words[1]);
            }
            else {
                let filesize = words[0].parse::<i32>().unwrap() as usize;
                if 0 != filesize {
                    println!("\tfilesize {}",filesize);
                    // Check which top directory we are in, and push to there.
                    if paths.len() >= 1 {
                        for i in 0..paths.len() {
                            let parent_dir = paths[i].clone();
                            let curr_size_opt = top_dir_sizes.get(&parent_dir);
                            let curr_size = if curr_size_opt == None { 0 as usize } else { *curr_size_opt.unwrap() };
                            let entry = curr_size + filesize;
                            top_dir_sizes.insert(parent_dir, entry);
                        }
                    }
                    else {
                        // Skip this directory
                        println!("\tskipping file since it's at top level {}", words[1]);
                    }
                }
            }
        }
    }

    return top_dir_sizes;

}



#[cfg(test)]
mod tests {
    

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn crates_check_0() {
        
        let raw_string: String = "$ cd /
        $ ls
        dir apath
        14848514 b.txt
        8504156 c.dat
        dir dpath
        $ cd apath
        $ ls
        dir epath
        29116 f
        2557 g
        62596 h.lst
        $ cd epath
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd dpath
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k".to_string();
            
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());


        let top_dir_sizes = parse_directories(v);
        let mut sum_of_most_100000:usize = 0;
        let _dirs_at_most_100000:Vec<(String, usize)> = top_dir_sizes.clone().into_iter().filter(|(_n,s)|{
            let is_lt = *s <= 100000;
            if is_lt {
                sum_of_most_100000 += *s;
            }            
            return is_lt;
        }).collect();
        println!("\tSum of all directories lt 100000 {}", sum_of_most_100000);        
        assert_eq!(95437, sum_of_most_100000);

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
    let top_dir_sizes = parse_directories(v);
    let mut sum_of_most_100000:usize = 0;
    let _dirs_at_most_100000:Vec<(String, usize)> = top_dir_sizes.clone().into_iter().filter(|(_n,s)|{
        let is_lt = *s <= 100000;
        if is_lt {
            sum_of_most_100000 += *s;
        }            
        return is_lt;
    }).collect();
    println!("\tSum of all directories lt 100000 {}", sum_of_most_100000);        


    Ok(())
}