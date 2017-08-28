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

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {

    // game loop

    let mut boost = (false, false); // use boost on next, boost used

    loop {
        let (x, y, next_checkpoint_x, next_checkpoint_y,
            next_checkpoint_dist, next_checkpoint_angle, opponent_x, opponent_y) = read_input();
        print_err!("{:?}", (&x, &y, &next_checkpoint_x, &next_checkpoint_y,
                            &next_checkpoint_dist, &next_checkpoint_angle, &opponent_x, &opponent_y));

        let mut thrust = 0_i32;
        if next_checkpoint_angle.abs() > 90 {
            thrust = 0
        } else {
            thrust = 100;
            if next_checkpoint_dist > 5000 && !boost.1 {
                boost.0 = true;
            }
        }


        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        if boost.0 && !boost.1 {
            println!("{} {} BOOST", next_checkpoint_x, next_checkpoint_y);
            boost.0 = false;
            boost.1 = true;
        } else {
            println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
        }
    }
}
fn read_input() -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x = parse_input!(inputs[0], i32);
    let y = parse_input!(inputs[1], i32);
    let next_checkpoint_x = parse_input!(inputs[2], i32); // x position of the next check point
    let next_checkpoint_y = parse_input!(inputs[3], i32); // y position of the next check point
    let next_checkpoint_dist = parse_input!(inputs[4], i32); // distance to the next checkpoint
    let next_checkpoint_angle = parse_input!(inputs[5], i32); // angle between your pod orientation and the direction of the next checkpoint
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let opponent_x = parse_input!(inputs[0], i32);
    let opponent_y = parse_input!(inputs[1], i32);
    return (x, y, next_checkpoint_x, next_checkpoint_y,
            next_checkpoint_dist, next_checkpoint_angle, opponent_x, opponent_y);
}
