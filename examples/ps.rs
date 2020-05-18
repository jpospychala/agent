/* Prints processes list with memory usage
 */
use agent::ps;
use std::io;

fn main() -> std::result::Result<(), io::Error> {
  let r = ps()?;
  println!("pid name rss vsize");
  for e in r {
    println!("{} {} {} {}", e.0.pid, e.0.name, e.0.rss, e.0.vsize);
  }
  Ok(())
}
