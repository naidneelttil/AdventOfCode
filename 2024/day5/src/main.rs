use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct Rule {
    before: i32,
    base: i32,
}

impl Rule {
    fn apply(&self, data: &Vec<i32>) -> bool {
        let idx: usize;
        (idx, _) = self.get_idxs(data);
        let found_base_before = self.has_base_and_before(data);

        if !(found_base_before) {
            return true;
        }

        for i in 0..idx {
            if data[i] == self.before {
                return true;
            }
        }

        false
    }

    fn has_base_and_before(&self, data: &Vec<i32>) -> bool {
        let mut found_base: bool = false;
        let mut found_before: bool = false;

        for i in 0..data.len() {
            if data[i] == self.base {
                found_base = true;
            }

            if data[i] == self.before {
                found_before = true;
            }
        }

        return found_base && found_before;
    }

    fn get_idxs(&self, data: &Vec<i32>) -> (usize, usize) {
        let mut idx: usize = 0;
        let mut idx2: usize = 0;
        for i in 0..data.len() {
            if data[i] == self.base {
                idx = i;
            }

            if data[i] == self.before {
                idx2 = i;
            }
        }

        return (idx, idx2);
    }
}

fn main() {
    //   let mut line = String::new();
    let mut readpages: bool = false;
    let mut rules: Vec<Rule> = Vec::new();
    let mut manuals: Vec<Vec<i32>> = Vec::new();

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

                    manuals.push(i32_nums);
                }
            }
        };
    }

    println!("1 printing out the rules {:?}", rules);
    println!("1 printing out the manual {:?}", manuals);

    // get a vec of is valid
    let mut is_valid: Vec<bool> = Vec::new();
    for i in 0..manuals.len() {
        let mut bool_vec: bool = true;
        for rule in rules.iter() {
            bool_vec = rule.apply(&manuals[i]) && bool_vec;
        }
        is_valid.push(bool_vec);
    }

    for i in 0..manuals.len() {
        println!("manual {} {:?}", i, manuals[i]);
    }

    /*  nightly code
        manuals
            .extract_if(|x| validate(rules, x))
            .collect::<Vec<_>>();
    */
    let mut i = 0;
    while i < manuals.len() {
        if validate(&rules, &mut manuals[i]) {
            let val = manuals.remove(i);
            // your code here
        } else {
            i += 1;
        }
    }

    println!("2printing out the manual {:?}", manuals);

    //mutate(&mut manuals, &rules);

    for manual in manuals.iter_mut() {
        //let vecCopy: Vec<i32> = manuals[i].clone();
        manual.sort_by(|a, b| cmp(*a, *b, &rules));
    }
    /////// debugging //////////////////////////
    println!("3printing out the manual {:?}", manuals);
    let mut is_valid: Vec<bool> = Vec::new();
    for i in 0..manuals.len() {
        let mut bool_vec: bool = true;
        for rule in rules.iter() {
            bool_vec = rule.apply(&manuals[i]) && bool_vec;
        }
        is_valid.push(bool_vec);
    }
    println!("2printing out the is valid vec {:?}", is_valid);
    //////////////////////////////////////////////

    let mut result: i32 = 0;

    for manual in manuals {
        result += manual[manual.len() / 2];
    }

    println!("this is the result: {}", result);
}

fn validate(rules: &Vec<Rule>, manual: &Vec<i32>) -> bool {
    let mut ret: bool = true;
    for rule in rules {
        ret = ret && rule.apply(manual);
    }
    ret
}

fn cmp(b: i32, a: i32, rules: &Vec<Rule>) -> Ordering {
    let mut idx_base: usize;
    let mut idx_before: usize;

    for rule in rules {
        if rule.before == a && rule.base == b {
            return Ordering::Equal;
        }
        if rule.before == b && rule.base == a {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

fn mutate(manuals: &mut Vec<Vec<i32>>, rules: &Vec<Rule>) {
    //let mut change: Vec<i32> = manuals[1].clone();
    let mut idx_base: usize;
    let mut idx_before: usize;
    for vec in manuals {
        for i in 0..rules.len() {
            // if the rule doesn't work on the vec, then modify
            if !rules[i].apply(vec) {
                (idx_base, idx_before) = rules[i].get_idxs(vec);
                let removed: i32 = vec.remove(idx_before);
                // vec.insert(idx_before, -1);
                let mut j: usize = 0;
                let mut valid: bool = true;

                while j < idx_base {
                    vec.insert(j, removed);

                    println!("this is the manual mutated {:?}", vec);
                    for k in 0..i + 1 {
                        valid = valid && rules[k].apply(vec);
                    }

                    if valid {
                        println!("this is the valid manual mutated {:?}", vec);
                        break;
                    } else {
                        vec.remove(j);
                        valid = true;
                    }
                    j += 1;
                }
                // if idx_base == 0 {
                //     vec.insert(0, removed);
                // } else {
                //     vec.insert(idx_base - 1, removed);
                // }
            }
        }
    }
}
