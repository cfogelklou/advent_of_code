use std::{ io::{ self }, cmp };
mod utils;
use std::cmp::Ordering;

/*
        const NEXT: [(usize, usize); 4] = [
            (1, 0),
            (usize::MAX, 0),
            (0, 1),
            (0, usize::MAX),
        ];
*/

#[derive(Clone, PartialEq, Eq)]
enum ArrElement {
    Integer(usize),
    Arr(Vec<ArrElement>),
}

#[allow(dead_code)]
fn consume_this_array(data_bytes: &Vec<char>, i: &mut usize) -> Option<ArrElement> {
    let mut arrays: Vec<ArrElement> = Vec::new();
    let mut done = false;
    let mut wip_num = String::from("");
    while !done && i < &mut data_bytes.len() {
        let c: char = data_bytes[*i];
        *i += 1;
        match c {
            // Opening bracket, start a new instance to consume it
            '[' => {
                let elem = consume_this_array(data_bytes, i);
                match elem {
                    Some(a) =>
                        match a {
                            ArrElement::Arr(arr) => {
                                arrays.push(ArrElement::Arr(arr));
                            }
                            _ => {}
                        }
                    _ => {}
                }
            }
            // Our closing bracket,
            ',' | ']' => {
                if wip_num.len() > 0 {
                    let x = utils::robust_to_int(&wip_num) as usize;
                    arrays.push(ArrElement::Integer(x));
                }
                wip_num.clear();
                // Set the done flag to true if this was the closing bracket
                done = c == ']';
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

#[allow(dead_code)]
fn compare_packets(p1: &ArrElement, p2: &ArrElement) -> Ordering {
    match (p1, p2) {
        (&ArrElement::Integer(l_int), &ArrElement::Integer(r_int)) => {
            let r = l_int.cmp(&r_int);
            return r;
        }
        (&ArrElement::Arr(ref l_arr), &ArrElement::Arr(ref r_arr)) => {
            let mut l_elems = l_arr.iter();
            let mut r_elems = r_arr.iter();

            loop {
                let left = l_elems.next();
                let right = r_elems.next();

                match (left, right) {
                    (None, Some(_)) => {
                        return Ordering::Less;
                    }
                    (Some(_), None) => {
                        return Ordering::Greater;
                    }
                    (None, None) => {
                        return Ordering::Equal;
                    }
                    (Some(left), Some(right)) =>
                        match compare_packets(left, right) {
                            Ordering::Equal => {
                                continue;
                            }
                            Ordering::Less => {
                                return Ordering::Less;
                            }
                            Ordering::Greater => {
                                return Ordering::Greater;
                            }
                        }
                }
            }
        }
        (&ArrElement::Integer(lint), right_packet_list) => {
            // Create a vector of the left integer
            compare_packets(&ArrElement::Arr(vec![ArrElement::Integer(lint)]), right_packet_list)
        }
        (left_packet_list, &ArrElement::Integer(rint)) => {
            // Create a vector of the right integer
            compare_packets(left_packet_list, &ArrElement::Arr(vec![ArrElement::Integer(rint)]))
        }
    } // match
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
        [1,[2,[3,[4,[5,6,0]]]],8,9]"
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
                    Some(a) =>
                        match a {
                            ArrElement::Arr(arr) => {
                                arrays.push(ArrElement::Arr(arr));
                            }
                            _ => {}
                        }
                    _ => {}
                }
            }
        }

        for i in (0..arrays.len()).step_by(2) {
            let res = compare_packets(&arrays[i + 0], &arrays[i + 1]);
            let pair_num = i / 2 + 1;
            match res {
                Ordering::Equal => {
                    println!("p1 == p2");           
                    assert!(false);         
                    
                }
                Ordering::Less => {
                    println!("p1 < p2: Correct order");
                    assert!( pair_num == 1 || pair_num == 2 || pair_num == 4 || pair_num == 6 );

                }
                Ordering::Greater => {
                    println!("p1 > p2: Wrong order");
                    assert!( pair_num == 3 || pair_num == 5 || pair_num == 7 || pair_num == 8 );
                }
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
        std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(&filename)).unwrap()
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