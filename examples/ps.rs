/* Prints processes list with memory usage
 */
use agent::ps;

fn main() {
    let r = ps();
    println!("pid name rss vsize");
    for e in r.unwrap() {
        println!("{} {} {} {}", e.pid, e.name, e.rss, e.vsize);
    }
}
