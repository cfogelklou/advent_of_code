use std::{ io::{ self } };
//use std::array;
mod utils;
use std::cmp::Ordering;
use std::collections::HashMap;

/*
        const NEXT: [(usize, usize); 4] = [
            (1, 0),
            (usize::MAX, 0),
            (0, 1),
            (0, usize::MAX),
        ];
*/



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn array_compare_1() {
        let data_bytes = String::from(
            "498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9"
        );
        let vec_in: Vec<String> = utils::test_input_to_vec(data_bytes, true);
        assert_ne!(0, vec_in.len());

        let mut world:HashMap<String, char> = HashMap::new();

        for line in vec_in {
            let sxy:Vec<(&str, i32,i32)> = line.split("->").map(|l| l.trim() ).map(|l2| {
                let v:Vec<&str> = l2.split(",").collect();                
                let x = utils::robust_to_int(v[0]);
                let y = utils::robust_to_int(v[1]);
                return (l2, x,y);
            }).collect();

            println!("len = {}", sxy.len());

            for item in sxy {
                world.insert(item.0.to_string(), '#');
            }
            
            
        }
    }
}

pub fn main() -> io::Result<()> {

    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("input.txt")
    };
    let data_bytes = std::fs::read_to_string(filename).unwrap();


    Ok(())
}