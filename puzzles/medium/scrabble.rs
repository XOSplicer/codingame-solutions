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

trait Scrabble {
   fn can_form(&self, other: &String) -> bool;
   fn score(&self, values: &HashMap<char, i32>) -> i32;
}

impl Scrabble for String {

    fn can_form(&self, other: &String) -> bool {
        let mut other_word = other.clone().chars().collect::<Vec<_>>();
        for c in self.chars() {
            //print_err!("word matching: {:?} against {:?}", &self, &other_word);
            match other_word.iter().position(|&x| x==c) {
                Some(pos) => {other_word.remove(pos);},
                None => {},
            };
        }
        return other_word.is_empty();
    }

    fn score(&self, values: &HashMap<char, i32>) -> i32 {
        self.chars().fold(0, |acc, x| acc + values.get(&x as &char).unwrap())
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");
    let (n, words, letters) = read_input();
    let values = generate_values();
    //print_err!("{:?}", (&n, &words, &letters));
    let word_to_chose = words.into_iter()
                                .filter(|x| letters.can_form(&x))
                                .rev()
                                .max_by_key(|x| x.score(&values))
                                .unwrap();
    println!("{}", word_to_chose);
}

fn read_input() -> (i32, Vec<String>, String) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut v = Vec::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let w = input_line.trim_right().to_string();
        v.push(w);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let letters = input_line.trim_right().to_string();
    return (n, v, letters);
}

fn generate_values() -> HashMap<char, i32> {
    let mut values = HashMap::new();
    values.insert('e', 1);
    values.insert('a', 1);
    values.insert('i', 1);
    values.insert('o', 1);
    values.insert('n', 1);
    values.insert('r', 1);
    values.insert('t', 1);
    values.insert('l', 1);
    values.insert('s', 1);
    values.insert('u', 1);
    values.insert('d', 2);
    values.insert('g', 2);
    values.insert('b', 3);
    values.insert('c', 3);
    values.insert('m', 3);
    values.insert('p', 3);
    values.insert('f', 4);
    values.insert('h', 4);
    values.insert('v', 4);
    values.insert('w', 4);
    values.insert('y', 4);
    values.insert('k', 5);
    values.insert('j', 8);
    values.insert('x', 8);
    values.insert('q', 10);
    values.insert('z', 10);
    return values;
}
