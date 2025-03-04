#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type PrintSupportExtensionSession = *mut ::core::ffi::c_void;
pub type PrintSupportExtensionTriggerDetails = *mut ::core::ffi::c_void;
pub type PrintSupportPrintDeviceCapabilitiesChangedEventArgs = *mut ::core::ffi::c_void;
pub type PrintSupportPrintTicketValidationRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PrintSupportSessionInfo = *mut ::core::ffi::c_void;
pub type PrintSupportSettingsActivatedEventArgs = *mut ::core::ffi::c_void;
pub type PrintSupportSettingsUISession = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: Self = Self(0i32);
    pub const UserDefaultPrintTicket: Self = Self(1i32);
}
impl ::core::marker::Copy for SettingsLaunchKind {}
impl ::core::clone::Clone for SettingsLaunchKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: Self = Self(0i32);
    pub const Conflicting: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
}
impl ::core::marker::Copy for WorkflowPrintTicketValidationStatus {}
impl ::core::clone::Clone for WorkflowPrintTicketValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
