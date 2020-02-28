Library for reading system information provided by `procfs`.
------------------------------------------------------------

Currently supports only reading `/proc/[pid]/stat` with basic process information.


Example:

```rust
    /* Prints processes list with memory usage
    */
    extern crate agent;
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

```