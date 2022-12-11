use std::io::BufReader;

// For standard test cases, converts the test input to a vector of strings.
// Todo, this should be in some sort of a utilities file. Figure out how to do that :-D
#[allow(dead_code)]
pub fn test_input_to_vec(s: String, do_trim: bool) -> Vec<String> {
    use std::io::BufRead;
    let b = BufReader::new(s.as_bytes());
    let mut v: Vec<String> = Vec::new();
    for (_, line) in b.lines().enumerate() {
        let l: String = if do_trim {
            line.unwrap().trim().to_string()
        } else {
            line.unwrap().to_string()
        };
        v.push(l);
    }
    return v;
}

#[allow(dead_code)]
pub fn robust_to_int(s: &str) -> i32 {
    // let x = words[i].parse::<i32>().unwrap();
    let mut rval: i32 = 0;
    let mut outstr = String::new();
    for c in s.chars() {
        if c >= '0' && c <= '9' {
            outstr.push(c);
        }
    }
    if outstr.len() > 0 {
        match outstr.parse::<i32>() {
            Ok(i) => {
                rval = i;
            }
            Err(_error) => {}
        }
    }

    return rval;
}