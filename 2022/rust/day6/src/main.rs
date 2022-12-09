
use std::{io::{self}, collections::{VecDeque}};

mod utils;

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