#![feature(concat_bytes)]

use std::slice;
use std::str::FromStr;

use affinity_helper::{calc_best_affinity, get_mask_cpu0, get_mask_smt_first_processors};
use dynamic_priority::init_dynamic_priority;
use ini::Properties;
use windows::Win32::Foundation::{HANDLE, HINSTANCE};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Threading::{
    GetCurrentProcess, GetCurrentProcessId, SetPriorityClass, SetProcessAffinityMask,
    ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS,
    IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, PROCESS_CREATION_FLAGS, REALTIME_PRIORITY_CLASS,
};

pub mod affinity_helper;
mod dynamic_priority;
mod games;

pub static mut ENABLED: bool = true;
pub static mut PRIORITY: PROCESS_CREATION_FLAGS = HIGH_PRIORITY_CLASS;
pub static mut AFFINITY: usize = 0;
pub static mut DISABLE_CPU0: bool = true;
pub static mut DISABLE_SMT: bool = false;
pub static mut MIN_THREADS: u32 = 6;

static mut CURRENT_PROCESS_HANDLE: HANDLE = HANDLE(0);
static mut CURRENT_PROCESS_HANDLE_ID: u32 = 0;

#[no_mangle]
pub extern "system" fn DllMain(
    #[allow(non_snake_case)] hinstDLL: HINSTANCE,
    #[allow(non_snake_case)] fdwReason: i32,
    #[allow(non_snake_case)] _lpvReserved: isize,
) -> i32 {
    if fdwReason == 1 {
        unsafe {
            CURRENT_PROCESS_HANDLE = GetCurrentProcess();
            CURRENT_PROCESS_HANDLE_ID = GetCurrentProcessId();
        }
        let lib_path: [u16; 1024] = [0; 1024];
        let lib_path_len = unsafe {
            let u8slice: &mut [u16] =
                slice::from_raw_parts_mut(lib_path.as_ptr() as *mut u16, lib_path.len());
            GetModuleFileNameW(hinstDLL, u8slice.as_mut())
        };
        let mut ini_path = unsafe {
            let slice = slice::from_raw_parts(lib_path.as_ptr() as *mut u16, lib_path_len as usize);
            String::from_utf16(&slice).unwrap()
        };
        ini_path.truncate(ini_path.rfind('.').unwrap());
        ini_path.push_str(".ini");

        let ini_exists = std::path::Path::new(&ini_path).exists();
        if ini_exists {
            let ini_file = ini::Ini::load_from_file(&ini_path).unwrap();
            let ini_section = ini_file.section(Some("FallrimPriority")).unwrap();
            unsafe {
                ENABLED = parse_ini_value(ini_section, "enabled", true);

                // These 3 options must be read before Affinity.
                DISABLE_CPU0 = parse_ini_value(ini_section, "disableCPU0", DISABLE_CPU0);
                DISABLE_SMT = parse_ini_value(ini_section, "disableSMT", DISABLE_SMT);
                MIN_THREADS = parse_ini_value(ini_section, "minThreads", MIN_THREADS);

                PRIORITY = parse_priority_class(ini_section.get("priority"));
                AFFINITY = parse_affinity_mask(ini_section.get("affinity"));
            }
            // Load the dynamic priority configuration
            // And run dynamic priority if it's enabled
            init_dynamic_priority(&ini_file)
        } else {
            // If the ini not exists.
            // Use the default value in global variable.
        }
        unsafe {
            SetPriorityClass(CURRENT_PROCESS_HANDLE, PRIORITY)
                .expect("Failed to set process priority");
            SetProcessAffinityMask(CURRENT_PROCESS_HANDLE, AFFINITY)
                .expect("Failed to set process affinity");

            // Debug
            report_debug(
                format!(
                    "Info: \nEnabled:{0}\nPRIORITY:{1}\nAFFINITY:{2}\nDISABLE_CPU0:{3}",
                    ENABLED, PRIORITY.0, AFFINITY, DISABLE_CPU0
                )
                .as_str(),
            );
        }
    }
    return 1;
}

fn parse_ini_value<T: FromStr + 'static>(section: &Properties, key: &str, default_value: T) -> T {
    match section.get(key) {
        Some(str_value) => match str_value.parse::<T>() {
            Ok(parsed_value) => return parsed_value,
            Err(_) => {
                report_error(
                    format!(
                        "Founded key \"{0}\", but unable to parse \"{1}\" as a valid value.",
                        key, str_value
                    )
                    .as_str(),
                );
                return default_value;
            }
        },
        None => {
            report_error(format!("Key \"{0}\" is not founded", key).as_str());
            return default_value;
        }
    }
}
#[cfg(debug_assertions)]
fn report_debug(text: &str) {
    match msgbox::create("Fallrim Priority Debug", text, msgbox::IconType::Info) {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[cfg(not(debug_assertions))]
fn report_debug(text: &str) {}

fn report_error(text: &str) {
    match msgbox::create("Fallrim Priority", text, msgbox::IconType::Error) {
        Ok(_) => {}
        Err(_) => {}
    }
}

fn parse_priority_class(priority_class: Option<&str>) -> PROCESS_CREATION_FLAGS {
    if let Some(priority_class) = priority_class {
        if let Ok(priority_class) = u32::from_str(priority_class) {
            return match priority_class {
                0 => IDLE_PRIORITY_CLASS,
                1 => BELOW_NORMAL_PRIORITY_CLASS,
                2 => NORMAL_PRIORITY_CLASS,
                3 => ABOVE_NORMAL_PRIORITY_CLASS,
                4 => HIGH_PRIORITY_CLASS,
                5 => REALTIME_PRIORITY_CLASS,

                _ => PROCESS_CREATION_FLAGS(0),
            };
        }
    }
    return PROCESS_CREATION_FLAGS(0);
}

fn parse_affinity_mask(affinity: Option<&str>) -> usize {
    match affinity {
        Some(affinity) => {
            let affinity = affinity.to_lowercase();
            let affinity = affinity.as_str().trim();
            match affinity {
                "auto" => {
                    let num_cores = num_cpus::get_physical();
                    let num_logical_processors = num_cpus::get();
                    let mask_smt_first_processors = get_mask_smt_first_processors();
                    let mask_cpu0 = get_mask_cpu0();
                    return unsafe {
                        calc_best_affinity(
                            num_cores,
                            num_logical_processors,
                            mask_smt_first_processors,
                            mask_cpu0,
                            DISABLE_CPU0,
                            DISABLE_SMT,
                            MIN_THREADS,
                        )
                    };
                }
                _ => match usize::from_str_radix(affinity.trim_start_matches("0x"), 16) {
                    Ok(value) => return value,
                    Err(_) => return 0,
                },
            }
        }
        None => return 0,
    }
}
