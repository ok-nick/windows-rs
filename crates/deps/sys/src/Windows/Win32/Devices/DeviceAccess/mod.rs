#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeviceAccessInstance(deviceinterfacepath: super::super::Foundation::PWSTR, desiredaccess: u32, createasync: *mut ICreateDeviceAccessAsync) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_DeviceIoControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 315876210,
    data2: 34635,
    data3: 17789,
    data4: [159, 223, 115, 151, 119, 120, 104, 108],
};
pub const DEV_PORT_1394: u32 = 8u32;
pub const DEV_PORT_ARTI: u32 = 7u32;
pub const DEV_PORT_COM1: u32 = 2u32;
pub const DEV_PORT_COM2: u32 = 3u32;
pub const DEV_PORT_COM3: u32 = 4u32;
pub const DEV_PORT_COM4: u32 = 5u32;
pub const DEV_PORT_DIAQ: u32 = 6u32;
pub const DEV_PORT_MAX: u32 = 9u32;
pub const DEV_PORT_MIN: u32 = 1u32;
pub const DEV_PORT_SIM: u32 = 1u32;
pub const DEV_PORT_USB: u32 = 9u32;
pub const ED_AUDIO_1: i32 = 1i32;
pub const ED_AUDIO_10: i32 = 512i32;
pub const ED_AUDIO_11: i32 = 1024i32;
pub const ED_AUDIO_12: i32 = 2048i32;
pub const ED_AUDIO_13: i32 = 4096i32;
pub const ED_AUDIO_14: i32 = 8192i32;
pub const ED_AUDIO_15: i32 = 16384i32;
pub const ED_AUDIO_16: i32 = 32768i32;
pub const ED_AUDIO_17: i32 = 65536i32;
pub const ED_AUDIO_18: i32 = 131072i32;
pub const ED_AUDIO_19: i32 = 262144i32;
pub const ED_AUDIO_2: i32 = 2i32;
pub const ED_AUDIO_20: i32 = 524288i32;
pub const ED_AUDIO_21: i32 = 1048576i32;
pub const ED_AUDIO_22: i32 = 2097152i32;
pub const ED_AUDIO_23: i32 = 4194304i32;
pub const ED_AUDIO_24: i32 = 8388608i32;
pub const ED_AUDIO_3: i32 = 4i32;
pub const ED_AUDIO_4: i32 = 8i32;
pub const ED_AUDIO_5: i32 = 16i32;
pub const ED_AUDIO_6: i32 = 32i32;
pub const ED_AUDIO_7: i32 = 64i32;
pub const ED_AUDIO_8: i32 = 128i32;
pub const ED_AUDIO_9: i32 = 256i32;
pub const ED_AUDIO_ALL: u32 = 268435456u32;
pub const ED_BASE: i32 = 4096i32;
pub const ED_BOTTOM: u32 = 4u32;
pub const ED_CENTER: u32 = 512u32;
pub const ED_LEFT: u32 = 256u32;
pub const ED_MIDDLE: u32 = 2u32;
pub const ED_RIGHT: u32 = 1024u32;
pub const ED_TOP: u32 = 1u32;
pub const ED_VIDEO: i32 = 33554432i32;
pub type ICreateDeviceAccessAsync = *mut ::core::ffi::c_void;
pub type IDeviceIoControl = *mut ::core::ffi::c_void;
pub type IDeviceRequestCompletionCallback = *mut ::core::ffi::c_void;
