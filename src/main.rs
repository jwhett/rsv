use std::env;
use std::fs;
use std::collections::HashMap;

struct CSV {
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
}

impl CSV {
    fn new_from_file(filename: String) -> CSV {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut lines = contents.lines();

        let mut headers: Vec<String> = Vec::new();
        for header in lines
            .next()
            .unwrap()
            .split(",")
        {
            // Taking ownership
            headers.push(String::from(header));
        }

        let mut rows: Vec<Vec<String>> = Vec::new();
        for line in lines {
            let mut row: Vec<String> = Vec::new();
            for item in line.split(",") {
                // Taking ownership
                row.push(String::from(item));
            }
            rows.push(row);
        }

        let mut hm_rows: Vec<HashMap<String, String>> = Vec::new();
        for row in rows {
            let mut hm: HashMap<String, String> = HashMap::new();
            for (k, v) in headers.iter().zip(row) {
                hm.insert(String::from(k), String::from(v));
            }
            hm_rows.push(hm);
        }

        CSV {
            headers: headers,
            rows: hm_rows,
        }
    }
}

impl CSV {
    fn print(&self) {
        for row in &self.rows {
            println!("{:?}", row);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);

    let csv = CSV::new_from_file(filename);

    csv.print();
}
