use std::io::{stdin, BufRead};
fn main() {
let mut s = stdin();
let i = s.lock().lines().skip(1).next().unwrap().unwrap();
let mut m = 6666i32;
for t in i.trim_right().to_string().split_whitespace().map(|x| x.parse::<i32>().unwrap()) {
if t.abs() < m.abs() || (t.abs() == m.abs() && t > 0) {m = t;}}
if m == 6666 {m = 0;}
println!("{}", m);}
