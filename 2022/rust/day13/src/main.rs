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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn pathfinding_test() {
        let data: &str = include_str!("../input.txt");
        let data_bytes = data.bytes();
        let data_bytes_no_newlines = data_bytes.filter(|b| b != &b'\n');
        let mut linear_vec_u8: Vec<u8> = data_bytes_no_newlines
            .map(|b| b.to_ascii_lowercase() - b'a')
            .collect();

        let w: usize = data
            .bytes()
            .position(|b| b == b'\n')
            .unwrap();
        let h: usize = linear_vec_u8.len() / w;

        // Get position of S and E in the linear vector
        let mut start_pos: usize = data
            .bytes()
            .position(|b| b == b'S')
            .unwrap();
        let mut end_pos: usize = data
            .bytes()
            .position(|b| b == b'E')
            .unwrap();

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