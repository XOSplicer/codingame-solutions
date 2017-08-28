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


fn conway_step(old: &Vec<u32>) -> Vec<u32> {
    let mut new = Vec::new();
    let mut iter = old.iter().cloned();
    let mut seq_value = None;
    let mut amount = 0;
    for v in iter {
        if None == seq_value {
            amount += 1;
            seq_value = Some(v);
        } else if Some(v) == seq_value {
            amount += 1;
        } else {
            new.push(amount);
            new.push(seq_value.unwrap());
            amount = 1;
            seq_value = Some(v);
        }
    }
    if seq_value.is_some() {
        new.push(amount);
        new.push(seq_value.unwrap());
    }
    new
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|s| s.unwrap());
    let r: u32 = lines.next().unwrap().parse().unwrap();
    let l: u32 = lines.next().unwrap().parse().unwrap();
    print_err!("input: r={}, l={}", r, l);
    let mut seq = vec![r];
    for _ in 1..l {
        seq = conway_step(&seq);
    }

    let as_strings: Vec<String> = seq.iter().map(|i| i.to_string()).collect();
    let output = as_strings.join(" ");

    println!("{}", output);
}
