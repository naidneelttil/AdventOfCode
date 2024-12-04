use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data: Vec<u8> = fs::read("input.txt")?;
    println!("this is what the vec looks like {:?}", data);
    
    let mut result :i32 = 0; 
    let mut first_digit:i32 ;
    let mut sec_digit: i32 ;
    let mut idx : usize = 0;

    while idx < data.len(){
       //find the mul(
       idx = find_mul(&data, idx);   
       if idx == data.len(){ continue;}
       println!("this is the position of char after mul( { }, and the char there { }", idx, data[idx] );


       (first_digit, idx) = find_first_digit(&data, idx);
       if first_digit == -1  {continue;}
       println!("this is the first digit found { } and idx {}", first_digit, idx );


       (sec_digit, idx) = find_second_digit(&data, idx);
       println!("this is the index, no sec digit yet {}", idx);
        if sec_digit == -1  {continue;}
        println!("this is the second digit found { }", sec_digit );
        println!("this is the length of the data {}", data.len());

        result += sec_digit * first_digit;

    }

    println!("the result is {}", result);
    Ok(())
}



// in the data vec, given a starting point, retun the index of the char after 
// a complete 'mul(' which is 109, 117, 108, 40 in ascii 
fn find_mul(data:&Vec<u8>, mut start:usize) -> usize {
      // s s1 s2 s3
      while  start + 3 < data.len() {
          if  data[start] == 109 && data[start + 1]==117 && data[start + 2] ==108 && data[start + 3] ==40 {
              return start + 4;
          }
          start += 1;
      }

      return data.len();
}

fn find_first_digit(data:&Vec<u8>,  mut start:usize) -> (i32, usize) {
   
      let mut num:i32 = 0; 
      let mut pow:i32 = 0;
      let origin:usize = start.clone();
      
      while  start < data.len() {
          if (data[start] as char).is_ascii_digit() {
              start += 1;
              continue;
          }
          else if data[start] == 44 && start != origin  { 
              let mut idx = start - 1; 
              while idx >= origin {
                  num += (data[idx] as i32 - 48) *(i32::pow(10, pow as u32 ) );
                  pow += 1;
                  idx -= 1;
              }

              return  (num, start + 1);


          }
          
          else{ return (-1, start); }
       
      }

      (-1, start)
      
}

fn find_second_digit(data:&Vec<u8>,  mut start:usize) -> (i32, usize) {
   
      let mut num:i32 = 0; 
      let mut pow:i32 = 0;
      let origin:usize = start.clone();
      
      while  start < data.len() {
          if (data[start] as char).is_ascii_digit() {
              start += 1;
              continue;
          }
          else if data[start] == 41 && start != origin  { 
              let mut idx = start - 1; 
              while idx >= origin {
                  num += (data[idx] as i32 - 48) *(i32::pow(10, pow as u32 ) );
                  pow += 1;
                  idx -= 1;
              }

              return  (num, start + 1);


          }
          
          else{ return (-1, start); }
       
      }

      (-1, start)
      
}
