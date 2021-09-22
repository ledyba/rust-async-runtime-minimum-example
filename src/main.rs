use std::time::{Duration, Instant};

use log::info;

pub mod mirai;

fn main() -> Result<(), anyhow::Error> {
  env_logger::init_from_env(
    env_logger::Env::from(env_logger::Env::default())
      .default_filter_or("trace"));
  let mut rt = mirai::runtime::new();
  let r = rt.block_on(async {
    let beg = Instant::now();
    mirai::task::sleep(Duration::from_millis(10)).await;
    let end = Instant::now();
    (end - beg).as_millis()
  });
  info!("Result: {}", r);
  Ok(())
}
