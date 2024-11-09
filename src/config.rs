use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};

pub static DEBUG_MODE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn enable_debug_mode() {
    println!("Debug mode enabled");
    DEBUG_MODE.store(true, Ordering::Relaxed);
}

pub fn is_debug() -> bool {
    DEBUG_MODE.load(Ordering::Relaxed)
}
