/* Prints top 5 processes by rss
 */
use agent::{Stat, IO, ps};
use std::{thread, time};


fn main() {
  let one_sec = time::Duration::from_millis(1000);
  loop {
    loop_fn();
    thread::sleep(one_sec);
  }
}

fn loop_fn() {
  let mut r: Vec<(Stat, IO)> = ps().unwrap();
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
