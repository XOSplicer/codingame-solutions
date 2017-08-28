use std::io;
use std::io::BufRead;
use std::iter;
use std::cmp;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}


fn read_init() -> Vec<String> {
    let stdin = io::stdin();
    let r = stdin.lock()
        .lines()
        .skip(1)
        .map(|s| s.unwrap())
        .collect();
    r
}

fn permute(of: &Vec<String>) -> Vec<Vec<String>> {
    if of.len() == 0 {
        return Vec::new();
    }
    if of.len() == 1 {
        return vec![vec![of[0].clone()]];
    }

    let mut vec = Vec::new();

    for i in 0..(of.len()) {

        let curr = of[i].clone();
        let mut other = of.clone();
        other.remove(i);
        let other_p = permute(&other);

        for mut p in other_p.into_iter() {
            let mut v = Vec::new();
            v.push(curr.clone());
            v.append(&mut p);
            vec.push(v);
        }

    }

    vec
}

fn combine(mut a: String, b: String) -> String {
    if a.contains(&b) {
        return a;
    }
    if b.contains(&a) {
        return b;
    }
    let max_overlap = cmp::min(a.len(), b.len());
    let mut overlap = 0;
    for pos in 0..max_overlap {
        if a.ends_with(&b[..(pos)]) {
            overlap = pos;
        }
    }
    a.push_str(&b[(overlap)..]);
    a
}

fn main() {
    let subseqs = read_init();
    print_err!("subseqs: {:?}", &subseqs);
    let perm = permute(&subseqs);
    print_err!("perm: {:?}", &perm);
    let combined: Vec<String> = perm.into_iter()
        .map(|v| {
            v.into_iter()
            .fold(String::new(),
                |acc, x| combine(acc, x))
        }).collect();
    print_err!("ombined: {:?}", &combined);
    let min = combined.iter().min_by_key(|x| x.len()).unwrap();
    print_err!("min: {:?}", &min);
    println!("{}", min.len());
}
