use std::ffi::c_void;

use crate::Result;

extern "system" {
    // Present in all windows processes (kernel32.dll)
    fn GetLastError() -> u32;
    fn GetCurrentProcess() -> *mut c_void;
    fn GetCurrentThread() -> *mut c_void;

    fn SetProcessAffinityMask(process_handle: *mut c_void, mask: usize) -> u32;
    fn GetProcessAffinityMask(
        process_handle: *mut c_void,
        cur_mask: *mut usize,
        system_mask: *mut usize,
    ) -> u32;

    fn SetThreadAffinityMask(thread_handle: *mut c_void, mask: usize) -> usize;
}
const ERROR_INVALID_PARAMETER: u32 = 0x57;

/// Binds the current __process__ to the specified core(s)
///
/// Note : This has a side effect of binding new child processes to the same cores
pub fn set_process_affinity<B: AsRef<[usize]>>(core_ids: B) -> Result<()> {
    let cur_proc = unsafe { GetCurrentProcess() };
    let mut wanted_mask = 0usize;

    // Create the bitmask
    for core_id in core_ids.as_ref() {
        wanted_mask |= 1usize << core_id;
    }

    //println!("Binding process to cores {:b}", wanted_mask);
    if let Err(last_error) = set_process_affinity_mask(cur_proc, wanted_mask) {
        return Err(From::from(format!(
            "SetProcessAffinityMask failed with error 0x{:x}",
            last_error
        )));
    }

    Ok(())
}
/// Returns a list of cores that the current __process__ is bound to
pub fn get_process_affinity() -> Result<Vec<usize>> {
    let mut affinity = Vec::new();
    let mut cur_core = 0usize;
    let mut cur_mask = 1usize;
    let cur_proc = unsafe { GetCurrentProcess() };

    match get_process_affinity_mask(cur_proc) {
        Ok(mask) => {
            while cur_mask != 0 {
                if cur_mask & mask != 0 {
                    affinity.push(cur_core);
                }
                cur_core += 1;
                cur_mask <<= 1;
            }
        }
        Err(last_error) => {
            return Err(From::from(format!(
                "GetProcessAffinityMask failed with error 0x{:x}",
                last_error
            )));
        }
    };

    Ok(affinity)
}

pub fn set_thread_affinity(core_ids: &[usize]) -> Result<()> {
    let cur_thread = unsafe { GetCurrentThread() };
    let mut wanted_mask = 0usize;

    // Create the bitmask
    for core_id in core_ids {
        wanted_mask |= 1usize << core_id;
    }

    //println!("Binding thread to cores {:b}", wanted_mask);
    if let Err(last_error) = set_thread_affinity_mask(cur_thread, wanted_mask) {
        return Err(From::from(format!(
            "SetThreadAffinityMask failed with error 0x{:x}",
            last_error
        )));
    }

    Ok(())
}
pub fn get_thread_affinity() -> Result<Vec<usize>> {
    let mut cur_mask = 1usize;
    let cur_thread = unsafe { GetCurrentThread() };
    let mut cur_core = 1usize;
    let mut affinity = Vec::new();

    // There is not GetThreadAffinityMask on Windows so we must use
    // the return value of SetThreadAffinityMask()
    while cur_mask != 0 {
        cur_core += 1;
        match set_thread_affinity_mask(cur_thread, cur_mask) {
            Ok(mask) => {
                // Restore the old affinity
                if let Err(e) = set_thread_affinity_mask(cur_thread, mask) {
                    return Err(From::from(format!(
                        "SetThreadAffinityMask failed with error 0x{:x}",
                        e
                    )));
                }

                // Count the set bits in the mask
                cur_core = 0;
                cur_mask = 1;
                while cur_mask != 0 {
                    if cur_mask & mask != 0 {
                        affinity.push(cur_core);
                    }
                    cur_core += 1;
                    cur_mask <<= 1;
                }

                break;
            }
            Err(last_error) => {
                // ERROR_INVALID_PARAMETER can happen if we try to set the thread affinity
                // to a core that our global process disabled (Through SetProcessAffinityMask)
                if last_error != ERROR_INVALID_PARAMETER {
                    return Err(From::from(format!(
                        "SetThreadAffinityMask failed with error 0x{:x}",
                        last_error
                    )));
                }

                // Try the next core
                cur_mask <<= 1;
                continue;
            }
        };
    }

    Ok(affinity)
}

/* Wrappers around unsafe OS calls */
fn set_thread_affinity_mask(thread: *mut c_void, mask: usize) -> std::result::Result<usize, u32> {
    let res = unsafe { SetThreadAffinityMask(thread, mask) };
    if res == 0 {
        return Err(unsafe { GetLastError() });
    }
    Ok(res)
}
fn set_process_affinity_mask(process: *mut c_void, mask: usize) -> std::result::Result<(), u32> {
    let res = unsafe { SetProcessAffinityMask(process, mask) };
    if res == 0 {
        return Err(unsafe { GetLastError() });
    }
    Ok(())
}
fn get_process_affinity_mask(process: *mut c_void) -> std::result::Result<usize, u32> {
    let mut cur_mask = 0usize;
    let mut system_mask = 0usize;
    let res = unsafe { GetProcessAffinityMask(process, &mut cur_mask, &mut system_mask) };
    if res == 0 {
        return Err(unsafe { GetLastError() });
    }
    Ok(cur_mask)
}
