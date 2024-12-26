use std::io;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn clone(&self) -> Coordinates {
        let new = Coordinates {
            x: self.x,
            y: self.y,
        };

        new
    }
}
struct Guard {
    orientation: Direction,
    current_location: Coordinates,
    visited: Vec<Coordinates>,
    left_map: bool,
}

impl Guard {
    fn change_orientation(&mut self) {
        self.orientation = match self.orientation {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}
struct Data {
    map: Vec<Vec<char>>,
    xlen: usize,
    ylen: usize,
}

impl Data {
    fn find_start(&mut self) -> Coordinates {
        let mut start: Coordinates = Coordinates { x: 0, y: 0 };
        for x in 0..self.xlen {
            for y in 0..self.ylen {
                if self.map[y][x] == '^' {
                    start.x = x;
                    start.y = y;

                    return start;
                }
            }
        }
        start
    }

    fn mark_map(&mut self, location: &Coordinates) {
        self.map[location.y][location.x] = 'X';
    }
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();

    // collect everything into a char vec
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Err(_) => break,
            Ok(_) => {
                //read in and store the rules
                let map_line: Vec<char> = line.trim().chars().collect::<Vec<_>>();
                map.push(map_line);
            }
        }
    }

    // store the map and initialize the matrix
    let x = map[0].len();
    let y = map.len();

    println!("print coordinates");

    println!("x is {} y is {}", x, y);

    let mut matrix: Data = Data {
        map: map,
        ylen: y,
        xlen: x,
    };

    let start: Coordinates = matrix.find_start();
    println!("print the coordinates of start {:?}", start);

    // print map
    for line in matrix.map.iter() {
        println!("{:?}", line);
    }

    let mut visited: Vec<Coordinates> = Vec::new();
    visited.push(start.clone());
    let mut guard = Guard {
        orientation: Direction::UP,
        current_location: start,
        visited: visited,
        left_map: false,
    };

    while !guard.left_map {
        println!("guard location:{:?}", guard.current_location);

        // print map
        for line in matrix.map.iter() {
            println!("{:?}", line);
        }
        if matrix.map[guard.current_location.y][guard.current_location.x] == '.' {
            guard.visited.push(guard.current_location.clone());
        }

        matrix.mark_map(&guard.current_location);
        guard.left_map = !is_legal_space(&guard.current_location, &matrix, &guard.orientation);
        println!(
            "guard left map?:{:?}, {}",
            guard.current_location, guard.left_map
        );

        if guard.left_map {
            if matrix.map[guard.current_location.y][guard.current_location.x] == '.' {
                guard.visited.push(guard.current_location.clone());
            }

            matrix.mark_map(&guard.current_location);
            break;
        }
        println!(
            "is the location legal {:?} current location is {:?}",
            guard.left_map, guard.current_location,
        );
        let mut new_location: Coordinates = move_guard(&guard);

        while matrix.map[new_location.y][new_location.x] == '#' {
            guard.change_orientation();
            new_location = move_guard(&guard);
        }

        guard.current_location = new_location;
        //let next_space :Coordinates = move(&gaurd, &matrix);
    }
    //
    // print map
    // print map
    for line in matrix.map.iter() {
        println!("{:?}", line);
    }

    println!(
        "the finished map and numbers of visited {}",
        guard.visited.len()
    );
}

fn move_guard(guard: &Guard) -> Coordinates {
    let mut new_coordinates: Coordinates = Coordinates {
        x: guard.current_location.x,
        y: guard.current_location.y,
    };

    match guard.orientation {
        Direction::LEFT => {
            new_coordinates.x = new_coordinates.x - 1;
        }
        Direction::RIGHT => {
            new_coordinates.x = new_coordinates.x + 1;
        }
        Direction::UP => {
            new_coordinates.y = new_coordinates.y - 1;
        }
        Direction::DOWN => {
            new_coordinates.y = new_coordinates.y + 1;
        }
    }

    return new_coordinates;
}

// checks to see if the given cooridnate is in the map
fn is_legal_space(coordinates: &Coordinates, map: &Data, orientation: &Direction) -> bool {
    match orientation {
        Direction::LEFT => {
            if coordinates.x == 0 {
                return false;
            }
        }
        Direction::RIGHT => {
            if coordinates.x == map.xlen - 1 {
                return false;
            }
        }
        Direction::UP => {
            if coordinates.y == 0 {
                return false;
            }
        }
        Direction::DOWN => {
            if coordinates.y == map.ylen - 1 {
                return false;
            }
        }
    }

    return true;
}
