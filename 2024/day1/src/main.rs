


use std::io;

fn main() {
    println!("Hello, world!");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    //let mut holdingSpace: Vec<i32> = Vec::new();

    loop {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read");
   
    let nums = line.trim().split(' ').collect::<Vec<_>>();
   
     if nums.len() == 1 { break; }
     println!("print nums atp {:?}", nums); 
     left.push(nums.get(0).expect("str num").parse().unwrap());
     right.push(nums.get(3).expect("str num").parse().unwrap());
    
   /* let leftadd: i32 = match nums.get(0).expect("str num").parse().unwrap() {
         Ok(numstr) => numstr,
         Err(_) => continue,

    };

    let rightadd: i32 = match nums.get(1).expect("str num").parse().unwrap() {
         Ok(numstr) => numstr,
         Err(_) => continue,

    };
    
    left.push(leftadd);
    right.push(rightadd);
   */
    left.sort();
    right.sort();

    println!("array of stuff left {:?}", left);
    println!("array of stuff left {:?}", right);

    let mut sumDist: i32 = 0;
    for i in 0 ..left.len(){
         sumDist += (left.get(i).unwrap() - right.get(i).unwrap()).abs();
println!("result of subtraction: {}", (left.get(i).unwrap() - right.get(i).unwrap().abs()));
    }
    println!("sum of all the distances {}", sumDist);
    
    }
}
