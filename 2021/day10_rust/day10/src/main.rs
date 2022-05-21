

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

        return if stack.is_empty() { 0 } else { -1 };
    }
}

fn check_score(v:Vec<&str>)->i64{    
    let mut score:i64 = 0;
    for i in 0..v.len(){
        let mut s = check_line(v[i].to_string());
        s = if s < 0 { 0 } else { s };
        score = score + s;
    }
    println!("check_score::Total score is {}", score);
    return score;
}

fn test_example_1() {
    let v = vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]"
    ];

    let s = check_score(v);
    assert_eq!(s, 26397);
}

fn main() {
    use std::io::BufRead;
    test_example_1();

    let filename = std::env::args().nth(1).expect("Expected filename");
    let file = std::io::BufReader::new(
        std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(
            &filename,
        ))
        .unwrap(),
    );
    for (y, line) in file.lines().enumerate() {
        println!("y = {}, line = {}", y, line.as_ref().unwrap());
        for (x, c) in line.unwrap().chars().enumerate() {
            //grid[(x, y)] = c.to_digit(10).unwrap_or(u8::MAX as u32) as u8;
            //println!("x = {}, c = {}", x, c);
        }
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