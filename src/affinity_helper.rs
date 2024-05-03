use std::ptr::null_mut;
use windows::Win32::System::SystemInformation::{
    GetLogicalProcessorInformationEx, RelationProcessorCore,
    SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
};

pub fn calc_best_affinity(
    num_cores: usize,
    num_logical_processors: usize,
    mask_smt_first_processors: usize,
    mask_cpu0: usize,
    disable_cpu0: bool,
    disable_smt: bool,
    min_threads: u32,
) -> usize {
    let smt_enabled = num_cores != num_logical_processors;
    let mut result: usize = {
        if num_logical_processors >= 64 {
            usize::MAX
        } else {
            (1 << num_logical_processors) - 1
        }
    };

    if disable_cpu0 {
        result &= !mask_cpu0;
    }

    if smt_enabled && disable_smt {
        result &= mask_smt_first_processors;
    }

    // TODO: Implement min_threads
    if min_threads > 0 {}

    return result;
}

pub fn get_mask_smt_first_processors() -> usize {
    let mut result = 0;

    // Get length and create buffer
    let mut len = 0;
    let _ = unsafe {
        GetLogicalProcessorInformationEx(RelationProcessorCore, Some(null_mut()), &mut len)
    };
    let mut buffer = vec![0u8; len as usize];

    let info = buffer.as_mut_ptr() as *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX;
    let _ =
        unsafe { GetLogicalProcessorInformationEx(RelationProcessorCore, Some(info), &mut len) };

    let mut current = info;
    while (current as usize) < (info as usize) + (len as usize) {
        if unsafe { (*current).Relationship } == RelationProcessorCore {
            for i in 0..unsafe { (*current).Anonymous.Processor.GroupCount } {
                let mask = unsafe { (*current).Anonymous.Processor.GroupMask[i as usize].Mask };
                let smt_mask = mask & !(1 << mask.trailing_zeros());
                result |= smt_mask;
            }
        }
        current = ((current as usize) + unsafe { (*current).Size as usize }) as *mut _;
    }
    return result;
}

pub fn get_mask_cpu0() -> usize {
    // Get length and create buffer
    let mut len = 0;
    let _ = unsafe {
        GetLogicalProcessorInformationEx(RelationProcessorCore, Some(null_mut()), &mut len)
    };
    let mut buffer = vec![0u8; len as usize];

    let info = buffer.as_mut_ptr() as *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX;
    let _ =
        unsafe { GetLogicalProcessorInformationEx(RelationProcessorCore, Some(info), &mut len) };

    let mut current = info;
    while (current as usize) < (info as usize) + (len as usize) {
        if unsafe { (*current).Relationship } == RelationProcessorCore {
            for i in 0..unsafe { (*current).Anonymous.Processor.GroupCount } {
                let mask = unsafe { (*current).Anonymous.Processor.GroupMask[i as usize].Mask };
                return mask;
            }
        }
        current = ((current as usize) + unsafe { (*current).Size as usize }) as *mut _;
    }
    return 1;
}
