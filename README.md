# Affinity
[![Build Status](https://github.com/elast0ny/affinity-rs/workflows/build/badge.svg)](https://github.com/elast0ny/affinity-rs/actions?query=workflow%3Abuild)
[![crates.io](https://img.shields.io/crates/v/affinity.svg)](https://crates.io/crates/affinity)
[![mio](https://docs.rs/affinity/badge.svg)](https://docs.rs/affinity/)
[![Lines of Code](https://tokei.rs/b1/github/elast0ny/affinity-rs?category=code)](https://tokei.rs/b1/github/elast0ny/affinity-rs?category=code)


This crate provides a consistent way to set core affinity for currently running threads and processes.

## Usage

```rust
use affinity::*;
fn bind_even_cores() {
    // Select every second core
    let cores: Vec<usize> = (0..get_core_num()).step_by(2).collect();
    println!("Binding thread to cores : {:?}", &cores);
    // Output : "Binding thread to cores : [0, 2, 4, 6]"
    
    set_thread_affinity(&cores).unwrap();
    println!("Current thread affinity : {:?}", get_thread_affinity().unwrap());
    // Output : "Current thread affinity : [0, 2, 4, 6]"
}
```

Complete example [here](https://github.com/elast0ny/affinity-rs/blob/master/examples/main.rs).

## Features

- Bind to multiple cores
- Return list of currently bound cores
- Reliably get number of cores (uses [num_cpus](https://crates.io/crates/num_cpus))
- Allow caller to handle errors
- Supports affinity inheritance for new child processes on Windows (through `set_process_affinity()`)

## Platforms
Currently only tested on :
- Windows
- Linux (Arch x64)
- macOS (see note below)

## macOS Caveats

macOS doesn't allow setting thread or process affinities in the same way as Linux and Windows.
The set_thread_affinity(&cores) call on macOS will only take a single value,
and it is not a core number, but rather a macOS-specific "affinity tag".
The following text may be helpful.

From the file <Kernel/thread_policy.h>

This policy is experimental.

This may be used to express affinity relationships between threads in
the task. Threads with the same affinity tag will be scheduled to
share an L2 cache if possible. That is, affinity tags are a hint to
the scheduler for thread placement.

The namespace of affinity tags is generally local to one task.
However, a child task created after the assignment of affinity tags by
its parent will share that namespace. In particular, a family of
forked processes may be created with a shared affinity namespace.


## License

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
