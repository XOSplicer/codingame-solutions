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

#[derive(Debug)]
struct Position {
    lon: f32,
    lat: f32,
    }

impl Position {
    fn distance(&self, other: &Position) -> f32 {
        let x = (other.lon.to_radians() - self.lon.to_radians())
                    * ((self.lat.to_radians() + other.lat.to_radians())/2 as f32).cos();
        let y = (other.lat.to_radians() - self.lat.to_radians());
        let d = (x.powi(2) + y.powi(2)).sqrt() * 6371f32;
        d
    }
}

fn main() {

    let (pos, n, defibs) = read_in_data();
    print_err!("(pos, n, defibs) = {:?}", (&pos, &n, &defibs));

    let distances: Vec<_> = defibs.iter()
                        .map(|ref x| mapper(&x, &pos)).collect();
    print_err!("(distances = {:?}", &distances);


    //let (nearest, _) = distances.iter().min_by(|a, b| comparer(a, b)).unwrap();

    let mut nearest = distances[0].clone();

    for (name, distance) in distances.into_iter() {
        print_err!("(name, distnace) = {:?}", (&name, &distance));
        if distance < nearest.1 {
            nearest = (name, distance)
        }
    }


    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");

    println!("{}", nearest.0);
}

fn mapper(defib: &(String, Position), pos: &Position) -> (String, f32) {
    (defib.0.clone(), pos.distance(&defib.1))
}


fn read_in_data() -> (Position, i32, Vec<(String, Position)>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lon = input_line.trim().to_string().replace(",", ".").parse::<f32>().unwrap();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = input_line.trim().to_string().replace(",", ".").parse::<f32>().unwrap();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    //print_err!("(lon, lat, n) = {:?}", (lon, lat, n));

    let mut defibs: Vec<(String, Position)> = Vec::new();

    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let defib = input_line.trim_right().to_string();
        let infos: Vec<&str> = defib.split(';').collect();
        //print_err!("defib infos = {:?}", infos);
        let defib_name = infos[1].clone().to_string();
        let defib_lon = infos[4].replace(",", ".").parse::<f32>().unwrap();
        let defib_lat = infos[5].replace(",", ".").parse::<f32>().unwrap();
        //print_err!("defib infos parsed = {:?}", (defib_name, Position(defib_lon, defib_lat))); // breaks the code
        defibs.push((defib_name, Position{lon: defib_lon, lat: defib_lat}));
    }

    return (Position{lon: lon, lat: lat}, n, defibs)
}
