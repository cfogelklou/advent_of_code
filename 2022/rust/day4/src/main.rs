
use std::{io::{self, BufReader}};

// For standard test cases, converts the test input to a vector of strings.
// Todo, this should be in some sort of a utilities file. Figure out how to do that :-D
#[allow(dead_code)]
fn test_input_to_vec(s: String) -> Vec<String> {
    use std::io::BufRead;
    let b = BufReader::new(s.as_bytes());
    let mut v:Vec<String> = Vec::new();
    for (_, line) in b.lines().enumerate() {    
        let l:String = line.unwrap().trim().to_string();
        v.push(l);
    }  
    return v;
}


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

fn is_range_contained(a:(i32,i32), b:(i32,i32)) -> bool {
    return  b.0 >= a.0 && b.1 <= a.1;
}

fn complete_containment(a:(i32,i32), b:(i32,i32)) -> bool {
    return is_range_contained(a, b) || is_range_contained(b, a);
}


// For part 1, split the rucksacks into two groups and get the score.
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn camp_cleanup_1() {
        
        let raw_string = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
    
        let v: Vec<String> = test_input_to_vec(raw_string.to_string());
        
        let s = how_many_assignments_is_one_contained_in_the_other(v.clone());
        //let (i, _) = rucksack_filter_groups(v.clone());
        println!("Overlapping sections {}", s);
        //println!("Badge scores is {}", i);
        assert_eq!(s, 2);
        //assert_eq!(i, 70);
    }

    #[test]
    fn camp_cleanup_2() {
        
        let raw_string = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
    
        let v: Vec<String> = test_input_to_vec(raw_string.to_string());
        
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
    //assert_eq!(s, 515);
    //assert_eq!(i, 883);    
    Ok(())
}