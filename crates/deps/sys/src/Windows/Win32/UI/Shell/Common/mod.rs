#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COMDLG_FILTERSPEC {
    pub pszName: super::super::super::Foundation::PWSTR,
    pub pszSpec: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMDLG_FILTERSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMDLG_FILTERSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DEVICE_SCALE_FACTOR = i32;
pub const DEVICE_SCALE_FACTOR_INVALID: DEVICE_SCALE_FACTOR = 0i32;
pub const SCALE_100_PERCENT: DEVICE_SCALE_FACTOR = 100i32;
pub const SCALE_120_PERCENT: DEVICE_SCALE_FACTOR = 120i32;
pub const SCALE_125_PERCENT: DEVICE_SCALE_FACTOR = 125i32;
pub const SCALE_140_PERCENT: DEVICE_SCALE_FACTOR = 140i32;
pub const SCALE_150_PERCENT: DEVICE_SCALE_FACTOR = 150i32;
pub const SCALE_160_PERCENT: DEVICE_SCALE_FACTOR = 160i32;
pub const SCALE_175_PERCENT: DEVICE_SCALE_FACTOR = 175i32;
pub const SCALE_180_PERCENT: DEVICE_SCALE_FACTOR = 180i32;
pub const SCALE_200_PERCENT: DEVICE_SCALE_FACTOR = 200i32;
pub const SCALE_225_PERCENT: DEVICE_SCALE_FACTOR = 225i32;
pub const SCALE_250_PERCENT: DEVICE_SCALE_FACTOR = 250i32;
pub const SCALE_300_PERCENT: DEVICE_SCALE_FACTOR = 300i32;
pub const SCALE_350_PERCENT: DEVICE_SCALE_FACTOR = 350i32;
pub const SCALE_400_PERCENT: DEVICE_SCALE_FACTOR = 400i32;
pub const SCALE_450_PERCENT: DEVICE_SCALE_FACTOR = 450i32;
pub const SCALE_500_PERCENT: DEVICE_SCALE_FACTOR = 500i32;
pub type IObjectArray = *mut ::core::ffi::c_void;
pub type IObjectCollection = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct ITEMIDLIST {
    pub mkid: SHITEMID,
}
impl ::core::marker::Copy for ITEMIDLIST {}
impl ::core::clone::Clone for ITEMIDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PERCEIVED = i32;
pub const PERCEIVED_TYPE_FIRST: PERCEIVED = -3i32;
pub const PERCEIVED_TYPE_CUSTOM: PERCEIVED = -3i32;
pub const PERCEIVED_TYPE_UNSPECIFIED: PERCEIVED = -2i32;
pub const PERCEIVED_TYPE_FOLDER: PERCEIVED = -1i32;
pub const PERCEIVED_TYPE_UNKNOWN: PERCEIVED = 0i32;
pub const PERCEIVED_TYPE_TEXT: PERCEIVED = 1i32;
pub const PERCEIVED_TYPE_IMAGE: PERCEIVED = 2i32;
pub const PERCEIVED_TYPE_AUDIO: PERCEIVED = 3i32;
pub const PERCEIVED_TYPE_VIDEO: PERCEIVED = 4i32;
pub const PERCEIVED_TYPE_COMPRESSED: PERCEIVED = 5i32;
pub const PERCEIVED_TYPE_DOCUMENT: PERCEIVED = 6i32;
pub const PERCEIVED_TYPE_SYSTEM: PERCEIVED = 7i32;
pub const PERCEIVED_TYPE_APPLICATION: PERCEIVED = 8i32;
pub const PERCEIVED_TYPE_GAMEMEDIA: PERCEIVED = 9i32;
pub const PERCEIVED_TYPE_CONTACTS: PERCEIVED = 10i32;
pub const PERCEIVED_TYPE_LAST: PERCEIVED = 10i32;
pub const PERCEIVEDFLAG_GDIPLUS: u32 = 16u32;
pub const PERCEIVEDFLAG_HARDCODED: u32 = 2u32;
pub const PERCEIVEDFLAG_NATIVESUPPORT: u32 = 4u32;
pub const PERCEIVEDFLAG_SOFTCODED: u32 = 1u32;
pub const PERCEIVEDFLAG_UNDEFINED: u32 = 0u32;
pub const PERCEIVEDFLAG_WMSDK: u32 = 32u32;
pub const PERCEIVEDFLAG_ZIPFOLDER: u32 = 64u32;
pub type SHCOLSTATE = i32;
pub const SHCOLSTATE_DEFAULT: SHCOLSTATE = 0i32;
pub const SHCOLSTATE_TYPE_STR: SHCOLSTATE = 1i32;
pub const SHCOLSTATE_TYPE_INT: SHCOLSTATE = 2i32;
pub const SHCOLSTATE_TYPE_DATE: SHCOLSTATE = 3i32;
pub const SHCOLSTATE_TYPEMASK: SHCOLSTATE = 15i32;
pub const SHCOLSTATE_ONBYDEFAULT: SHCOLSTATE = 16i32;
pub const SHCOLSTATE_SLOW: SHCOLSTATE = 32i32;
pub const SHCOLSTATE_EXTENDED: SHCOLSTATE = 64i32;
pub const SHCOLSTATE_SECONDARYUI: SHCOLSTATE = 128i32;
pub const SHCOLSTATE_HIDDEN: SHCOLSTATE = 256i32;
pub const SHCOLSTATE_PREFER_VARCMP: SHCOLSTATE = 512i32;
pub const SHCOLSTATE_PREFER_FMTCMP: SHCOLSTATE = 1024i32;
pub const SHCOLSTATE_NOSORTBYFOLDERNESS: SHCOLSTATE = 2048i32;
pub const SHCOLSTATE_VIEWONLY: SHCOLSTATE = 65536i32;
pub const SHCOLSTATE_BATCHREAD: SHCOLSTATE = 131072i32;
pub const SHCOLSTATE_NO_GROUPBY: SHCOLSTATE = 262144i32;
pub const SHCOLSTATE_FIXED_WIDTH: SHCOLSTATE = 4096i32;
pub const SHCOLSTATE_NODPISCALE: SHCOLSTATE = 8192i32;
pub const SHCOLSTATE_FIXED_RATIO: SHCOLSTATE = 16384i32;
pub const SHCOLSTATE_DISPLAYMASK: SHCOLSTATE = 61440i32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct SHELLDETAILS {
    pub fmt: i32,
    pub cxChar: i32,
    pub str: STRRET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SHELLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SHELLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
pub struct SHITEMID {
    pub cb: u16,
    pub abID: [u8; 1],
}
impl ::core::marker::Copy for SHITEMID {}
impl ::core::clone::Clone for SHITEMID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRRET {
    pub uType: u32,
    pub Anonymous: STRRET_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRRET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRRET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union STRRET_0 {
    pub pOleStr: super::super::super::Foundation::PWSTR,
    pub uOffset: u32,
    pub cStr: [u8; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRRET_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRRET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type STRRET_TYPE = i32;
pub const STRRET_WSTR: STRRET_TYPE = 0i32;
pub const STRRET_OFFSET: STRRET_TYPE = 1i32;
pub const STRRET_CSTR: STRRET_TYPE = 2i32;
