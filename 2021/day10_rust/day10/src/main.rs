

use std::collections::VecDeque;

fn is_open_bracket(c:char)->bool {
    return c == '[' || c == '{' || c == '<' || c == '(';
}

fn score_for_close_bracket(c:char)->i64 {
    return match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
}

fn matching_close_bracket(c:char)->char {
    return match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => ' ',
    };
}

fn score_for_close_bracket_2(c:char)->i64 {
    return match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    };
}

fn check_line(s:String)->i64{
    //println!("{}", s);
    let mut stack:VecDeque<char> = VecDeque::new();
    let mut score = 0;
    let mut i = 0;
    while i < s.len() && score == 0 {
        let c:char = s.chars().nth(i).unwrap();
        if is_open_bracket(c){
            stack.push_back(c);
        }
        else {
            if stack.is_empty(){
                println!("Stack is empty, cannot pop")
            }
            else {
                let oe:Option<&char> = stack.back();
                let isok:bool = match c {
                    ')' => oe == Some(&'('),
                    '}' => oe == Some(&'{'),
                    ']' => oe == Some(&'['),
                    '>' => oe == Some(&'<'),
                    _ => false
                };
                
                if isok {
                    stack.pop_back();
                }
                else {
                    println!("{} - Expected {} but found {} instead.", s, matching_close_bracket(*oe.unwrap()), c);
                    score = score_for_close_bracket(c);
                }
            }
        }
        i = i+1;
    }
    if score != 0 {
        return score;
    }
    else {
        if stack.is_empty() { 
            return 0; 
        } else { 

            // Line is incomplete. Complete it.
            score = 0;
            let mut completion:String = String::new();
            while !stack.is_empty(){
                let o:char = *stack.back().unwrap();
                let c:char = matching_close_bracket(o);
                completion.push(c);
                score = score * 5 + score_for_close_bracket_2(c);
                stack.pop_back();
            }
            println!("{} - Complete by adding {}. score = {}", s, completion, 0 - score);
            return 0 - score; 
        };
    }
}

fn check_score(v:Vec<String>)->(i64, Vec<i64>){    
    let mut incomplete_scores:Vec<i64> = Vec::new();
    let mut score:i64 = 0;
    for i in 0..v.len(){
        let s0 = check_line(v[i].to_string());
        let s = if s0 < 0 { 0 } else { s0 };
        if s0 < 0 {
            incomplete_scores.push(0 - s0);
        }
        score = score + s;
    }
    println!("check_score::Total score is {}", score);
    return (score, incomplete_scores);
}

fn test_example_1() {
    let v:Vec<String> = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string()
    ];

    let (s,_) = check_score(v.clone());
    assert_eq!(s, 26397);
}



fn main() {
    use std::io::BufRead;
    test_example_1();

    if false {
        let filename = std::env::args().nth(1).expect("Expected filename");
        let file = std::io::BufReader::new(
            std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(
                &filename,
            ))
            .unwrap(),
        );
        let mut v:Vec<String> = Vec::new();
        for (_, line) in file.lines().enumerate() {
            //println!("y = {}, line = {}", y, line.as_ref().unwrap());
            let l:String = line.unwrap();
            v.push(l);
        }    
        let (s,_) = check_score(v.clone());
        println!("main::Total score is {}", s);
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn t1() {
        test_example_1();
    }

}