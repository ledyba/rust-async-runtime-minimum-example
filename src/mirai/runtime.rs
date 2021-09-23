use std::fmt::Debug;
use std::pin::Pin;
use std::future::Future;
use std::sync::{Mutex, Arc, Condvar};
use std::task::{Context, Poll};

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
    let mutex = Arc::new(Mutex::new(false));
    let condvar = Arc::new(Condvar::new());
    let waker = super::waker::Waker::new(mutex.clone(), condvar.clone());
    let mut context:Context = Context::from_waker(&waker);
    let mut b: Pin<Box<F>> = Box::pin(f);
    loop {
      let r = b.as_mut().poll(&mut context);
      match r {
        Poll::Pending => {
          trace!("Pending");
          let _ = condvar.wait(mutex.lock().unwrap()).unwrap();
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
