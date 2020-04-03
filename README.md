# Affinity

[![crates.io](https://img.shields.io/crates/v/affinity.svg)](https://crates.io/crates/affinity)
[![mio](https://docs.rs/affinity/badge.svg)](https://docs.rs/affinity/)
![Lines of Code](https://tokei.rs/b1/github/elast0ny/affinity-rs)

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


## License

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
