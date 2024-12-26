use std::io;

#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coordinates {
    x: usize,
    y: usize,
}

struct Guard {
    orientation: Direction,
    current_location: Coordinates,
    //visited: Vec<Coordinates>,
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

    fn clone(&self) -> Guard {
        let new = Guard {
            orientation: self.orientation.clone(),
            current_location: self.current_location.clone(),
            left_map: self.left_map,
        };
        new
    }
}

#[derive(Clone)]
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

    let guard = Guard {
        orientation: Direction::UP,
        current_location: start,
        //visited: visited,
        left_map: false,
    };

    let mut num_paradox_found: i32 = 0;
    let mut new_matrix: Data = matrix.clone();
    let mut new_guard: Guard = guard.clone();

    let locations: Vec<Coordinates> =
        get_possible_paradox_locations(&mut new_guard, &mut new_matrix);

    for spot in locations.iter() {
        new_matrix = matrix.clone();
        new_guard = guard.clone();
        new_matrix.map[spot.y][spot.x] = 'O';
        if run_simulation(&mut new_guard, &mut new_matrix) {
            num_paradox_found = num_paradox_found + 1;
        }
    }

    println!(
        "this is the number of paradoxes found {}",
        num_paradox_found
    );

    println!(
        "this is the board dimensions, {} by {}",
        matrix.xlen, matrix.ylen
    );
}

fn run_simulation(guard: &mut Guard, matrix: &mut Data) -> bool {
    // let mut loop_count: i32 = 0;

    let mut visited: Vec<Coordinates> = Vec::new();
    let mut num_consecutive_visit: i32 = 0;
    while !guard.left_map {
        matrix.mark_map(&guard.current_location);
        if visited.contains(&guard.current_location) {
            num_consecutive_visit = num_consecutive_visit + 1;
        } else {
            visited.push(guard.current_location.clone());
            num_consecutive_visit = 0;
        }

        // if u are travelling the entire board's length on paths already travelled, ur in a loop
        if num_consecutive_visit >= (matrix.xlen as i32 * matrix.ylen as i32) {
            println!("this is the paradox map found");

            // print map
            for line in matrix.map.iter() {
                println!("{:?}", line);
            }
            return true;
        }
        guard.left_map = !is_legal_space(&guard.current_location, &matrix, &guard.orientation);

        if guard.left_map {
            matrix.mark_map(&guard.current_location);
            break;
        }
        let mut new_location: Coordinates = move_guard(&guard);

        while matrix.map[new_location.y][new_location.x] == '#'
            || matrix.map[new_location.y][new_location.x] == 'O'
        {
            // get the locations of the next 4 obsturctions maybe its a loop
            guard.change_orientation();
            new_location = move_guard(&guard);
        }

        guard.current_location = new_location;
        //let next_space :Coordinates = move(&gaurd, &matrix);
    }
    return false;
}

fn get_possible_paradox_locations(guard: &mut Guard, matrix: &mut Data) -> Vec<Coordinates> {
    // let mut loop_count: i32 = 0;

    let mut visited: Vec<Coordinates> = Vec::new();
    let start: Coordinates = guard.current_location.clone();
    while !guard.left_map {
        if guard.current_location != start
            && matrix.map[guard.current_location.y][guard.current_location.x] != 'X'
        {
            visited.push(guard.current_location.clone());
        }
        matrix.mark_map(&guard.current_location);
        guard.left_map = !is_legal_space(&guard.current_location, &matrix, &guard.orientation);

        if guard.left_map {
            if matrix.map[guard.current_location.y][guard.current_location.x] != 'X' {
                visited.push(guard.current_location.clone());
            }
            matrix.mark_map(&guard.current_location);

            // print map
            for line in matrix.map.iter() {
                println!("{:?}", line);
            }
            println!("this is the possible locations {:?}", visited);
            break;
        }
        let mut new_location: Coordinates = move_guard(&guard);

        while matrix.map[new_location.y][new_location.x] == '#'
            || matrix.map[new_location.y][new_location.x] == 'O'
        {
            // get the locations of the next 4 obsturctions maybe its a loop
            guard.change_orientation();
            new_location = move_guard(&guard);
        }

        guard.current_location = new_location;
        //let next_space :Coordinates = move(&gaurd, &matrix);
    }
    return visited;
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
