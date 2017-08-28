use std::io;

/*

int main()
{
    int lightX; // the X position of the light of power
    int lightY; // the Y position of the light of power
    int initialTX; // Thor's starting X position
    int initialTY; // Thor's starting Y position
    scanf("%d%d%d%d", &lightX, &lightY, &initialTX, &initialTY);

    int thorX=initialTX;
    int thorY=initialTY;

    // game loop
    while (1) {
        int remainingTurns;
        scanf("%d", &remainingTurns);
        char directionX[2] = "", directionY[2]="";
        if(thorX>lightX) {
            strcpy(directionX, "W");
            thorX--;
        }
        else if(thorX<lightX) {
            strcpy(directionX, "E");
            thorX++;
        }
        if(thorY>lightY) {
            strcpy(directionY, "N");
            thorY--;
        }
        else if(thorY<lightY) {
            strcpy(directionY, "S");
            thorY++;
        }

        printf("%s%s\n", directionY, directionX);

        //printf("SE\n"); // A single line providing the move to be made: N NE E SE S SW W or NW

    }

    return 0;
}
*/

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
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let (light_x, light_y, initial_tx, initial_ty) = read_init();
    let mut thor_x = initial_tx;
    let mut thor_y = initial_ty;
    // game loop
    loop {
        let remaining_turns = read_turn();
        let mut direction_x = String::new();
        let mut direction_y = String::new();
        if thor_x > light_x {
            direction_x = "W".to_string();
            thor_x -= 1;
        } else if thor_x < light_x  {
            direction_x = "E".to_string();
            thor_x += 1;
        }
        if thor_y > light_y {
            direction_y = "N".to_string();
            thor_y -=1;
        }else if thor_y < light_y {
            direction_y = "S".to_string();
            thor_y += 1;
        }

        println!("{}{}", direction_y, direction_x);
    }
}

fn read_init() -> (i32, i32, i32, i32) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position
    return (light_x, light_y, initial_tx, initial_ty);
}

fn read_turn() -> i32 {
    let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.
        return remaining_turns;
}
