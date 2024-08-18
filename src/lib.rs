use windows::{
    core::{s, Error, PCSTR},
    Win32::{
        Foundation::{BOOL, FALSE, HINSTANCE, HWND, TRUE},
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

#[no_mangle]
pub extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: u32, _reserved: isize) -> BOOL {
    const DLL_PROCESS_ATTACH: u32 = 1;

    match call_reason {
        DLL_PROCESS_ATTACH => match init() {
            Ok(_) => TRUE,
            Err(err) => {
                unsafe {
                    MessageBoxA(
                        HWND::default(),
                        PCSTR::from_raw(err.to_string().as_ptr()),
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

fn init() -> Result<(), Error> {
    Ok(())
}
