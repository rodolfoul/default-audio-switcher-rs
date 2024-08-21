use com_guard::ComScopeGuard;

use windows::core::{Interface, Result, GUID, HSTRING, PCWSTR};
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::{eCommunications, eConsole, eMultimedia, eRender, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL, STGM_READ};
use crate::audio_ses_definitions::IPolicyConfigVistaClient;
use crate::com_guard;
use crate::sink::Sink;

pub struct AudioController {
	com_scope_guard: ComScopeGuard,
	imm_device_enumerator: IMMDeviceEnumerator,
}

impl AudioController {
	pub fn new() -> Result<Self> {
		let controller = AudioController {
			com_scope_guard: ComScopeGuard::new()?,
			imm_device_enumerator: unsafe {
				CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?
			},
		};
		Ok(controller)
	}
	pub fn process_switching(&self, device_a: &str, device_b: &str) {}

	pub fn get_default_endpoint(&self) -> Result<Sink> {
		unsafe {
			let dev = self.imm_device_enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
			let id = dev.GetId()?.to_string()?;
			let name = Self::mmdevice_name(&dev)?;
			Ok(Sink::new(id, name))
		}
	}
	pub fn set_default_audio_sink(&self, device_id: &str) -> Result<()> {
		unsafe {
			let com_interface: IPolicyConfigVistaClient = CoCreateInstance(&GUID::from("294935ce-f637-4e7c-a41b-ab255460b862"), None, CLSCTX_ALL)?;

			let audio_sink_uid = PCWSTR::from_raw(HSTRING::from(device_id).as_ptr());

			let com_ptr = com_interface.as_raw();

			(com_interface.vtable().SetDefaultEndpoint)(com_ptr, audio_sink_uid, eConsole).ok()?;
			(com_interface.vtable().SetDefaultEndpoint)(com_ptr, audio_sink_uid, eMultimedia).ok()?;
			(com_interface.vtable().SetDefaultEndpoint)(com_ptr, audio_sink_uid, eCommunications).ok()?;
		}


		Ok(())
	}

	pub fn list_audio_sinks(&self) -> Result<Vec<Sink>> {
		let mut listing: Vec<Sink>;
		unsafe {
			let device_collection = self.imm_device_enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;

			let sink_count = device_collection.GetCount()?;
			listing = Vec::with_capacity(sink_count as usize);
			for i in 0..sink_count {
				let item = device_collection.Item(i)?;
				let parsed_sink = Sink::new(item.GetId()?.to_string()?, Self::mmdevice_name(&item)?);
				listing.push(parsed_sink);
			}
		}

		Ok(listing)
	}

	fn mmdevice_name(dev: &IMMDevice) -> Result<String> {
		unsafe {
			Ok(dev.OpenPropertyStore(STGM_READ)?.GetValue(&PKEY_Device_FriendlyName)?.to_string())
		}
	}
}