use std::io;
use std::collections::HashMap;

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
struct Letter {
    rows: Vec<String>
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");
    let (letter_width, letter_heigth, text, rows) = read_input();
    //print_err!("Read: {:?}", (&letter_width, &letter_heigth, &text, &rows));
    let mut letters = Vec::new();
    for i in 0..27 {
        letters.push(Letter{rows: Vec::new()});
    }
    for input_row in 0..letter_heigth {
        print_err!("{}", &rows[input_row as usize]);
        for letter_nr in 0..27 {
            letters[letter_nr as usize].rows.push(
                rows[input_row as usize].clone().chars()
                    .skip((letter_nr * letter_width) as usize)
                    .take(letter_width as usize).collect::<String>());
        }
    }
    let mut letters_map = HashMap::new();
    for i in 0..26 {
        letters_map.insert((0..26).map(|x| (x + 'a' as u8) as char).nth(i as usize).unwrap(),
            &letters[i as usize]);
    }
    letters_map.insert('?', &letters[26usize]);
    for r in 0..letter_heigth {
        //print_err!("{}", letters_map.get(&'a').unwrap().rows[r as usize]);
        //print_err!("{} ", &letters[26usize].rows[r as usize]);
        print_err!("{}", letters_map.get(&'?').unwrap().rows[r as usize]);
    }

    let text = text.to_lowercase();
    for output_row in 0..letter_heigth {
        for output_char in text.chars() {
            match output_char {
                'a'...'z' => print!("{}", letters_map.get(&output_char).unwrap().rows[output_row as usize]),
                _ => print!("{}", letters_map.get(&'?').unwrap().rows[output_row as usize]),
            };
        }
        println!("");
    }
}

fn read_input() -> (i32, i32, String, Vec<String>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t = input_line.trim_right().to_string();

    let mut v = Vec::new();
    for i in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.to_string();
        v.push(row);
    }
    return (l, h, t, v);
}
