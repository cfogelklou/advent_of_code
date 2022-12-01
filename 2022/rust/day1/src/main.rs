
fn snacks(v:Vec<String>)->(i64, i64){    
    let mut current_elf_idx = 0;
    let mut current_elf_sum:i64 = 0;
    let mut richest_elf_idx = -1;
    let mut richest_elf_sum = -1;

    for i in 0..v.len() {
        let next_line = v[i].to_string();
        let this_snack:i64 = if next_line.len() > 0 { next_line.parse::<i64>().unwrap() } else { 0 };
        
        // Give this elf his calories
        current_elf_sum += this_snack;

        // If this snack had no calories, then push the current snack onto this elf
        if this_snack == 0 {
            // New elf
            if current_elf_sum != 0 {
                if current_elf_sum > richest_elf_sum {
                    richest_elf_sum = current_elf_sum;
                    richest_elf_idx = current_elf_idx;
                }
                current_elf_idx += 1;
            }
            current_elf_sum = 0;
        }
    }
    
    return (richest_elf_idx + 1, richest_elf_sum);
}


fn main() {
    use std::io::BufRead;

    if true {
        let filename = std::env::args().nth(1).expect("Expected filename");
        let file = std::io::BufReader::new(
            std::fs::File::open(<String as AsRef<std::path::Path>>::as_ref(
                &filename,
            ))
            .unwrap(),
        );
        let mut v:Vec<String> = Vec::new();
        for (_, line) in file.lines().enumerate() {
            //println!("y = {}, line = {}", y, line.as_ref().unwrap());
            let l:String = line.unwrap();
            v.push(l);
        }    
        let (s, i) = snacks(v.clone());
        println!("The elf with the most calories is {}", s);
        println!("The elf's total calories are {}", i);
    }
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
    println!("The elf with the most calories is {}", s);
    println!("The elf's total calories are {}", i);
    assert_eq!(s, 4);
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