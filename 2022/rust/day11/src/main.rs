use std::{ io::{ self }, collections::VecDeque };
mod utils;

enum MonkeyOperations {
    Squared,
    Mul,
    Add,
}

struct Monkey {
    items: VecDeque<i64>,
    divisible: i64,
    throw_targets: (i32, i32),
    op: MonkeyOperations,
    op_param: i32,
    inspections: i64,
}

impl Monkey {
    fn new(v: &Vec<String>) -> Monkey {
        let mut monkey = Monkey {
            items: VecDeque::new(),
            divisible: 0,
            throw_targets: (-1, -1),
            op: MonkeyOperations::Mul,
            op_param: -1,
            inspections: 0,
        };

        for line in v {
            let words: Vec<&str> = line.split_whitespace().collect();
            if "Starting" == words[0] {
                for i in 2..words.len() {
                    let x = utils::robust_to_int(words[i]) as i64;
                    monkey.catch(x);
                }
            } else if "Operation:" == words[0] {
                if line.contains("+") {
                    monkey.op = MonkeyOperations::Add;
                    monkey.op_param = utils::robust_to_int(words[5]);
                } else if "old" == words[5] {
                    monkey.op = MonkeyOperations::Squared;
                    monkey.op_param = 1;
                } else {
                    monkey.op = MonkeyOperations::Mul;
                    monkey.op_param = utils::robust_to_int(words[5]);
                }
            } else if "Test:" == words[0] {
                let x = utils::robust_to_int(words[3]);
                assert!(monkey.divisible == 0);
                monkey.divisible = x  as i64;
            } else if "If" == words[0] {
                let target_monkey = utils::robust_to_int(words[5]);
                if "true:" == words[1] {
                    assert!(monkey.throw_targets.0 < 0);
                    monkey.throw_targets.0 = target_monkey;
                } else {
                    assert!(monkey.throw_targets.1 < 0);
                    monkey.throw_targets.1 = target_monkey;
                }
            }
        }
        assert!(monkey.divisible > 0);
        assert!(monkey.throw_targets.0 >= 0);
        assert!(monkey.throw_targets.1 >= 0);
        assert!(monkey.items.len() > 0);
        assert!(monkey.op_param > 0);
        return monkey;
    }

    fn catch(&mut self, item: i64) {
        //self.items.push_back(item);
        self.items.push_back(item);
    }

    fn get_items(&self) -> Vec<i64> {
        let x: Vec<i64> = self.items
            .iter()
            .map(|val| {
                let v: i64 = *val;
                return v;
            })
            .collect();

        return x;
    }

    fn do_operations(&mut self, really_worried:bool) -> (i64, usize) {
        let mut worry_level: i64 = -1;
        let mut target_monkey: usize = 100000;
        if self.items.len() > 0 {
            let new_item = self.items.pop_front().unwrap() as i64;
            println!("  Monkey inspects an item with a worry level of {}.", new_item);
            self.inspections += 1;

            worry_level = match self.op {
                MonkeyOperations::Mul => {
                    let res = new_item * self.op_param as i64;
                    println!("    Worry level is multiplied by {} to {}", self.op_param, res);
                    res
                }
                MonkeyOperations::Add => {
                    let res = new_item + self.op_param as i64;
                    println!("    Worry level increases by {} to {}.", self.op_param, res);
                    res
                }
                MonkeyOperations::Squared => {
                    let res = new_item * new_item;
                    println!("    Worry level is multiplied by itself to {}.", res);
                    res
                }
            };
            if !really_worried {
                worry_level = worry_level / 3;
            }
            println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level);
            let divisible = worry_level % (self.divisible as i64) == 0;
            if divisible {
                println!("    Current worry level is divisible by {}.", self.divisible);
            } else {
                println!("    Current worry level is not divisible by {}.", self.divisible);
            }

            let tm = if divisible { self.throw_targets.0 } else { self.throw_targets.1 };
            target_monkey = tm as usize;

            println!(
                "    Item with worry level {worry_level} is thrown to monkey {target_monkey}."
            );
        }

        return (worry_level, target_monkey);
    }
}

fn load_vector_into_monkeys(v: Vec<String>, monkeys: &mut Vec<Monkey>) {
    let mut i = 0;
    while i < v.len() {
        if i >= v.len() {
            break;
        }
        let line = v[i].clone();
        i += 1;
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() > 0 {
            if "Monkey" == words[0] {
                let new_vec: Vec<String> = v[i..i + 5].to_vec().clone();
                i += 5;
                let monkey = Monkey::new(&new_vec);
                monkeys.push(monkey);
            }
        }
    }
}

