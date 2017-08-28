use std::io;
fn main() {
let mut i = String::new();
io::stdin().read_line(&mut i).unwrap();
let mut z=i.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
loop {
io::stdin().read_line(&mut String::new());
let mut g:&str="";
let mut h:&str="";
if z[2] > z[0] {g ="W";z[2]-=1;
}else if z[2] < z[0] {g="E";z[2]+=1;}
if z[3] > z[1] {h="N";z[3]-=1;
}else if z[3] < z[1] {h="S";z[3]+=1;}
println!("{}{}",h,g);}}