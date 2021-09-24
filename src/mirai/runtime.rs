use std::fmt::Debug;
use std::pin::Pin;
use std::future::Future;
use std::sync::mpsc::{SyncSender, Receiver};
use std::task::{Context, Poll};

use log::trace;

pub fn new() -> Runtime {
  let (sender, receiver) = std::sync::mpsc::sync_channel::<()>(10);
  Runtime {
    sender,
    receiver,
  }
}

pub struct Runtime {
  sender: SyncSender<()>,
  receiver: Receiver<()>,
}

impl Runtime {
  pub fn block_on<F, R>(&mut self, f: F) -> R
    where
      F: Future<Output=R> + Send,
      R: Debug,
  {
    let waker = super::waker::Waker::new(self.sender.clone());
    let mut context:Context = Context::from_waker(&waker);
    let mut b: Pin<Box<F>> = Box::pin(f);
    loop {
      let r = b.as_mut().poll(&mut context);
      match r {
        Poll::Pending => {
          trace!("Pending");
          let _ = self.receiver.recv();
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
