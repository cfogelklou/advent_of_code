
use std::{io::{self}, collections::{VecDeque}, cmp};

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

#[allow(dead_code)]
fn split_into_tuples(s:String) -> ((i32, i32), (i32, i32)){
    
    let two_pairs:Vec<&str> = s.trim().split(",").collect();
    let mut range_tuples:Vec<(i32,i32)> = Vec::new();
    for p in two_pairs {
        let start_end:Vec<&str> = p.split("-").collect();
        let start = start_end[0].parse::<i32>().unwrap();
        let end = start_end[1].parse::<i32>().unwrap();
        range_tuples.push((start, end));
    }

    return (range_tuples[0],range_tuples[1]);
}

#[allow(dead_code)]
fn is_range_contained(a:(i32,i32), b:(i32,i32)) -> bool {
    return  b.0 >= a.0 && b.1 <= a.1;
}

#[allow(dead_code)]
fn complete_containment(a:(i32,i32), b:(i32,i32)) -> bool {
    return is_range_contained(a, b) || is_range_contained(b, a);
}


// For part 1, split the rucksacks into two groups and get the score.
#[allow(dead_code)]
fn how_many_assignments_is_one_contained_in_the_other(v:Vec<String>)->i32{   
    
    let mut total: i32 = 0;
    for next_line in v.iter() {
        let partners: ((i32, i32), (i32, i32)) = split_into_tuples(next_line.clone());
        if complete_containment(partners.0, partners.1) {
            total += 1;
        }
    }

    return total;
}

#[allow(dead_code)]
fn partial_containment(a:(i32,i32), b:(i32,i32)) -> bool {
    // I'm too tired to think, so using a method that is guaranteed to work rather than a potentially simpler comparison
    let mut overlap = false;
    let mut a_sections:Vec<i32> = Vec::new();
    for i in a.0..(a.1+1) {
        a_sections.push(i);
    }
    for i in b.0..(b.1+1) {
        overlap = overlap || a_sections.contains(&i);
    }

    return overlap;
}

// For part 2, Check if there is any overlap
#[allow(dead_code)]
fn how_many_assignments_is_there_any_overlap(v:Vec<String>)->i32{   
    
    let mut total: i32 = 0;
    for next_line in v.iter() {
        let partners: ((i32, i32), (i32, i32)) = split_into_tuples(next_line.clone());
        if partial_containment(partners.0, partners.1) {
            total += 1;
        }
    }

    return total;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TypeOfLine {
    Crate = 0,
    NumCrates = 1,
    Nothing = 2,
    Instruction = 3
}

#[allow(dead_code)]
fn get_type_of_line(s_in:String) -> TypeOfLine {
    let mut t = TypeOfLine::Nothing;
    let s = s_in.trim();
    if s.contains("[") { 
        t = TypeOfLine::Crate;
    } else if s.contains("move") { 
        t = TypeOfLine::Instruction;
    } else if s.contains("1") {
        t = TypeOfLine::NumCrates;
    }
    return t;
}



#[allow(dead_code)]
fn get_num_stacks(s:String)->i32 {
    let v:Vec<&str> = s.split_whitespace().into_iter().collect();
    let l = v.len();
    return l as i32;
}

#[allow(dead_code)]
fn extract_num_stacks(v:Vec<String>)->i32{   
    let mut num_stacks: i32 = 0;
    //let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for next_line in v.iter() {
        let t = get_type_of_line(next_line.to_string());
        if t == TypeOfLine::NumCrates {
            num_stacks = get_num_stacks(next_line.to_string());
        }
    }

    return num_stacks;
}

fn get_instruction(s:&str)->(i32, i32, i32){
    let vec: Vec<&str> = s.split_whitespace().collect();
    let m:i32 = vec[1].parse::<i32>().unwrap();
    let f:i32 = vec[3].parse::<i32>().unwrap();
    let t:i32 = vec[5].parse::<i32>().unwrap();
    return (m,f,t);

}

