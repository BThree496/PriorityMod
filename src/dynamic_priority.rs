use std::{thread, time::Duration};

use ini::Ini;
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    System::Threading::{
        SetPriorityClass, SetProcessAffinityMask, IDLE_PRIORITY_CLASS, PROCESS_CREATION_FLAGS,
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetForegroundWindow, GetWindow, GetWindowThreadProcessId, IsWindowVisible,
        SendMessageTimeoutW, GW_OWNER, SMTO_ABORTIFHUNG, WM_NULL,
    },
};

use crate::{
    parse_affinity_mask, parse_ini_value, parse_priority_class, AFFINITY, CURRENT_PROCESS_HANDLE,
    CURRENT_PROCESS_HANDLE_ID, PRIORITY,
};

pub static mut ENABLED_DYNAMIC_PRIORITY: bool = false;
pub static mut IDLE_PRIORITY: PROCESS_CREATION_FLAGS = IDLE_PRIORITY_CLASS;
pub static mut IDLE_AFFINITY: usize = 0;

static mut GAME_WINDOW_HWND: HWND = HWND(0);
static mut IS_IDLE_MODE: bool = false;

pub fn init_dynamic_priority(ini_file: &Ini) {
    let ini_section = ini_file.section(Some("DynamicPriority")).unwrap();
    unsafe {
        ENABLED_DYNAMIC_PRIORITY =
            parse_ini_value(ini_section, "enabled", ENABLED_DYNAMIC_PRIORITY);

        IDLE_PRIORITY = parse_priority_class(ini_section.get("idle_priority"));
        IDLE_AFFINITY = parse_affinity_mask(ini_section.get("idle_affinity"));

        if ENABLED_DYNAMIC_PRIORITY {
            thread::spawn(thread_fn_dynamic_priority);
        }
    }
}

fn thread_fn_dynamic_priority() {
    find_game_window_handle();
    loop {
        thread::sleep(Duration::from_secs(1));
        unsafe {
            let current_focus_window = GetForegroundWindow();
            if current_focus_window == GAME_WINDOW_HWND {
                // Game Window focused

                if IS_IDLE_MODE == false {
                    continue;
                }

                SetPriorityClass(CURRENT_PROCESS_HANDLE, PRIORITY)
                    .expect("Failed to set process priority");
                SetProcessAffinityMask(CURRENT_PROCESS_HANDLE, AFFINITY)
                    .expect("Failed to set process affinity");
                IS_IDLE_MODE = false;
            } else {
                // Game Window is not focused

                // if the game window is not responding, go back to high priority
                // SendMessageTimeoutW is more accurate than IsHungAppWindow
                if SendMessageTimeoutW(
                    GAME_WINDOW_HWND,
                    WM_NULL,
                    None,
                    None,
                    SMTO_ABORTIFHUNG,
                    500,
                    None,
                )
                .0 == 0
                {
                    // Set back to high priority
                    SetPriorityClass(CURRENT_PROCESS_HANDLE, PRIORITY)
                        .expect("Failed to set process priority");
                    SetProcessAffinityMask(CURRENT_PROCESS_HANDLE, AFFINITY)
                        .expect("Failed to set process affinity");
                    IS_IDLE_MODE = false;
                    continue;
                }

                if IS_IDLE_MODE == true {
                    continue;
                }

                SetPriorityClass(CURRENT_PROCESS_HANDLE, IDLE_PRIORITY)
                    .expect("Failed to set process priority");
                SetProcessAffinityMask(CURRENT_PROCESS_HANDLE, IDLE_AFFINITY)
                    .expect("Failed to set process affinity");
                IS_IDLE_MODE = true;
            }
        }
    }
}

fn find_game_window_handle() {
    unsafe {
        while GAME_WINDOW_HWND.0 == 0 {
            let _ = EnumWindows(Some(enum_windows_callback), None);
            thread::sleep(Duration::from_millis(1000));
        }
    };
}

extern "system" fn enum_windows_callback(hwnd: HWND, _: LPARAM) -> BOOL {
    let mut process_id = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        if CURRENT_PROCESS_HANDLE_ID != process_id || !is_main_window(hwnd) {
            return BOOL::from(true);
        }
        GAME_WINDOW_HWND = hwnd;
    };
    return BOOL::from(false);
}

fn is_main_window(handle: HWND) -> bool {
    return unsafe { GetWindow(handle, GW_OWNER).0 == 0 && IsWindowVisible(handle).as_bool() };
}
