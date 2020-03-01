/* Prints processes list with memory usage
 */
use agent::ps;
use std::io;

fn main() -> std::result::Result<(), io::Error> {
  let r = ps()?;
  println!("pid name rss vsize");
  for e in r {
    println!("{} {} {} {}", e.pid, e.name, e.rss, e.vsize);
  }
  Ok(())
}
