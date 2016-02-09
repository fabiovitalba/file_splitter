use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

fn main() {
    let reader = io::stdin();
    let mut new_name;

    for line in reader.lock().lines()   {
        let path = line.unwrap();
        let tmp_filename = path.to_string() + "Leitner_2009R2_CompleteExport.txt";
        let f = File::open(tmp_filename).unwrap();
        let f = BufReader::new(f);

        let tmp_f = File::create("tmp.txt").unwrap();
        let mut new_f = BufWriter::new(tmp_f);

        for line2 in f.lines() {
            let curr_line = line2.unwrap_or("".to_string());

            println!("{}", curr_line.to_string());

            if curr_line.len() > 6 {
                if curr_line[..6] == "OBJECT".to_string() {
                    new_name = curr_line.to_string();
                    new_name = new_name.replace("/", "_");
                    new_f.flush();
                    let tmp_curr_filename = path.to_string() + &new_name + ".txt";
                    let curr_file = File::create(tmp_curr_filename).unwrap();
                    new_f = BufWriter::new(curr_file);
                }
            }

            let write_line = curr_line.to_string() + "\n";
            new_f.write(&write_line.into_bytes());
        }
    }
}
