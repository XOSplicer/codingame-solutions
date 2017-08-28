use std::io;
use std::io::BufRead;
use std::collections::HashMap;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2D {
    x: i64,
    y: i64
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let n: u64 = lines.next().unwrap().parse().unwrap();
    let buildings: Vec<Vec2D> = lines
        .map(|l| {
            let mut v = l.split(" ");
            let x: i64 = v.next().unwrap().parse().unwrap();
            let y: i64 = v.next().unwrap().parse().unwrap();
            Vec2D { x:x, y:y }
        }).collect();

    let x_max: i64 = buildings.iter().max_by_key(|v| v.x).unwrap().x;
    let x_min: i64 = buildings.iter().min_by_key(|v| v.x).unwrap().x;
    let x_span = x_max - x_min;

    let mut ys: Vec<i64> = buildings.iter().map(|b| b.y).collect();
    ys.sort();

    let y_median = match ys.len() % 2 == 0 {
        true => (ys[(ys.len() / 2) - 1] + ys[(ys.len() / 2)]) / 2,
        false => ys[(ys.len() / 2)],
    };

    let y_dist_sum: i64= ys.iter().map(|y| (y - y_median).abs()).sum();

    let cable_len = y_dist_sum + x_span;

    print_err!("n: {:?}", &n);
    //print_err!("buildings: {:?}", &buildings);
    print_err!("x max: {:?}", &x_max);
    print_err!("x min: {:?}", &x_min);
    print_err!("x span: {:?}", &x_span);
    print_err!("y median: {:?}", &y_median);
    print_err!("y dist sum: {:?}", &y_dist_sum);
    print_err!("cable length: {:?}", &cable_len);

    println!("{}", &cable_len);
}
