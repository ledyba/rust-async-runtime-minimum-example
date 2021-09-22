use std::{task::Poll, time::{Duration, Instant}};

use futures::Future;
pub fn sleep(duration: Duration) -> Sleep {
  Sleep::new(duration)
}

pub struct Sleep {
  deadline: Instant,
}

impl Sleep {
  pub fn new(duration: Duration) -> Self {
    Sleep {
      deadline: Instant::now() + duration,
    }
  }
  fn has_passed(&self) -> bool {
    self.deadline < Instant::now()
  }
}

impl Future for Sleep {
  type Output = ();

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output>
  {
    if self.has_passed() {
      Poll::Ready(())
    } else {
      cx.waker().wake_by_ref();
      Poll::Pending
    }
  }
}
