
use std::{io::{self}, collections::{VecDeque}, cmp};

mod utils;

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


#[cfg(test)]
mod tests {
    use std::cmp;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn crates_check_0() {
        
        let raw_string = " 
                                        [D]    
                                    [N] [C]    
                                    [Z] [M] [P]
                                     1   2   3 
                                            
                                    move 1 from 2 to 1
                                    move 3 from 1 to 3
                                    move 2 from 2 to 1
                                    move 1 from 1 to 2".to_string();
    
        
        let v1: Vec<String> = utils::test_input_to_vec(raw_string, false);
        // Trim the whitespace that we unfortunately inserted to make the formatting nice.
        let v:Vec<String> = v1.iter().map(|x| { 
            let beg = cmp::min(36, x.len());
            let s:&str = &x[beg..x.len()];
            return s.to_string();
        }).collect();
        let num_stacks = extract_num_stacks(v.clone());
        assert_eq!(3, num_stacks);        
        let top_chars = process_stacks(v, num_stacks, false);
        for i in 0..top_chars.len(){
            println!("{}", top_chars[i]);
        }
        assert_eq!('C', top_chars[0]);
        assert_eq!('M', top_chars[1]);
        assert_eq!('Z', top_chars[2]);
    }

    #[test]
    fn crates_check_1() {
        
        let raw_string = " 
                                        [D]    
                                    [N] [C]    
                                    [Z] [M] [P]
                                     1   2   3 
                                            
                                    move 1 from 2 to 1
                                    move 3 from 1 to 3
                                    move 2 from 2 to 1
                                    move 1 from 1 to 2".to_string();
    
        
        let v1: Vec<String> = utils::test_input_to_vec(raw_string, false);
        // Trim the whitespace that we unfortunately inserted to make the formatting nice.
        let v:Vec<String> = v1.iter().map(|x| { 
            let beg = cmp::min(36, x.len());
            let s:&str = &x[beg..x.len()];
            return s.to_string();
        }).collect();

        assert_eq!(3, get_num_stacks(v[3].to_string()));
        let num_stacks = extract_num_stacks(v.clone());
        assert_eq!(3, num_stacks);

        assert_eq!(TypeOfLine::Crate, get_type_of_line(v[1].to_string()));
        assert_eq!(TypeOfLine::Crate, get_type_of_line(v[2].to_string()));
        assert_eq!(TypeOfLine::Crate, get_type_of_line(v[3].to_string()));
        assert_eq!(TypeOfLine::NumCrates, get_type_of_line(v[4].to_string()));
        assert_eq!(TypeOfLine::Nothing, get_type_of_line(v[5].to_string()));        
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[6].to_string()));
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[7].to_string()));
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[8].to_string()));
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[9].to_string()));
        
        //let s = how_many_assignments_is_one_contained_in_the_other(v.clone());
        //println!("Overlapping sections {}", s);
        //assert_eq!(s, 2);
    }

    #[test]
    fn check_stack_sep() {
        
        let raw_string = " 
                                        [D]    
                                    [N] [C]    
                                    [Z] [M] [P]
                                     1   2   3 
                                            
                                    move 1 from 2 to 1
                                    move 3 from 1 to 3
                                    move 2 from 2 to 1
                                    move 1 from 1 to 2".to_string();
    
        
        let v1: Vec<String> = utils::test_input_to_vec(raw_string, false);
        // Trim the whitespace that we unfortunately inserted to make the formatting nice.
        let v:Vec<String> = v1.iter().map(|x| { 
            let beg = cmp::min(36, x.len());
            let s:&str = &x[beg..x.len()];
            return s.to_string();
        }).collect();

        {
            let (m,f,t) = get_instruction(&v[6]);
            assert_eq!(m, 1);
            assert_eq!(f, 2);
            assert_eq!(t, 1);
        }

        {
            let (m,f,t) = get_instruction("move 22 from 3 to 5");
            assert_eq!(m, 22);
            assert_eq!(f, 3);
            assert_eq!(t, 5);
        }


        //let s = how_many_assignments_is_one_contained_in_the_other(v.clone());
        //println!("Overlapping sections {}", s);
        //assert_eq!(s, 2);
    }

    #[test]
    fn crates_check_9001() {
        
        let raw_string = " 
                                        [D]    
                                    [N] [C]    
                                    [Z] [M] [P]
                                     1   2   3 
                                            
                                    move 1 from 2 to 1
                                    move 3 from 1 to 3
                                    move 2 from 2 to 1
                                    move 1 from 1 to 2".to_string();
    
        
        let v1: Vec<String> = utils::test_input_to_vec(raw_string, false);
        // Trim the whitespace that we unfortunately inserted to make the formatting nice.
        let v:Vec<String> = v1.iter().map(|x| { 
            let beg = cmp::min(36, x.len());
            let s:&str = &x[beg..x.len()];
            return s.to_string();
        }).collect();
        let num_stacks = extract_num_stacks(v.clone());
        assert_eq!(3, num_stacks);        
        let top_chars = process_stacks(v, num_stacks, true);
        assert_eq!('M', top_chars[0]);
        assert_eq!('C', top_chars[1]);
        assert_eq!('D', top_chars[2]);
        let s: String = top_chars.into_iter().collect();
        println!("Cratemover 9000: {}", s);
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
    let num_stacks = extract_num_stacks(v.clone());
    let top_chars = process_stacks(v.clone(), num_stacks, false);
    {
        let s: String = top_chars.into_iter().collect();
        println!("Cratemover 9000: {}", s);
    }
    println!("");
    
    let top_chars = process_stacks(v.clone(), num_stacks, true);

    {
        let s: String = top_chars.into_iter().collect();
        println!("Cratemover 9001: {}", s);
    }

    Ok(())
}