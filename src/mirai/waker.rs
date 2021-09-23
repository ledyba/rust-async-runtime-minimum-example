pub struct Waker {
  condvar: std::sync::Condvar,
}

impl Clone for Waker {
  fn clone(&self) -> Self {
    Self {
      condvar: std::sync::Condvar::new(),
    }
  }
}

impl Waker {
  fn from_raw(self) -> std::task::RawWaker {
    std::task::RawWaker::new(
      Box::into_raw(Box::new(self)).cast::<()>(),
      internal::get_vtable(),
    )
  }
  fn new_raw() -> std::task::RawWaker {
    Waker::from_raw(Waker{
      condvar: std::sync::Condvar::new(),
    })
  }
  pub fn new() -> std::task::Waker {
    unsafe {
      std::task::Waker::from_raw(Waker::new_raw())
    }
  }
}

mod internal {
  use std::alloc::dealloc;

  use super::Waker;
  unsafe fn clone(p: *const ()) -> std::task::RawWaker {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    Waker::from_raw(waker.clone())
  }
  unsafe fn wake(p: *const ()) {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    waker.condvar.notify_one();
    drop(p);
  }
  unsafe fn wake_by_ref(p: *const ()) {
    let waker = &mut *(p.cast::<Waker>() as *mut Waker);
    waker.condvar.notify_one();
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
