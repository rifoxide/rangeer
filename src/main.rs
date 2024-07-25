use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::{read_dir, ReadDir};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"([\w\-\.]+@[\w]+[\.]+[\w\-\.]+[\w])"#).unwrap();
}

fn main() {
    let mut count = 1;
    let dir = read_dir("./").unwrap();
    let mut st: HashSet<String> = HashSet::new();
    travarse(dir, &mut st, &mut count);
    if st.len() > 0 {
        save(&mut st, &mut count);
    }
}

fn travarse(ls: ReadDir, st: &mut HashSet<String>, count: &mut i32) {
    for i in ls {
        let path = i.unwrap();
        if path.file_type().unwrap().is_dir() {
            let dir = read_dir(path.path()).unwrap();
            travarse(dir, st, count);
        } else {
            println!("file: {}", path.path().display());
            let file = File::open(path.path()).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for m in RE.find_iter(&line).into_iter() {
                        st.insert(m.as_str().to_string());
                        if st.len() >= 10 {
                            save(st, count);
                        }
                    }
                }
            }
        }
    }
}

fn save(st: &mut HashSet<String>, count: &mut i32) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(format!("out_{}_{}.txt", count, now))
        .unwrap();
    *count += 1;
    let mut writer = LineWriter::new(file);
    for i in st.iter() {
        writer.write(format!("{}\n", i).as_bytes()).unwrap();
    }

    st.clear();
}
