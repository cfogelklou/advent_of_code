
use std::{io::{self}, collections::{VecDeque, HashMap}, cmp};
mod utils;


#[allow(dead_code)]
fn remove_parent_path_from_pwd(up_path: String, pwd: &String)->String {
    let up_len = cmp::min(pwd.len(), up_path.len());
    let new_pwd = &pwd.to_string()[0..(pwd.len()-up_len)];
    return String::from(new_pwd);
}

#[allow(dead_code)]
fn find_dirs_to_delete(total_sz: usize, top_dir_sizes: HashMap<String, usize>)->(String, usize) {
    // The ssd has a total size of 70000000
    // We need 30000000.
    // Calculate how much needs to be freed
    let space_available = 70000000 - total_sz as i32;
    let space_to_free = 30000000 - space_available;

    let mut first_dir: (String, usize) = ("".to_string(), 0); // Default value of the first smallest directory
    if space_to_free > 0 {
        // Get a list of directories that, if deleted, would free up enough space.
        let mut dirs_ge_space_to_free:Vec<(String, usize)> = top_dir_sizes.clone().into_iter().filter(|(_n,s)|{
            return *s >= space_to_free as usize;            
        }).collect();

        // Sort the list smallest to largest.
        dirs_ge_space_to_free.sort_by(|a, b| {
            let first:i32 = a.1 as i32;
            let second:i32 = b.1 as i32;
            return first.cmp(&second);
        });

        // First entry is the correct entry
        if dirs_ge_space_to_free.len() >= 1 {
            first_dir = dirs_ge_space_to_free.get(0).unwrap().clone();
        }
    }
    return first_dir;
}

// tmp_path is basically a copy of the pwd variable used while adding a found file.
fn add_size_to_parent_dir(tmp_pwd_path: &mut String, parent_dir: String, top_dir_sizes: &mut HashMap<String, usize>, filesize: usize) {
    tmp_pwd_path.push_str("/");
    tmp_pwd_path.push_str(&parent_dir.clone());
    let curr_size_opt = top_dir_sizes.get(&*tmp_pwd_path);
    let curr_size = if curr_size_opt == None { 0 as usize } else { *curr_size_opt.unwrap() };
    let entry = curr_size + filesize;
    top_dir_sizes.insert(tmp_pwd_path.clone(), entry);
}

// Note...
// https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust?noredirect=1&lq=1
#[allow(dead_code)]
fn parse_directories(v: Vec<String>)->(HashMap<String, usize>, usize) {
    let mut line_cnt = 0;
    let mut pwd: String = String::from("");
    let mut paths_stack: VecDeque<String> = VecDeque::new();
    let mut top_dir_size_map:HashMap<String, usize> = HashMap::new();
    let mut total_space_used:usize = 0;

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
                    // Going up... Pop the stack and update path with the popped parent directory.
                    let old_parent_path = paths_stack.pop_back().unwrap();
                    pwd = remove_parent_path_from_pwd(old_parent_path, &pwd);
                    println!("\tCurrent path is {}", pwd );
                }                    
                else {
                    if words[2] == "/" {
                        // Went to root. Update paths_stack to reset it and reset pwd.
                        paths_stack.clear();
                        pwd = String::from("");
                    }
                    else {
                        // Went into a path, push it onto the paths stack and update pwd.
                        let mut new_path: String = String::from("/");
                        new_path.push_str(words[2].clone());                            
                        paths_stack.push_back(new_path.clone());
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
                    total_space_used += filesize;
                    let mut tmp_path:String = String::new();
                    // Check which top directory we are in, and push to there.
                    if paths_stack.len() >= 1 {                        
                        // For each path in the paths stack, add this file to the size.
                        for i in 0..paths_stack.len() {
                            add_size_to_parent_dir(&mut tmp_path, paths_stack[i].clone(), &mut top_dir_size_map, filesize);
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

    return (top_dir_size_map, total_space_used);

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


        let (top_dir_sizes, total_sz): (HashMap<String, usize>, usize) = parse_directories(v);
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
        println!("\tConsumed data {}", total_sz);        
        assert_eq!(48381165, total_sz);
        let smallest_dir = find_dirs_to_delete(total_sz, top_dir_sizes);
        assert_eq!(smallest_dir.1, 24933642 as usize);


    }



    

    

}



fn main()  -> io::Result<()> {
    use std::io::BufRead;

    let filename = if std::env::args().len() >= 2 { std::env::args().nth(1).unwrap() } else { String::from("input.txt")};
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
    let (top_dir_sizes, total_sz): (HashMap<String, usize>, usize) = parse_directories(v);
    let mut sum_of_most_100000:usize = 0;
    let _dirs_at_most_100000:Vec<(String, usize)> = top_dir_sizes.clone().into_iter().filter(|(_n,s)|{
        let is_lt = *s <= 100000;
        if is_lt {
            sum_of_most_100000 += *s;
        }            
        return is_lt;
    }).collect();
    println!("\tSum of all directories with size less than 100000 {}", sum_of_most_100000);
    if filename == "input.txt"{
        assert_eq!(1367870, sum_of_most_100000);
    }

    let smallest_dir = find_dirs_to_delete(total_sz, top_dir_sizes);
    if filename == "input.txt"{
        assert_eq!(smallest_dir.1, 549173 as usize);
    }
    println!("Size of the smallest directory that, if deleted, would free up enough space on the filesystem to run the update: {}", smallest_dir.1);

    Ok(())
}