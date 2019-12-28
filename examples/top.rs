/* Prints processes list sorted by CPU usage
 */
use agent::ps;
use std::{thread, time};

struct Delta<'a> {
    pid: usize,
    ppid: usize,
    name: &'a str,
    delta: isize,
}

fn main() {
    loop {
        loop_fn();
    }
}

fn loop_fn() {
    let one_sec = time::Duration::from_millis(1000);
    let mut ps1 = ps().unwrap();
    thread::sleep(one_sec);
    let mut ps2 = ps().unwrap();
    thread::sleep(one_sec);
    ps2.append(&mut ps1);
    ps2.sort_by(|a, b| a.pid.partial_cmp(&b.pid).unwrap());
    let mut i = ps2.iter();
    let mut prev = i.next().unwrap();

    let mut deltas = vec!();
    for a in i {
        if prev.pid == a.pid {
            let delta = prev.utime as isize - a.utime as isize;
            deltas.push(Delta {
                pid: prev.pid,
                ppid: prev.ppid,
                name: prev.name.as_str(),
                delta,
            });
        }
        prev = a;
    }
    deltas.sort_by(|a, b| a.delta.partial_cmp(&b.delta).unwrap());
    for d in deltas {
        println!("{} {} {} {}", d.pid, d.ppid, d.name, d.delta);
    }
}
