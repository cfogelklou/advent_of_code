
fn snacks(v:Vec<String>)->(i64, i64){   
    let mut elves:Vec<i64> = Vec::new(); 
    let mut current_elf_sum:i64 = 0;

    for i in 0..v.len() {
        let next_line = v[i].to_string();
        // Parse, and handle error elegantly
        match next_line.trim().parse::<i64>(){
            Ok(this_snack) => {
                // Give this elf his calories
                current_elf_sum += this_snack;
            },
            Err(_) => {
                // New elf
                if current_elf_sum != 0 {
                    elves.push(current_elf_sum);
                }
                current_elf_sum = 0;
            }
        }
        
    }
    elves.sort();
    elves.reverse();
    let mut top_three_sum:i64 = 0;
    for i in 0..3 {
        println!("Checking elf {}", i);
        top_three_sum += elves[i];
    }
    
    return (top_three_sum, elves[0]);
}

fn main() {
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn t1() {
        test_example_1();
    }

}