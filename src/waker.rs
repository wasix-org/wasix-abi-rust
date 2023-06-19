extern crate alloc;
extern crate core;

use alloc::boxed::Box;

// Callback when a waker is woken
#[no_mangle]
pub extern "C" fn _waker_wake(waker: u64) {
    let waker = unsafe {
        let waker = waker as *mut RawWasiWaker;
        Box::from_raw(waker)
    };
    waker.wake();
}

// Callback when a waker is dropped
#[no_mangle]
pub extern "C" fn _waker_drop(waker: u64) {
    let waker = unsafe {
        let waker = waker as *mut RawWasiWaker;
        Box::from_raw(waker)
    };
    drop(waker);
}

pub trait WasiWaker {
    fn wake(&mut self);
}

pub fn register_waker(waker: impl WasiWaker + 'static) -> u64 {
    let waker = Box::new(RawWasiWaker {
        inner: Box::new(waker)
    });
    let waker = Box::into_raw(waker) as *mut RawWasiWaker;
    waker as u64
}

struct RawWasiWaker {
    inner: Box<dyn WasiWaker>,
}

impl RawWasiWaker {
    pub fn wake(mut self) {
        self.inner.wake();
    }
}
