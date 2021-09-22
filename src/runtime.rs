use std::future::{Future};
use std::task::{Context, Poll, Waker};
use std::fmt::Debug;
use log::info;

pub struct Runtime {
}

impl Runtime {
  pub fn new() -> Self {
    Runtime {
    }
  }

  pub fn block_on<F, R>(&mut self, f: F) -> R
    where
      F: Future<Output=R> + Send,
      R: Debug,
  {
    let waker: Waker = futures::task::noop_waker();
    let mut context = Context::from_waker(&waker);
    let mut b = Box::pin(f);
    loop {
      let r = b.as_mut().poll(&mut context);
      match r {
        Poll::Pending => {
          info!("Pending....");
          continue;
        }
        Poll::Ready(r) => {
          info!("Ready!: {:?}", r);
          return r;
        }
      }
    }
  }
}