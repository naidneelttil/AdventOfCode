struct Rule {
    before: i32,
    base: i32,
}

impl Rule {
    fn apply(&self, data: &Vec<i32>) -> bool {
        let mut idx: usize = 0;

        for i in 0..data.len() {
            if data[i] == self.base {
                idx = i;
                continue;
            }
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
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {}
}