fn get_top_two_monkey_inspections(monkeys: Vec<Monkey>) -> Vec<i64> {
    let mut inspections: Vec<i64> = monkeys
        .iter()
        .map(|m| {
            return m.inspections;
        })
        .collect();
    inspections.sort();
    inspections.reverse();
    inspections
}

fn monkey_business_test(v: Vec<String>, rounds:i32, really_worried:bool) ->i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    load_vector_into_monkeys(v, &mut monkeys);
    for _rounds in 0..rounds {
        for m_idx in 0..monkeys.len() {
            println!("Monkey {}:", m_idx);
            let mut is_done: bool = false;
            while !is_done {
                let m = monkeys.get_mut(m_idx).unwrap();
                let (wl, tm) = m.do_operations(really_worried);
                if wl < 0 {
                    is_done = true;
                } else {
                    let t = monkeys.get_mut(tm).unwrap();
                    t.catch(wl);
                }
            }
        }
    }
    let inspections = get_top_two_monkey_inspections(monkeys);
    let monkey_business = inspections[0] * inspections[1];
    println!("Monkey business = {monkey_business}");
    return monkey_business;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn monkey_biz() {
        let raw_string: String =
            "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3
      
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
      
      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3
      
      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1".to_string();
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());

        let mut monkeys: Vec<Monkey> = Vec::new();

        load_vector_into_monkeys(v.clone(), &mut monkeys);

        assert_eq!(monkeys[0].get_items(), [79, 98].to_vec());
        assert_eq!(monkeys[1].get_items(), [54, 65, 75, 74].to_vec());
        assert_eq!(monkeys[2].get_items(), [79, 60, 97].to_vec());
        assert_eq!(monkeys[3].get_items(), [74].to_vec());

        for _rounds in 0..20 {
            for m_idx in 0..monkeys.len() {
                println!("Monkey {}:", m_idx);
                let mut is_done: bool = false;
                while !is_done {
                    let m = monkeys.get_mut(m_idx).unwrap();
                    let (wl, tm) = m.do_operations(false);
                    if wl < 0 {
                        is_done = true;
                    } else {
                        //if tm != m_idx {
                        let t = monkeys.get_mut(tm).unwrap();
                        t.catch(wl);
                        //}
                    }
                }
            }
            match _rounds {
                0 => {
                    assert_eq!(monkeys[0].get_items(), [20, 23, 27, 26].to_vec());
                    assert_eq!(monkeys[1].get_items(), [2080, 25, 167, 207, 401, 1046].to_vec());
                    assert_eq!(monkeys[2].get_items(), [].to_vec());
                    assert_eq!(monkeys[3].get_items(), [].to_vec());
                }
                1 => {
                    assert_eq!(monkeys[0].get_items(), [695, 10, 71, 135, 350].to_vec());
                    assert_eq!(monkeys[1].get_items(), [43, 49, 58, 55, 362].to_vec());
                    assert_eq!(monkeys[2].get_items(), [].to_vec());
                    assert_eq!(monkeys[3].get_items(), [].to_vec());
                }
                19 => {
                    assert_eq!(monkeys[0].get_items(), [10, 12, 14, 26, 34].to_vec());
                    assert_eq!(monkeys[1].get_items(), [245, 93, 53, 199, 115].to_vec());
                    assert_eq!(monkeys[2].get_items(), [].to_vec());
                    assert_eq!(monkeys[3].get_items(), [].to_vec());
                    assert_eq!(monkeys[0].inspections, 101);
                    assert_eq!(monkeys[1].inspections, 95);
                    assert_eq!(monkeys[2].inspections, 7);
                    assert_eq!(monkeys[3].inspections, 105);
                }
                _ => {}
            }
        }

        let inspections = get_top_two_monkey_inspections(monkeys);

        let monkey_business = inspections[0] * inspections[1];
        println!("Monkey business = {monkey_business}");
        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn monkey_biz_2() {
        let raw_string: String =
            "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3
      
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
      
      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3
      
      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1".to_string();
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());

        if true {
            let mb = monkey_business_test(v, 10000, true);
            assert_eq!(mb, 2713310158);
        }
    }

    //}
}

fn main() -> io::Result<()> {
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

    // Part 1
    if true {
        monkey_business_test(v.clone(), 20, false);
    }

    // Part 2
    if false {
        monkey_business_test(v.clone(), 10000, true);
    }

    //assert_eq!(monkey_business, 10605);
    Ok(())
}





