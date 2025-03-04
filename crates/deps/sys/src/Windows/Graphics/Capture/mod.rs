#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type Direct3D11CaptureFrame = *mut ::core::ffi::c_void;
pub type Direct3D11CaptureFramePool = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct GraphicsCaptureAccessKind(pub i32);
impl GraphicsCaptureAccessKind {
    pub const Borderless: Self = Self(0i32);
    pub const Programmatic: Self = Self(1i32);
}
impl ::core::marker::Copy for GraphicsCaptureAccessKind {}
impl ::core::clone::Clone for GraphicsCaptureAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GraphicsCaptureItem = *mut ::core::ffi::c_void;
pub type GraphicsCapturePicker = *mut ::core::ffi::c_void;
pub type GraphicsCaptureSession = *mut ::core::ffi::c_void;
