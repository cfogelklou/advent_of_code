use std::{ collections::VecDeque, io::{ self } };
mod utils;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn pathfinding_test() {
        const NEXT: [(usize, usize); 4] = [
            (1, 0),
            (usize::MAX, 0),
            (0, 1),
            (0, usize::MAX),
        ];

        fn get_wrapped_successors(
            linear_vec_u8: &Vec<u8>,
            y: &usize,
            w: usize,
            x: &usize,
            h: usize
        ) -> Vec<(usize, usize)> {
            let cur: u8 = linear_vec_u8[y * w + x];
            let cur_sub = cur.saturating_sub(1);
            let rval: Vec<(usize, usize)> = NEXT.iter()
                .map(|(xx, yy)| {
                    // Wrapping (modular) addition. Computes self + rhs, wrapping around at the boundary of the type.
                    let xxyy = (x.wrapping_add(*xx), y.wrapping_add(*yy));
                    println!("xx = {}, yy = {}, xxyy = {},{}", xx, yy, xxyy.0, xxyy.1);
                    return xxyy;
                })
                .filter(|(x, y)| {
                    // Borrow w and h, and use to
                    return x < &w && y < &h && linear_vec_u8[y * w + x] >= cur_sub;
                })
                .collect::<Vec<_>>();
            return rval;
        }

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

        // Get row of start and end positions
        let start_row = start_pos / (w + 1);
        let end_row = end_pos / (w + 1);
        start_pos = start_pos - start_row;
        end_pos = end_pos - end_row;
        let aa = ('a' as u8) - ('a' as u8);
        linear_vec_u8[start_pos] = aa;
        let zz = ('z' as u8) - ('a' as u8);
        linear_vec_u8[end_pos] = zz;
        let start_x = end_pos % w;
        let start_y = end_pos / w;

        let optimal_path: usize =
            pathfinding::directed::bfs
                ::bfs(
                    &(start_x, start_y),
                    |(x, y)| -> Vec<(usize, usize)> {
                        let new_vec = get_wrapped_successors(&linear_vec_u8, y, w, x, h);
                        for v in &new_vec {
                            println!("  Returned vector ({},{})", v.0, v.1);
                        }
                        return new_vec;
                    },
                    |&p| p == (start_pos % w, start_pos / w)
                )
                .unwrap()
                .len() - 1;
        assert_eq!(484, optimal_path);
        println!("Optimal path = {}", optimal_path);
    }

    #[test]
    fn pathfinding_test_ref() {
        const NEXT: [(usize, usize); 4] = [
            (1, 0),
            (usize::MAX, 0),
            (0, 1),
            (0, usize::MAX),
        ];

        let data: &str = include_str!("../input.txt");
        let mut map: Vec<_> = data
            .bytes()
            .filter(|b| b != &b'\n')
            .map(|b| b.to_ascii_lowercase() - b'a')
            .collect();

        let w: usize = data
            .bytes()
            .position(|b| b == b'\n')
            .unwrap();
        let h: usize = map.len() / w;
        let mut start: usize = data
            .bytes()
            .position(|b| b == b'S')
            .unwrap();
        let mut end: usize = data
            .bytes()
            .position(|b| b == b'E')
            .unwrap();
        (start, end, map[start], map[end]) = (start - start / (w + 1), end - end / (w + 1), 0, 25);

        let optimal_path: usize =
            pathfinding::directed::bfs
                ::bfs(
                    &(end % w, end / w),
                    |(x, y)| -> Vec<(usize, usize)> {
                        let cur: u8 = map[y * w + x];
                        NEXT.iter()
                            .map(|(xx, yy)| (x.wrapping_add(*xx), y.wrapping_add(*yy)))
                            .filter(
                                |(x, y)| x < &w && y < &h && map[y * w + x] >= cur.saturating_sub(1)
                            )
                            .collect::<Vec<_>>()
                    },
                    |&p| p == (start % w, start / w)
                )
                .unwrap()
                .len() - 1;
        assert_eq!(484, optimal_path);
        println!("Optimal path = {}", optimal_path);
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(i32, i32);
    impl Pos {
        fn successors(&self) -> Vec<Pos> {
            let &Pos(x, y) = self;
            vec![
                Pos(x + 1, y + 2),
                Pos(x + 1, y - 2),
                Pos(x - 1, y + 2),
                Pos(x - 1, y - 2),
                Pos(x + 2, y + 1),
                Pos(x + 2, y - 1),
                Pos(x - 2, y + 1),
                Pos(x - 2, y - 1)
            ]
        }
    }

    /// static GOAL: Pos = Pos(4, 6);
    /// let result = bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
    /// assert_eq!(result.expect("no path found").len(), 5);

    #[test]
    fn monkey_biz() {

        fn successors(p: &Pos, vec2d: &Vec<Vec<i32>>) -> Vec<Pos> {
            let &Pos(x, y) = p;
            let mut rval:Vec<Pos> = Vec::new();
            
            
            return rval;
        }

        let raw_string = String::from(
            "Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi"
        );
        let vec_in: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, vec_in.len());
        let width = vec_in[0].len();
        assert_eq!(8, width);
        let height = vec_in.len();
        assert_eq!(5, height);

        let vec2d: Vec<Vec<i32>> = Vec::new();
        let mut start_pos = (-1, -1);
        let mut end_pos = (-1, -1);
        let mut y = 0;
        for y_str in vec_in {
            let mut y_line: Vec<i32> = Vec::new();
            let mut x = 0;
            for c in y_str.chars() {
                match c {
                    'S' => {
                        y_line.push(0);
                        start_pos = (x, y);
                    }
                    'E' => {
                        y_line.push(100);
                        end_pos = (x, y);
                    }
                    _ => y_line.push((c as i32) - ('a' as i32) + 1),
                }
                x += 1;
            }
            y += 1;
        }

        let GOAL: Pos = Pos(end_pos.0, end_pos.1);
        let result = pathfinding::directed::bfs::bfs(
            &Pos(start_pos.0, start_pos.1), 
            |p| successors(p, &vec2d), |p| *p == GOAL);
        
        println!("start_pos = {},{}", start_pos.0, start_pos.1);
        println!("end_pos = {},{}", end_pos.0, end_pos.1);
        println!("h = {}", vec2d.len());
    }

    //}
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
    let mut v: Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {
        let l: String = line.unwrap();
        v.push(l);
    }
    assert_ne!(0, v.len());

    //assert_eq!(monkey_business, 10605);
    Ok(())
}