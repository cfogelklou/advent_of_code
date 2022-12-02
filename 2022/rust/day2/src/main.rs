
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
    let (s, i) = paper_rock_scissors(v.clone());
    println!("My score by misinterpreting the plan is {}", s);
    println!("My score by the real plan is {}", i);
    Ok(())
}



#[allow(dead_code)]
fn test_example_1() {
    use std::io::BufRead;
    let raw_string = "A Y\n\
                            B X\n\
                            C Z";

    let b = BufReader::new(raw_string.as_bytes());
    let mut v:Vec<String> = Vec::new();
    for (_, line) in b.lines().enumerate() {    
        let l:String = line.unwrap().trim().to_string();
        v.push(l);
    }  
    
    let (s, i) = paper_rock_scissors(v.clone());
    println!("Your score {}", s);
    assert_eq!(s, 15);
    assert_eq!(i, 0);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn t1() {
        assert_eq!( MatchResult::Draw, do_i_win( RockPaperScissors::Paper, RockPaperScissors::Paper));
        assert_eq!( MatchResult::Draw, do_i_win( RockPaperScissors::Rock, RockPaperScissors::Rock));
        assert_eq!( MatchResult::Draw, do_i_win( RockPaperScissors::Scissors, RockPaperScissors::Scissors));
        
        assert_eq!( MatchResult::IWin, do_i_win( RockPaperScissors::Scissors, RockPaperScissors::Paper));
        assert_eq!( MatchResult::IWin, do_i_win( RockPaperScissors::Rock, RockPaperScissors::Scissors));
        assert_eq!( MatchResult::IWin, do_i_win( RockPaperScissors::Paper, RockPaperScissors::Rock));
        
        assert_eq!( MatchResult::YouWin, do_i_win( RockPaperScissors::Scissors, RockPaperScissors::Rock));
        assert_eq!( MatchResult::YouWin, do_i_win( RockPaperScissors::Rock, RockPaperScissors::Paper));
        assert_eq!( MatchResult::YouWin, do_i_win( RockPaperScissors::Paper, RockPaperScissors::Scissors));
    }

    #[test]
    fn t2() {
        assert_eq!( RockPaperScissors::Paper, get_my_move( RockPaperScissors::Paper, MatchResult::Draw));
        assert_eq!( RockPaperScissors::Rock, get_my_move( RockPaperScissors::Rock, MatchResult::Draw));
        assert_eq!( RockPaperScissors::Scissors, get_my_move( RockPaperScissors::Scissors, MatchResult::Draw));
        
        assert_eq!( RockPaperScissors::Scissors, get_my_move( RockPaperScissors::Paper, MatchResult::IWin));
        assert_eq!( RockPaperScissors::Paper, get_my_move( RockPaperScissors::Rock, MatchResult::IWin));
        assert_eq!( RockPaperScissors::Rock, get_my_move( RockPaperScissors::Scissors, MatchResult::IWin));

        assert_eq!( RockPaperScissors::Rock, get_my_move( RockPaperScissors::Paper, MatchResult::YouWin));
        assert_eq!( RockPaperScissors::Scissors, get_my_move( RockPaperScissors::Rock, MatchResult::YouWin));
        assert_eq!( RockPaperScissors::Paper, get_my_move( RockPaperScissors::Scissors, MatchResult::YouWin));

    }    

    #[test]
    fn t3() {
        test_example_1();
    }

}