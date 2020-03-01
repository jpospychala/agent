use agent::{Stat, ps};
use std::{thread, time, env};
use std::collections::HashMap;
use std::cmp;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    println!("usage: agent <root_process> <pattern2>...");
    return
  }

  let root_pid = await_root_pid(&args[1]);
  record(root_pid, &args[1..]);
}

fn await_root_pid(name: &String) -> usize {
  let processes = vec![name.clone()];
  loop {
    let result = loop_fn(&processes[..]);
    if result.len() > 0 {
      return result.get(0).unwrap().pid;
    }
  }
}

fn record(root_pid: usize, name_filters: &[String]) {
  let mut stats: HashMap<usize, ProcessStat> = HashMap::new();
  let one_sec = time::Duration::from_millis(1000);
  const M: usize = 1024*1024;
  let mut root_process_is_running = true;

  while root_process_is_running {
    let processes = loop_fn(name_filters);
    root_process_is_running = processes.iter().filter(|p| p.pid == root_pid).count() > 0;
    if root_process_is_running {
      for process in processes {
        if let Some(stat) = stats.get_mut(&process.pid) {
          stat.update(&process);
        } else {
          stats.insert(process.pid, ProcessStat::new(&process));
        }
      }
      thread::sleep(one_sec);
    }
  }
  for i in stats.values() {
    println!("{} {} {}M {}M", i.pid, i.name, i.min_rss/M, i.max_rss/M);
  }
}

fn loop_fn(name_filters: &[String]) -> Vec<Stat> {
  let rowned = ps().unwrap();
  let mut r: Vec<Stat> = rowned.iter().filter(|s| {
    for name in name_filters {
      if s.name.contains(name) {
        return true;
      }
    }
    return false
  }).cloned().collect();
  r.sort_by(|a, b| b.rss.partial_cmp(&a.rss).unwrap());
  return r;
}

struct ProcessStat {
  pid: usize,
  name: String,
  min_rss: usize,
  max_rss: usize,
}

impl ProcessStat {
  fn new(stat: &Stat) -> Self {
    ProcessStat {
      pid: stat.pid,
      name: String::from(&stat.name[..]),
      min_rss: stat.rss,
      max_rss: stat.rss,
    }
  }
  fn update(&mut self, s: &Stat) {
    self.min_rss = cmp::min(s.rss, self.min_rss);
    self.max_rss = cmp::max(s.rss, self.min_rss);
  }
}