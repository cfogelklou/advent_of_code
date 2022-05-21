

use std::collections::VecDeque;

fn is_open_bracket(c:char)->bool {
    return c == '[' || c == '{' || c == '<' || c == '(';
}

fn check_line(s:String){
    //println!("is: {}", s);
    let mut stack:VecDeque<char> = VecDeque::new();
    for i in 0..s.len(){
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
                if oe != None {
                    let pe:&char = oe.unwrap();
                    let e:char = *pe;
                    if e == '('{
                        println!("Brace brace")
                    }
                    
                }
                stack.pop_back();
            }
        }

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
        check_line(v[i].to_string())
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