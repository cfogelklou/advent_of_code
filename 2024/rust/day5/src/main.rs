/*
--- Day 5: Print Queue ---
Satisfied with their search on Ceres, the squadron of scholars suggests subsequently scanning the stationery stacks of sub-basement 17.

The North Pole printing department is busier than ever this close to Christmas, and while The Historians continue their search of this historically significant facility, an Elf operating a very familiar printer beckons you over.

The Elf must recognize you, because they waste no time explaining that the new sleigh launch safety manual updates won't print correctly. Failure to update the safety manuals would be dire indeed, so you offer your services.

Safety protocols clearly indicate that new pages for the safety manuals must be printed in a very specific order. The notation X|Y means that if both page number X and page number Y are to be produced as part of an update, page number X must be printed at some point before page number Y.

The Elf has for you both the page ordering rules and the pages to produce in each update (your puzzle input), but can't figure out whether each update has the pages in the right order.

For example:

47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
The first section specifies the page ordering rules, one per line. The first rule, 47|53, means that if an update includes both page number 47 and page number 53, then page number 47 must be printed at some point before page number 53. (47 doesn't necessarily need to be immediately before 53; other pages are allowed to be between them.)

The second section specifies the page numbers of each update. Because most safety manuals are different, the pages needed in the updates are different too. The first update, 75,47,61,53,29, means that the update consists of page numbers 75, 47, 61, 53, and 29.

To get the printers going as soon as possible, start by identifying which updates are already in the right order.

In the above example, the first update (75,47,61,53,29) is in the right order:

75 is correctly first because there are rules that put each other page after it: 75|47, 75|61, 75|53, and 75|29.
47 is correctly second because 75 must be before it (75|47) and every other page must be after it according to 47|61, 47|53, and 47|29.
61 is correctly in the middle because 75 and 47 are before it (75|61 and 47|61) and 53 and 29 are after it (61|53 and 61|29).
53 is correctly fourth because it is before page number 29 (53|29).
29 is the only page left and so is correctly last.
Because the first update does not include some page numbers, the ordering rules involving those missing page numbers are ignored.

The second and third updates are also in the correct order according to the rules. Like the first update, they also do not include every page number, and so only some of the ordering rules apply - within each update, the ordering rules that involve missing page numbers are not used.

The fourth update, 75,97,47,61,53, is not in the correct order: it would print 75 before 97, which violates the rule 97|75.

The fifth update, 61,13,29, is also not in the correct order, since it breaks the rule 29|13.

The last update, 97,13,75,29,47, is not in the correct order due to breaking several rules.

For some reason, the Elves also need to know the middle page number of each update being printed. Because you are currently only printing the correctly-ordered updates, you will need to find the middle page number of each correctly-ordered update. In the above example, the correctly-ordered updates are:

75,47,61,53,29
97,61,53,29,13
75,29,13
These have middle page numbers of 61, 53, and 29 respectively. Adding these page numbers together gives 143.

Of course, you'll need to be careful: the actual list of page ordering rules is bigger and more complicated than the above example.

Determine which updates are already in the correct order. What do you get if you add up the middle page number from those correctly-ordered updates?

*/

/*
    Chris Solution:
    1. Parse the input into a list of tuples
    3. Create a hashmap of the pages
        Each element will have a list of pages that it must come before
    4. Do an insertion sort on the list of pages
        For each page, check if it is in the hashmap
        If it is, check if the pages it must come before are in the list
        If they are, insert the page before them
        If they are not, insert the page at the end

*/
use std::collections::HashMap;
#[allow(unused_imports)]
use std::ffi::CString;
use std::io::{self};
//use std::array;
mod utils;

#[allow(dead_code)]
#[derive(Debug, Default)]
struct PageRules {
    pages_before_me: Vec<i32>,
    pages_after_me: Vec<i32>,
}

#[allow(dead_code)]
type PageMap = HashMap<i32, PageRules>;

// Optional helper methods
#[allow(dead_code)]
impl PageRules {
    fn new() -> Self {
        PageRules {
            pages_before_me: Vec::new(),
            pages_after_me: Vec::new(),
        }
    }

    fn add_page_before_me(&mut self, page: i32) {
        self.pages_before_me.push(page);
    }

    fn add_page_after_me(&mut self, page: i32) {
        self.pages_after_me.push(page);
    }
}

