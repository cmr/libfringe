use core::atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};

pub use self::imp::{map_stack, protect_stack, unmap_stack};
use self::imp::sys_page_size;

#[cfg(unix)]
#[path = "unix.rs"]
mod imp;

static PAGE_SIZE_CACHE: AtomicUsize = ATOMIC_USIZE_INIT;
pub fn page_size() -> usize {
  match PAGE_SIZE_CACHE.load(Ordering::Relaxed) {
    0 => {
      let page_size = sys_page_size();
      PAGE_SIZE_CACHE.store(page_size, Ordering::Relaxed);
      page_size
    }
    page_size => page_size
  }
}