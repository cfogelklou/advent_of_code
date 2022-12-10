use std::io::{ self };
mod utils;

#[allow(dead_code)]
fn get_head_movement(v: &Vec<String>) -> i32 {
    return 0;
}

enum Instructions {
    Noop,
    AddX(i32),
}

struct Cpu {
    x: i32,
    clock: usize,
    hist_before: Vec<i32>,
    hist_after: Vec<i32>,
}

impl Cpu {
    fn new() -> Cpu {
        let cpu = Cpu {
            x: 1,
            clock: 0,
            hist_before: Vec::new(), // Stores the value before each cycle
            hist_after: Vec::new(), // Stores the value after each cycle
        };

        return cpu;
    }

    fn exec(&mut self, inst: Instructions) {
        let curr_clock = self.clock;
        match inst {
            Instructions::Noop => {
                self.hist_before.push(self.x);
                self.clock += 1;
                self.hist_after.push(self.x);
            }
            Instructions::AddX(num) => {
                self.hist_before.push(self.x);
                self.hist_before.push(self.x);
                self.clock += 2;
                self.hist_after.push(self.x);
                self.x += num;
                self.hist_after.push(self.x);
            }
        }
    }

    fn get_x_before(&self, ticks: usize) -> i32 {
        return if ticks < self.hist_before.len() { self.hist_before[ticks - 1] } else { 0 };
    }

    fn get_x_after(&self, ticks: usize) -> i32 {
        return if ticks < self.hist_after.len() { self.hist_after[ticks - 1] } else { 0 };
    }

    fn get_signal_strengths(&self, ticks:usize) -> (i32, i64) {
        let mut ss:i32 = 0;
        let mut ss_sum:i64 = 0;
        if (ticks >= self.hist_before.len()){
            return (ss, ss_sum);
        }        
        if ticks < 20 {
            return (ss, ss_sum);
        }
        let mut idx = 20;
        while idx <= ticks {
            let x = self.get_x_before(idx);
            ss = x * (idx as i32);
            ss_sum += (ss as i64);
            idx += 40;
        }

        return (ss, ss_sum);
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn cpu_test() {
        let raw_string: String = "noop
            addx 3
            addx -5".to_string();
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());

        let mut cpu = Cpu::new();
        for line in v {
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut action = Instructions::Noop;
            let mut x: i32 = 0;
            if words[0] == "addx" {
                x = words[1].parse::<i32>().unwrap();
                action = Instructions::AddX(x);
            }
            cpu.exec(action);
        }
        // One more exec
        cpu.exec(Instructions::Noop);

        // At the start of the first cycle, the noop instruction begins execution. During the first cycle, X is 1. After the first cycle, the noop instruction finishes execution, doing nothing.
        assert_eq!(cpu.hist_before[1 - 1], 1);
        assert_eq!(cpu.get_x_after(1), 1);

        // At the start of the second cycle, the addx 3 instruction begins execution. During the second cycle, X is still 1.
        assert_eq!(cpu.hist_before[2 - 1], 1);
        assert_eq!(cpu.get_x_after(2), 1);

        // During the third cycle, X is still 1. After the third cycle, the addx 3 instruction finishes execution, setting X to 4.
        assert_eq!(cpu.hist_before[3 - 1], 1);
        assert_eq!(cpu.get_x_after(3), 4);

        // At the start of the fourth cycle, the addx -5 instruction begins execution. During the fourth cycle, X is still 4.
        assert_eq!(cpu.hist_before[4 - 1], 4);
        assert_eq!(cpu.get_x_after(4), 4);

        // During the fifth cycle, X is still 4. After the fifth cycle, the addx -5 instruction finishes execution, setting X to -1.
        assert_eq!(cpu.hist_before[5 - 1], 4);
        assert_eq!(cpu.get_x_after(5), -1);
    }