#[allow(dead_code)]
fn sorted_insert_before(pages: &mut Vec<i32>, page: i32, insert_before: i32) {
    let mut insert_index = pages.len();
    for (i, p) in pages.iter().enumerate() {
        if *p == insert_before {
            insert_index = i;
            break;
        }
    }
    pages.insert(insert_index, page);
}

#[allow(dead_code)]
fn sorted_insert_after(pages: &mut Vec<i32>, page: i32, insert_after: i32) {
    let mut insert_index = pages.len();
    for (i, p) in pages.iter().enumerate() {
        if *p == insert_after {
            insert_index = i + 1;
            break;
        }
    }
    pages.insert(insert_index, page);
}

#[allow(dead_code)]
fn parse_rules(rule_bytes: &str) -> Vec<(i32, i32)> {
    let rules_vec = utils::test_input_to_vec(rule_bytes.to_string(), true);
    let mut rules: Vec<(i32, i32)> = Vec::new();
    for rule in rules_vec {
        let rule_vec: Vec<&str> = rule.split("|").collect();
        let rule1 = rule_vec[0].parse::<i32>().unwrap();
        let rule2 = rule_vec[1].parse::<i32>().unwrap();
        rules.push((rule1, rule2));
    }
    rules
}

#[allow(dead_code)]
fn build_rule_map(rules: Vec<(i32, i32)>) -> PageMap {
    let mut rule_map: PageMap = HashMap::new();
    for rule in rules {
        rule_map
            .entry(rule.0)
            .or_insert_with(PageRules::new)
            .add_page_before_me(rule.1);
        rule_map
            .entry(rule.1)
            .or_insert_with(PageRules::new)
            .add_page_after_me(rule.0);
    }
    rule_map
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse() {
        println!("Testing parse");
        let rule_bytes = String::from(
            "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13",
        );

        println!("rule_bytes: {}", rule_bytes);
        let rules = parse_rules(&rule_bytes);
        println!("rules: {:?}", rules);

        let rule_map = build_rule_map(rules);

        let pages_bytes = String::from(
            "75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47",
        );
        let pages = utils::test_comma_delimited_input_to_int_vec(pages_bytes, true);
        let mut sorted_pages: Vec<Vec<i32>> = Vec::new();
        let mut correct_pages: Vec<bool> = Vec::new();
        for _page in pages {
            let mut sorted_page = _page.clone();
            let mut correct = true;
            // Compare each element in the page with the elements after it to
            // see if they are in the right order if the element
            // after it is in the comes_before list, remove it from its current
            // location and insert it before the current element
            let mut sorted = false;
            let mut iterations = 0;
            while !sorted {
                sorted = true;
                iterations += 1;
                for index in 0..sorted_page.len() {
                    let current_page = sorted_page[index];
                    for j in index + 1..sorted_page.len() {
                        let next_page = sorted_page[j];
                        if rule_map.contains_key(&next_page) {
                            if rule_map[&current_page].pages_after_me.contains(&next_page) {
                                correct = false;
                                println!("{} must come before {}", next_page, current_page);
                                // Remove the element from the list and insert it after the current
                                // element
                                sorted_page.remove(index);
                                sorted_insert_after(&mut sorted_page, current_page, next_page);
                                sorted = false;
                                break;
                            }
                        }
                    }
                    if !sorted {
                        break;
                    }
                }
                // Print iterations and current list of pages
                println!("Iterations: {}", iterations);
                println!("{:?}", sorted_page);
            }
            sorted_pages.push(sorted_page);
            correct_pages.push(correct);
        }

        println!("sorted_pages: {:?}", sorted_pages);
        println!("correct_pages: {:?}", correct_pages);
        assert_eq!(correct_pages, vec![true, true, true, false, false, false]);

        // Print the middle page of each correct page
        let mut middle_pages: Vec<i32> = Vec::new();
        for i in 0..sorted_pages.len() {
            if correct_pages[i] {
                let middle_index = sorted_pages[i].len() / 2;
                middle_pages.push(sorted_pages[i][middle_index]);
            }
        }

        println!("middle_pages: {:?}", middle_pages);

        // Sum the middle pages
        let mut sum = 0;
        for page in middle_pages {
            sum += page;
        }
        println!("Sum: {}", sum);
        assert_eq!(sum, 143);
    }
}

pub fn main() -> io::Result<()> {
    let filename = if std::env::args().len() >= 2 {
        std::env::args().nth(1).unwrap()
    } else {
        String::from("day3/input.txt")
    };
    let data_bytes = std::fs::read_to_string(filename).unwrap();
    println!("{}", data_bytes);

    Ok(())
}
