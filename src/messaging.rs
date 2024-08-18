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
pub(crate) use error_log;
