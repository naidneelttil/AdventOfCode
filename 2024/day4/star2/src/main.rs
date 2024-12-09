
use std::fs;

struct Data {
    line :Vec<Vec<u8>>,
    xlen :usize,
}
impl Data {
    fn msms(&self, x:usize, y:usize) -> i32 {
        let mut num_xmas:i32 = 0;

       let y_inrange:bool = y + 1 <self.line.len() && y >= 1;
       let x_inrange:bool = x + 1 <self.xlen && x >= 1;


        // is there space for xmas?
        if y_inrange && x_inrange  {

                

            if self.line[y][x] == 'A' as u8 && self.line[y-1][x-1] == 'M' as u8 && self.line[y+1][x-1] == 'M' as u8 { 
 
                if self.line[y+1][x + 1] == 'S' as u8  &&  self.line[y-1][x + 1] == 'S' as u8{
                
                    println!("msms at {} x {}", x, y);
     
                    num_xmas +=1;
                } 
               
    
            }  
        }
        return num_xmas;
    } 

     fn ssmm(&self, x:usize, y:usize) -> i32 {
        let mut num_xmas:i32 = 0;

       let y_inrange:bool = y + 1 <self.line.len() && y >= 1;
       let x_inrange:bool = x + 1 <self.xlen && x >= 1;


        // is there space for xmas?
        if y_inrange && x_inrange  {

                

            if self.line[y][x] == 'A' as u8 && self.line[y-1][x-1] == 'S' as u8 && self.line[y+1][x-1] == 'M' as u8 { 
 
                if self.line[y+1][x + 1] == 'M' as u8  &&  self.line[y-1][x + 1] == 'S' as u8{
                
                    println!("ssmm at {} x {}", x, y);
     
                    num_xmas +=1;
                } 
               
     
            }  
        }
        return num_xmas;
    } 

      fn smsm(&self, x:usize, y:usize) -> i32 {
        let mut num_xmas:i32 = 0;

       let y_inrange:bool = y + 1 <self.line.len() && y >= 1;
       let x_inrange:bool = x + 1 <self.xlen && x >= 1;


        // is there space for xmas?
        if y_inrange && x_inrange  {

                

            if self.line[y][x] == 'A' as u8 && self.line[y-1][x-1] == 'S' as u8 && self.line[y+1][x-1] == 'S' as u8 { 
 
                if self.line[y+1][x + 1] == 'M' as u8  &&  self.line[y-1][x + 1] == 'M' as u8{
                
                    println!("smsm at {} x {}", x, y);
     
                    num_xmas +=1;
                } 
               
      
            }  
        }
        return num_xmas;
    } 

     fn mmss(&self, x:usize, y:usize) -> i32 {
        let mut num_xmas:i32 = 0;

       let y_inrange:bool = y + 1 <self.line.len() && y >= 1;
       let x_inrange:bool = x + 1 <self.xlen && x >= 1;


        // is there space for xmas?
        if y_inrange && x_inrange  {

                

            if self.line[y][x] == 'A' as u8 && self.line[y-1][x-1] == 'M' as u8 && self.line[y+1][x-1] == 'S' as u8 { 
 
                if self.line[y+1][x + 1] == 'S' as u8  &&  self.line[y-1][x + 1] == 'M' as u8{
                
                    println!("mmss at {} x {}", x, y);
     
                    num_xmas +=1;
                } 
               
       
            }  
        }
        return num_xmas;
    } 


} 


fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data: Vec<u8> = fs::read("input1.txt")?;
    let len: usize = findlength(&data);
    let mut idx :usize = 0;
    let mut matrix:Data = Data { line:Vec::new(), xlen:len, };
    let mut total_xmas = 0;
  // getting all of the data into the proper data format 
    while idx < data.len() {
  
       let added_line:Vec<u8>; 
       (added_line, idx) = get_vec(&data, idx);
       
       matrix.line.push(added_line);
    }

    println!("this is what the vec looks like {:?}", data); 

    println!("this is what the matrix looks like {:?}", matrix.line);

    for y in 0.. matrix.xlen{
        for x in 0.. matrix.line.len() {
    
           total_xmas += matrix.msms(x, y);
           total_xmas += matrix.ssmm(x,y);
           total_xmas += matrix.smsm(x,y);
           total_xmas += matrix.mmss(x,y);
        }
    }

    println!("this is the number of xmas in the input {}", total_xmas);
    Ok(())
}

fn findlength (data : &Vec<u8>) -> usize { 
    for i in 0 ..data.len(){
       if data[i] == 10 { return i;}
    }


    return 0;
}
fn get_vec(data: &Vec<u8>, mut i : usize ) -> (Vec<u8>, usize) {
    let mut line:Vec<u8> = Vec::new();

    while data[i] != 10 {
       line.push(data[i]);
       i+=1;
    }
    return (line, i+1);
}

