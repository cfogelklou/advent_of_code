
use std::io::{self, BufReader};

#[derive(Clone, Copy,PartialEq, Eq, Debug)]
enum RockPaperScissors {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MatchResult {
    YouWin = 0,
    Draw = 3,
    IWin = 6
}

#[allow(dead_code)]
fn get_move(c:char)->RockPaperScissors {
    return match c {
        'A' => RockPaperScissors::Rock,
        'B' => RockPaperScissors::Paper,
        'X' => RockPaperScissors::Rock,
        'Y' => RockPaperScissors::Paper,
        _ => RockPaperScissors::Scissors,
    };
}

#[allow(dead_code)]
fn get_plan(c:char)->MatchResult {
    return match c {
        'X' => MatchResult::YouWin,
        'Y' => MatchResult::Draw,
        _ => MatchResult::IWin,
    };
}

#[allow(dead_code)]
fn do_i_win(me:RockPaperScissors, you:RockPaperScissors) ->MatchResult {
    let ime:i32 = me as i32;
    let iyou:i32 = you as i32;
    let result = ime - iyou;
    let m:MatchResult;
    if result == 0 {
        m = MatchResult::Draw;
    }
    else if result == -2 {
        m = MatchResult::IWin;
    }
    else if result == 2 {
        m = MatchResult::YouWin;
    }
    else {
        m = if result < 0 { MatchResult::YouWin } else { MatchResult::IWin }
    }
    return m;
}

// First challenge, misinterpret the table
#[allow(dead_code)]
fn paper_rock_scissors_1(v:Vec<String>)->(i64, i64){   
    let mut total:i64 = 0;
    for next_line in v.iter() {

        let strategy_enc: Vec<&str> = next_line.trim().split_whitespace().collect();
        let you_enc = strategy_enc[0].chars().nth(0).unwrap();
        let me_enc = strategy_enc[1].chars().nth(0).unwrap();
        let me = get_move(me_enc);
        let you = get_move(you_enc);

        let me_win = do_i_win(me, you);
        let my_score = me_win as i32 + (me.clone() as i32) + 1;

        total += my_score as i64;
    }


    return (total, 0);
}

#[allow(dead_code)]
fn get_my_move(you:RockPaperScissors, result:MatchResult) -> RockPaperScissors {

    let inc:i32 = match result {
        MatchResult::Draw => 0,
        MatchResult::IWin => 1,
        _ => -1,
    };
    let me:i32 = ((you as i32) + inc) % 3;
      
    let mymove = match me {
        -1 => RockPaperScissors::Scissors,
        0 => RockPaperScissors::Rock,
        1 => RockPaperScissors::Paper,
        2 => RockPaperScissors::Scissors,
        _ => RockPaperScissors::Paper
    };

    return mymove;
}

// Second challenge, 
#[allow(dead_code)]
fn paper_rock_scissors_2(v:Vec<String>)->(i64, i64){   
    let mut total:i64 = 0;
    for next_line in v.iter() {

        let strategy_enc: Vec<&str> = next_line.trim().split_whitespace().collect();
        let you_enc = strategy_enc[0].chars().nth(0).unwrap();
        let me_enc = strategy_enc[1].chars().nth(0).unwrap();
        let you = get_move(you_enc);
        let my_plan:MatchResult = get_plan(me_enc);

        let me: RockPaperScissors = get_my_move(you, my_plan);
        
        let me_win = do_i_win(me, you);
        assert_eq!(me_win, my_plan);
        let my_score = me_win as i32 + (me.clone() as i32) + 1;

        total += my_score as i64;
    }
    return (total, 0);
}

fn paper_rock_scissors(v:Vec<String>)->(i64, i64){   
    let (r1, _) = paper_rock_scissors_1(v.clone());
    let (r2, _) = paper_rock_scissors_2(v.clone());
    return (r1, r2);
}




#[allow(dead_code)]
fn get_score_for_char(c:char) -> i32 {
    let cc = c as i32;
    let aa = 'a' as i32;
    let aaa = 'A' as i32;
    return if c >= 'a' && c <= 'z' { cc - aa + 1 } else { cc - aaa + 27 }
}

fn rucksack_filter(v:Vec<String>)->(i64, i64){   
    let mut total: i64 = 0;
    for next_line in v.iter() {
        let mut arr: Vec<char> = Vec::new();
        next_line.chars().for_each(|c| {
            println!("{}", c);
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


fn rucksack_filter_groups(v:Vec<String>)->(i64, i64){   
    let mut total: i64 = 0;
    for i in (0..v.len()).step_by(3) {
        let common_items:Vec<char> = Vec::new();
        for j in (0..2){
            let l = v[i*3 + j];
            let r = v[i*3 + j + 1];
            

            let next_line:Vec<char> = v[i*3 + j];
            let arr:Vec<char> = Vec::new();
            next_line.chars().for_each(|c| {
                println!("{}", c);
                arr.push(c);
            });
    
        }
    }

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

// For standard test cases, converts the test input to a vector of strings.
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
fn rucksack_test() {
    
    let raw_string = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
    PmmdzqPrVvPwwTWBwg\n\
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
    ttgJtRGJQctTZtZT\n\
    CrZsJsPPZsGzwwsLwLmpwMDw";

    let v: Vec<String> = test_input_to_vec(raw_string.to_string());
    
    let (s, i) = rucksack_filter(v.clone());
    println!("Score for common items is {}", s);
    println!("My score by the real plan is {}", i);
    assert_eq!(s, 157);
    //assert_eq!(i, 12);
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
    let (s, i) = rucksack_filter(v.clone());
    println!("Score for common items is {}", s);
    println!("My score by the real plan is {}", i);
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

    #[test]
    fn t3() {
        rucksack_test();
    }

}