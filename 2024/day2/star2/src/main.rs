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
                // check if removing a level makes it safe
                else {
                    for i in 0..(int_nums.len()) {
                        let mut newVec: Vec<i32> = int_nums.clone();
                        newVec = modify_vec(newVec, i);
                        if (is_dec(&newVec) ^ is_inc(&newVec)) && has_safe_diff(&newVec) {
                            println!("state of the old vec {:?}", int_nums);

                            println!("state of the new vec {:?}", newVec);

                            println!("safe");
                            count_safe += 1;
                            break;
                        }
                    }
                }
                println!("this is the num of safe reports {}", count_safe);
            }
        };
    }
}

fn modify_vec(mut vec: Vec<i32>, removeInt: usize) -> Vec<i32> {
    vec.remove(removeInt);
    vec
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
    for i in 0..list.len() - 1 {
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