#[allow(dead_code)]
fn process_stacks(v:Vec<String>, num_stacks:i32, crate_mover_9000:bool)->Vec<char>{
    let mut stacks_arr: Vec<VecDeque<char>> = Vec::new();
    for _i in 0..num_stacks {
        stacks_arr.push(VecDeque::new());
    }

    for next_line in v.iter() {
        let t = get_type_of_line(next_line.to_string());
        match t {
            TypeOfLine::Crate => {
                let len = next_line.len();
                let mut stack_num = 0;
                for i in (0..len).step_by(4){                    
                    let slice_begin = i;
                    let slice_end = cmp::min(slice_begin + 3, next_line.len());        
                    let crate_str: &str = &next_line[slice_begin..slice_end];
                    let c = crate_str.chars().nth(1).unwrap();
                    if c >= 'A' && c <= 'Z' {
                        stacks_arr[stack_num].push_front(c); // front is bottom.
                    }
                    stack_num += 1;
                }
            },
            TypeOfLine::NumCrates => {},
            TypeOfLine::Nothing => {},
            TypeOfLine::Instruction => {
                let (m, f, t) = get_instruction(next_line);

                // Todo: How to do this without cloning?
                let mut from_stack: VecDeque<char> = stacks_arr[(f-1) as usize].clone();
                let mut to_stack: VecDeque<char> = stacks_arr[(t-1) as usize].clone();
                assert_eq!(true, from_stack.len() >= m as usize);
                if !crate_mover_9000 {
                    for _i in 0..m {
                        let c: char = from_stack.pop_back().unwrap();
                        if c >= 'A' && c <= 'Z' {
                            to_stack.push_back(c);
                        }
                        else {
                            println!("Could not pop a crate.");
                        }
                    }
                }
                else {
                    let mut tmp_stack: VecDeque<char> = stacks_arr[(f-1) as usize].clone();
                    for _i in 0..m {
                        let c: char = from_stack.pop_back().unwrap();
                        if c >= 'A' && c <= 'Z' {
                            tmp_stack.push_back(c);
                        }
                        else {
                            println!("Could not pop a crate.");
                        }
                    }
                    for _i in 0..m {
                        let c: char = tmp_stack.pop_back().unwrap();
                        to_stack.push_back(c);
                    }
                }
                // Todo: How to do this without cloning? It's inefficient as all hell.
                stacks_arr[(f-1) as usize] = from_stack;
                stacks_arr[(t-1) as usize] = to_stack;

            },
        };
    }

    return stacks_arr.iter().map(|stack| {
        let top_char:char = if stack.len() > 0 { *stack.back().unwrap() } else { ' ' };
        return top_char;
    }).collect();
}

#[allow(dead_code)]
fn get_chars_to_start_seq(a:&str, start_length:usize) -> i32 {
    let mut v:VecDeque<char> = VecDeque::new();
    let mut c_idx = 0;
    let mut first_start:i32 = -1;    
    a.chars().for_each(|c: char| {
        v.push_back(c);
        if v.len() > start_length {
            v.pop_front();
        }

        if v.len() >= start_length {
            let mut deduped_vector:Vec<char> = v.iter().map(|c| { return *c; } ).collect();
            deduped_vector.sort();
            deduped_vector.dedup();
            if deduped_vector.len() == start_length {
                first_start = if first_start >= 0 { first_start } else { c_idx + 1 };
            }
        }
        
        c_idx+=1;
    });
    return first_start;
}


#[cfg(test)]
mod tests {
    

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn crates_check_0() {
        
        let raw_string: String = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
        bvwbjplbgvbhsrlpgdmjqwftvncz
        nppdvjthqldpwncqszvftbrmjlhg
        nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
        zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
            
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());
        {
            let a = &v[0];
            assert_eq!(7, get_chars_to_start_seq(a, 4));
        }
        {
            let a = &v[1];
            assert_eq!(5, get_chars_to_start_seq(a, 4));
        }
        {
            let a = &v[2];
            assert_eq!(6, get_chars_to_start_seq(a, 4));
        }
        {
            let a = &v[3];
            assert_eq!(10, get_chars_to_start_seq(a, 4));
        }
        {
            let a = &v[4];
            assert_eq!(11, get_chars_to_start_seq(a, 4));
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
    
    for next_line in v.iter() {
        let onestar = get_chars_to_start_seq(&next_line, 4);
        println!("*  Start was at char {}", onestar);
        assert_eq!(onestar, 1198);
    }

    for next_line in v.iter() {
        let onestar = get_chars_to_start_seq(&next_line, 14);
        println!("** Start was at char {}", onestar);
        assert_eq!(3120, onestar);
    }    

    Ok(())
}