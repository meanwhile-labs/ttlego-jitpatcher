use std::ffi::CString;

use windows::{
    core::{s, PCSTR},
    Win32::{
        Foundation::{BOOL, FALSE, HINSTANCE, HWND, TRUE},
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

mod init;
mod messaging;
mod patches;

#[no_mangle]
pub extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: u32, _reserved: isize) -> BOOL {
    const DLL_PROCESS_ATTACH: u32 = 1;

    match call_reason {
        DLL_PROCESS_ATTACH => match init::init() {
            Ok(_) => TRUE,
            Err(err) => {
                unsafe {
                    let message = CString::new(err.to_string()).unwrap();
                    MessageBoxA(
                        HWND::default(),
                        PCSTR::from_raw(message.as_ptr() as *const u8),
                        s!("Error patching game"),
                        MESSAGEBOX_STYLE(0x00000000),
                    );
                }
                FALSE
            }
        },
        _ => TRUE,
    }
}
