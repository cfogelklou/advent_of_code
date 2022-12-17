use std::{ io::{ self } };
//use std::array;
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
                if elem.is_some() {
                    if let ArrElement::Arr(arr) = elem.unwrap() { // Instead of match
                        arrays.push(ArrElement::Arr(arr));
                    }
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
fn parse_string_to_arrays(data_bytes: &String) -> Vec<ArrElement> {
    let mut arrays: Vec<ArrElement> = Vec::new();
    let data_bytes_no_newlines: Vec<char> = data_bytes
        .clone()
        .chars()
        .filter(|b| *b != '\n' && *b != ' ')
        .collect();
    let mut i = 0;
    while i < data_bytes_no_newlines.len() {
        let c: char = data_bytes_no_newlines[i];
        i += 1;
        if c == '[' {
            let elem = consume_this_array(&data_bytes_no_newlines, &mut i);
            if elem.is_some() {
                if let ArrElement::Arr(arr) = elem.unwrap() { // Instead of match
                    arrays.push(ArrElement::Arr(arr));
                }
            }
        }
    }
    return arrays;
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

#[allow(dead_code)]
fn process_pairs_and_get_correct_indices(arrays: &Vec<ArrElement>) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    for i in (0..arrays.len()).step_by(2) {
        let res = compare_packets(&arrays[i + 0], &arrays[i + 1]);
        let pair_num = i / 2 + 1;
        match res {
            Ordering::Equal => {
                println!("p1 == p2");
            }
            Ordering::Less => {
                println!("p1 < p2: Correct order");
                indices.push(pair_num);
            }
            Ordering::Greater => {
                println!("p1 > p2: Wrong order");
            }
        }
    }
    return indices;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn array_compare_1() {
        let data_bytes = String::from(
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

        let arrays = parse_string_to_arrays(&data_bytes);

        let indices = process_pairs_and_get_correct_indices(&arrays);
        assert_eq!(indices, [1, 2, 4, 6]);
        let sum: usize = indices.iter().sum();
        assert_eq!(13, sum);
    }
}

pub fn main() -> io::Result<()> {

    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("input.txt")
    };
    let data_bytes = std::fs::read_to_string(filename).unwrap();

    let arrays = parse_string_to_arrays(&data_bytes);

    let indices = process_pairs_and_get_correct_indices(&arrays);
    //assert_eq!(indices, [1,2,4,6]);
    let sum: usize = indices.iter().sum();
    //assert_eq!(13, sum);
    println!("Sum of indices = {}", sum);
    assert_eq!(sum, 6656);

    // Now part 2
    {
        let mut arrays2 = arrays.clone();
        let d2 = ArrElement::Arr(vec![ArrElement::Integer(2)]);
        let d6 = ArrElement::Arr(vec![ArrElement::Integer(6)]);
        arrays2.push(d2.clone());
        arrays2.push(d6.clone());
        arrays2.sort_by(compare_packets);

        
        let i2 = 1 + arrays2.binary_search_by(|arr| compare_packets(arr, &d2)).ok().unwrap();
        let i6 = 1 + arrays2.binary_search_by(|arr| compare_packets(arr, &d6)).ok().unwrap();

        let prod = i2*i6;
        assert_eq!(prod, 19716);

        println!("Prod of indices = {}", prod);
    }

    Ok(())
}