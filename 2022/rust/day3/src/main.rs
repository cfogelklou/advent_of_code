
use std::io::{self, BufReader};

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

// Gets the score for a character
fn get_score_for_char(c:char) -> i32 {
    let cc = c as i32;
    let aa = 'a' as i32;
    let aaa = 'A' as i32;
    return if c >= 'a' && c <= 'z' { cc - aa + 1 } else { cc - aaa + 27 }
}

// For part 1, split the rucksacks into two groups and get the score.
fn rucksack_filter(v:Vec<String>)->(i64, i64){   
    let mut total: i64 = 0;
    for next_line in v.iter() {
        let mut arr: Vec<char> = Vec::new();
        next_line.chars().for_each(|c| {
            arr.push(c);
        });
        //(0..next_line.len()).for_each(|i: usize| {
        //    let c:char = next_line.chars().nth(i).unwrap();
        //        arr.push(c);
        //});
        let items = arr.len();
        let compartment_items = items / 2;
        let l = arr[0..compartment_items].to_vec();
        let r = arr[compartment_items..items].to_vec();
        let mut common_items:Vec<char> = Vec::new();
        l.iter().for_each(|x| {
            if r.contains(x){
                common_items.push(*x);
            }
        });
        let score = get_score_for_char(common_items[0]);
        total += score as i64;

    }

    return (total, 0);
}

// For the second one, split into groups of 3 and look for common items
fn rucksack_filter_groups(v:Vec<String>)->(i64, i64){   
    let mut total: i64 = 0;

    // Go through each group
    for i in (0..v.len()).step_by(3) {
        let mut common_items:Vec<char> = Vec::new();
        let first:Vec<char> = v[i].chars().collect();
        for j in 1..3{
            let r:Vec<char> = v[i + j].chars().collect();
            let mut this_common:Vec<char> = Vec::new();
            
            // Either compare the the first vector, or to the common_items from last pass
            let cmp: &Vec<char> = if j == 1 { &first } else { &common_items };
            cmp.iter().for_each(|x| {
                if r.contains(x){
                    if !this_common.contains(x) {
                        this_common.push(*x);
                    }
                }
            });
            common_items = this_common;
        }
        assert_eq!(1, common_items.len());
        let score = get_score_for_char(common_items[0]);
        total += score as i64;

    }

    return (total, 0);
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
    let (s, _) = rucksack_filter(v.clone());
    let (i, _) = rucksack_filter_groups(v.clone());
    println!("Score for common items is {}", s);
    println!("Badge scores is {}", i);
    //assert_eq!(s, 14375);
    //assert_eq!(i, 10274);    
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
 
    #[test]
    fn t1() {
        assert_eq!(get_score_for_char('a'), 1);
        assert_eq!(get_score_for_char('b'), 2);
        assert_eq!(get_score_for_char('z'), 26);
        assert_eq!(get_score_for_char('A'), 27);
        assert_eq!(get_score_for_char('Z'), 52);
    }

    // This test
    #[test]
    fn rucksack_test() {
        
        let raw_string = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";
    
        let v: Vec<String> = test_input_to_vec(raw_string.to_string());
        
        let (s, _) = rucksack_filter(v.clone());
        let (i, _) = rucksack_filter_groups(v.clone());
        println!("Score for common items is {}", s);
        println!("Badge scores is {}", i);
        assert_eq!(s, 157);
        assert_eq!(i, 70);
    }


}