use std::time::{Duration, Instant};

use log::info;

pub mod mirai;

fn main() -> Result<(), anyhow::Error> {
  env_logger::init_from_env(
    env_logger::Env::from(env_logger::Env::default())
      .default_filter_or("info"));
  let mut rt = mirai::runtime::new();
  let r = rt.block_on(async {
    let beg = Instant::now();
    mirai::task::sleep(Duration::from_millis(10)).await;
    let end = Instant::now();
    let elapsed = (end - beg).as_millis();
    assert_eq!(elapsed, 10);
    elapsed
  });
  info!("Result: {}", r);
  Ok(())
}
