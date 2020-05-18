//! # Agent
//!
//! `agent` is a library for reading system information provided by `procfs`.
extern crate sysconf;

use std::{fs, io};
use std::io::prelude::*;
use sysconf::pagesize;

pub mod log;

// List all processes
pub fn ps() -> Result<Vec<(Stat, IO)>, io::Error> {
  let mut v: Vec<(Stat, IO)> = vec!();
  for entry in fs::read_dir("/proc")? {
    let path = entry?.path();
    if path.ends_with("self") {
      continue;
    }
    if path.is_dir() {
      let mut stat_path = path.clone();
      stat_path.push("stat");
      let mut io_path = path.clone();
      io_path.push("io");
      if stat_path.exists() && stat_path.is_file() {
        let mut stat_file = fs::File::open(stat_path)?;
        let mut stat_contents = String::new();
        stat_file.read_to_string(&mut stat_contents)?;

        let mut io_file = fs::File::open(io_path)?;
        let mut io_contents = String::new();
        io_file.read_to_string(&mut io_contents)?;
        v.push((Stat::from(stat_contents.as_str()), IO::from(io_contents.as_str())))
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
  pub rss: usize,  // bytes
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

// Process IO statistics as read in /proc/<pid>/io
#[derive(Debug, PartialEq, Clone)]
pub struct IO {
  pub rchar: usize,
  pub wchar: usize,
  pub syscr: usize,
  pub syscw: usize,
  pub read_bytes: usize,
  pub write_bytes: usize,
  pub cancelled_write_bytes: usize,
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

impl From<&str> for IO {
  fn from(contents: &str) -> Self {
    let words: Vec<&str> = contents.split_ascii_whitespace().collect();

    IO {
      rchar: words[1].parse().unwrap(),
      wchar: words[3].parse().unwrap(),
      syscr: words[5].parse().unwrap(),
      syscw: words[7].parse().unwrap(),
      read_bytes: words[9].parse().unwrap(),
      write_bytes: words[11].parse().unwrap(),
      cancelled_write_bytes: words[13].parse().unwrap(),
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
      rss: 98304,
      rsslim: 25,
      exit_code: 52,
    };
    assert_eq!(Stat::from(stat), pinfo);
  }

  #[test]
  fn test_io_from() {
    let s = "rchar: 1693633473
wchar: 929127024
syscr: 700351
syscw: 208865
read_bytes: 199962624
write_bytes: 580358144
cancelled_write_bytes: 53547008";

    let expected = IO {
      rchar: 1693633473,
      wchar: 929127024,
      syscr: 700351,
      syscw: 208865,
      read_bytes: 199962624,
      write_bytes: 580358144,
      cancelled_write_bytes: 53547008
    };

    assert_eq!(IO::from(s), expected);
  }
}