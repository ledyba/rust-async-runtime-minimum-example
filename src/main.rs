use log::{error, info};

pub mod runtime;

fn main() -> Result<(), anyhow::Error> {
  env_logger::init_from_env(
    env_logger::Env::from(env_logger::Env::default())
      .default_filter_or("info"));
  let mut rt = runtime::Runtime::new();
  let r = rt.block_on(async {
    let k = async { 1 };
    10 + k.await
  });
  println!("Result: {}", r);
  Ok(())
}
