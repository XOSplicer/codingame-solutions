use std::io;
use std::io::BufRead;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction { N,E,S,W }

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum GridCell { Black, White }

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    width: usize,
    height: usize,
    ant_x: usize,
    ant_y: usize,
    ant_dir: Direction,
    turns_left: usize,
    grid: Vec<Vec<GridCell>>,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            &Direction::N => Direction::W,
            &Direction::E => Direction::N,
            &Direction::S => Direction::E,
            &Direction::W => Direction::S,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            &Direction::N => Direction::E,
            &Direction::E => Direction::S,
            &Direction::S => Direction::W,
            &Direction::W => Direction::N,
        }
    }
}

impl GridCell {
    fn flip(&self) -> Self {
        match self {
            &GridCell::Black => GridCell::White,
            &GridCell::White => GridCell::Black,
        }
    }
}

impl State {
    fn step(&self) -> Self {
        let mut next = self.clone();


        print_err!("old color: {:?}", self.grid[self.ant_x][self.ant_y]);

        print_err!("old dir: {:?}", self.ant_dir);
        next.ant_dir = match self.grid[self.ant_y][self.ant_x] {
            GridCell::Black => self.ant_dir.turn_right(),
            GridCell::White => self.ant_dir.turn_left(),
        };
        print_err!("new dir: {:?}", next.ant_dir);

        next.grid[self.ant_y][self.ant_x] = self.grid[self.ant_y][self.ant_x].flip();

        print_err!("new color: {:?}", next.grid[self.ant_y][self.ant_x]);

        print_err!("old pos xy: {} {}", self.ant_x, self.ant_y);
        next.ant_x = (self.ant_x as i32 + match next.ant_dir {
            Direction::E => 1,
            Direction::W => -1,
            _ => 0,
        }) as usize;

        next.ant_y = (self.ant_y as i32 + match next.ant_dir {
            Direction::S => 1,
            Direction::N => -1,
            _ => 0,
        }) as usize;


        print_err!("new pos xy: {} {}", next.ant_x, next.ant_y);

        next.turns_left = self.turns_left - 1;
        next
    }

    fn grid_string(&self) -> String {
        let lines: Vec<String> = self.grid.iter().map(|r|
            r.iter().map(|c| match c {
             &GridCell::Black => '#',
             &GridCell::White => '.',
            }).collect()
        ).collect();
        lines.join("\n").to_string()
    }
}

fn read_init() -> State {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let line = lines.next().unwrap();
    let mut wh = line.split(" ");
    let w: u32= wh.next().unwrap().parse().unwrap();
    let h: u32= wh.next().unwrap().parse().unwrap();


    let line = lines.next().unwrap();
    let mut xy = line.split(" ");
    let x: u32= xy.next().unwrap().parse().unwrap();
    let y: u32= xy.next().unwrap().parse().unwrap();

    let dir = match lines.next().unwrap().trim() {
        "N" => Direction::N,
        "E" => Direction::E,
        "S" => Direction::S,
        "W" => Direction::W,
        _ => unreachable!(),
    };

    let t: u32 = lines.next().unwrap().parse().unwrap();

    let grid = lines
        .map(|l|
            l.chars()
            .map(|c| match c {
                '#' => GridCell::Black,
                '.' => GridCell::White,
                _ => unreachable!(),
            })
            .collect()
        )
        .collect::<Vec<Vec<_>>>();

    State {
        width: w as usize,
        height: h as usize,
        ant_x: x as usize,
        ant_y: y as usize,
        ant_dir: dir,
        turns_left: t as usize,
        grid: grid,
    }

}

fn main() {
    let mut state = read_init();
    print_err!("ant xy: {} {} dir: {:?} grid: \n{}",
        state.ant_x, state.ant_y, state.ant_dir, state.grid_string());
    while state.turns_left > 0 {
        state = state.step();
        print_err!("ant xy: {} {} dir: {:?} grid: \n{}",
            state.ant_x, state.ant_y, state.ant_dir, state.grid_string());
    }
    println!("{}", state.grid_string());
}