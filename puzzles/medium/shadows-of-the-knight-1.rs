use std::io;

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

#[derive(Debug)]
struct Borders {
    x_upper: i32,
    x_lower: i32,
    y_upper: i32,
    y_lower: i32,
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");
    let (width, heigth, max_jumps, x0, y0) = read_init();
    print_err!("{:?}", (&width, &heigth, &max_jumps, &x0, &y0));
    let mut x_current = x0;
    let mut y_current = y0;
    let mut borders = Borders { x_upper: width+1,
                            x_lower: -1,
                            y_upper: heigth+1,
                            y_lower: -1, }; // not included boundaries
    // game loop
    loop {
        let bomb_dir = read_loop();
        print_err!("Bomb direction: {:?}", &bomb_dir);
        print_err!("Before: {:?}", &borders);
        print_err!("Position before: {:?}", (&x_current, &y_current));
        match &bomb_dir[..] { //typecasting String to &str
            "U"     => { borders.x_lower = x_current - 1;
                         borders.x_upper = x_current + 1;
                         borders.y_upper = y_current; },
            "UR"    => { borders.y_upper = y_current;
                         borders.x_lower = x_current; },
            "R"     => { borders.y_lower = y_current - 1;
                         borders.y_upper = y_current + 1;
                         borders.x_lower = x_current; },
            "DR"    => { borders.y_lower = y_current;
                         borders.x_lower = x_current; },
            "D"     => { borders.x_lower = x_current - 1;
                         borders.x_upper = x_current + 1;
                         borders.y_lower = y_current; },
            "DL"    => { borders.y_lower = y_current;
                         borders.x_upper = x_current; },
            "L"     => { borders.y_lower = y_current - 1;
                         borders.y_upper = y_current + 1;
                         borders.x_upper = x_current; },
            "UL"    => { borders.y_upper = y_current;
                         borders.x_upper = x_current; },
            _       => unreachable!(),
        };
        x_current = ((borders.x_lower + borders.x_upper) / 2);
        y_current = ((borders.y_lower + borders.y_upper) / 2);
        print_err!("Borders after:{:?}", &borders);
        print_err!("Position after: {:?}", (&x_current, &y_current));
        // the location of the next window Batman should jump to.
        println!("{} {}", &x_current, &y_current);
    }
}

fn read_init() -> (i32, i32, i32, i32, i32) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);
    return (w, h, n, x0, y0);
}

fn read_loop() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
    let bomb_dir = input_line.trim().to_string();
    return bomb_dir;
}
