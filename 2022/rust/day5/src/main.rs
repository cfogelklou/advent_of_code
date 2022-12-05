
use std::{io::{self, BufReader}, collections::{VecDeque, HashMap}};

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
    if (s.contains("[")) { 
        t = TypeOfLine::Crate;
    } else if (s.contains("move")) { 
        t = TypeOfLine::Instruction;
    } else if s.contains("1") {
        t = TypeOfLine::NumCrates;
    }
    return t;
}

#[allow(dead_code)]
fn get_crate_arr(s:String)->Vec<char> {
    let c = s.chars();
    let len = s.len();
    let stacks = len / 3;
    let mut rvec:Vec<char> = Vec::new();

    for i in 0..stacks {
        let slice_begin = i * 3;
        let slice_end = slice_begin + 3;
        let the_slice = &s[slice_begin..slice_end];
        let c = the_slice.chars().nth(1).unwrap();
        rvec.push(c);
    }

    return rvec;
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
        if (t == TypeOfLine::NumCrates){
            num_stacks = get_num_stacks(next_line.to_string());
        }
    }

    return num_stacks;
}

#[allow(dead_code)]
fn process_stacks(v:Vec<String>, num_stacks:i32)->i32{   
    let mut total: i32 = 0;
    let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();
    for next_line in v.iter() {
        let t = get_type_of_line(next_line.to_string());
        match t {
            TypeOfLine::Crate => {
                let len = next_line.len();
                for i in (0..len).step_by(4){                    
                    let slice_begin = i + 1;
                    let slice_end = slice_begin + 3;        
                    let crate_str: &str = &next_line[slice_begin..slice_end];
                    println!("{}", crate_str);
                }
            },
            TypeOfLine::NumCrates => {},
            TypeOfLine::Nothing => {},
            TypeOfLine::Instruction => {},
        };
    }

    return total;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn camp_cleanup_1() {
        
        let raw_string = 
       "    [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";
    
        let v: Vec<String> = utils::test_input_to_vec(raw_string.to_string(), false);
        assert_eq!(3, get_num_stacks(v[3].to_string()));
        let num_stacks = extract_num_stacks(v.clone());
        assert_eq!(3, num_stacks);

        assert_eq!(TypeOfLine::Crate, get_type_of_line(v[0].to_string()));
        assert_eq!(TypeOfLine::Crate, get_type_of_line(v[1].to_string()));
        assert_eq!(TypeOfLine::Crate, get_type_of_line(v[2].to_string()));
        assert_eq!(TypeOfLine::NumCrates, get_type_of_line(v[3].to_string()));
        assert_eq!(TypeOfLine::Nothing, get_type_of_line(v[4].to_string()));        
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[5].to_string()));
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[6].to_string()));
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[7].to_string()));
        assert_eq!(TypeOfLine::Instruction, get_type_of_line(v[8].to_string()));
        
        //let s = how_many_assignments_is_one_contained_in_the_other(v.clone());
        //println!("Overlapping sections {}", s);
        //assert_eq!(s, 2);
    }

    #[test]
    fn check_stack_sep() {
        
        let raw_string = 
"   [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 
        
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    
        let v: Vec<String> = utils::test_input_to_vec(raw_string.to_string(), false);
        get_crate_arr(v[0].to_string());
        get_crate_arr(v[1].to_string());
        get_crate_arr(v[2].to_string());

        //let s = how_many_assignments_is_one_contained_in_the_other(v.clone());
        //println!("Overlapping sections {}", s);
        //assert_eq!(s, 2);
    }

    #[test]
    fn camp_cleanup_2() {
        
        let raw_string = 
"   [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    
        let v: Vec<String> = utils::test_input_to_vec(raw_string.to_string(), false);
        let num_stacks = extract_num_stacks(v.clone());
        assert_eq!(3, num_stacks);        
        process_stacks(v, num_stacks);
        
        //let s = how_many_assignments_is_one_contained_in_the_other(v.clone());
        //println!("Overlapping sections {}", s);
        //assert_eq!(s, 2);
    }

    #[test]
    fn camp_cleanup_3() {
        
        let raw_string = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
    
        let v: Vec<String> = utils::test_input_to_vec(raw_string.to_string(), false);
        
        let i = how_many_assignments_is_there_any_overlap(v.clone());
        //let (i, _) = rucksack_filter_groups(v.clone());
        println!("Overlapping sections {}", i);
        //println!("Badge scores is {}", i);
        assert_eq!(i, 4);
        //assert_eq!(i, 70);
    }


    #[test]
    fn t1() {
        let x = split_into_tuples("2-4,6-8".to_string());
        assert_eq!(2, x.0.0);
        assert_eq!(4, x.0.1);
        assert_eq!(6, x.1.0);
        assert_eq!(8, x.1.1);
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
    let s = how_many_assignments_is_one_contained_in_the_other(v.clone());
    let i = how_many_assignments_is_there_any_overlap(v.clone());
    println!("Overlapping sections {}", s);
    println!("Part 2 is {}", i);
    assert_eq!(s, 515);
    assert_eq!(i, 883);    
    Ok(())
}