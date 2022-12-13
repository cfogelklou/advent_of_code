use std::{ io::{ self }, collections::VecDeque };
mod utils;



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn monkey_biz() {
        let raw_string =  String::from("Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi");
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());


        //let monkey_business = inspections[0] * inspections[1];
        //println!("Monkey business = {monkey_business}");
        //assert_eq!(monkey_business, 10605);
    }

   

    //}
}

fn _main() -> io::Result<()> {
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


const NEXT: [(usize, usize); 4] = [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];
pub fn main() {
    let data = include_str!("../input.txt");
    let mut map: Vec<_> = data
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();

    let w = data.bytes().position(|b| b == b'\n').unwrap();
    let h = map.len() / w;
    let mut start = data.bytes().position(|b| b == b'S').unwrap();
    let mut end = data.bytes().position(|b| b == b'E').unwrap();
    (start, end, map[start], map[end]) = (start - start / (w + 1), end - end / (w + 1), 0, 25);

    println!(
        "{}",
        pathfinding::directed::bfs::bfs(
            &(end % w, end / w),
            |(x, y)| {
                let cur = map[y * w + x];
                NEXT.iter()
                    .map(|(xx, yy)| (x.wrapping_add(*xx), y.wrapping_add(*yy)))
                    .filter(|(x, y)| x < &w && y < &h && map[y * w + x] >= cur.saturating_sub(1))
                    .collect::<Vec<_>>()
            },
            |&p| p == (start % w, start / w),
        )
        .unwrap()
        .len()
            - 1
    );
}