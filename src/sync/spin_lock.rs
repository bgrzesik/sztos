
use core::{
    sync::atomic::{
        AtomicBool,
        Ordering
    },
    cell::UnsafeCell
};

pub struct SpinLock<T> {
    lock: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct LockGuard<'l, T> {
    spin: &'l mut SpinLock<T>,
}

impl <'l, T> core::ops::Drop for LockGuard<'l, T> {
    fn drop(&mut self) {
        self.spin.unlock()
    }
}

impl <'l, T> core::ops::Deref for LockGuard<'l, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // self.value.get()
        unreachable!()
    }
}

impl <'l, T> core::ops::DerefMut for LockGuard<'l, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.spin.value.get_mut()
    }
}

impl <T> SpinLock<T> {

    pub const fn new(value :T) -> Self {
        Self {
            lock: AtomicBool::new(true),
            value: UnsafeCell::new(value)
        }
    }

    pub fn lock<'l>(&'l mut self) -> LockGuard<'l, T> {
        loop {
            if self.lock.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                assert!(!self.lock.load(Ordering::SeqCst));
                return LockGuard { spin: self };
            }
        }
    }

    pub fn try_lock<'l> (&'l mut self) -> Option<LockGuard<'l, T>> {
        if self.lock.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            return None
        }

        Some(LockGuard { spin: self })
    }

    pub fn unlock(&mut self) {
        assert!(!self.lock.load(Ordering::SeqCst));
        self.lock.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
        assert!(self.lock.load(Ordering::SeqCst));
    }

}

unsafe impl <T> Sync for SpinLock<T> {}
unsafe impl <T> Send for SpinLock<T> {}
