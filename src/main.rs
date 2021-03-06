use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct CSV {
    headers: Vec<String>,
    rows: Vec<HashMap<String, String>>,
}

impl CSV {
    fn new_from_file(filename: String) -> CSV {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut lines = contents.lines();

        let mut headers: Vec<String> = Vec::new();
        for header in lines.next().unwrap().split(",") {
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

    fn kv_in(&self, key: &str, value: &str) -> Option<Vec<&HashMap<String, String>>> {
        let res: Vec<&HashMap<String, String>> = self.rows
            .iter()
            .filter(|hm| hm.get(key) == Some(&String::from(value)))
            .collect();
        if res.len() == 0 {
            None
        } else {
            Some(res)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);

    let csv = CSV::new_from_file(filename);

    println!("{:#?}", csv);
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use tempdir::TempDir;

    fn csv_setup() -> CSV {
        let filename = String::from("test.csv");
        let file_content = b"id,first_name,last_name
        0,jane,doe
        1,john,doe";
        let dir = TempDir::new("testdir").unwrap();
        let file_path = dir.path().join(&filename);

        let mut f = File::create(&file_path).unwrap();
        f.write_all(file_content)
            .expect("Could not write to test file");
        f.sync_all().expect("Could not sync during test");

        let csv = CSV::new_from_file(String::from(file_path.to_str().unwrap()));
        dir.close().expect("Failed to close test dir");
        csv
    }

    #[test]
    fn new_csv_from_file() {
        let csv = csv_setup();
        assert_eq!(csv.headers, ["id", "first_name", "last_name"]);
    }

    #[test]
    fn search_key_value() {
        let csv = csv_setup();
        let res = csv.kv_in("first_name", "jane");
        assert_eq!(res.unwrap()[0].get("first_name").unwrap(), &String::from("jane"));
    }

    #[test]
    fn fail_key_value() {
        let csv = csv_setup();
        let res = csv.kv_in("first_name", "steve");
        assert_eq!(res, None);
    }
}
