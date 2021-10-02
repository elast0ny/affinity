use crate::Result;
use ::libc::*;
use std::mem::{size_of, MaybeUninit};

pub fn set_thread_affinity(core_ids: &[usize]) -> Result<()> {
    #[allow(clippy::uninit_assumed_init)]
    let mut set: cpu_set_t = unsafe { MaybeUninit::uninit().assume_init() };
    unsafe {
        CPU_ZERO(&mut set);
        for core_id in core_ids {
            CPU_SET(*core_id, &mut set);
        }
    }

    if let Err(e) = _sched_setaffinity(0, size_of::<cpu_set_t>(), &set) {
        return Err(From::from(format!(
            "sched_setaffinity failed with errno {}",
            e
        )));
    }
    Ok(())
}

pub fn get_thread_affinity() -> Result<Vec<usize>> {
    let mut affinity = Vec::new();
    #[allow(clippy::uninit_assumed_init)]
    let mut set: cpu_set_t = unsafe { MaybeUninit::uninit().assume_init() };
    unsafe { CPU_ZERO(&mut set) };

    if let Err(e) = _sched_getaffinity(0, size_of::<cpu_set_t>(), &mut set) {
        return Err(From::from(format!(
            "sched_getaffinity failed with errno {}",
            e
        )));
    }

    for i in 0..CPU_SETSIZE as usize {
        if unsafe { CPU_ISSET(i, &set) } {
            affinity.push(i);
        }
    }

    Ok(affinity)
}

/* Wrappers around unsafe OS calls */
fn _sched_setaffinity(
    pid: pid_t,
    cpusetsize: usize,
    mask: &cpu_set_t,
) -> std::result::Result<(), i32> {
    let res = unsafe { sched_setaffinity(pid, cpusetsize, mask) };
    if res != 0 {
        return Err(errno::errno().into());
    }
    Ok(())
}

fn _sched_getaffinity(
    pid: pid_t,
    cpusetsize: usize,
    mask: &mut cpu_set_t,
) -> std::result::Result<(), i32> {
    let res = unsafe { sched_getaffinity(pid, cpusetsize, mask) };
    if res != 0 {
        return Err(errno::errno().into());
    }
    Ok(())
}
