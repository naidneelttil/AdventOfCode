use std::io;

fn main() {
    println!("Hello, world!");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    //let mut holdingSpace: Vec<i32> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read");

        let nums = line.trim().split(' ').collect::<Vec<_>>();

        if nums.len() == 1 {
            break;
        }
        println!("print nums atp {:?}", nums);
        left.push(nums.get(0).expect("str num").parse().unwrap());
        right.push(nums.get(3).expect("str num").parse().unwrap());

        left.sort();
        right.sort();

        println!("array of stuff left {:?}", left);
        println!("array of stuff left {:?}", right);

        let mut complexity: i32 = 0;
        for i in 0..left.len() {
            let mut samenum = 0;
            for j in 0..right.len() {
                if left.get(i).unwrap() == right.get(j).unwrap() {
                    samenum += 1;
                }
            }
            complexity += left.get(i).unwrap() * samenum;
            println!(
                "left val is {}, num of time it appears is {}",
                left.get(i).unwrap(),
                complexity
            );
        }
        println!("resulting complexity {}", complexity);
    }
}
