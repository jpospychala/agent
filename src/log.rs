use std::collections::HashMap;
use std::{time, cmp, io};
use std::io::Write;
use crate::Stat;
use sysconf::raw::{sysconf, SysconfVariable};

pub type Pid = usize;

#[derive(Debug)]
pub struct Log {
  start_ts: time::SystemTime,
  processes: HashMap<Pid, ProcessDescr>,
  timeline: Vec<Snapshot>,
}

#[derive(Default, Debug)]
pub struct Snapshot {
  ts: u128,
  processes: Vec<ProcessSnap>,
}

#[derive(Debug, PartialEq)]
pub struct ProcessDescr {
  pid: Pid,
  name: String,
  max_rss: usize,
  utime: usize,
  stime: usize,
  cutime: usize,
  cstime: usize,
  last_utime: usize,
  last_stime: usize,
  last_cutime: usize,
  last_cstime: usize,
}

#[derive(Debug, PartialEq)]
pub struct ProcessSnap {
  pid: Pid,
  rss: usize,
  cpu_usage: usize,
}

impl From<&Stat> for ProcessDescr {
  fn from(stat: &Stat) -> Self {
    ProcessDescr {
      name: stat.name.clone(),
      pid: stat.pid,
      max_rss: 0,
      utime: stat.utime,
      stime: stat.stime,
      cutime: stat.cutime,
      cstime: stat.cstime,
      last_utime: stat.utime,
      last_stime: stat.stime,
      last_cutime: stat.cutime,
      last_cstime: stat.cstime,
    }
  }
}

impl ProcessSnap {
  fn from(stat: &Stat, pdescr: &ProcessDescr) -> Self {
    let cpu_spent = pdescr.utime + pdescr.stime + pdescr.cutime + pdescr.cstime;
    let last_cpu_spent = pdescr.last_utime + pdescr.last_stime + pdescr.last_cutime + pdescr.last_cstime;
    let hertz  = sysconf(SysconfVariable::ScClkTck).unwrap() as usize;
    let seconds = 1;
    let cpu_usage =  ((cpu_spent - last_cpu_spent) * 100 / hertz) / seconds;
    ProcessSnap {
      pid: stat.pid,
      rss: stat.rss,
      cpu_usage,
    }
  }
}

impl Log {
  pub fn new() -> Self {
    Log {
      start_ts: time::SystemTime::now(),
      processes: HashMap::new(),
      timeline: Vec::new(),
    }
  }

  pub fn append(&mut self, ps: Vec<Stat>) {
    for stat in ps.iter() {
      match self.processes.get_mut(&stat.pid) {
       None => { self.processes.insert(stat.pid, ProcessDescr::from(stat)); },
       Some(p)  => {
         p.max_rss = cmp::max(p.max_rss, stat.rss);
         p.last_utime = p.utime;
         p.last_stime = p.stime;
         p.last_cutime = p.cutime;
         p.last_cstime = p.cstime;
         p.utime = stat.utime;
         p.stime = stat.stime;
         p.cstime = stat.cstime;
         p.cutime = stat.cutime;
        },
      };
    }
    let snapshot = Snapshot {
      ts: time::SystemTime::now().duration_since(self.start_ts).unwrap().as_millis(),
      processes: ps.iter().map(|stat| ProcessSnap::from(stat, self.processes.get(&stat.pid).unwrap())).collect()
    };
    self.timeline.push(snapshot);
  }

  pub fn json(&self) -> String {
    let descrs: Vec<String> = self.processes.iter().map(|p| format!("[{},\"{}\"]", p.0, p.1.name)).collect();
    let mut result = format!("{{\"descriptors\":[{}],\n", descrs.join(","));
    result.push_str("\"timeline\":[");
    for t in self.timeline.iter() {
      let snapshot: Vec<String> = t.processes.iter().map(|p| format!("[{},{}]", p.pid, p.rss)).collect();
      result.push_str(&format!("[{},{}]", t.ts,snapshot.join(",")))
    }
    result.push_str("]\n}");
    result
  }

  pub fn dat(&self, mut write: Box<dyn Write>) -> Result<(), io::Error> {
    const M: usize = 1024*1024;
    const PS_COUNT: usize = 10;
    let mut cols: HashMap<usize, usize> = HashMap::new();
    let mut idx = 0;
    write.write(b"\"ts\"")?;

    let mut processes: Vec<&ProcessDescr> = self.processes.values().collect();
    processes.sort_by(|a, b| b.max_rss.cmp(&a.max_rss));
    for p in &processes[0..PS_COUNT] {
      cols.insert(p.pid, idx);
      write.write(format!(" \"{}\"", p.name).as_bytes())?;
      write.write(format!(" \"{}\"", p.name).as_bytes())?;
      idx += 1;
    }
    write.write(b"\n")?;
    for t in self.timeline.iter() {
      let mut values = vec!["-".to_string(); cols.len()];
      for p in t.processes.iter() {
        if let Some(col) = cols.get(&p.pid) {
          values[*col] = format!("{} {}", p.rss/M, p.cpu_usage);
        }
      }
      let line = format!("{} {}\n", t.ts, values.join(" "));
      write.write(line.as_bytes())?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process_descr_from_stat() {
    let stat = having_stat();
    let descr = ProcessDescr::from(&stat);
    assert_eq!(descr, ProcessDescr {
      name: "a b c".to_string(),
      pid: 1,
      max_rss: 0,
      utime: 0,
      stime: 0,
      cutime: 0,
      cstime: 0,
      last_utime: 0,
      last_stime: 0,
      last_cutime: 0,
      last_cstime: 0,
    })
  }

  #[test]
  fn test_process_stat_from_stat() {
    let stat = having_stat();
    let descr = having_descr();
    let pstat = ProcessSnap::from(&stat, &descr);
    assert_eq!(pstat, ProcessSnap {
      pid: 1,
      rss: 98304,
      cpu_usage: 0,
    })
  }

  fn having_stat() -> Stat {
    Stat::from("1 (a b c) R 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52")
  }

  fn having_descr() -> ProcessDescr {
    ProcessDescr {
      pid: 0,
      name: "".to_string(),
      max_rss: 0,
      utime: 0,
      stime: 0,
      cutime: 0,
      cstime: 0,
      last_utime: 0,
      last_stime: 0,
      last_cutime: 0,
      last_cstime: 0,
    }
  }
}