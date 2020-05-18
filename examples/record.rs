/* Record memory usage
 */
use agent::{Stat, IO, ps};
use std::{thread, time, env};


fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    println!("usage: record <pattern1> <pattern2>...");
    return
  }
  let name_filters = &args[1..];
  let one_sec = time::Duration::from_millis(1000);
  loop {
    loop_fn(name_filters);
    thread::sleep(one_sec);
  }
}

fn loop_fn(name_filters: &[String]) {
  let rowned = ps().unwrap();
  let mut r: Vec<&(Stat, IO)> = rowned.iter().filter(|s| {
    for name in name_filters {
      if s.0.name.contains(name) {
        return true;
      }
    }
    return false
  }).collect();
  r.sort_by(|a, b| b.0.rss.partial_cmp(&a.0.rss).unwrap());
  let mut max = 5;
  for i in r.iter() {
    println!("{} {} {} {}", i.0.pid, i.0.name, i.0.rss, i.0.vsize);
    max -= 1;
    if max == 0 {
      break;
    }
  }
}
