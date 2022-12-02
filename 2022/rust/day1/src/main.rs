
use std::io::{self, BufReader};

fn snacks(v:Vec<String>)->(i64, i64){   
    let mut elves:Vec<i64> = Vec::new(); 
    let mut current_elf_sum:i64 = 0;

    // A lesson in mutating variables.
    fn create_new_elf_if_nonzero_sum(sum:&mut i64, elves:&mut Vec<i64>){
        if *sum != 0 {
            elves.push(*sum);
        }
        *sum = 0;
    }

    for next_line in v.iter() {

        // Parse, and handle error elegantly
        match next_line.trim().parse::<i64>(){
            
            Ok(this_snack) => {
                // Give this elf his calories
                current_elf_sum += this_snack;
            },
            Err(_) => {
                create_new_elf_if_nonzero_sum(&mut current_elf_sum, &mut elves);
            }
        }        
    }

    // After last line, check if there is still an elf processing.
    create_new_elf_if_nonzero_sum(&mut current_elf_sum, &mut elves);

    elves.sort();
    elves.reverse();
    let top_three_sum:i64 = elves[0..3].iter().sum();    
    return (top_three_sum, elves[0]);
}

fn main()  -> io::Result<()> {
    use std::io::BufRead;

    let filename = std::env::args().nth(1).expect("Expected filename");
    let file = std::io::BufReader::new(
        std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(
            &filename,
        ))
        .unwrap(),
    );
    let mut v:Vec<String> = Vec::new();
    for (_, line) in file.lines().enumerate() {    
        let l:String = line.unwrap();
        v.push(l);
    }    
    let (s, i) = snacks(v.clone());
    println!("The top three elves carry {}", s);
    println!("The elf's total calories are {}", i);
    Ok(())
}

// The test case given the samples from AoC.
#[allow(dead_code)]
fn test_example_1() {
    let v:Vec<String> = vec![
        "1000".to_string(),
        "2000".to_string(),
        "3000".to_string(),
        "".to_string(),
        "4000".to_string(),
        "".to_string(),
        "5000".to_string(),
        "6000".to_string(),
        "".to_string(),
        "7000".to_string(),
        "8000".to_string(),
        "9000".to_string(),
        "".to_string(),
        "10000".to_string(),
        "".to_string()
    ];

    let (s, i) = snacks(v.clone());
    println!("The top three elves have {}", s);
    println!("The elf's total calories are {}", i);
    assert_eq!(s, 45000);
    assert_eq!(i, 24000);
}


#[allow(dead_code)]
fn test_example_2() {
    use std::io::BufRead;
    let raw_string = "
    1000\n\
    2000\n\
    3000\n\
    \n\
    4000\n\
    \n\
    5000\n\
    6000\n\
    \n\
    7000\n\
    8000\n\
    9000\n\
    \n\
    10000";

    let b = BufReader::new(raw_string.as_bytes());
    let mut v:Vec<String> = Vec::new();
    for (_, line) in b.lines().enumerate() {    
        let l:String = line.unwrap().trim().to_string();
        v.push(l);
    }  
    
    let (s, i) = snacks(v.clone());
    println!("The top three elves have {}", s);
    println!("The elf's total calories are {}", i);
    assert_eq!(s, 45000);
    assert_eq!(i, 24000);
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn t1() {
        test_example_1();
    }

    #[test]
    fn t2() {
        test_example_2();
    }

}