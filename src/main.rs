use log::info;

pub mod mirai;

fn main() -> Result<(), anyhow::Error> {
  env_logger::init_from_env(
    env_logger::Env::from(env_logger::Env::default())
      .default_filter_or("info"));
  let mut rt = mirai::runtime::new();
  let r = rt.block_on(async {
    let k = async { 1 };
    10 + k.await
  });
  info!("Result: {}", r);
  Ok(())
}
