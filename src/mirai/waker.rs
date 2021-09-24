use std::sync::{Arc, Condvar, Mutex, mpsc::SyncSender};

use log::trace;

#[derive(Clone)]
pub struct Waker {
  sender: SyncSender<()>,
}

impl Waker {
  fn new_self(sender: SyncSender<()>) -> Self {
    Waker{
      sender,
    }
  }
  pub fn new(sender: SyncSender<()>) -> std::task::Waker {
    unsafe {
      std::task::Waker::from_raw(Waker::new_self(sender).into_raw())
    }
  }
  fn into_raw(self) -> std::task::RawWaker {
    let b = Box::into_raw(Box::new(self)).cast::<()>();
    trace!("Created: {:?}", b as *mut u8);
    std::task::RawWaker::new(
      b,
      internal::get_vtable(),
    )
  }
}

mod internal {
  use std::alloc::dealloc;
  use log::trace;
  use super::Waker;
  unsafe fn clone(p: *const ()) -> std::task::RawWaker {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    waker.clone().into_raw()
  }
  unsafe fn wake(p: *const ()) {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    {
      waker.sender.send(());
    }
    drop(p);
  }
  unsafe fn wake_by_ref(p: *const ()) {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    {
      waker.sender.send(());
    }
  }
  unsafe fn drop(p: *const ()) {
    let p = p.cast::<Waker>();
    let ptr = p as *mut Waker;
    std::ptr::drop_in_place(ptr);
    dealloc(p as *mut u8, std::alloc::Layout::new::<Waker>());
    trace!("Dropped: {:?}", p as *mut u8);
  }
  pub fn get_vtable() -> &'static std::task::RawWakerVTable {
    static VTABLE: once_cell::sync::OnceCell<std::task::RawWakerVTable> = once_cell::sync::OnceCell::new();
    VTABLE.get_or_init(|| {
      std::task::RawWakerVTable::new(
        clone,
        wake,
        wake_by_ref,
        drop,
      )
    })
  }
}
