use std::ffi::{c_int, c_void};
use windows::core::{IUnknown, Interface, GUID, HRESULT, PCWSTR};
use windows::Win32::Media::Audio::ERole;

#[repr(C)]
#[derive(Clone)]
pub struct IPolicyConfigVistaClient(IUnknown);
unsafe impl Interface for IPolicyConfigVistaClient {
	type Vtable = IPolicyConfigVistaClient_Vtbl;
	const IID: GUID = GUID::from_u128(0x568b9108_44bf_40b4_9006_86afe5b5a620);
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct IPolicyConfigVistaClient_Vtbl {
	pub base__: windows::core::IUnknown_Vtbl,
	pub GetMixFormat: unsafe extern "system" fn(PCWSTR, *const *const c_void) -> HRESULT,
	pub GetDeviceFormat: unsafe extern "system" fn(PCWSTR, c_int, *const *const c_void) -> HRESULT,
	pub SetDeviceFormat: unsafe extern "system" fn(PCWSTR, *const c_void, *const c_void) -> HRESULT,
	pub GetProcessingPeriod: unsafe extern "system" fn(PCWSTR, c_int, u64, u64) -> HRESULT,
	pub SetProcessingPeriod: unsafe extern "system" fn(PCWSTR, u64) -> HRESULT,
	pub GetShareMode: unsafe extern "system" fn(PCWSTR, *const c_void) -> HRESULT,
	pub SetShareMode: unsafe extern "system" fn(PCWSTR, *const c_void) -> HRESULT,
	pub GetPropertyValue: unsafe extern "system" fn(PCWSTR, *const c_void, *const c_void) -> HRESULT,
	pub SetPropertyValue: unsafe extern "system" fn(PCWSTR, *const c_void, *const c_void) -> HRESULT,
	pub SetDefaultEndpoint: unsafe extern "system" fn(*mut c_void, PCWSTR, ERole) -> HRESULT,
	pub SetEndpointVisibility: unsafe extern "system" fn(PCWSTR, c_int) -> HRESULT,
}