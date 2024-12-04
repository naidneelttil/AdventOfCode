
use std::io;
fn main() {
   let mut count_safe : i32 = 0;

   loop {

      let mut line = String::new();
      match io::stdin().read_line(&mut line){
         Ok(0) => break,
         Err(_) => break,
         Ok(_) => {
         

           let mut nums  = line.trim().split_whitespace();            
           let mut next = nums.next(); 
           let mut int_nums : Vec<i32> = vec![];


            while next != None{
               int_nums.push(next.unwrap().parse().unwrap());
               next  = nums.next();
      
            }
            println!("state of the vec {:?}", int_nums); 
            
            if  ( is_dec(&int_nums, 0, true) ^ is_inc(&int_nums, 0, true) ) && has_safe_diff(&int_nums, 0, true){
                println!("safe");
                count_safe += 1;
            }

            // check if removing a level makes it safe
            else {

                for i in 0 .. int_nums.len() {
                  if  ( is_dec(&int_nums, i, false) ^ is_inc(&int_nums, i, false) ) && has_safe_diff(&int_nums, i, false) {
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


fn has_safe_diff (list:&Vec<i32>, skip:usize, valid_skip:bool) -> bool {
    for i in 0 .. list.len()-1 {
  if !valid_skip {println!("index we are skipping {}, and val {}", skip, list[skip]); }
        if valid_skip || i != skip {

        let mut skipReplace:usize = i+1;
        if (!valid_skip && skip == (i+1) && (i+2) < list.len() ) { skipReplace = i+2; }

        let diff : i32 = (list[i] - list[skipReplace]).abs(); 
        if diff != 1 && diff != 2 && diff != 3 { 
            println!("diff reported {}", diff); 
            return false; 
        } 
        }
    }

    return true;
}


fn is_dec(list:&Vec<i32>, skip:usize, valid_skip:bool) -> bool {
    for i in 0 .. list.len()-1 {
  if !valid_skip {println!("index we are skipping {}, and val {}", skip, list[skip]); }
       if valid_skip || i != skip {

        let mut skipReplace:usize = i+1;
        if (!valid_skip  && skip == (i+1)  && (i+2) < list.len() ) { skipReplace = i+2; }
        println!("i + 1 = {}, skip is {} skipReplace is {}", (i+1), skip, skipReplace );

        if list[i] <= list[skipReplace] {
          println!("dec reported {:?} and {:?}", list[i], list[i+1]); 

            return false; 
          
        } 
     } 
    }

    return true;
}

fn is_inc(list:&Vec<i32>, skip:usize, valid_skip:bool) -> bool {
 
    for i in 0 .. list.len()-1 {
  if !valid_skip {println!("index we are skipping {}, and val {}", skip, list[skip]); }
       if valid_skip || i != skip {

        let mut skipReplace:usize = i+1;
        if (!valid_skip  && skip == (i+1) && (i+2) < list.len()) { skipReplace = i+2; }

        println!("i + 1 = {}, skip is {} skipReplace is {}", (i+1), skip, skipReplace );


        if list[i] >= list[skipReplace] {
            println!("inc reported {:?} and {:?}", list[i], list[i+1]); 


            return false; } 
       }
    }

    return true;
}
