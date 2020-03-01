Measure selected processes memory usage
---------------------------------------

For benchmarking experiments I needed a tool that would measure memory usage of specific processes
for a period of test run. This could be done in many ways, for example as part of test framework
or by using some shell scripts, but I chose to
write a small Rust app. The reason was to have universal solution
not baked into testing framework, small memory footprint, flexible to satisfy any
future needs and obviously to play with Rust.

Usage:

```bash
$ agent
agent <root_process> <pattern1> <pattern2>...
```

The app starts and waits for the `root-process` to run and runs for as long as the `root-process` runs.
`root-process` and `pattern` arguments are process names or parts of.

For example:

```bash
$ agent benchmark-rabbit beam agent > agent.log
$ ./benchmark-rabbit
benchmark done
$ cat agent.log
6452 agent 2M 2M
6469 benchmark-rabbit 1M 8M
6977 beam.smp 37M 37M
6274 beam.smp 89M 200M
```

In example above, `agent` awaits for `benchmark-rabbit` to start and then does it's measurements until
benchmark terminates. Further it prints short 4 column summary containing `pid`, `name`, `min_rss` and `max_rss`.

It all stared by reading process stats stored in `/proc/[pid]/stat`. Once you have it, writing a
`ps` replacement is as simple as few lines of code:

```rust
  fn main() -> std::result::Result<(), io::Error> {
    let r = ps()?;
    println!("pid name rss vsize");
    for e in r {
      println!("{} {} {} {}", e.pid, e.name, e.rss, e.vsize);
    }
    Ok(())
  }

```

Todo
----

1. The output should be json to simplify agregating results into charts.
2. Agent should be integrated into my perf project.
