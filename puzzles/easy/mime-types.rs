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

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {

    let (n, q, mime_table, fnames) = read_data();
    print_err!("{:?}", (&n, &q, &mime_table, &fnames));

    let fnames: Vec<_> = fnames.into_iter().map(|x| x.to_lowercase()).collect();
    let mut mime_map: HashMap<String, String> = HashMap::new();
    for (ext, mt) in mime_table.into_iter() {
        mime_map.insert(ext.to_lowercase(), mt);
    }


    for fname in fnames.into_iter() {
         match mime_map.get(&get_ext(fname)) {
            Some(mt) => println!("{}", mt),
            None => println!("UNKNOWN"),
         }
    }


    // Write an action using println!("message...");
    // To debug: print_err!("Debug message...");


    // For each of the Q filenames, display on a line the corresponding MIME type. If there is no corresponding type, then display UNKNOWN.

}

fn get_ext(fname: String) -> String {
    let parts: Vec<_> = fname.split('.').collect();
    if parts.len() <= 1 {
        return "UNKNOWN".to_string();
    }
    else {
        return parts.last().unwrap().to_string();
    }
}

fn read_data() -> (i32, i32, Vec<(String, String)>, Vec<String>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // Number of elements which make up the association table.

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let q = parse_input!(input_line, i32); // Number Q of file names to be analyzed.


    let mut mime_table: Vec<(String, String)> = Vec::new();
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let ext = inputs[0].trim().to_string(); // file extension
        let mt = inputs[1].trim().to_string(); // MIME type.
        mime_table.push((ext, mt));
    }

    let mut fnames: Vec<String> = Vec::new();
    for i in 0..q as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let fname = input_line.trim_right().to_string(); // One file name per line.
        fnames.push(fname);
    }

    return (n, q, mime_table, fnames);
}
