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

fn draw_world(min_dim: &(i32, i32), max_dim: &(i32, i32), world: &mut HashMap<(i32, i32), char>) {
    println!("\n");
    let y0 = min_dim.1 - 20;
    let y1 = max_dim.1;
    for y in y0..y1 + 1 {
        let x0 = min_dim.0 - 10;
        let x1 = max_dim.0 + 10;
        for x in x0..x1 + 1 {
            let p = (x, y);
            let c = world.get(&p);
            match c {
                None => print!("."),
                Some(cc) => print!("{}", cc),
            }
        }
        println!("");        
    }
}

fn extract_lines_to_vec(vec_in: Vec<String>) -> Vec<Vec<(i32, i32)>>{
    let mut vecs: Vec<Vec<(i32, i32)>> = Vec::new();
    for line in vec_in {
        let xy: Vec<(i32, i32)> = line
            .split("->")
            .map(|l| l.trim())
            .map(|l2| {
                let v: Vec<&str> = l2.split(",").collect();
                let x = utils::robust_to_int(v[0]);
                let y = utils::robust_to_int(v[1]);
                return (x, y);
            })
            .collect();

        println!("len = {}", xy.len());
        vecs.push(xy);
    }
    return vecs;
}

fn fill_world(vecs: Vec<Vec<(i32, i32)>>, min_dim: &mut (i32, i32), max_dim: &mut (i32, i32), world: &mut HashMap<(i32, i32), char>) {
    for xy in vecs {
        xy.iter().for_each(|p| {
            let (x, y): (i32, i32) = p.clone();
            min_dim.0 = if x < min_dim.0 { x } else { min_dim.0 };
            max_dim.0 = if x > max_dim.0 { x } else { max_dim.0 };
            min_dim.1 = if y < min_dim.1 { y } else { min_dim.1 };
            max_dim.1 = if y > max_dim.1 { y } else { max_dim.1 };
        });

        // Use a hashtable as a hitbox
        for i in 0..xy.len() - 1 {
            let i0 = xy[i + 0];
            let i1 = xy[i + 1];
            if i0.1 != i1.1 {
                assert_eq!(i0.0, i1.0);
                // y has changed
                let y0 = if i0.1 < i1.1 { i0.1 } else { i1.1 };
                let y1 = if i0.1 < i1.1 { i1.1 } else { i0.1 };
                for y in y0..y1 + 1 {
                    world.insert((i0.0, y), '#');
                }
            } else if i0.0 != i1.0 {
                // x has changed
                assert_eq!(i0.1, i1.1);
                let x0 = if i0.0 < i1.0 { i0.0 } else { i1.0 };
                let x1 = if i0.0 < i1.0 { i1.0 } else { i0.0 };
                for x in x0..x1 + 1 {
                    world.insert((x, i0.1), '#');
                }
            } else {
                assert!(false);
            }
        }
    }
}


fn drop_sand(
    sandsource: &(i32, i32),
    min_dim: &(i32, i32),
    max_dim: &(i32, i32),
    world: &mut HashMap<(i32, i32), char>
) -> i32 {
    let mut sand_kernels = -1;
    let mut still_going = true;
    while still_going {
        let mut still_falling = true;
        let mut x = sandsource.0;
        let mut y = sandsource.1;
        let mut next_y = y + 1;
        sand_kernels += 1;
        while still_falling {
            if y > max_dim.1 {
                still_falling = false;
                still_going = false;
                println!("Infinite looping, sand will fall forever");
                draw_world(min_dim, max_dim, world);
            } else if !world.contains_key(&(x, next_y)) {
                y = next_y;
                next_y += 1;
            } else if !world.contains_key(&(x - 1, next_y)) {
                y = next_y;
                next_y += 1;
                x = x - 1;
            } else if !world.contains_key(&(x + 1, next_y)) {
                y = next_y;
                next_y += 1;
                x = x + 1;
            } else {
                // Stuck here, insert it.
                still_falling = false;
                if y == sandsource.1 {
                    // Totally stopped
                    still_going = false;
                } else {
                    world.insert((x, y), 'o');
                    draw_world(min_dim, max_dim, world);
                }
            }
        }
    }
    return sand_kernels;
}



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

        let mut world: HashMap<(i32, i32), char> = HashMap::new();
        let mut min_dim: (i32, i32) = (i32::MAX, i32::MAX);
        let mut max_dim: (i32, i32) = (i32::MIN, i32::MIN);

        {
            let vecs: Vec<Vec<(i32, i32)>> = extract_lines_to_vec(vec_in);
            fill_world(vecs, &mut min_dim, &mut max_dim, &mut world);
        }

        let sandsource = (500, 0);
        draw_world(&min_dim, &max_dim, &mut world);
        let sand_kernels = drop_sand(&sandsource, &min_dim, &max_dim, &mut world);
        println!("sand kernels: {}", sand_kernels);
        println!("");
    }

}


pub fn main() -> io::Result<()> {
    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("input.txt")
    };
    let data_bytes = std::fs::read_to_string(filename).unwrap();

    let vec_in: Vec<String> = utils::test_input_to_vec(data_bytes, true);
    assert_ne!(0, vec_in.len());

    let mut world: HashMap<(i32, i32), char> = HashMap::new();
    let mut min_dim: (i32, i32) = (i32::MAX, i32::MAX);
    let mut max_dim: (i32, i32) = (i32::MIN, i32::MIN);

    {
        let vecs: Vec<Vec<(i32, i32)>> = extract_lines_to_vec(vec_in);
        fill_world(vecs, &mut min_dim, &mut max_dim, &mut world);
    }

    let sandsource = (500, 0);
    draw_world(&min_dim, &max_dim, &mut world);
    let sand_kernels = drop_sand(&sandsource, &min_dim, &max_dim, &mut world);
    println!("sand kernels: {}", sand_kernels);
    println!("");    

    Ok(())
}