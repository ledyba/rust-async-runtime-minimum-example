use std::fmt::Debug;
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll, Waker};

use log::trace;

pub fn new() -> Runtime {
  Runtime {
  }
}

pub struct Runtime {
}

impl Runtime {
  pub fn block_on<F, R>(&mut self, f: F) -> R
    where
      F: Future<Output=R> + Send,
      R: Debug,
  {
    let waker: Waker = futures::task::noop_waker();
    let mut context:Context = Context::from_waker(&waker);
    let mut b: Pin<Box<F>> = Box::pin(f);
    loop {
      let r = b.as_mut().poll(&mut context);
      match r {
        Poll::Pending => {
          trace!("Pending");
          continue;
        }
        Poll::Ready(r) => {
          trace!("Ready: {:?}", r);
          return r;
        }
      }
    }
  }
}