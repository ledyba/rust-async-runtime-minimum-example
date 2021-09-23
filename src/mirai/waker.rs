use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone)]
pub struct Waker {
  mutex: Arc<Mutex<()>>,
  condvar: std::sync::Arc<std::sync::Condvar>,
}

impl Waker {
  fn new_self(mutex: Arc<Mutex<()>>, condvar: Arc<Condvar>) -> Self {
    Waker{
      mutex,
      condvar,
    }
  }
  pub fn new(mutex: Arc<Mutex<()>>, condvar: Arc<Condvar>) -> std::task::Waker {
    unsafe {
      std::task::Waker::from_raw(Waker::new_self(mutex, condvar).into_raw())
    }
  }
  fn into_raw(self) -> std::task::RawWaker {
    std::task::RawWaker::new(
      Box::into_raw(Box::new(self)).cast::<()>(),
      internal::get_vtable(),
    )
  }
}

mod internal {
  use std::alloc::dealloc;

  use super::Waker;
  unsafe fn clone(p: *const ()) -> std::task::RawWaker {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    waker.clone().into_raw()
  }
  unsafe fn wake(p: *const ()) {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    {
      let _ = waker.mutex.lock().unwrap();
      waker.condvar.notify_one();
    }
    drop(p);
  }
  unsafe fn wake_by_ref(p: *const ()) {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    {
      let _ = waker.mutex.lock().unwrap();
      waker.condvar.notify_one();
    }
  }
  unsafe fn drop(p: *const ()) {
    let p = p.cast::<Waker>();
    let ptr = p as *mut Waker;
    std::ptr::drop_in_place(ptr);
    dealloc(p as *mut u8, std::alloc::Layout::new::<Waker>());
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