    #[test]
    fn cpu_test_2() {
        let raw_string: String =
            "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop".to_string();
        let v: Vec<String> = utils::test_input_to_vec(raw_string, true);
        assert_ne!(0, v.len());

        let mut cpu = Cpu::new();
        for line in v {
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut action = Instructions::Noop;
            let mut x: i32 = 0;
            if words[0] == "addx" {
                x = words[1].parse::<i32>().unwrap();
                action = Instructions::AddX(x);
            }
            cpu.exec(action);
        }
        // One more exec
        cpu.exec(Instructions::Noop);
                
        assert_eq!(cpu.hist_before[20 - 1], 21);
        assert_eq!(cpu.get_x_before(20), 21);
        {
            let (ss, _ss_sum) = cpu.get_signal_strengths(20);
            assert_eq!(ss, 420);
        }

        assert_eq!(cpu.hist_before[60 - 1], 19);
        assert_eq!(cpu.get_x_before(60), 19);
        {
            let (ss, _ss_sum) = cpu.get_signal_strengths(60);
            assert_eq!(ss, 1140);
        }


        assert_eq!(cpu.hist_before[100 - 1], 18);
        assert_eq!(cpu.get_x_before(100), 18);
        {
            let (ss, _ss_sum) = cpu.get_signal_strengths(100);
            assert_eq!(ss, 1800);
        }

        assert_eq!(cpu.hist_before[140 - 1], 21);
        assert_eq!(cpu.get_x_before(140), 21);
        {
            let (ss, _ss_sum) = cpu.get_signal_strengths(140);
            assert_eq!(ss, 2940);
        }

        assert_eq!(cpu.hist_before[180 - 1], 16);
        assert_eq!(cpu.get_x_before(180), 16);
        {
            let (ss, _ss_sum) = cpu.get_signal_strengths(180);
            assert_eq!(ss, 2880);
        }

        assert_eq!(cpu.hist_before[220 - 1], 18);
        assert_eq!(cpu.get_x_before(220), 18);
        {
            let (ss, ss_sum) = cpu.get_signal_strengths(220);
            assert_eq!(ss, 3960);
            assert_eq!(ss_sum, 13140);
        }

        // During the 20th cycle, register X has the value 21, so the signal strength is 20 * 21 = 420.
        // (The 20th cycle occurs in the middle of the second addx -1, so the value of register X is the starting value, 1, plus all of the other addx values up to that point: 1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4 = 21.)
        // During the 60th cycle, register X has the value 19, so the signal strength is 60 * 19 = 1140.
        // During the 100th cycle, register X has the value 18, so the signal strength is 100 * 18 = 1800.
        // During the 140th cycle, register X has the value 21, so the signal strength is 140 * 21 = 2940.
        // During the 180th cycle, register X has the value 16, so the signal strength is 180 * 16 = 2880.
        // During the 220th cycle, register X has the value 18, so the signal strength is 220 * 18 = 3960.
    }
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

    /*
    let hm = get_head_movement(&v);
    assert_ne!(0, v.len());

    // Part 1
    if true {
        let tm = get_tail_movement(&hm);
        let mut unique_tm = tm.clone();
        unique_tm.sort();
        unique_tm.dedup();
        assert_eq!(6197, unique_tm.len());
        println!("Unique tail positions: {}", unique_tm.len());
    }

    // Part 2
    if true {
        let mut tails: Vec<Vec<(i32, i32)>> = Vec::new();
        let num_knots = 10;
        for i in 0..(num_knots - 1) {
            let my_head = if i == 0 {
                hm.clone()
            } else {
                tails[i - 1].clone()
            };
            let tm = get_tail_movement(&my_head);
            tails.push(tm);
        }

        let mut unique_tm = tails[tails.len() - 1].clone();
        unique_tm.sort();
        unique_tm.dedup();
        assert_eq!(2562, unique_tm.len());
        println!("Unique tail positions for part 2: {}", unique_tm.len());
    } */

    Ok(())
}