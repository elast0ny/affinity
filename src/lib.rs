//! This crate provides a consistent way to set core affinity for currently running threads and processes.
//!
//! ## Usage
//!
//! ```rust
//! use affinity::*;
//! fn bind_even_cores() {
//!     // Select every second core
//!     let cores: Vec<usize> = (0..get_core_num()).step_by(2).collect();
//!     println!("Binding thread to cores : {:?}", &cores);
//!     // Output : "Binding thread to cores : [0, 2, 4, 6]"
//!     
//!     set_thread_affinity(&cores).unwrap();
//!     println!("Current thread affinity : {:?}", get_thread_affinity().unwrap());
//!     // Output : "Current thread affinity : [0, 2, 4, 6]"
//! }
//! ```
//!
//! See
//!
//! ## Features
//!
//! - Bind to multiple cores
//! - Return list of currently bound cores
//! - Reliably get number of cores (uses [num_cpus](https://crates.io/crates/num_cpus))
//! - Allow caller to handle errors
//! - Supports affinity inheritance for new child processes on Windows (through [set_process_affinity](../x86_64-pc-windows-msvc/affinity/fn.set_process_affinity.html))

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        use windows as os;
        pub use os::{
            set_process_affinity,
            get_process_affinity,
        };
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        use linux as os;
    } else {
        unimplemented!("This crate does not support your OS yet !");
    }
}

/// Binds the current __thread__ to the specified core(s)
pub fn set_thread_affinity<B: AsRef<[usize]>>(core_ids: B) -> Result<()> {
    os::set_thread_affinity(core_ids.as_ref())
}
/// Returns a list of cores that the current __thread__ is bound to
pub fn get_thread_affinity() -> Result<Vec<usize>> {
    os::get_thread_affinity()
}

/// Returns the number of available cores
pub fn get_core_num() -> usize {
    num_cpus::get()
}
