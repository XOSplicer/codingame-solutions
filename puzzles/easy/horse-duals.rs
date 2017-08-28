use std::io;
use std::iter;
use std::collections::BTreeSet;


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

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, u32);
    let mut set = BTreeSet::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let pi = parse_input!(input_line, u32);
        if set.contains(&pi) {
            println!("0");
            return;
        } else {
            set.insert(pi);
        }
    }
    print_err!("{:?}",  &set);

    /*let (a, b) = set.iter() // sorted
                    .flat_map(|p| iter::repeat(p).zip(set.iter()))
                    .filter(|&(a, b)| a > b)
                    .min_by_key(|&(&a, &b)| a - b)
                    .unwrap();
    println!("{:?}",  a-b);*/

    let mut iter = set.into_iter(); //sorted
    let mut min_diff = u32::max_value();
    let mut prev = iter.next().unwrap();
    let mut curr;
    for i in iter {
        curr = i;
        let diff = curr - prev;
        if diff < min_diff {
            min_diff = diff;
        }
        prev = curr;
    }
    println!("{}", min_diff);
}
