use std::collections::VecDeque;

// Returns TRUE if C is an open bracket.
fn is_open_bracket(c:char)->bool {
    return c == '[' || c == '{' || c == '<' || c == '(';
}

// For part 1, returns the score for a found close bracket.
fn score_for_close_bracket(c:char)->i64 {
    return match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
}

// Gets the close bracket for a given open bracket.
fn matching_close_bracket(c:char)->char {
    return match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => ' ',
    };
}

// For part 2, gets the score for a given close bracket.
fn score_for_close_bracket_2(c:char)->i64 {
    return match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    };
}

// Checks the line for corruption or incomplete.
// If corrupt, returns the corruption score.
// If incomplete, retuns 0 - the completion score (negative!)
fn check_line(s:String)->i64{

    let mut stack:VecDeque<char> = VecDeque::new();
    let mut score = 0;
    let mut i = 0;
    // Todo: Figure out better way to do this in rust. I don't like i++ at the end
    while i < s.len() && score == 0 {
        let c:char = s.chars().nth(i).unwrap();
        i = i+1;
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
    }

    if score != 0 {
        // There was corruption, return immediately.
        return score;
    }
    else {
        // Either the line is incomplete or OK.
        if stack.is_empty() { 
            // Line is complete.
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
            println!("{} - Complete by adding {}.", s, completion);
            return 0 - score; 
        };
    }
}

// Checks a vector of strings for their scores.
fn check_score(v:Vec<String>)->(i64, i64){    
    let mut incomplete_scores:Vec<i64> = Vec::new();
    let mut incomplete_score:i64 = 0;
    let mut score:i64 = 0;
    // Iterate through all lines, update scores, and gather incomplete lines.
    for i in 0..v.len(){
        let s0 = check_line(v[i].to_string());
        let s = if s0 < 0 { 0 } else { s0 };
        if s0 < 0 {
            incomplete_scores.push(0 - s0);
        }
        score = score + s;
    }

    // Part 2: Gets the "middle" incomplete score.
    if !incomplete_scores.is_empty(){
        incomplete_scores.sort();
        incomplete_score = incomplete_scores[incomplete_scores.len()/2];
    }
    
    return (score, incomplete_score);
}

// The test case given the samples from AoC.
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

    let (s, i) = check_score(v.clone());
    println!("test_example_1::Total score is {}", s);
    println!("test_example_1::Incomplete score is {}", i);
    assert_eq!(s, 26397);
    assert_eq!(i, 288957);
}



fn main() {
    use std::io::BufRead;

    // Run unit test first
    test_example_1();

    if true {
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
        let (s,incomplete_score) = check_score(v.clone());
        println!("main::Total score is {}", s);
        println!("main::Incomplete score is {}", incomplete_score);
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn t1() {
        test_example_1();
    }

}