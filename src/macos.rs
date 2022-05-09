use crate::Result;
use libc::*;

pub fn set_thread_affinity(core_ids: &[usize]) -> Result<()> {
    if core_ids.len() != 1 {
        return Err(From::from(
            "Can only accept a single value/affinity_tag for set_thread_affinity on macos",
        ));
    }

    if let Err(e) = _sched_setaffinity(core_ids[0] as i32) {
        return Err(From::from(format!(
            "sched_setaffinity failed with errno {}",
            e
        )));
    }
    Ok(())
}

pub fn get_thread_affinity() -> Result<Vec<usize>> {
    let mut affinity = Vec::new();
    // let mut set: cpu_set_t = unsafe { zeroed() };

    let x = _sched_getaffinity();
    if let Err(e) = x {
        return Err(From::from(format!(
            "sched_getaffinity failed with errno {}",
            e
        )));
    }

    affinity.push(x.unwrap() as usize);

    Ok(affinity)
}

/* Wrappers around unsafe OS calls */
fn _sched_setaffinity(affinity_tag: i32) -> std::result::Result<(), i32> {
    let mut policy_data = thread_affinity_policy_data_t {
        affinity_tag: affinity_tag,
    };

    let tid = unsafe { mach_thread_self() };

    let res = unsafe {
        thread_policy_set(
            tid,
            THREAD_AFFINITY_POLICY as u32,
            (&mut policy_data) as *mut _ as thread_policy_t,
            1,
        )
    };
    if res != 0 {
        return Err(errno::errno().into());
    }
    Ok(())
}

fn _sched_getaffinity() -> std::result::Result<i32, i32> {
    let mut policy_data = thread_affinity_policy_data_t { affinity_tag: -1 };

    let tid = unsafe { mach_thread_self() };

    // false: we want to get the current value, not the default value. If this is `false` after
    // returning, it means there are no current settings because of other factor, and the
    // default was returned instead.
    let mut get_default: boolean_t = 0;

    let mut count: mach_msg_type_number_t = 1;
    let res = unsafe {
        thread_policy_get(
            tid,
            THREAD_AFFINITY_POLICY as u32,
            (&mut policy_data) as *mut _ as thread_policy_t,
            &mut count,
            &mut get_default,
        )
    };
    if res != 0 {
        return Err(errno::errno().into());
    }

    Ok(policy_data.affinity_tag)
}
