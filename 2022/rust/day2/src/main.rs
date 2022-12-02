
use std::io::{self, BufReader};

#[derive(Clone, Copy)]
enum RockPaperScissors {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

#[derive(Clone, Copy)]
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

#[allow(dead_code)]
fn paper_rock_scissors(v:Vec<String>)->(i64, i64){   
    let mut total:i64 = 0;
    for next_line in v.iter() {

        let xy: Vec<&str> = next_line.trim().split_whitespace().collect();
        let x = xy[1].chars().nth(0).unwrap();
        let y = xy[0].chars().nth(0).unwrap();
        let me = get_move(x);
        let you = get_move(y);

        let me_win = do_i_win(me, you);
        let mut my_score = me_win as i32;
        my_score +=  (me.clone() as i32) + 1;

        total += my_score as i64;
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
    let (s, i) = paper_rock_scissors(v.clone());
    println!("The top three elves carry {}", s);
    println!("The elf's total calories are {}", i);
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
        assert_eq!( MatchResult::Draw as i32, do_i_win( RockPaperScissors::Paper, RockPaperScissors::Paper) as i32);
        assert_eq!( MatchResult::Draw as i32, do_i_win( RockPaperScissors::Rock, RockPaperScissors::Rock) as i32);
        assert_eq!( MatchResult::Draw as i32, do_i_win( RockPaperScissors::Scissors, RockPaperScissors::Scissors) as i32);
        assert_eq!( MatchResult::IWin as i32, do_i_win( RockPaperScissors::Scissors, RockPaperScissors::Paper) as i32);
        assert_eq!( MatchResult::IWin as i32, do_i_win( RockPaperScissors::Rock, RockPaperScissors::Scissors) as i32);
        assert_eq!( MatchResult::IWin as i32, do_i_win( RockPaperScissors::Paper, RockPaperScissors::Rock) as i32);
        assert_eq!( MatchResult::YouWin as i32, do_i_win( RockPaperScissors::Scissors, RockPaperScissors::Rock) as i32);
        assert_eq!( MatchResult::YouWin as i32, do_i_win( RockPaperScissors::Rock, RockPaperScissors::Paper) as i32);
        assert_eq!( MatchResult::YouWin as i32, do_i_win( RockPaperScissors::Paper, RockPaperScissors::Scissors) as i32);
    }

    #[test]
    fn t2() {
        test_example_1();
    }

}