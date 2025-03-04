#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type IWebApplicationActivation = *mut ::core::ffi::c_void;
pub type IWebApplicationAuthoringMode = *mut ::core::ffi::c_void;
pub type IWebApplicationHost = *mut ::core::ffi::c_void;
pub type IWebApplicationNavigationEvents = *mut ::core::ffi::c_void;
pub type IWebApplicationScriptEvents = *mut ::core::ffi::c_void;
pub type IWebApplicationUIEvents = *mut ::core::ffi::c_void;
pub type IWebApplicationUpdateEvents = *mut ::core::ffi::c_void;
pub type RegisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(authoringmodeobject: IWebApplicationAuthoringMode, host: IWebApplicationHost) -> ::windows_sys::core::HRESULT>;
pub type UnregisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(host: IWebApplicationHost) -> ::windows_sys::core::HRESULT>;
