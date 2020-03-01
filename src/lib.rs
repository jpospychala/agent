//! # Agent
//!
//! `agent` is a library for reading system information provided by `procfs`.
extern crate sysconf;

use std::{fs, io};
use std::io::prelude::*;
use sysconf::pagesize;

// List all processes
pub fn ps() -> Result<Vec<Stat>, io::Error> {
  let mut v: Vec<Stat> = vec!();
  for entry in fs::read_dir("/proc")? {
    let mut path = entry?.path();
    if path.ends_with("self") {
      continue;
    }
    if path.is_dir() {
      path.push("stat");
      if path.exists() && path.is_file() {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        v.push(Stat::from(contents.as_str()))
      }
    }
  }
  Ok(v)
}

// Process stat as read in /proc/<pid>/stat
#[derive(Debug, PartialEq, Clone)]
pub struct Stat {
  pub pid: usize,
  pub name: String,
  // state: R/S/D/Z/T/t/W/X/x/K/P
  pub ppid: usize,
  pub pgrp: usize,
  pub session: usize,
  // tty_nr
  // tpgid
  // flags
  // minflt
  // cminflt
  // majflt
  // cmajflt
  pub utime: usize, // 13
  pub stime: usize,
  pub cutime: usize,
  pub cstime: usize,
  pub priority: isize,
  pub nice: isize,
  pub num_threads: usize,
  pub itrealvalue: usize,
  pub starttime: usize,
  pub vsize: usize,  // kbytes
  pub rss: usize,  // blocks
  pub rsslim: usize,
  // startcode
  // endcode
  // startstack
  // kstkesp
  // ksteip
  // signal
  // blocked
  // sigignore
  // sigcatch
  // wchan
  // nswap
  // cnswap
  // exit_signal
  // processor
  // rt_priority
  // policy
  // delayacct_blkio_ticks
  // guest_time
  // cguest_time
  // start_data
  // end_data
  // start_brk
  // arg_start
  // arg_end
  // env_start
  // env_end
  pub exit_code: usize,
}

impl From<&str> for Stat {
  fn from(contents: &str) -> Self {
    let stat: Vec<&str> = contents.split_ascii_whitespace().collect();
    let name_start = contents.find('(').unwrap() + 1;
    let name_end = contents.rfind(')').unwrap();
    let name = String::from(&contents[name_start..name_end]);

    const TOTAL_COL_COUNT: usize = 52;
    let shift = stat.len() - TOTAL_COL_COUNT;
    Stat {
      pid: stat[0].parse().unwrap(),
      name,
      ppid: stat[3 + shift].parse().unwrap(),
      pgrp: stat[4 + shift].parse().unwrap(),
      session: stat[5 + shift].parse().unwrap(),
      utime: stat[13 + shift].parse().unwrap(),
      stime: stat[14 + shift].parse().unwrap(),
      cutime: stat[15 + shift].parse().unwrap(),
      cstime: stat[16 + shift].parse().unwrap(),
      priority: stat[17 + shift].parse().unwrap(),
      nice: stat[18 + shift].parse().unwrap(),
      num_threads: stat[19 + shift].parse().unwrap(),
      itrealvalue: stat[20 + shift].parse().unwrap(),
      starttime: stat[21 + shift].parse().unwrap(),
      vsize: stat[22 + shift].parse().unwrap(),
      rss: stat[23 + shift].parse::<usize>().unwrap() * pagesize(),
      rsslim: stat[24 + shift].parse().unwrap(),
      exit_code: stat[51 + shift].parse().unwrap(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from() {
    let stat = "1 (a b c) R 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52";
    let pinfo = Stat {
      pid: 1,
      name: String::from("a b c"),
      ppid: 4,
      pgrp: 5,
      session: 6,
      utime: 14,
      stime: 15,
      cutime: 16,
      cstime: 17,
      priority: 18,
      nice: 19,
      num_threads: 20,
      itrealvalue: 21,
      starttime: 22,
      vsize: 23,
      rss: 24,
      rsslim: 25,
      exit_code: 52,
    };
    assert_eq!(Stat::from(stat), pinfo);
  }
}