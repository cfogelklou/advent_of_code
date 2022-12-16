use std::{ io::{ self } };
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

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

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

        let arr: Vec<ArrElement> = Vec::new();

        //let data: &str = include_str!("../input.txt");
        //let data_bytes = data.bytes();
        let data_bytes_no_newlines: Vec<char> = data_bytes
            .clone()
            .chars()
            .filter(|b| *b != '\n' && *b != ' ')
            .collect();
        println!("len = {}", data_bytes_no_newlines.len());
        let mut arr: Vec<ArrElement> = Vec::new();
        let mut stack: VecDeque<Vec<ArrElement>> = VecDeque::new();
        let mut wip_num: String = String::from("");
        for c in data_bytes_no_newlines {
            match c {
                '[' => {
                    stack.push_back(Vec::new());
                }
                ']' => {
                    transfer_wip_integer(&mut wip_num, &mut stack);
                    let vec = stack.pop_back().unwrap();
                    arr.push(ArrElement::Arr(vec));
                }
                ',' => {
                    transfer_wip_integer(&mut wip_num, &mut stack);
                }
                _ => {
                    if c >= '0' && c <= '9' {
                        wip_num.push(c);
                    }
                }
            }
        }
    }

    fn transfer_wip_integer(wip_num: &mut String, stack: &mut VecDeque<Vec<ArrElement>>) {
        if wip_num.len() > 0 {
            let mut back = stack.back_mut().unwrap();
            let x = utils::robust_to_int(wip_num) as usize;
            back.push(ArrElement::Integer(x));
        }
        wip_num.clear();
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