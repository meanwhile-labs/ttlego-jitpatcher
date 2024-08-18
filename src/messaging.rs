pub(crate) static mut HAS_CONSOLE: bool = false;

macro_rules! error_log {
	($($arg:tt)*) => {
		unsafe {
      use windows::Win32::Foundation::HANDLE;
      use windows::Win32::System::Console::{AllocConsole, SetStdHandle, STD_OUTPUT_HANDLE};
			if !crate::messaging::HAS_CONSOLE {
				AllocConsole().unwrap();
				SetStdHandle(STD_OUTPUT_HANDLE, HANDLE::default()).unwrap();
				crate::messaging::HAS_CONSOLE = true;
			}
			eprintln!($($arg)*);
		}
	}
}
use std::ffi::CString;

pub(crate) use error_log;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

pub unsafe fn show_message_box(title: &str, message: &str) {
    let title = CString::new(title).unwrap();
    let message = CString::new(message).unwrap();
    MessageBoxA(
        HWND::default(),
        PCSTR::from_raw(message.as_ptr() as *const u8),
        PCSTR::from_raw(title.as_ptr() as *const u8),
        MESSAGEBOX_STYLE(0x00000000),
    );
}
