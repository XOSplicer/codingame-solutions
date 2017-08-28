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
    let message = read_input();
    let mut bits: Vec<bool> = Vec::new();
    for c in message.chars() {
        print_err!("{:b}", c as u8);
        for i in 0..7 {
            bits.push(c as u8 & (1 << (6-i)) != 0);
        }
    }
    print_err!("{:?}", &bits);
    //accumulate the series
    let mut encoded: Vec<(bool, i32)> = Vec::new();
    let mut acc = (false, 0i32);
    for bit in bits.into_iter() {
        if bit == acc.0 {
            acc = (bit, acc.1+1);
        } else {
            encoded.push((acc.0, acc.1));
            acc = (bit, 1);
        }
    }
    encoded.push((acc.0, acc.1));
    encoded = encoded.into_iter().filter(|x| x.1 > 0).collect();
    print_err!("{:?}", &encoded);
    let mut count = 1i32;
    for seq in encoded.iter() {
        if seq.0 {
            print!("0 ");
        } else {
            print!("00 ");
        }
        for i in 0..seq.1 {
            print!("0");
        }
        count += 1;
        if count <= encoded.len() as i32 {
            print!(" ");
        }
    }
}

fn read_input() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    return input_line.trim_right().to_string();
}
