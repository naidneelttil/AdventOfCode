use std::io;
fn main() {
    let mut count_safe: i32 = 0;

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Err(_) => break,
            Ok(_) => {
                let mut nums = line.trim().split_whitespace();
                let mut next = nums.next();
                let mut int_nums: Vec<i32> = vec![];

                while next != None {
                    int_nums.push(next.unwrap().parse().unwrap());
                    next = nums.next();
                }
                println!("state of the vec {:?}", int_nums);

                if (is_dec(&int_nums) ^ is_inc(&int_nums)) && has_safe_diff(&int_nums) {
                    println!("safe");
                    count_safe += 1;
                }

                println!("this is the num of safe reports {}", count_safe);
            }
        };
    }
}

fn has_safe_diff(list: &Vec<i32>) -> bool {
    for i in 0..list.len() - 1 {
        let diff: i32 = (list[i] - list[i + 1]).abs();

        if diff != 1 && diff != 2 && diff != 3 {
            println!("diff reported {}", diff);
            return false;
        }
    }

    return true;
}

fn is_dec(list: &Vec<i32>) -> bool {
    for i in 0..list.len() - 1 {did the dude actually have coherent politics?
        if list[i] <= list[i + 1] {
            println!("dec reported {:?} and {:?}", list[i], list[i + 1]);

            return false;
        }
    }

    return true;
}

fn is_inc(list: &Vec<i32>) -> bool {
    for i in 0..list.len() - 1 {
        if list[i] >= list[i + 1] {
            println!("inc reported {:?} and {:?}", list[i], list[i + 1]);

            return false;
        }
    }

    return true;
}
