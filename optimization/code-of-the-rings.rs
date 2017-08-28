use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::iter;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Action {
    Left,
    Right,
    Plus,
    Minus,
    Print
}

use Action::*;

impl Action {
    fn to_string(&self) -> String {
        (match self {
            &Left => "<",
            &Right => ">",
            &Plus => "+",
            &Minus => "-",
            &Print => "."
        }).to_owned()
    }
}

fn seq_to_string(seq: &Vec<Action>) -> String {
    seq.iter().map(|a| a.to_string()).collect()
}

fn sort_letters(map: &HashMap<char, i32>) -> Vec<char> {
    let mut vec: Vec<(char, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    vec.sort_by_key(|&(k, v)| v);
    vec.iter().map(|&(k, v)| k).collect()
}

fn count_letters(s: &str) -> HashMap<char, i32> {
    s.chars()
    .fold(HashMap::new(), |mut acc, c| {
        {
            let counter = acc.entry(c).or_insert(0);
            *counter += 1;
        }
        acc
    })
}

fn create_letters(v: &Vec<char>) -> (Vec<Action>, u32) {
    let a = v.iter()
        .flat_map(|&c|
            iter::repeat(Plus)
                .take( if c != ' '
                    { ((c as u8) - 64) as usize }
                    else { 0 })
                .chain(iter::once(Right)))
            .collect();
    (a, v.len() as u32)
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    print_err!("input: {:?}", &input);
    let mut seq = Vec::new();
    let letters = count_letters(input.as_str());
    print_err!("letters: {:?}", &letters);
    let sorted = sort_letters(&letters);
    print_err!("sorted: {:?}", &sorted);
    let (alph, pos) = create_letters(&sorted);
    seq.extend(alph);
    let mut position = pos;
    for c in input.chars() {
        let goto = sorted.iter().enumerate()
            .filter(|&(i, &s)| s == c)
            .map(|(i, _)| i)
            .next().unwrap() as u32;
        print_err!("pos: {}, goto: {}", &pos, &goto);
        let mov = (goto as i32) - (position as i32);
        let dir = if mov > 0 { Right } else { Left };
        seq.extend(iter::repeat(dir).take(mov.abs() as usize));
        seq.push(Print);
        position = goto;
    }

    let output = seq_to_string(&seq);
    println!("{}", output);
}
