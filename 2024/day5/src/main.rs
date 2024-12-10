use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Manual {
    pages: HashMap<Vec<i32>, bool>,
}

#[derive(Debug)]
struct Rule {
    before: i32,
    base: i32,
}

impl Rule {
    fn apply(&self, data: &Vec<i32>) -> bool {
        let mut idx: usize = 0;
        let mut found_base: bool = false;
        let mut found_before: bool = false;

        for i in 0..data.len() {
            if data[i] == self.base {
                idx = i;
                found_base = true;
            }

            if data[i] == self.before {
                found_before = true;
            }
        }

        if !(found_base && found_before) {
            return true;
        }

        for i in 0..idx {
            if data[i] == self.before {
                return true;
            }
        }

        false
    }
}

fn main() {
    //   let mut line = String::new();
    let mut readpages: bool = false;
    let mut rules: Vec<Rule> = Vec::new();
    let mut manual: Manual = Manual {
        pages: HashMap::new(),
    };

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Err(_) => break,
            Ok(_) => {
                //read in and store the rules
                if !readpages {
                    println!("Raw line: {:?}", line);
                    let nums: Vec<&str> = line.trim().split('|').collect();

                    if nums.len() < 2 {
                        readpages = true;
                        continue;
                    }

                    println!("nums array {:?}", nums);
                    let ruletoadd: Rule = Rule {
                        before: nums[0].parse().unwrap(),
                        base: nums[1].parse().unwrap(),
                    };

                    rules.push(ruletoadd);
                } else {
                    let nums: Vec<&str> = line.trim().split(',').collect();
                    let mut i32_nums: Vec<i32> = Vec::new();
                    for i in 0..nums.len() {
                        i32_nums.push(nums[i].parse().unwrap());
                    }

                    manual.pages.insert(i32_nums, true);
                }
            }
        };
    }

    println!("printing out the rules {:?}", rules);
    println!("printing out the manual {:?}", manual);

    for i in 0..rules.len() {
        for (pages, is_valid) in manual.pages.iter_mut() {
            println!("the rule is {:?}", rules[i]);
            println!("the applied to page is {:?}", pages);

            println!(
                "the result of the rule applied is {}",
                rules[i].apply(pages)
            );
            *is_valid = *is_valid && rules[i].apply(pages);
        }
    }

    println!("printing out the manual {:?}", manual);

    let mut result: i32 = 0;
    for (pages, is_valid) in manual.pages.iter_mut() {
        if *is_valid {
            result += pages[pages.len() / 2];
        }
    }

    println!("result is {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {}
}
