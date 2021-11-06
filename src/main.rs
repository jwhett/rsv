use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let headers: Vec<&str> = lines
        .next()
        .unwrap()
        .split(",")
        .collect();
    println!("headers: {:?}", headers);

    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let row: Vec<&str> = line.split(",").collect();
        rows.push(row);
    }
    println!("rows: {:?}", rows);
}
