use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use windows::core::Result;

pub struct ComScopeGuard {
	initialized: bool,
}

impl ComScopeGuard {
	pub fn new() -> Result<Self> {
		unsafe {
			CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
			Ok(ComScopeGuard { initialized: true })
		}
	}
}

impl Drop for ComScopeGuard {
	fn drop(&mut self) {
		if self.initialized {
			unsafe {
				CoUninitialize();
			}
		}
	}
}