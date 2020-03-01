/* Record memory usage
 */
use agent::{Stat, ps};
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
  let mut r: Vec<&Stat> = rowned.iter().filter(|s| {
    for name in name_filters {
      if s.name.contains(name) {
        return true;
      }
    }
    return false
  }).collect();
  r.sort_by(|a, b| b.rss.partial_cmp(&a.rss).unwrap());
  let mut max = 5;
  for i in r.iter() {
    println!("{} {} {} {}", i.pid, i.name, i.rss, i.vsize);
    max -= 1;
    if max == 0 {
      break;
    }
  }
}
