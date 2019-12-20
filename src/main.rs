use std::{fs, io};
use std::io::prelude::*;

fn main() {
    let r = ps();
    for e in r.unwrap() {
        println!("{:?}", e);
    }
}

fn ps() -> Result<Vec<ProcessInfo>, io::Error> {
    let mut v: Vec<ProcessInfo> = vec!();
    for entry in fs::read_dir("/proc")? {
        let entry = entry?;
        let mut path = entry.path();
        if path.is_dir() {
            path.push("stat");
            if path.exists() && path.is_file() {
                let mut file = fs::File::open(path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                v.push(ProcessInfo::from(contents))
            }
        }
    }
    Ok(v)
}

#[derive(Debug)]
struct ProcessInfo {
    name: String,
    pid: usize,
    rss: usize,  // blocks
    vsize: u32,  // kbytes
}

impl From<String> for ProcessInfo {
    fn from(contents: String) -> Self {
        let stat: Vec<&str> = contents.split_ascii_whitespace().collect();
        const TOTAL_COL_COUNT: usize = 52;
        let name_cols = stat.len() - TOTAL_COL_COUNT;
        let vsize_idx = 23 + name_cols - 1;
        let rss_idx = 24 + name_cols - 1;
        ProcessInfo {
            name: stat[1..name_cols+2].join(" "),
            pid: stat[0].parse().unwrap(),
            rss: stat[rss_idx].parse().unwrap(),
            vsize: stat[vsize_idx].parse::<u32>().unwrap() >> 10,
        }
    }
}