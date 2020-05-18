use agent::{Stat, IO, ps};
use agent::log::Log;
use std::{thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::stdout;
use std::io;
use std::boxed::Box;

fn main() {
  record().unwrap();
}

fn record() -> Result<(), io::Error> {
  let mut log: Log = Log::new();
  let one_sec = time::Duration::from_millis(1000);
  let is_up = Arc::new(AtomicBool::new(true));

  let is_up_clone = Arc::clone(&is_up);
  ctrlc::set_handler(move || {
    is_up_clone.store(false, Ordering::Relaxed);
  })
  .expect("Error setting Ctrl-C handler");

  while is_up.load(Ordering::Relaxed) {
    let ps1 : Vec<(Stat, IO)> = ps().unwrap();
    log.append(ps1);
    thread::sleep(one_sec);
  }

  log.dat(Box::new(stdout()))?;
  Ok(())
}
