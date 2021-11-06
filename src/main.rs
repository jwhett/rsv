use std::env;
use std::fs;

struct CSV<'a, 'b> {
    headers: Vec<&'a str>,
    rows: Vec<Vec<&'b str>>,
}

impl<'a, 'b> CSV<'a, 'b> {
    fn print(&self) {
        for row in &self.rows {
            println!("{{");
            for (h, r) in self.headers.iter().zip(row) {
                println!("  {}: {},", h, r);
            }
            println!("}}");
        }
    }
}

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

    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let row: Vec<&str> = line.split(",").collect();
        rows.push(row);
    }

    let csv = CSV {
        headers: headers,
        rows: rows,
    };

    csv.print();
}
