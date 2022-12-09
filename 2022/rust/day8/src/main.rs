
use std::{io::{self}, collections::{VecDeque, HashMap}};
mod utils;


#[allow(dead_code)]
fn remove_parent_path_from_pwd(up_path: String, pwd: &String)->String {
    let up_len = up_path.len();
    let new_pwd = &pwd.to_string()[0..(pwd.len()-up_len)];
    return new_pwd.to_string();
}

#[allow(dead_code)]
fn find_dirs_to_delete(total_sz: usize, top_dir_sizes: HashMap<String, usize>)->(String, usize) {
    let space_available = 70000000 - total_sz as i32;
    let space_to_free = 30000000 - space_available;
    let mut first_dir: (String, usize) = ("".to_string(), 0);
    if space_to_free > 0 {
        let mut dirs_ge_space_to_free:Vec<(String, usize)> = top_dir_sizes.clone().into_iter().filter(|(_n,s)|{
            return *s >= space_to_free as usize;            
        }).collect();
        dirs_ge_space_to_free.sort_by(|a, b| {
            let first:i32 = a.1 as i32;
            let second:i32 = b.1 as i32;
            return first.cmp(&second);
        });
        if dirs_ge_space_to_free.len() >= 1 {
            first_dir = dirs_ge_space_to_free.get(0).unwrap().clone();        
        }
    }
    return first_dir;
}

// Note...
// https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust?noredirect=1&lq=1
#[allow(dead_code)]
fn parse_directories(v: Vec<String>)->(HashMap<String, usize>, usize) {
    let mut line_cnt = 0;
    let mut pwd: String = String::from("");
    let mut paths: VecDeque<String> = VecDeque::new();
    let mut top_dir_sizes:HashMap<String, usize> = HashMap::new();
    let mut total_used:usize = 0;

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
                    pwd = remove_parent_path_from_pwd(up_path, &pwd);
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
                    total_used += filesize;
                    // Check which top directory we are in, and push to there.
                    if paths.len() >= 1 {
                        let mut tmp_path:String = String::new();
                        for i in 0..paths.len() {
                            let parent_dir = paths[i].clone();
                            tmp_path.push_str("/");
                            tmp_path.push_str(&parent_dir.clone());
                            let curr_size_opt = top_dir_sizes.get(&tmp_path);
                            let curr_size = if curr_size_opt == None { 0 as usize } else { *curr_size_opt.unwrap() };
                            let entry = curr_size + filesize;
                            top_dir_sizes.insert(tmp_path.clone(), entry);
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

    return (top_dir_sizes, total_used);

}





#[cfg(test)]
mod tests {
    

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn trees_test() {
        
        let raw_string: String = "30373
        25512
        65332
        33549
        35390".to_string();
            
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());

        /*
        We have sides l, t, r, b
        Visible trees have height that is > previous height
        Visible trees map has entries stored at "x,y", to prevent duplicates.
        We don't need to store in vectors. lines are y and character index is x.
        But, maybe we should...?
        */
        /* 
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
        */


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