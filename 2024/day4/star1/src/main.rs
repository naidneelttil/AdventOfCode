use std::fs;

struct Data {
    line: Vec<Vec<u8>>,
    xlen: usize,
}
impl Data {
    fn vertical(&self, x: usize, y: usize) -> i32 {
        let mut num_xmas: i32 = 0;

        // is there space for xmas?
        if (self.line.len() - 1) - y >= 3 {
            if self.line[y][x] == 'X' as u8
                && self.line[y + 1][x] == 'M' as u8
                && self.line[y + 2][x] == 'A' as u8
                && self.line[y + 3][x] == 'S' as u8
            {
                num_xmas += 1;

                println!("vertical at {} x {}", x, y);
            }
        }
        // is there space for xmas to the left?
        if y >= 3 {
            if self.line[y][x] == 'X' as u8
                && self.line[y - 1][x] == 'M' as u8
                && self.line[y - 2][x] == 'A' as u8
                && self.line[y - 3][x] == 'S' as u8
            {
                num_xmas += 1;
                println!("vertical at {} x {}", x, y);
            }
        }

        return num_xmas;
    }

    fn horizontal(&self, x: usize, y: usize) -> i32 {
        let mut num_xmas: i32 = 0;

        // is there space for xmas?
        if x + 3 < self.xlen {
            if self.line[y][x] == 'X' as u8
                && self.line[y][x + 1] == 'M' as u8
                && self.line[y][x + 2] == 'A' as u8
                && self.line[y][x + 3] == 'S' as u8
            {
                num_xmas += 1;
                println!("horizontal at {} x {}", x, y);
            }
        }
        // is there space for xmas to the left?
        if x >= 3 {
            if self.line[y][x] == 'X' as u8
                && self.line[y][x - 1] == 'M' as u8
                && self.line[y][x - 2] == 'A' as u8
                && self.line[y][x - 3] == 'S' as u8
            {
                num_xmas += 1;
                println!("horizontal at {} x {}", x, y);
            }
        }

        return num_xmas;
    }

    fn diagonal(&self, x: usize, y: usize) -> i32 {
        let mut num_xmas: i32 = 0;

        // is there space for xmas? upwards to the right
        //       S
        //     A
        //   M
        // X
        if x + 3 < self.xlen && y >= 3 {
            if self.line[y][x] == 'X' as u8
                && self.line[y - 1][x + 1] == 'M' as u8
                && self.line[y - 2][x + 2] == 'A' as u8
                && self.line[y - 3][x + 3] == 'S' as u8
            {
                num_xmas += 1;
                println!("diagonal at {} x {}", x, y);
            }
        }
        if x >= 3 && y >= 3 {
            if self.line[y][x] == 'X' as u8
                && self.line[y - 1][x - 1] == 'M' as u8
                && self.line[y - 2][x - 2] == 'A' as u8
                && self.line[y - 3][x - 3] == 'S' as u8
            {
                num_xmas += 1;
                println!("diagonal at {} x {}", x, y);
            }
        }

        if x + 3 < self.xlen && y + 3 < self.line.len() {
            if self.line[y][x] == 'X' as u8
                && self.line[y + 1][x + 1] == 'M' as u8
                && self.line[y + 2][x + 2] == 'A' as u8
                && self.line[y + 3][x + 3] == 'S' as u8
            {
                num_xmas += 1;
                println!("diagonal at {} x {}", x, y);
            }
        }

        if x >= 3 && y + 3 < self.line.len() {
            if self.line[y][x] == 'X' as u8
                && self.line[y + 1][x - 1] == 'M' as u8
                && self.line[y + 2][x - 2] == 'A' as u8
                && self.line[y + 3][x - 3] == 'S' as u8
            {
                num_xmas += 1;
                println!("diagonal at {} x {}", x, y);
            }
        }

        return num_xmas;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data: Vec<u8> = fs::read("input1.txt")?;
    let len: usize = findlength(&data);
    let mut idx: usize = 0;
    let mut matrix: Data = Data {
        line: Vec::new(),
        xlen: len,
    };
    let mut total_xmas = 0;
    // getting all of the data into the proper data format
    while idx < data.len() {
        let added_line: Vec<u8>;
        (added_line, idx) = get_vec(&data, idx);

        matrix.line.push(added_line);
    }

    println!("this is what the vec looks like {:?}", data);

    println!("this is what the matrix looks like {:?}", matrix.line);

    for y in 0..matrix.xlen {
        for x in 0..matrix.line.len() {
            total_xmas += matrix.vertical(x, y);
            total_xmas += matrix.diagonal(x, y);
            total_xmas += matrix.horizontal(x, y);
        }
    }

    println!("this is the number of xmas in the input {}", total_xmas);
    Ok(())
}

fn findlength(data: &Vec<u8>) -> usize {
    for i in 0..data.len() {
        if data[i] == 10 {
            return i;
        }
    }

    return 0;
}

fn get_vec(data: &Vec<u8>, mut i: usize) -> (Vec<u8>, usize) {
    let mut line: Vec<u8> = Vec::new();

    while data[i] != 10 {
        line.push(data[i]);
        i += 1;
    }
    return (line, i + 1);
}
