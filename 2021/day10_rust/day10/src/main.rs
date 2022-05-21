

use std::collections::VecDeque;

fn is_open_bracket(c:char)->bool {
    return c == '[' || c == '{' || c == '<' || c == '(';
}

fn matching_open_bracket(c:char)->char {
    return match c {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        '>' => '<',
        _ => ' ',
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
                
                if !isok{
                    println!("{} - Expected {} but found {} instead.", s, matching_close_bracket(*oe.unwrap()), c);
                }

                //if oe != None {
                //    let pe:&char = oe.unwrap();
                //    let e:char = *pe;
                //    if e == '('{
                //        println!("Brace brace")
                //    }                    
                //}
                if isok {
                    stack.pop_back();
                }
                else {
                    score = match oe {
                        Some(&'(') => 3,
                        Some(&'[') => 57,
                        Some(&'{') => 1197,
                        Some(&'<') => 25137,
                        _ => score
                    };
                    println!("score = {}", score);
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

fn main() {
    println!("Hello, world!");
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
    //vec.for_each(s:String)
    for i in 0..v.len(){
        check_line(v[i].to_string());
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


}