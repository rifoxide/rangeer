use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::{read_dir, ReadDir};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"([\w\-\.]+@[\w]+[\.]+[\w\-\.]+[\w])"#).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let custom_regex = args.get(2).map_or_else(|| r#"([\w\-\.]+@[\w]+[\.]+[\w\-\.]+[\w])"#.to_string(), |s| s.clone());
    let number_of_content: i128 = args.get(3)
        .map_or_else(
            || 1000000,
            |s| s.parse().unwrap_or(1000000)
        );
    let output_file_name = args.get(4).map_or_else(
            || "out_".to_string(), // Default prefix for the output file name
            |s| s.clone()
        );

    let re = Regex::new(&custom_regex).unwrap();
    let mut count = 1;
    let dir = read_dir(file_path).unwrap();
    let mut st: HashSet<String> = HashSet::new();
    travarse(dir, &mut st, &mut count ,&re,number_of_content,&output_file_name);
    if st.len() > 0 {
        save(&mut st, &mut count,&output_file_name);
    }
}

fn travarse(ls: ReadDir, st: &mut HashSet<String>, count: &mut i32,re: &Regex ,number_of_content:i128,output_file_name:&str) {
    for i in ls {
        let path = i.unwrap();
        if path.file_type().unwrap().is_dir() {
            let dir = read_dir(path.path()).unwrap();
            travarse(dir, st, count,&re,number_of_content,output_file_name);
        } else {
            println!("file: {}", path.path().display());
            let file = File::open(path.path()).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for m in re.find_iter(&line).into_iter() {
                        st.insert(m.as_str().to_string());
                        if st.len() as i128 >= number_of_content {
                            save(st, count,output_file_name);
                        }
                    }
                }
            }
        }
    }
}

fn save(st: &mut HashSet<String>, count: &mut i32,output_file_name:&str) {
   let folder_path = "out";
   if !Path::new(folder_path).exists() {
       fs::create_dir_all(folder_path).unwrap();
   }

   let now = SystemTime::now()
       .duration_since(UNIX_EPOCH)
       .unwrap()
       .as_nanos();
    let file_path = format!("{}/{}_{}_{}.txt", folder_path, output_file_name, count, now);

   *count += 1;

   let file = OpenOptions::new()
       .append(true)
       .create(true)
       .write(true)
       .open(file_path)
       .unwrap();
   let mut writer = LineWriter::new(file);
   
   for i in st.iter() {
       writer.write_all(format!("{}\n", i).as_bytes()).unwrap();
   }

   st.clear();
}
