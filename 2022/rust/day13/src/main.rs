use std::{io::{self}, cmp};
mod utils;

/*
        const NEXT: [(usize, usize); 4] = [
            (1, 0),
            (usize::MAX, 0),
            (0, 1),
            (0, usize::MAX),
        ];
*/

enum ArrElement {
    Integer(usize),
    Arr(Vec<ArrElement>),
}

fn consume_this_array(data_bytes: &Vec<char>, i: &mut usize) -> Option<ArrElement> {
    let mut arrays: Vec<ArrElement> = Vec::new();
    let mut done = false;
    let mut wip_num = String::from("");
    while !done && i < &mut data_bytes.len() {
        let c: char = data_bytes[*i];
        *i += 1;
        match c {
            '[' => {
                let elem = consume_this_array(data_bytes, i);
                match elem {
                    Some(a) => match a {
                        ArrElement::Arr(arr) => {
                            arrays.push(ArrElement::Arr(arr));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            ']' | ',' => {
                done = c == ']';
                if wip_num.len() > 0 {
                    let x = utils::robust_to_int(&wip_num) as usize;
                    arrays.push(ArrElement::Integer(x));
                }
                wip_num.clear();
            }
            _ => {
                if c >= '0' && c <= '9' {
                    wip_num.push(c);
                }
            }
        }
    }
    return Some(ArrElement::Arr(arrays));
}

fn compare_arrays(left:&Vec<ArrElement>, right: Vec<ArrElement>){
    let mut res:i32 = 0;
    let mut i = 0;
    let minlen = cmp::min(left.len(), right.len());
    while res == 0 && i < minlen {
        let mut left_is_vec = false;
        let mut right_is_vec = false;
        let mut left_int:i32 = -1;
        let mut right_int:i32 = -1;
        match &left[i] {
            ArrElement::Arr(_) => {
                left_is_vec = true;   
            },
            ArrElement::Integer(x) => { }
            _ => {}
        }
        match &right[i] {
            ArrElement::Arr(_) => {
                right_is_vec = true;   
            }
            _ => {}
        }        
        if left_is_vec != right_is_vec {
            let the_vector = if left_is_vec { &left } else { &right };
            let not_a_vector = if !left_is_vec { right } else { left };
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {

    use std::array;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn array_compare_1() {
        let mut data_bytes = String::from(
            "[1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]",
        );

        let data_bytes_no_newlines: Vec<char> = data_bytes
            .clone()
            .chars()
            .filter(|b| *b != '\n' && *b != ' ')
            .collect();

        let mut arrays: Vec<ArrElement> = Vec::new();

        let mut i = 0;
        while i < data_bytes_no_newlines.len() {
            let c: char = data_bytes_no_newlines[i];
            i += 1;
            if c == '[' {
                let elem = consume_this_array(&data_bytes_no_newlines, &mut i);
                match elem {
                    Some(a) => match a {
                        ArrElement::Arr(arr) => {
                            arrays.push(ArrElement::Arr(arr));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        for i in (0..arrays.len()).step_by(2) {
            let res = compare_arrays(&arrays[i+0], &arrays[i+1]);
            if (0 == i){
                assert!(res > 1);
            }
        }

        println!("len = {}", data_bytes_no_newlines.len());
    }
}

pub fn main() -> io::Result<()> {
    use std::io::BufRead;

    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("input.txt")
    };
    let file = std::io::BufReader::new(
        std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(&filename)).unwrap(),
    );
    let mut vec_in: Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {
        let l: String = line.unwrap();
        vec_in.push(l);
    }
    assert_ne!(0, vec_in.len());

    //assert_eq!(monkey_business, 10605);
    Ok(())
}
