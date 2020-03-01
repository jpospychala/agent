/* Prints top 5 processes by rss
 */
use agent::{Stat, ps};
use std::{thread, time};


fn main() {
  let one_sec = time::Duration::from_millis(1000);
  loop {
    loop_fn();
    thread::sleep(one_sec);
  }
}

fn loop_fn() {
  let mut r: Vec<Stat> = ps().unwrap();
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
