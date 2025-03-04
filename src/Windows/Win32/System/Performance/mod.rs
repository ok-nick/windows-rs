#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_Performance_HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
pub const AppearPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49741e9_93a8_4ab1_8e96_bf4482282e9c);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AutoPathFormat(pub i32);
pub const plaNone: AutoPathFormat = AutoPathFormat(0i32);
pub const plaPattern: AutoPathFormat = AutoPathFormat(1i32);
pub const plaComputer: AutoPathFormat = AutoPathFormat(2i32);
pub const plaMonthDayHour: AutoPathFormat = AutoPathFormat(256i32);
pub const plaSerialNumber: AutoPathFormat = AutoPathFormat(512i32);
pub const plaYearDayOfYear: AutoPathFormat = AutoPathFormat(1024i32);
pub const plaYearMonth: AutoPathFormat = AutoPathFormat(2048i32);
pub const plaYearMonthDay: AutoPathFormat = AutoPathFormat(4096i32);
pub const plaYearMonthDayHour: AutoPathFormat = AutoPathFormat(8192i32);
pub const plaMonthDayHourMinute: AutoPathFormat = AutoPathFormat(16384i32);
impl ::core::convert::From<i32> for AutoPathFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutoPathFormat {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BackupPerfRegistryToFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0, szcommentstring: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BackupPerfRegistryToFileW(szfilename: super::super::Foundation::PWSTR, szcommentstring: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(BackupPerfRegistryToFileW(szfilename.into_param().abi(), szcommentstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BootTraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClockType(pub i32);
pub const plaTimeStamp: ClockType = ClockType(0i32);
pub const plaPerformance: ClockType = ClockType(1i32);
pub const plaSystem: ClockType = ClockType(2i32);
pub const plaCycle: ClockType = ClockType(3i32);
impl ::core::convert::From<i32> for ClockType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ClockType {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CommitMode(pub i32);
pub const plaCreateNew: CommitMode = CommitMode(1i32);
pub const plaModify: CommitMode = CommitMode(2i32);
pub const plaCreateOrModify: CommitMode = CommitMode(3i32);
pub const plaUpdateRunningInstance: CommitMode = CommitMode(16i32);
pub const plaFlushTrace: CommitMode = CommitMode(32i32);
pub const plaValidateOnly: CommitMode = CommitMode(4096i32);
impl ::core::convert::From<i32> for CommitMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CommitMode {
    type Abi = Self;
}
pub const CounterItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004348);
pub const CounterItem2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43196c62_c31f_4ce3_a02e_79efe0f6a525);
pub type CounterPathCallBack = unsafe extern "system" fn(param0: usize) -> i32;
pub const CounterPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf948561_ede8_11ce_941e_008029004347);
pub const Counters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2b066d2_2aac_11cf_942f_008029004347);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DICounterItem(pub ::windows::core::IUnknown);
impl DICounterItem {}
unsafe impl ::windows::core::Interface for DICounterItem {
    type Vtable = DICounterItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
}
impl ::core::convert::From<DICounterItem> for ::windows::core::IUnknown {
    fn from(value: DICounterItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DICounterItem> for ::windows::core::IUnknown {
    fn from(value: &DICounterItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DICounterItem> for super::Com::IDispatch {
    fn from(value: DICounterItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DICounterItem> for super::Com::IDispatch {
    fn from(value: &DICounterItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DICounterItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
pub const DIID_DICounterItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc08c4ff2_0e2e_11cf_942c_008029004347);
pub const DIID_DILogFileItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
pub const DIID_DISystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
pub const DIID_DISystemMonitorEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
pub const DIID_DISystemMonitorInternal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DILogFileItem(pub ::windows::core::IUnknown);
impl DILogFileItem {}
unsafe impl ::windows::core::Interface for DILogFileItem {
    type Vtable = DILogFileItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d093ffc_f777_4917_82d1_833fbc54c58f);
}
impl ::core::convert::From<DILogFileItem> for ::windows::core::IUnknown {
    fn from(value: DILogFileItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DILogFileItem> for ::windows::core::IUnknown {
    fn from(value: &DILogFileItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DILogFileItem> for super::Com::IDispatch {
    fn from(value: DILogFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DILogFileItem> for super::Com::IDispatch {
    fn from(value: &DILogFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DILogFileItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DISystemMonitor(pub ::windows::core::IUnknown);
impl DISystemMonitor {}
unsafe impl ::windows::core::Interface for DISystemMonitor {
    type Vtable = DISystemMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13d73d81_c32e_11cf_9398_00aa00a3ddea);
}
impl ::core::convert::From<DISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitor> for super::Com::IDispatch {
    fn from(value: DISystemMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitor> for super::Com::IDispatch {
    fn from(value: &DISystemMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DISystemMonitorEvents(pub ::windows::core::IUnknown);
impl DISystemMonitorEvents {}
unsafe impl ::windows::core::Interface for DISystemMonitorEvents {
    type Vtable = DISystemMonitorEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84979930_4ab3_11cf_943a_008029004347);
}
impl ::core::convert::From<DISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitorEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitorEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorEvents> for super::Com::IDispatch {
    fn from(value: DISystemMonitorEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorEvents> for super::Com::IDispatch {
    fn from(value: &DISystemMonitorEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DISystemMonitorInternal(pub ::windows::core::IUnknown);
impl DISystemMonitorInternal {}
unsafe impl ::windows::core::Interface for DISystemMonitorInternal {
    type Vtable = DISystemMonitorInternal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb242_c32c_11cf_9398_00aa00a3ddea);
}
impl ::core::convert::From<DISystemMonitorInternal> for ::windows::core::IUnknown {
    fn from(value: DISystemMonitorInternal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DISystemMonitorInternal> for ::windows::core::IUnknown {
    fn from(value: &DISystemMonitorInternal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DISystemMonitorInternal> for super::Com::IDispatch {
    fn from(value: DISystemMonitorInternal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DISystemMonitorInternal> for super::Com::IDispatch {
    fn from(value: &DISystemMonitorInternal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &DISystemMonitorInternal {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DISystemMonitorInternal_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
pub const DataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataCollectorSetStatus(pub i32);
pub const plaStopped: DataCollectorSetStatus = DataCollectorSetStatus(0i32);
pub const plaRunning: DataCollectorSetStatus = DataCollectorSetStatus(1i32);
pub const plaCompiling: DataCollectorSetStatus = DataCollectorSetStatus(2i32);
pub const plaPending: DataCollectorSetStatus = DataCollectorSetStatus(3i32);
pub const plaUndefined: DataCollectorSetStatus = DataCollectorSetStatus(4i32);
impl ::core::convert::From<i32> for DataCollectorSetStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DataCollectorSetStatus {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataCollectorType(pub i32);
pub const plaPerformanceCounter: DataCollectorType = DataCollectorType(0i32);
pub const plaTrace: DataCollectorType = DataCollectorType(1i32);
pub const plaConfiguration: DataCollectorType = DataCollectorType(2i32);
pub const plaAlert: DataCollectorType = DataCollectorType(3i32);
pub const plaApiTrace: DataCollectorType = DataCollectorType(4i32);
impl ::core::convert::From<i32> for DataCollectorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DataCollectorType {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataManagerSteps(pub i32);
pub const plaCreateReport: DataManagerSteps = DataManagerSteps(1i32);
pub const plaRunRules: DataManagerSteps = DataManagerSteps(2i32);
pub const plaCreateHtml: DataManagerSteps = DataManagerSteps(4i32);
pub const plaFolderActions: DataManagerSteps = DataManagerSteps(8i32);
pub const plaResourceFreeing: DataManagerSteps = DataManagerSteps(16i32);
impl ::core::convert::From<i32> for DataManagerSteps {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DataManagerSteps {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataSourceTypeConstants(pub i32);
pub const sysmonNullDataSource: DataSourceTypeConstants = DataSourceTypeConstants(-1i32);
pub const sysmonCurrentActivity: DataSourceTypeConstants = DataSourceTypeConstants(1i32);
pub const sysmonLogFiles: DataSourceTypeConstants = DataSourceTypeConstants(2i32);
pub const sysmonSqlLog: DataSourceTypeConstants = DataSourceTypeConstants(3i32);
impl ::core::convert::From<i32> for DataSourceTypeConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DataSourceTypeConstants {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DisplayTypeConstants(pub i32);
pub const sysmonLineGraph: DisplayTypeConstants = DisplayTypeConstants(1i32);
pub const sysmonHistogram: DisplayTypeConstants = DisplayTypeConstants(2i32);
pub const sysmonReport: DisplayTypeConstants = DisplayTypeConstants(3i32);
pub const sysmonChartArea: DisplayTypeConstants = DisplayTypeConstants(4i32);
pub const sysmonChartStackedArea: DisplayTypeConstants = DisplayTypeConstants(5i32);
impl ::core::convert::From<i32> for DisplayTypeConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DisplayTypeConstants {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FileFormat(pub i32);
pub const plaCommaSeparated: FileFormat = FileFormat(0i32);
pub const plaTabSeparated: FileFormat = FileFormat(1i32);
pub const plaSql: FileFormat = FileFormat(2i32);
pub const plaBinary: FileFormat = FileFormat(3i32);
impl ::core::convert::From<i32> for FileFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FileFormat {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FolderActionSteps(pub i32);
pub const plaCreateCab: FolderActionSteps = FolderActionSteps(1i32);
pub const plaDeleteData: FolderActionSteps = FolderActionSteps(2i32);
pub const plaSendCab: FolderActionSteps = FolderActionSteps(4i32);
pub const plaDeleteCab: FolderActionSteps = FolderActionSteps(8i32);
pub const plaDeleteReport: FolderActionSteps = FolderActionSteps(16i32);
impl ::core::convert::From<i32> for FolderActionSteps {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FolderActionSteps {
    type Abi = Self;
}
pub const GeneralPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e5d3d2_1a03_11cf_942d_008029004347);
pub const GraphPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e5d3d3_1a03_11cf_942d_008029004347);
pub const H_WBEM_DATASOURCE: i32 = -1i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAlertDataCollector(pub ::windows::core::IUnknown);
impl IAlertDataCollector {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: <IDataCollectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataCollectorSet>(result__)
    }
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: <DataCollectorType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataCollectorType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: <AutoPathFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AutoPathFormat>(result__)
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AlertThresholds(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAlertThresholds(&self, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(alerts)).ok()
    }
    pub unsafe fn EventLog(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEventLog(&self, log: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(log)).ok()
    }
    pub unsafe fn SampleInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(interval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    pub unsafe fn TaskRunAsSelf(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(runasself)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskUserTextArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskUserTextArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TriggerDataCollectorSet(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTriggerDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAlertDataCollector {
    type Vtable = IAlertDataCollector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837516_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IAlertDataCollector> for ::windows::core::IUnknown {
    fn from(value: IAlertDataCollector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAlertDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IAlertDataCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAlertDataCollector> for IDataCollector {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAlertDataCollector> for IDataCollector {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAlertDataCollector> for super::Com::IDispatch {
    fn from(value: IAlertDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAlertDataCollector> for super::Com::IDispatch {
    fn from(value: &IAlertDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IAlertDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlertDataCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, latest: i16, location: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alerts: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, log: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, log: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interval: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, runasself: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, runasself: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IApiTracingDataCollector(pub ::windows::core::IUnknown);
impl IApiTracingDataCollector {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: <IDataCollectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataCollectorSet>(result__)
    }
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: <DataCollectorType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataCollectorType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: <AutoPathFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AutoPathFormat>(result__)
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn LogApiNamesOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogApiNamesOnly(&self, logapinames: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(logapinames)).ok()
    }
    pub unsafe fn LogApisRecursively(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogApisRecursively(&self, logrecursively: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(logrecursively)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, exepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), exepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFilePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFilePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, logfilepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), logfilepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeModules(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeModules(&self, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(includemodules)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IncludeApis(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIncludeApis(&self, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(includeapis)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeApis(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeApis(&self, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(excludeapis)).ok()
    }
}
unsafe impl ::windows::core::Interface for IApiTracingDataCollector {
    type Vtable = IApiTracingDataCollector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383751a_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IApiTracingDataCollector> for ::windows::core::IUnknown {
    fn from(value: IApiTracingDataCollector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IApiTracingDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IApiTracingDataCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IApiTracingDataCollector> for IDataCollector {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApiTracingDataCollector> for IDataCollector {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IApiTracingDataCollector> for super::Com::IDispatch {
    fn from(value: IApiTracingDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IApiTracingDataCollector> for super::Com::IDispatch {
    fn from(value: &IApiTracingDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IApiTracingDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApiTracingDataCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, latest: i16, location: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logapinames: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logapinames: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logrecursively: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logrecursively: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, exepath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, exepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logfilepath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, includemodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, includeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, excludeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConfigurationDataCollector(pub ::windows::core::IUnknown);
impl IConfigurationDataCollector {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: <IDataCollectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataCollectorSet>(result__)
    }
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: <DataCollectorType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataCollectorType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: <AutoPathFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AutoPathFormat>(result__)
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn FileMaxCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFileMaxCount(&self, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn FileMaxRecursiveDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFileMaxRecursiveDepth(&self, depth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth)).ok()
    }
    pub unsafe fn FileMaxTotalSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFileMaxTotalSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Files(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFiles(&self, files: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(files)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ManagementQueries(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetManagementQueries(&self, queries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(queries)).ok()
    }
    pub unsafe fn QueryNetworkAdapters(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetQueryNetworkAdapters(&self, network: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(network)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistryKeys(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRegistryKeys(&self, query: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(query)).ok()
    }
    pub unsafe fn RegistryMaxRecursiveDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(depth)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SystemStateFile(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSystemStateFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IConfigurationDataCollector {
    type Vtable = IConfigurationDataCollector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837514_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IConfigurationDataCollector> for ::windows::core::IUnknown {
    fn from(value: IConfigurationDataCollector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConfigurationDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IConfigurationDataCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IConfigurationDataCollector> for IDataCollector {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConfigurationDataCollector> for IDataCollector {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IConfigurationDataCollector> for super::Com::IDispatch {
    fn from(value: IConfigurationDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IConfigurationDataCollector> for super::Com::IDispatch {
    fn from(value: &IConfigurationDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IConfigurationDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConfigurationDataCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, latest: i16, location: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, depth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, depth: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, files: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, files: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, queries: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, queries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, network: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, network: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, query: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, query: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, depth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, depth: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICounterItem(pub ::windows::core::IUnknown);
impl ICounterItem {
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICounterItem {
    type Vtable = ICounterItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x771a9520_ee28_11ce_941e_008029004347);
}
impl ::core::convert::From<ICounterItem> for ::windows::core::IUnknown {
    fn from(value: ICounterItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICounterItem> for ::windows::core::IUnknown {
    fn from(value: &ICounterItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICounterItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iscale: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstrvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICounterItem2(pub ::windows::core::IUnknown);
impl ICounterItem2 {
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn SetSelected(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn Selected(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetVisible(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn Visible(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iwhich), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for ICounterItem2 {
    type Vtable = ICounterItem2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeefcd4e1_ea1c_4435_b7f4_e341ba03b4f9);
}
impl ::core::convert::From<ICounterItem2> for ::windows::core::IUnknown {
    fn from(value: ICounterItem2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICounterItem2> for ::windows::core::IUnknown {
    fn from(value: &ICounterItem2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ICounterItem2> for ICounterItem {
    fn from(value: ICounterItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICounterItem2> for ICounterItem {
    fn from(value: &ICounterItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICounterItem> for ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICounterItem> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICounterItem> for &ICounterItem2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICounterItem> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounterItem2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iscale: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstrvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: i32, iwhich: SysmonDataType, pvariant: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICounters(pub ::windows::core::IUnknown);
impl ICounters {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<DICounterItem> {
        let mut result__: <DICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<DICounterItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pathname: Param0) -> ::windows::core::Result<DICounterItem> {
        let mut result__: <DICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pathname.into_param().abi(), &mut result__).from_abi::<DICounterItem>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ICounters {
    type Vtable = ICounters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79167962_28fc_11cf_942f_008029004347);
}
impl ::core::convert::From<ICounters> for ::windows::core::IUnknown {
    fn from(value: ICounters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICounters> for ::windows::core::IUnknown {
    fn from(value: &ICounters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICounters> for super::Com::IDispatch {
    fn from(value: ICounters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICounters> for super::Com::IDispatch {
    fn from(value: &ICounters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ICounters {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICounters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plong: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataCollector(pub ::windows::core::IUnknown);
impl IDataCollector {
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: <IDataCollectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataCollectorSet>(result__)
    }
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: <DataCollectorType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataCollectorType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: <AutoPathFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AutoPathFormat>(result__)
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDataCollector {
    type Vtable = IDataCollector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x038374ff_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IDataCollector> for ::windows::core::IUnknown {
    fn from(value: IDataCollector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IDataCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollector> for super::Com::IDispatch {
    fn from(value: IDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollector> for super::Com::IDispatch {
    fn from(value: &IDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, latest: i16, location: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataCollectorCollection(pub ::windows::core::IUnknown);
impl IDataCollectorCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IDataCollector> {
        let mut result__: <IDataCollector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<IDataCollector>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IDataCollector>>(&self, collector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), collector.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, collector: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), collector.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorCollection>>(&self, collectors: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), collectors.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDataCollectorFromXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrxml: Param0, pvalidation: *mut ::core::option::Option<IValueMap>, pcollector: *mut ::core::option::Option<IDataCollector>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrxml.into_param().abi(), ::core::mem::transmute(pvalidation), ::core::mem::transmute(pcollector)).ok()
    }
    pub unsafe fn CreateDataCollector(&self, r#type: DataCollectorType) -> ::windows::core::Result<IDataCollector> {
        let mut result__: <IDataCollector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<IDataCollector>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDataCollectorCollection {
    type Vtable = IDataCollectorCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837502_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IDataCollectorCollection> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataCollectorCollection> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorCollection> for super::Com::IDispatch {
    fn from(value: IDataCollectorCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorCollection> for super::Com::IDispatch {
    fn from(value: &IDataCollectorCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollectorCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collector: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collector: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collectors: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvalidation: *mut ::windows::core::RawPtr, pcollector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: DataCollectorType, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataCollectorSet(pub ::windows::core::IUnknown);
impl IDataCollectorSet {
    pub unsafe fn DataCollectors(&self) -> ::windows::core::Result<IDataCollectorCollection> {
        let mut result__: <IDataCollectorCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataCollectorCollection>(result__)
    }
    pub unsafe fn Duration(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetDuration(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DescriptionUnresolved(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, displayname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), displayname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayNameUnresolved(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Keywords(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetKeywords(&self, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(keywords)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RootPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRootPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, folder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), folder.into_param().abi()).ok()
    }
    pub unsafe fn Segment(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSegment(&self, segment: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(segment)).ok()
    }
    pub unsafe fn SegmentMaxDuration(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSegmentMaxDuration(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn SegmentMaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSegmentMaxSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    pub unsafe fn SerialNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSerialNumber(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Server(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<DataCollectorSetStatus> {
        let mut result__: <DataCollectorSetStatus as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataCollectorSetStatus>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subdirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubdirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, folder: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), folder.into_param().abi()).ok()
    }
    pub unsafe fn SubdirectoryFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: <AutoPathFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AutoPathFormat>(result__)
    }
    pub unsafe fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubdirectoryFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubdirectoryFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    pub unsafe fn TaskRunAsSelf(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(runasself)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, task: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), task.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TaskUserTextArguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTaskUserTextArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, usertext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), usertext.into_param().abi()).ok()
    }
    pub unsafe fn Schedules(&self) -> ::windows::core::Result<IScheduleCollection> {
        let mut result__: <IScheduleCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IScheduleCollection>(result__)
    }
    pub unsafe fn SchedulesEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSchedulesEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsecurity: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), bstrsecurity.into_param().abi()).ok()
    }
    pub unsafe fn StopOnCompletion(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStopOnCompletion(&self, stop: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(stop)).ok()
    }
    pub unsafe fn DataManager(&self) -> ::windows::core::Result<IDataManager> {
        let mut result__: <IDataManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataManager>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, user: Param0, password: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), user.into_param().abi(), password.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, server: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), name.into_param().abi(), server.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, server: Param1, mode: CommitMode) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), name.into_param().abi(), server.into_param().abi(), ::core::mem::transmute(mode), &mut result__).from_abi::<IValueMap>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Start(&self, synchronous: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(synchronous)).ok()
    }
    pub unsafe fn Stop(&self, synchronous: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), ::core::mem::transmute(synchronous)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), xml.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), key.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), key.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDataCollectorSet {
    type Vtable = IDataCollectorSet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837520_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IDataCollectorSet> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorSet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataCollectorSet> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorSet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSet> for super::Com::IDispatch {
    fn from(value: IDataCollectorSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSet> for super::Com::IDispatch {
    fn from(value: &IDataCollectorSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollectorSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, collectors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displayname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, keywords: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, segment: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, segment: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, server: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: *mut DataCollectorSetStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, runasself: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, runasself: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usertext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usertext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppschedules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrsecurity: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrsecurity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stop: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stop: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, datamanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mode: CommitMode, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, synchronous: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, synchronous: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataCollectorSetCollection(pub ::windows::core::IUnknown);
impl IDataCollectorSetCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: <IDataCollectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<IDataCollectorSet>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, set: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), set.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, set: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), set.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSetCollection>>(&self, sets: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), sets.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDataCollectorSets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0, filter: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), server.into_param().abi(), filter.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDataCollectorSetCollection {
    type Vtable = IDataCollectorSetCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837524_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IDataCollectorSetCollection> for ::windows::core::IUnknown {
    fn from(value: IDataCollectorSetCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataCollectorSetCollection> for ::windows::core::IUnknown {
    fn from(value: &IDataCollectorSetCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataCollectorSetCollection> for super::Com::IDispatch {
    fn from(value: IDataCollectorSetCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataCollectorSetCollection> for super::Com::IDispatch {
    fn from(value: &IDataCollectorSetCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataCollectorSetCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSetCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, set: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, set: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, set: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sets: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataManager(pub ::windows::core::IUnknown);
impl IDataManager {
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, fenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fenabled)).ok()
    }
    pub unsafe fn CheckBeforeRunning(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetCheckBeforeRunning(&self, fcheck: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fcheck)).ok()
    }
    pub unsafe fn MinFreeDisk(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMinFreeDisk(&self, minfreedisk: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(minfreedisk)).ok()
    }
    pub unsafe fn MaxSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxSize(&self, ulmaxsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxsize)).ok()
    }
    pub unsafe fn MaxFolderCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxfoldercount)).ok()
    }
    pub unsafe fn ResourcePolicy(&self) -> ::windows::core::Result<ResourcePolicy> {
        let mut result__: <ResourcePolicy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ResourcePolicy>(result__)
    }
    pub unsafe fn SetResourcePolicy(&self, policy: ResourcePolicy) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(policy)).ok()
    }
    pub unsafe fn FolderActions(&self) -> ::windows::core::Result<IFolderActionCollection> {
        let mut result__: <IFolderActionCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFolderActionCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportSchema(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReportSchema<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, reportschema: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), reportschema.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReportFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pbstrfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RuleTargetFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRuleTargetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventsFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventsFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), pbstrfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Rules(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRules<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrxml: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), bstrxml.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Run<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, steps: DataManagerSteps, bstrfolder: Param1) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(steps), bstrfolder.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Extract<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, cabfilename: Param0, destinationpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), cabfilename.into_param().abi(), destinationpath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDataManager {
    type Vtable = IDataManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837541_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IDataManager> for ::windows::core::IUnknown {
    fn from(value: IDataManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataManager> for ::windows::core::IUnknown {
    fn from(value: &IDataManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDataManager> for super::Com::IDispatch {
    fn from(value: IDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDataManager> for super::Com::IDispatch {
    fn from(value: &IDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fenabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfcheck: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fcheck: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minfreedisk: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minfreedisk: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulmaxsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulmaxsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulmaxfoldercount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulmaxfoldercount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppolicy: *mut ResourcePolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, policy: ResourcePolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reportschema: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reportschema: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, steps: DataManagerSteps, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cabfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destinationpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFolderAction(pub ::windows::core::IUnknown);
impl IFolderAction {
    pub unsafe fn Age(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAge(&self, ulage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulage)).ok()
    }
    pub unsafe fn Size(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSize(&self, ulage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulage)).ok()
    }
    pub unsafe fn Actions(&self) -> ::windows::core::Result<FolderActionSteps> {
        let mut result__: <FolderActionSteps as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FolderActionSteps>(result__)
    }
    pub unsafe fn SetActions(&self, steps: FolderActionSteps) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(steps)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendCabTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSendCabTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdestination: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrdestination.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IFolderAction {
    type Vtable = IFolderAction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837543_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IFolderAction> for ::windows::core::IUnknown {
    fn from(value: IFolderAction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFolderAction> for ::windows::core::IUnknown {
    fn from(value: &IFolderAction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderAction> for super::Com::IDispatch {
    fn from(value: IFolderAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderAction> for super::Com::IDispatch {
    fn from(value: &IFolderAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IFolderAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderAction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulage: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulage: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulage: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulage: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, steps: *mut FolderActionSteps) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, steps: FolderActionSteps) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdestination: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFolderActionCollection(pub ::windows::core::IUnknown);
impl IFolderActionCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IFolderAction> {
        let mut result__: <IFolderAction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<IFolderAction>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, IFolderAction>>(&self, action: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), action.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IFolderActionCollection>>(&self, actions: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), actions.into_param().abi()).ok()
    }
    pub unsafe fn CreateFolderAction(&self) -> ::windows::core::Result<IFolderAction> {
        let mut result__: <IFolderAction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IFolderAction>(result__)
    }
}
unsafe impl ::windows::core::Interface for IFolderActionCollection {
    type Vtable = IFolderActionCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837544_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IFolderActionCollection> for ::windows::core::IUnknown {
    fn from(value: IFolderActionCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFolderActionCollection> for ::windows::core::IUnknown {
    fn from(value: &IFolderActionCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFolderActionCollection> for super::Com::IDispatch {
    fn from(value: IFolderActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFolderActionCollection> for super::Com::IDispatch {
    fn from(value: &IFolderActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IFolderActionCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderActionCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#enum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, action: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, actions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folderaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILogFileItem(pub ::windows::core::IUnknown);
impl ILogFileItem {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for ILogFileItem {
    type Vtable = ILogFileItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b518dd_05c7_418a_89e6_4f9ce8c6841e);
}
impl ::core::convert::From<ILogFileItem> for ::windows::core::IUnknown {
    fn from(value: ILogFileItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILogFileItem> for ::windows::core::IUnknown {
    fn from(value: &ILogFileItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILogFileItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstrvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILogFiles(pub ::windows::core::IUnknown);
impl ILogFiles {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<DILogFileItem> {
        let mut result__: <DILogFileItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<DILogFileItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pathname: Param0) -> ::windows::core::Result<DILogFileItem> {
        let mut result__: <DILogFileItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pathname.into_param().abi(), &mut result__).from_abi::<DILogFileItem>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), index.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ILogFiles {
    type Vtable = ILogFiles_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a2a97e6_6851_41ea_87ad_2a8225335865);
}
impl ::core::convert::From<ILogFiles> for ::windows::core::IUnknown {
    fn from(value: ILogFiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILogFiles> for ::windows::core::IUnknown {
    fn from(value: &ILogFiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILogFiles> for super::Com::IDispatch {
    fn from(value: ILogFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILogFiles> for super::Com::IDispatch {
    fn from(value: &ILogFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ILogFiles {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFiles_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plong: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPerformanceCounterDataCollector(pub ::windows::core::IUnknown);
impl IPerformanceCounterDataCollector {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: <IDataCollectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataCollectorSet>(result__)
    }
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: <DataCollectorType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataCollectorType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: <AutoPathFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AutoPathFormat>(result__)
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DataSourceName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDataSourceName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dsn: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), dsn.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PerformanceCounters(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPerformanceCounters(&self, counters: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(counters)).ok()
    }
    pub unsafe fn LogFileFormat(&self) -> ::windows::core::Result<FileFormat> {
        let mut result__: <FileFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<FileFormat>(result__)
    }
    pub unsafe fn SetLogFileFormat(&self, format: FileFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    pub unsafe fn SampleInterval(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(interval)).ok()
    }
    pub unsafe fn SegmentMaxRecords(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSegmentMaxRecords(&self, records: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(records)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPerformanceCounterDataCollector {
    type Vtable = IPerformanceCounterDataCollector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837506_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IPerformanceCounterDataCollector> for ::windows::core::IUnknown {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPerformanceCounterDataCollector> for ::windows::core::IUnknown {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPerformanceCounterDataCollector> for IDataCollector {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerformanceCounterDataCollector> for IDataCollector {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPerformanceCounterDataCollector> for super::Com::IDispatch {
    fn from(value: IPerformanceCounterDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPerformanceCounterDataCollector> for super::Com::IDispatch {
    fn from(value: &IPerformanceCounterDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IPerformanceCounterDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerformanceCounterDataCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, latest: i16, location: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dsn: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, counters: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, counters: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut FileFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: FileFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interval: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, records: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, records: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISchedule(pub ::windows::core::IUnknown);
impl ISchedule {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StartDate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetStartDate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, start: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), start.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EndDate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetEndDate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, end: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), end.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, start: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), start.into_param().abi()).ok()
    }
    pub unsafe fn Days(&self) -> ::windows::core::Result<WeekDays> {
        let mut result__: <WeekDays as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WeekDays>(result__)
    }
    pub unsafe fn SetDays(&self, days: WeekDays) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(days)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISchedule {
    type Vtable = ISchedule_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383753a_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<ISchedule> for ::windows::core::IUnknown {
    fn from(value: ISchedule) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISchedule> for ::windows::core::IUnknown {
    fn from(value: &ISchedule) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISchedule> for super::Com::IDispatch {
    fn from(value: ISchedule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISchedule> for super::Com::IDispatch {
    fn from(value: &ISchedule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISchedule {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISchedule_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, start: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, end: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, end: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, start: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, days: *mut WeekDays) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, days: WeekDays) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IScheduleCollection(pub ::windows::core::IUnknown);
impl IScheduleCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<ISchedule> {
        let mut result__: <ISchedule as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<ISchedule>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ISchedule>>(&self, pschedule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pschedule.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vschedule: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), vschedule.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IScheduleCollection>>(&self, pschedules: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pschedules.into_param().abi()).ok()
    }
    pub unsafe fn CreateSchedule(&self) -> ::windows::core::Result<ISchedule> {
        let mut result__: <ISchedule as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISchedule>(result__)
    }
}
unsafe impl ::windows::core::Interface for IScheduleCollection {
    type Vtable = IScheduleCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383753d_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IScheduleCollection> for ::windows::core::IUnknown {
    fn from(value: IScheduleCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IScheduleCollection> for ::windows::core::IUnknown {
    fn from(value: &IScheduleCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IScheduleCollection> for super::Com::IDispatch {
    fn from(value: IScheduleCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IScheduleCollection> for super::Com::IDispatch {
    fn from(value: &IScheduleCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IScheduleCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduleCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppschedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pschedule: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vschedule: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pschedules: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, schedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISystemMonitor(pub ::windows::core::IUnknown);
impl ISystemMonitor {
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: <super::Ole::IFontDisp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: <ICounters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ICounters>(result__)
    }
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: <DisplayTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DisplayTypeConstants>(result__)
    }
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: <ICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), &mut result__).from_abi::<ICounterItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: <ICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), bspath.into_param().abi(), &mut result__).from_abi::<ICounterItem>(result__)
    }
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: <ReportValueTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ReportValueTypeConstants>(result__)
    }
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: <ILogFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ILogFiles>(result__)
    }
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: <DataSourceTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitor {
    type Vtable = ISystemMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x194eb241_c32c_11cf_9398_00aa00a3ddea);
}
impl ::core::convert::From<ISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISystemMonitor> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISystemMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iappearance: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fvalue: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstitle: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstitle: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bsfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqldsnname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqllogsetname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISystemMonitor2(pub ::windows::core::IUnknown);
impl ISystemMonitor2 {
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: <super::Ole::IFontDisp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: <ICounters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ICounters>(result__)
    }
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: <DisplayTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DisplayTypeConstants>(result__)
    }
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: <ICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), &mut result__).from_abi::<ICounterItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: <ICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), bspath.into_param().abi(), &mut result__).from_abi::<ICounterItem>(result__)
    }
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: <ReportValueTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ReportValueTypeConstants>(result__)
    }
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: <ILogFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ILogFiles>(result__)
    }
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: <DataSourceTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn EnableDigitGrouping(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnableToolTips(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn EnableToolTips(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowTimeAxisLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetChartScroll(&self, bscroll: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(bscroll)).ok()
    }
    pub unsafe fn ChartScroll(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewcount)).ok()
    }
    pub unsafe fn DataPointCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(bselectedcountersonly)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Relog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype), ::core::mem::transmute(ifilter)).ok()
    }
    pub unsafe fn ClearData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn LogSourceStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn LogSourceStopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    pub unsafe fn BatchingLock(&self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(flock), ::core::mem::transmute(ebatchreason)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsettingfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), bstrsettingfilename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitor2 {
    type Vtable = ISystemMonitor2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08e3206a_5fd2_4fde_a8a5_8cb3b63d2677);
}
impl ::core::convert::From<ISystemMonitor2> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitor2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISystemMonitor2> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitor2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISystemMonitor2> for ISystemMonitor {
    fn from(value: ISystemMonitor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISystemMonitor2> for ISystemMonitor {
    fn from(value: &ISystemMonitor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemMonitor> for ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemMonitor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISystemMonitor> for &ISystemMonitor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISystemMonitor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitor2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iappearance: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fvalue: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstitle: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstitle: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bsfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqldsnname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqllogsetname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bscroll: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbscroll: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inewcount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pidatapointcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bselectedcountersonly: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISystemMonitorEvents(pub ::windows::core::IUnknown);
impl ISystemMonitorEvents {
    pub unsafe fn OnCounterSelected(&self, index: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn OnCounterAdded(&self, index: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn OnCounterDeleted(&self, index: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn OnSampleCollected(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn OnDblClick(&self, index: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
}
unsafe impl ::windows::core::Interface for ISystemMonitorEvents {
    type Vtable = ISystemMonitorEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee660ea0_4abd_11cf_943a_008029004347);
}
impl ::core::convert::From<ISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: ISystemMonitorEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISystemMonitorEvents> for ::windows::core::IUnknown {
    fn from(value: &ISystemMonitorEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISystemMonitorEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMonitorEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITraceDataCollector(pub ::windows::core::IUnknown);
impl ITraceDataCollector {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn DataCollectorSet(&self) -> ::windows::core::Result<IDataCollectorSet> {
        let mut result__: <IDataCollectorSet as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDataCollectorSet>(result__)
    }
    pub unsafe fn SetDataCollectorSet<'a, Param0: ::windows::core::IntoParam<'a, IDataCollectorSet>>(&self, group: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), group.into_param().abi()).ok()
    }
    pub unsafe fn DataCollectorType(&self) -> ::windows::core::Result<DataCollectorType> {
        let mut result__: <DataCollectorType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataCollectorType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn FileNameFormat(&self) -> ::windows::core::Result<AutoPathFormat> {
        let mut result__: <AutoPathFormat as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AutoPathFormat>(result__)
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(format)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNameFormatPattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNameFormatPattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pattern.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LatestOutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLatestOutputLocation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), path.into_param().abi()).ok()
    }
    pub unsafe fn LogAppend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogAppend(&self, append: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(append)).ok()
    }
    pub unsafe fn LogCircular(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogCircular(&self, circular: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(circular)).ok()
    }
    pub unsafe fn LogOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogOverwrite(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OutputLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetIndex(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXml<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, xml: Param0) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), xml.into_param().abi(), &mut result__).from_abi::<IValueMap>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOutputLocation(&self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(latest), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBufferSize(&self, size: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    pub unsafe fn BuffersLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBuffersLost(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    pub unsafe fn BuffersWritten(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBuffersWritten(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    pub unsafe fn ClockType(&self) -> ::windows::core::Result<ClockType> {
        let mut result__: <ClockType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ClockType>(result__)
    }
    pub unsafe fn SetClockType(&self, clock: ClockType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(clock)).ok()
    }
    pub unsafe fn EventsLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetEventsLost(&self, events: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(events)).ok()
    }
    pub unsafe fn ExtendedModes(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetExtendedModes(&self, mode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn FlushTimer(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFlushTimer(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn FreeBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFreeBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), guid.into_param().abi()).ok()
    }
    pub unsafe fn IsKernelTrace(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn MaximumBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaximumBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    pub unsafe fn MinimumBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMinimumBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    pub unsafe fn NumberOfBuffers(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNumberOfBuffers(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    pub unsafe fn PreallocateFile(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPreallocateFile(&self, allocate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(allocate)).ok()
    }
    pub unsafe fn ProcessMode(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetProcessMode(&self, process: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(process)).ok()
    }
    pub unsafe fn RealTimeBuffersLost(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetRealTimeBuffersLost(&self, buffers: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffers)).ok()
    }
    pub unsafe fn SessionId(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn SetSessionId(&self, id: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SessionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSessionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn SessionThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSessionThreadId(&self, tid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(tid)).ok()
    }
    pub unsafe fn StreamMode(&self) -> ::windows::core::Result<StreamMode> {
        let mut result__: <StreamMode as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<StreamMode>(result__)
    }
    pub unsafe fn SetStreamMode(&self, mode: StreamMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn TraceDataProviders(&self) -> ::windows::core::Result<ITraceDataProviderCollection> {
        let mut result__: <ITraceDataProviderCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITraceDataProviderCollection>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITraceDataCollector {
    type Vtable = ITraceDataCollector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383750b_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<ITraceDataCollector> for ::windows::core::IUnknown {
    fn from(value: ITraceDataCollector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITraceDataCollector> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITraceDataCollector> for IDataCollector {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITraceDataCollector> for IDataCollector {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDataCollector> for &ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, IDataCollector> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataCollector> for super::Com::IDispatch {
    fn from(value: ITraceDataCollector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataCollector> for super::Com::IDispatch {
    fn from(value: &ITraceDataCollector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ITraceDataCollector {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: *mut AutoPathFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: AutoPathFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, append: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, circular: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overwrite: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, latest: i16, location: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clock: *mut ClockType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clock: ClockType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, events: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, events: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kernel: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allocate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allocate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, process: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, process: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffers: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: *mut StreamMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: StreamMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITraceDataProvider(pub ::windows::core::IUnknown);
impl ITraceDataProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), guid.into_param().abi()).ok()
    }
    pub unsafe fn Level(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IValueMap>(result__)
    }
    pub unsafe fn KeywordsAny(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IValueMap>(result__)
    }
    pub unsafe fn KeywordsAll(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IValueMap>(result__)
    }
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IValueMap>(result__)
    }
    pub unsafe fn FilterEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFilterEnabled(&self, filterenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(filterenabled)).ok()
    }
    pub unsafe fn FilterType(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFilterType(&self, ultype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(ultype)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilterData(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFilterData(&self, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstrserver: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), bstrserver.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resolve<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, pfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pfrom.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, sddl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), sddl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurity(&self, securityinfo: u32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(securityinfo), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetRegisteredProcesses(&self) -> ::windows::core::Result<IValueMap> {
        let mut result__: <IValueMap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IValueMap>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITraceDataProvider {
    type Vtable = ITraceDataProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837512_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<ITraceDataProvider> for ::windows::core::IUnknown {
    fn from(value: ITraceDataProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITraceDataProvider> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProvider> for super::Com::IDispatch {
    fn from(value: ITraceDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProvider> for super::Com::IDispatch {
    fn from(value: &ITraceDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ITraceDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pplevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filterenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filterenabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pultype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ultype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdata: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfrom: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, securityinfo: u32, sddl: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, processes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITraceDataProviderCollection(pub ::windows::core::IUnknown);
impl ITraceDataProviderCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<ITraceDataProvider> {
        let mut result__: <ITraceDataProvider as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<ITraceDataProvider>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ITraceDataProvider>>(&self, pprovider: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pprovider.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vprovider: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), vprovider.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, ITraceDataProviderCollection>>(&self, providers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), providers.into_param().abi()).ok()
    }
    pub unsafe fn CreateTraceDataProvider(&self) -> ::windows::core::Result<ITraceDataProvider> {
        let mut result__: <ITraceDataProvider as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITraceDataProvider>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTraceDataProviders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), server.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTraceDataProvidersByProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, server: Param0, pid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), server.into_param().abi(), ::core::mem::transmute(pid)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITraceDataProviderCollection {
    type Vtable = ITraceDataProviderCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837510_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<ITraceDataProviderCollection> for ::windows::core::IUnknown {
    fn from(value: ITraceDataProviderCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITraceDataProviderCollection> for ::windows::core::IUnknown {
    fn from(value: &ITraceDataProviderCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITraceDataProviderCollection> for super::Com::IDispatch {
    fn from(value: ITraceDataProviderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITraceDataProviderCollection> for super::Com::IDispatch {
    fn from(value: &ITraceDataProviderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ITraceDataProviderCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProviderCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vprovider: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IValueMap(pub ::windows::core::IUnknown);
impl IValueMap {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, index: Param0) -> ::windows::core::Result<IValueMapItem> {
        let mut result__: <IValueMapItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), index.into_param().abi(), &mut result__).from_abi::<IValueMapItem>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    pub unsafe fn ValueMapType(&self) -> ::windows::core::Result<ValueMapType> {
        let mut result__: <ValueMapType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ValueMapType>(result__)
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddRange<'a, Param0: ::windows::core::IntoParam<'a, IValueMap>>(&self, map: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), map.into_param().abi()).ok()
    }
    pub unsafe fn CreateValueMapItem(&self) -> ::windows::core::Result<IValueMapItem> {
        let mut result__: <IValueMapItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IValueMapItem>(result__)
    }
}
unsafe impl ::windows::core::Interface for IValueMap {
    type Vtable = IValueMap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837534_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IValueMap> for ::windows::core::IUnknown {
    fn from(value: IValueMap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IValueMap> for ::windows::core::IUnknown {
    fn from(value: &IValueMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMap> for super::Com::IDispatch {
    fn from(value: IValueMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMap> for super::Com::IDispatch {
    fn from(value: &IValueMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IValueMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueMap_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut ValueMapType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ValueMapType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, map: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IValueMapItem(pub ::windows::core::IUnknown);
impl IValueMapItem {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Key(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, key: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), key.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    pub unsafe fn ValueMapType(&self) -> ::windows::core::Result<ValueMapType> {
        let mut result__: <ValueMapType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ValueMapType>(result__)
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
}
unsafe impl ::windows::core::Interface for IValueMapItem {
    type Vtable = IValueMapItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837533_098b_11d8_9414_505054503030);
}
impl ::core::convert::From<IValueMapItem> for ::windows::core::IUnknown {
    fn from(value: IValueMapItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IValueMapItem> for ::windows::core::IUnknown {
    fn from(value: &IValueMapItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IValueMapItem> for super::Com::IDispatch {
    fn from(value: IValueMapItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IValueMapItem> for super::Com::IDispatch {
    fn from(value: &IValueMapItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IValueMapItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueMapItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: *mut ValueMapType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ValueMapType) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallPerfDllA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szcomputername: Param0, lpinifile: Param1, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallPerfDllA(szcomputername: super::super::Foundation::PSTR, lpinifile: super::super::Foundation::PSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(InstallPerfDllA(szcomputername.into_param().abi(), lpinifile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallPerfDllW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szcomputername: Param0, lpinifile: Param1, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallPerfDllW(szcomputername: super::super::Foundation::PWSTR, lpinifile: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(InstallPerfDllW(szcomputername.into_param().abi(), lpinifile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const LIBID_SystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b773e42_2509_11cf_942f_008029004347);
pub const LegacyDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPerfCounterTextStringsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(LoadPerfCounterTextStringsA(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPerfCounterTextStringsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(LoadPerfCounterTextStringsW(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const LogFileItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ec5be8_df93_4237_94e4_9ee918111d71);
pub const LogFiles: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735d9fd_f6b9_4f19_a5d9_e2d068584bc5);
pub const MAX_COUNTER_PATH: u32 = 256u32;
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
pub const PDH_ACCESS_DENIED: i32 = -1073738789i32;
pub const PDH_ASYNC_QUERY_TIMEOUT: i32 = -2147481637i32;
pub const PDH_BINARY_LOG_CORRUPT: i32 = -1073738761i32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: super::super::Foundation::PSTR,
    pub szReturnPathBuffer: super::super::Foundation::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: ::core::option::Option<CounterPathCallBack>,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_BROWSE_DLG_CONFIG_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_A {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_BROWSE_DLG_CONFIG_A")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_A {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.szDataSource == other.szDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack.map(|f| f as usize) == other.pCallBack.map(|f| f as usize)
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_A {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: super::super::Foundation::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: ::core::option::Option<CounterPathCallBack>,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_BROWSE_DLG_CONFIG_HA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_HA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_BROWSE_DLG_CONFIG_HA")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HA {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.hDataSource == other.hDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack.map(|f| f as usize) == other.pCallBack.map(|f| f as usize)
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_HA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: super::super::Foundation::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: ::core::option::Option<CounterPathCallBack>,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_BROWSE_DLG_CONFIG_HW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_HW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_HW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_BROWSE_DLG_CONFIG_HW")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("hDataSource", &self.hDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_HW {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.hDataSource == other.hDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack.map(|f| f as usize) == other.pCallBack.map(|f| f as usize)
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_HW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_HW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub szDataSource: super::super::Foundation::PWSTR,
    pub szReturnPathBuffer: super::super::Foundation::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: ::core::option::Option<CounterPathCallBack>,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_BROWSE_DLG_CONFIG_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_BROWSE_DLG_CONFIG_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_BROWSE_DLG_CONFIG_W {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_BROWSE_DLG_CONFIG_W")
            .field("_bitfield", &self._bitfield)
            .field("hWndOwner", &self.hWndOwner)
            .field("szDataSource", &self.szDataSource)
            .field("szReturnPathBuffer", &self.szReturnPathBuffer)
            .field("cchReturnPathLength", &self.cchReturnPathLength)
            .field("dwCallBackArg", &self.dwCallBackArg)
            .field("CallBackStatus", &self.CallBackStatus)
            .field("dwDefaultDetailLevel", &self.dwDefaultDetailLevel)
            .field("szDialogBoxCaption", &self.szDialogBoxCaption)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_BROWSE_DLG_CONFIG_W {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
            && self.hWndOwner == other.hWndOwner
            && self.szDataSource == other.szDataSource
            && self.szReturnPathBuffer == other.szReturnPathBuffer
            && self.cchReturnPathLength == other.cchReturnPathLength
            && self.pCallBack.map(|f| f as usize) == other.pCallBack.map(|f| f as usize)
            && self.dwCallBackArg == other.dwCallBackArg
            && self.CallBackStatus == other.CallBackStatus
            && self.dwDefaultDetailLevel == other.dwDefaultDetailLevel
            && self.szDialogBoxCaption == other.szDialogBoxCaption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_BROWSE_DLG_CONFIG_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_BROWSE_DLG_CONFIG_W {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const PDH_CALC_NEGATIVE_DENOMINATOR: i32 = -2147481642i32;
pub const PDH_CALC_NEGATIVE_TIMEBASE: i32 = -2147481641i32;
pub const PDH_CALC_NEGATIVE_VALUE: i32 = -2147481640i32;
pub const PDH_CANNOT_CONNECT_MACHINE: i32 = -1073738813i32;
pub const PDH_CANNOT_CONNECT_WMI_SERVER: i32 = -1073738776i32;
pub const PDH_CANNOT_READ_NAME_STRINGS: i32 = -1073738808i32;
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: i32 = -2147481636i32;
pub const PDH_COUNTER_ALREADY_IN_QUERY: i32 = -1073738762i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: super::super::Foundation::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: super::super::Foundation::PSTR,
    pub DataBuffer: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: super::super::Foundation::PSTR,
    pub szObjectName: super::super::Foundation::PSTR,
    pub szInstanceName: super::super::Foundation::PSTR,
    pub szParentInstance: super::super::Foundation::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_INFO_A_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName && self.szObjectName == other.szObjectName && self.szInstanceName == other.szInstanceName && self.szParentInstance == other.szParentInstance && self.dwInstanceIndex == other.dwInstanceIndex && self.szCounterName == other.szCounterName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_A_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: super::super::Foundation::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: super::super::Foundation::PWSTR,
    pub DataBuffer: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: super::super::Foundation::PWSTR,
    pub szObjectName: super::super::Foundation::PWSTR,
    pub szInstanceName: super::super::Foundation::PWSTR,
    pub szParentInstance: super::super::Foundation::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_INFO_W_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName && self.szObjectName == other.szObjectName && self.szInstanceName == other.szInstanceName && self.szParentInstance == other.szParentInstance && self.dwInstanceIndex == other.dwInstanceIndex && self.szCounterName == other.szCounterName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_INFO_W_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: super::super::Foundation::PSTR,
    pub szObjectName: super::super::Foundation::PSTR,
    pub szInstanceName: super::super::Foundation::PSTR,
    pub szParentInstance: super::super::Foundation::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_A {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_COUNTER_PATH_ELEMENTS_A")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName && self.szObjectName == other.szObjectName && self.szInstanceName == other.szInstanceName && self.szParentInstance == other.szParentInstance && self.dwInstanceIndex == other.dwInstanceIndex && self.szCounterName == other.szCounterName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_PATH_ELEMENTS_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: super::super::Foundation::PWSTR,
    pub szObjectName: super::super::Foundation::PWSTR,
    pub szInstanceName: super::super::Foundation::PWSTR,
    pub szParentInstance: super::super::Foundation::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_COUNTER_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_COUNTER_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_COUNTER_PATH_ELEMENTS_W {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_COUNTER_PATH_ELEMENTS_W")
            .field("szMachineName", &self.szMachineName)
            .field("szObjectName", &self.szObjectName)
            .field("szInstanceName", &self.szInstanceName)
            .field("szParentInstance", &self.szParentInstance)
            .field("dwInstanceIndex", &self.dwInstanceIndex)
            .field("szCounterName", &self.szCounterName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_COUNTER_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName && self.szObjectName == other.szObjectName && self.szInstanceName == other.szInstanceName && self.szParentInstance == other.szParentInstance && self.dwInstanceIndex == other.dwInstanceIndex && self.szCounterName == other.szCounterName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_COUNTER_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_COUNTER_PATH_ELEMENTS_W {
    type Abi = Self;
}
pub const PDH_CSTATUS_BAD_COUNTERNAME: i32 = -1073738816i32;
pub const PDH_CSTATUS_INVALID_DATA: i32 = -1073738822i32;
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: i32 = -2147481645i32;
pub const PDH_CSTATUS_NEW_DATA: i32 = 1i32;
pub const PDH_CSTATUS_NO_COUNTER: i32 = -1073738823i32;
pub const PDH_CSTATUS_NO_COUNTERNAME: i32 = -1073738817i32;
pub const PDH_CSTATUS_NO_INSTANCE: i32 = -2147481647i32;
pub const PDH_CSTATUS_NO_MACHINE: i32 = -2147481648i32;
pub const PDH_CSTATUS_NO_OBJECT: i32 = -1073738824i32;
pub const PDH_CSTATUS_VALID_DATA: i32 = 0i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: super::super::Foundation::PSTR,
    pub ObjectGUID: ::windows::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_DATA_ITEM_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_A").field("szMachineName", &self.szMachineName).field("ObjectGUID", &self.ObjectGUID).field("dwItemId", &self.dwItemId).field("szInstanceName", &self.szInstanceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName && self.ObjectGUID == other.ObjectGUID && self.dwItemId == other.dwItemId && self.szInstanceName == other.szInstanceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: super::super::Foundation::PWSTR,
    pub ObjectGUID: ::windows::core::GUID,
    pub dwItemId: u32,
    pub szInstanceName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_DATA_ITEM_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_DATA_ITEM_PATH_ELEMENTS_W").field("szMachineName", &self.szMachineName).field("ObjectGUID", &self.ObjectGUID).field("dwItemId", &self.dwItemId).field("szInstanceName", &self.szInstanceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn eq(&self, other: &Self) -> bool {
        self.szMachineName == other.szMachineName && self.ObjectGUID == other.ObjectGUID && self.dwItemId == other.dwItemId && self.szInstanceName == other.szInstanceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    type Abi = Self;
}
pub const PDH_DATA_SOURCE_IS_LOG_FILE: i32 = -1073738802i32;
pub const PDH_DATA_SOURCE_IS_REAL_TIME: i32 = -1073738801i32;
pub const PDH_DIALOG_CANCELLED: i32 = -2147481639i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDH_DLL_VERSION(pub u32);
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = PDH_DLL_VERSION(1280u32);
pub const PDH_VERSION: PDH_DLL_VERSION = PDH_DLL_VERSION(1283u32);
impl ::core::convert::From<u32> for PDH_DLL_VERSION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PDH_DLL_VERSION {
    type Abi = Self;
}
impl ::core::ops::BitOr for PDH_DLL_VERSION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PDH_DLL_VERSION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PDH_DLL_VERSION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PDH_DLL_VERSION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PDH_DLL_VERSION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PDH_END_OF_LOG_FILE: i32 = -2147481638i32;
pub const PDH_ENTRY_NOT_IN_LOG_FILE: i32 = -1073738803i32;
pub const PDH_FILE_ALREADY_EXISTS: i32 = -1073738798i32;
pub const PDH_FILE_NOT_FOUND: i32 = -1073738799i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDH_FMT(pub u32);
pub const PDH_FMT_DOUBLE: PDH_FMT = PDH_FMT(512u32);
pub const PDH_FMT_LARGE: PDH_FMT = PDH_FMT(1024u32);
pub const PDH_FMT_LONG: PDH_FMT = PDH_FMT(256u32);
impl ::core::convert::From<u32> for PDH_FMT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PDH_FMT {
    type Abi = Self;
}
impl ::core::ops::BitOr for PDH_FMT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PDH_FMT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PDH_FMT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PDH_FMT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PDH_FMT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_FMT_COUNTERVALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: i32,
    pub doubleValue: f64,
    pub largeValue: i64,
    pub AnsiStringValue: super::super::Foundation::PSTR,
    pub WideStringValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_FMT_COUNTERVALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: super::super::Foundation::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_FMT_COUNTERVALUE_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_ITEM_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: super::super::Foundation::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_FMT_COUNTERVALUE_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_FMT_COUNTERVALUE_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_FMT_COUNTERVALUE_ITEM_W {
    type Abi = Self;
}
pub const PDH_FUNCTION_NOT_FOUND: i32 = -1073738818i32;
pub const PDH_INCORRECT_APPEND_TIME: i32 = -1073738757i32;
pub const PDH_INSUFFICIENT_BUFFER: i32 = -1073738814i32;
pub const PDH_INVALID_ARGUMENT: i32 = -1073738819i32;
pub const PDH_INVALID_BUFFER: i32 = -1073738815i32;
pub const PDH_INVALID_DATA: i32 = -1073738810i32;
pub const PDH_INVALID_DATASOURCE: i32 = -1073738787i32;
pub const PDH_INVALID_HANDLE: i32 = -1073738820i32;
pub const PDH_INVALID_INSTANCE: i32 = -1073738811i32;
pub const PDH_INVALID_PATH: i32 = -1073738812i32;
pub const PDH_INVALID_SQLDB: i32 = -1073738786i32;
pub const PDH_INVALID_SQL_LOG_FORMAT: i32 = -1073738763i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDH_LOG(pub u32);
pub const PDH_LOG_READ_ACCESS: PDH_LOG = PDH_LOG(65536u32);
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = PDH_LOG(131072u32);
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = PDH_LOG(262144u32);
impl ::core::convert::From<u32> for PDH_LOG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PDH_LOG {
    type Abi = Self;
}
impl ::core::ops::BitOr for PDH_LOG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PDH_LOG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PDH_LOG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PDH_LOG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PDH_LOG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PDH_LOGSVC_NOT_OPENED: i32 = -1073738791i32;
pub const PDH_LOGSVC_QUERY_NOT_FOUND: i32 = -1073738792i32;
pub const PDH_LOG_FILE_CREATE_ERROR: i32 = -1073738807i32;
pub const PDH_LOG_FILE_OPEN_ERROR: i32 = -1073738806i32;
pub const PDH_LOG_FILE_TOO_SMALL: i32 = -1073738788i32;
pub const PDH_LOG_SAMPLE_TOO_SMALL: i32 = -1073738760i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: super::super::Foundation::PSTR,
    pub szDefaultDir: super::super::Foundation::PSTR,
    pub szBaseFileName: super::super::Foundation::PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: super::super::Foundation::PSTR,
    pub PdlCounterList: super::super::Foundation::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct")
            .field("PdlAutoNameInterval", &self.PdlAutoNameInterval)
            .field("PdlAutoNameUnits", &self.PdlAutoNameUnits)
            .field("PdlCommandFilename", &self.PdlCommandFilename)
            .field("PdlCounterList", &self.PdlCounterList)
            .field("PdlAutoNameFormat", &self.PdlAutoNameFormat)
            .field("PdlSampleInterval", &self.PdlSampleInterval)
            .field("PdlLogStartTime", &self.PdlLogStartTime)
            .field("PdlLogEndTime", &self.PdlLogEndTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PdlAutoNameInterval == other.PdlAutoNameInterval && self.PdlAutoNameUnits == other.PdlAutoNameUnits && self.PdlCommandFilename == other.PdlCommandFilename && self.PdlCounterList == other.PdlCounterList && self.PdlAutoNameFormat == other.PdlAutoNameFormat && self.PdlSampleInterval == other.PdlSampleInterval && self.PdlLogStartTime == other.PdlLogStartTime && self.PdlLogEndTime == other.PdlLogEndTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.TlNumberOfBuffers == other.TlNumberOfBuffers && self.TlMinimumBuffers == other.TlMinimumBuffers && self.TlMaximumBuffers == other.TlMaximumBuffers && self.TlFreeBuffers == other.TlFreeBuffers && self.TlBufferSize == other.TlBufferSize && self.TlEventsLost == other.TlEventsLost && self.TlLoggerThreadId == other.TlLoggerThreadId && self.TlBuffersWritten == other.TlBuffersWritten && self.TlLogHandle == other.TlLogHandle && self.TlLogFileName == other.TlLogFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: super::super::Foundation::PWSTR,
    pub szDefaultDir: super::super::Foundation::PWSTR,
    pub szBaseFileName: super::super::Foundation::PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: super::super::Foundation::PWSTR,
    pub PdlCounterList: super::super::Foundation::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: super::super::Foundation::FILETIME,
    pub PdlLogEndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct")
            .field("PdlAutoNameInterval", &self.PdlAutoNameInterval)
            .field("PdlAutoNameUnits", &self.PdlAutoNameUnits)
            .field("PdlCommandFilename", &self.PdlCommandFilename)
            .field("PdlCounterList", &self.PdlCounterList)
            .field("PdlAutoNameFormat", &self.PdlAutoNameFormat)
            .field("PdlSampleInterval", &self.PdlSampleInterval)
            .field("PdlLogStartTime", &self.PdlLogStartTime)
            .field("PdlLogEndTime", &self.PdlLogEndTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.PdlAutoNameInterval == other.PdlAutoNameInterval && self.PdlAutoNameUnits == other.PdlAutoNameUnits && self.PdlCommandFilename == other.PdlCommandFilename && self.PdlCounterList == other.PdlCounterList && self.PdlAutoNameFormat == other.PdlAutoNameFormat && self.PdlSampleInterval == other.PdlSampleInterval && self.PdlLogStartTime == other.PdlLogStartTime && self.PdlLogEndTime == other.PdlLogEndTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct")
            .field("TlNumberOfBuffers", &self.TlNumberOfBuffers)
            .field("TlMinimumBuffers", &self.TlMinimumBuffers)
            .field("TlMaximumBuffers", &self.TlMaximumBuffers)
            .field("TlFreeBuffers", &self.TlFreeBuffers)
            .field("TlBufferSize", &self.TlBufferSize)
            .field("TlEventsLost", &self.TlEventsLost)
            .field("TlLoggerThreadId", &self.TlLoggerThreadId)
            .field("TlBuffersWritten", &self.TlBuffersWritten)
            .field("TlLogHandle", &self.TlLogHandle)
            .field("TlLogFileName", &self.TlLogFileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.TlNumberOfBuffers == other.TlNumberOfBuffers && self.TlMinimumBuffers == other.TlMinimumBuffers && self.TlMaximumBuffers == other.TlMaximumBuffers && self.TlFreeBuffers == other.TlFreeBuffers && self.TlBufferSize == other.TlBufferSize && self.TlEventsLost == other.TlEventsLost && self.TlLoggerThreadId == other.TlLoggerThreadId && self.TlBuffersWritten == other.TlBuffersWritten && self.TlLogHandle == other.TlLogHandle && self.TlLogFileName == other.TlLogFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDH_LOG_TYPE(pub u32);
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = PDH_LOG_TYPE(0u32);
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = PDH_LOG_TYPE(1u32);
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = PDH_LOG_TYPE(7u32);
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = PDH_LOG_TYPE(2u32);
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = PDH_LOG_TYPE(8u32);
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = PDH_LOG_TYPE(6u32);
impl ::core::convert::From<u32> for PDH_LOG_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PDH_LOG_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for PDH_LOG_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PDH_LOG_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PDH_LOG_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PDH_LOG_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PDH_LOG_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PDH_LOG_TYPE_NOT_FOUND: i32 = -1073738805i32;
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
pub const PDH_MAX_SCALE: i32 = 7i32;
pub const PDH_MEMORY_ALLOCATION_FAILURE: i32 = -1073738821i32;
pub const PDH_MIN_SCALE: i32 = -7i32;
pub const PDH_MORE_DATA: i32 = -2147481646i32;
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
pub const PDH_NOT_IMPLEMENTED: i32 = -1073738797i32;
pub const PDH_NO_COUNTERS: i32 = -1073738785i32;
pub const PDH_NO_DATA: i32 = -2147481643i32;
pub const PDH_NO_DIALOG_DATA: i32 = -1073738809i32;
pub const PDH_NO_MORE_DATA: i32 = -1073738804i32;
pub const PDH_OS_EARLIER_VERSION: i32 = -1073738758i32;
pub const PDH_OS_LATER_VERSION: i32 = -1073738759i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDH_PATH_FLAGS(pub u32);
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(1u32);
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = PDH_PATH_FLAGS(2u32);
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = PDH_PATH_FLAGS(0u32);
impl ::core::convert::From<u32> for PDH_PATH_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PDH_PATH_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for PDH_PATH_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PDH_PATH_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PDH_PATH_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PDH_PATH_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PDH_PATH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: i32 = -1073738775i32;
pub const PDH_PLA_COLLECTION_NOT_FOUND: i32 = -1073738773i32;
pub const PDH_PLA_ERROR_ALREADY_EXISTS: i32 = -1073738770i32;
pub const PDH_PLA_ERROR_FILEPATH: i32 = -1073738768i32;
pub const PDH_PLA_ERROR_NAME_TOO_LONG: i32 = -1073738764i32;
pub const PDH_PLA_ERROR_NOSTART: i32 = -1073738771i32;
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: i32 = -1073738772i32;
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: i32 = -1073738774i32;
pub const PDH_PLA_ERROR_TYPE_MISMATCH: i32 = -1073738769i32;
pub const PDH_PLA_SERVICE_ERROR: i32 = -1073738767i32;
pub const PDH_PLA_VALIDATION_ERROR: i32 = -1073738766i32;
pub const PDH_PLA_VALIDATION_WARNING: i32 = -2147480589i32;
pub const PDH_QUERY_PERF_DATA_TIMEOUT: i32 = -1073738754i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: super::super::Foundation::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_RAW_COUNTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_RAW_COUNTER").field("CStatus", &self.CStatus).field("TimeStamp", &self.TimeStamp).field("FirstValue", &self.FirstValue).field("SecondValue", &self.SecondValue).field("MultiCount", &self.MultiCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.CStatus == other.CStatus && self.TimeStamp == other.TimeStamp && self.FirstValue == other.FirstValue && self.SecondValue == other.SecondValue && self.MultiCount == other.MultiCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: super::super::Foundation::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_RAW_COUNTER_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER_ITEM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_A {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_RAW_COUNTER_ITEM_A").field("szName", &self.szName).field("RawValue", &self.RawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_A {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.RawValue == other.RawValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER_ITEM_A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: super::super::Foundation::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_RAW_COUNTER_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_RAW_COUNTER_ITEM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PDH_RAW_COUNTER_ITEM_W {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_RAW_COUNTER_ITEM_W").field("szName", &self.szName).field("RawValue", &self.RawValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_RAW_COUNTER_ITEM_W {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.RawValue == other.RawValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_RAW_COUNTER_ITEM_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_RAW_COUNTER_ITEM_W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl PDH_RAW_LOG_RECORD {}
impl ::core::default::Default for PDH_RAW_LOG_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PDH_RAW_LOG_RECORD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_RAW_LOG_RECORD").field("dwStructureSize", &self.dwStructureSize).field("dwRecordType", &self.dwRecordType).field("dwItems", &self.dwItems).field("RawBytes", &self.RawBytes).finish()
    }
}
impl ::core::cmp::PartialEq for PDH_RAW_LOG_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructureSize == other.dwStructureSize && self.dwRecordType == other.dwRecordType && self.dwItems == other.dwItems && self.RawBytes == other.RawBytes
    }
}
impl ::core::cmp::Eq for PDH_RAW_LOG_RECORD {}
unsafe impl ::windows::core::Abi for PDH_RAW_LOG_RECORD {
    type Abi = Self;
}
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
pub const PDH_RETRY: i32 = -2147481644i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PDH_SELECT_DATA_SOURCE_FLAGS(pub u32);
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(1u32);
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = PDH_SELECT_DATA_SOURCE_FLAGS(0u32);
impl ::core::convert::From<u32> for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PDH_SELECT_DATA_SOURCE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for PDH_SELECT_DATA_SOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PDH_SELECT_DATA_SOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PDH_SELECT_DATA_SOURCE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PDH_SELECT_DATA_SOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PDH_SQL_ALLOCCON_FAILED: i32 = -1073738783i32;
pub const PDH_SQL_ALLOC_FAILED: i32 = -1073738784i32;
pub const PDH_SQL_ALTER_DETAIL_FAILED: i32 = -1073738755i32;
pub const PDH_SQL_BIND_FAILED: i32 = -1073738777i32;
pub const PDH_SQL_CONNECT_FAILED: i32 = -1073738778i32;
pub const PDH_SQL_EXEC_DIRECT_FAILED: i32 = -1073738782i32;
pub const PDH_SQL_FETCH_FAILED: i32 = -1073738781i32;
pub const PDH_SQL_MORE_RESULTS_FAILED: i32 = -1073738779i32;
pub const PDH_SQL_ROWCOUNT_FAILED: i32 = -1073738780i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl PDH_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PDH_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PDH_STATISTICS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PDH_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PDH_STATISTICS {
    type Abi = Self;
}
pub const PDH_STRING_NOT_FOUND: i32 = -1073738796i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
impl PDH_TIME_INFO {}
impl ::core::default::Default for PDH_TIME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PDH_TIME_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PDH_TIME_INFO").field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("SampleCount", &self.SampleCount).finish()
    }
}
impl ::core::cmp::PartialEq for PDH_TIME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.SampleCount == other.SampleCount
    }
}
impl ::core::cmp::Eq for PDH_TIME_INFO {}
unsafe impl ::windows::core::Abi for PDH_TIME_INFO {
    type Abi = Self;
}
pub const PDH_UNABLE_MAP_NAME_FILES: i32 = -2147480619i32;
pub const PDH_UNABLE_READ_LOG_HEADER: i32 = -1073738800i32;
pub const PDH_UNKNOWN_LOGSVC_COMMAND: i32 = -1073738793i32;
pub const PDH_UNKNOWN_LOG_FORMAT: i32 = -1073738794i32;
pub const PDH_UNMATCHED_APPEND_COUNTER: i32 = -1073738756i32;
pub const PDH_WBEM_ERROR: i32 = -1073738790i32;
pub type PERFLIBREQUEST = unsafe extern "system" fn(requestcode: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
pub const PERF_ADD_COUNTER: u32 = 1u32;
pub const PERF_AGGREGATE_MAX: u32 = 4u32;
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
pub const PERF_COLLECT_END: u32 = 6u32;
pub const PERF_COLLECT_START: u32 = 5u32;
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: ::windows::core::GUID,
    pub ProviderGuid: ::windows::core::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl PERF_COUNTERSET_INFO {}
impl ::core::default::Default for PERF_COUNTERSET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTERSET_INFO").field("CounterSetGuid", &self.CounterSetGuid).field("ProviderGuid", &self.ProviderGuid).field("NumCounters", &self.NumCounters).field("InstanceType", &self.InstanceType).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid && self.ProviderGuid == other.ProviderGuid && self.NumCounters == other.NumCounters && self.InstanceType == other.InstanceType
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INFO {}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: ::windows::core::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
impl PERF_COUNTERSET_INSTANCE {}
impl ::core::default::Default for PERF_COUNTERSET_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_INSTANCE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTERSET_INSTANCE").field("CounterSetGuid", &self.CounterSetGuid).field("dwSize", &self.dwSize).field("InstanceId", &self.InstanceId).field("InstanceNameOffset", &self.InstanceNameOffset).field("InstanceNameSize", &self.InstanceNameSize).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid && self.dwSize == other.dwSize && self.InstanceId == other.InstanceId && self.InstanceNameOffset == other.InstanceNameOffset && self.InstanceNameSize == other.InstanceNameSize
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_INSTANCE {}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_INSTANCE {
    type Abi = Self;
}
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: ::windows::core::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl PERF_COUNTERSET_REG_INFO {}
impl ::core::default::Default for PERF_COUNTERSET_REG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTERSET_REG_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTERSET_REG_INFO").field("CounterSetGuid", &self.CounterSetGuid).field("CounterSetType", &self.CounterSetType).field("DetailLevel", &self.DetailLevel).field("NumCounters", &self.NumCounters).field("InstanceType", &self.InstanceType).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTERSET_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid && self.CounterSetType == other.CounterSetType && self.DetailLevel == other.DetailLevel && self.NumCounters == other.NumCounters && self.InstanceType == other.InstanceType
    }
}
impl ::core::cmp::Eq for PERF_COUNTERSET_REG_INFO {}
unsafe impl ::windows::core::Abi for PERF_COUNTERSET_REG_INFO {
    type Abi = Self;
}
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PERF_COUNTER_AGGREGATE_FUNC(pub u32);
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(0u32);
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(1u32);
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(2u32);
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = PERF_COUNTER_AGGREGATE_FUNC(3u32);
impl ::core::convert::From<u32> for PERF_COUNTER_AGGREGATE_FUNC {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PERF_COUNTER_AGGREGATE_FUNC {
    type Abi = Self;
}
impl ::core::ops::BitOr for PERF_COUNTER_AGGREGATE_FUNC {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PERF_COUNTER_AGGREGATE_FUNC {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PERF_COUNTER_AGGREGATE_FUNC {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PERF_COUNTER_AGGREGATE_FUNC {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PERF_COUNTER_AGGREGATE_FUNC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PERF_COUNTER_BASE: u32 = 196608u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
impl PERF_COUNTER_BLOCK {}
impl ::core::default::Default for PERF_COUNTER_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_BLOCK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_BLOCK").field("ByteLength", &self.ByteLength).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_BLOCK {}
unsafe impl ::windows::core::Abi for PERF_COUNTER_BLOCK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
impl PERF_COUNTER_DATA {}
impl ::core::default::Default for PERF_COUNTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_DATA").field("dwDataSize", &self.dwDataSize).field("dwSize", &self.dwSize).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDataSize == other.dwDataSize && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_DATA {}
unsafe impl ::windows::core::Abi for PERF_COUNTER_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength && self.CounterNameTitleIndex == other.CounterNameTitleIndex && self.CounterNameTitle == other.CounterNameTitle && self.CounterHelpTitleIndex == other.CounterHelpTitleIndex && self.CounterHelpTitle == other.CounterHelpTitle && self.DefaultScale == other.DefaultScale && self.DetailLevel == other.DetailLevel && self.CounterType == other.CounterType && self.CounterSize == other.CounterSize && self.CounterOffset == other.CounterOffset
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_COUNTER_DEFINITION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: super::super::Foundation::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: super::super::Foundation::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_COUNTER_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_COUNTER_DEFINITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("CounterNameTitleIndex", &self.CounterNameTitleIndex)
            .field("CounterNameTitle", &self.CounterNameTitle)
            .field("CounterHelpTitleIndex", &self.CounterHelpTitleIndex)
            .field("CounterHelpTitle", &self.CounterHelpTitle)
            .field("DefaultScale", &self.DefaultScale)
            .field("DetailLevel", &self.DetailLevel)
            .field("CounterType", &self.CounterType)
            .field("CounterSize", &self.CounterSize)
            .field("CounterOffset", &self.CounterOffset)
            .finish()
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_COUNTER_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength && self.CounterNameTitleIndex == other.CounterNameTitleIndex && self.CounterNameTitle == other.CounterNameTitle && self.CounterHelpTitleIndex == other.CounterHelpTitleIndex && self.CounterHelpTitle == other.CounterHelpTitle && self.DefaultScale == other.DefaultScale && self.DetailLevel == other.DetailLevel && self.CounterType == other.CounterType && self.CounterSize == other.CounterSize && self.CounterOffset == other.CounterOffset
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_COUNTER_DEFINITION {
    type Abi = Self;
}
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
impl PERF_COUNTER_HEADER {}
impl ::core::default::Default for PERF_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_HEADER").field("dwStatus", &self.dwStatus).field("dwType", &self.dwType).field("dwSize", &self.dwSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.dwType == other.dwType && self.dwSize == other.dwSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_HEADER {}
unsafe impl ::windows::core::Abi for PERF_COUNTER_HEADER {
    type Abi = Self;
}
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: ::windows::core::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
impl PERF_COUNTER_IDENTIFIER {}
impl ::core::default::Default for PERF_COUNTER_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTIFIER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_IDENTIFIER").field("CounterSetGuid", &self.CounterSetGuid).field("Status", &self.Status).field("Size", &self.Size).field("CounterId", &self.CounterId).field("InstanceId", &self.InstanceId).field("Index", &self.Index).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid && self.Status == other.Status && self.Size == other.Size && self.CounterId == other.CounterId && self.InstanceId == other.InstanceId && self.Index == other.Index && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTIFIER {}
unsafe impl ::windows::core::Abi for PERF_COUNTER_IDENTIFIER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: ::windows::core::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
impl PERF_COUNTER_IDENTITY {}
impl ::core::default::Default for PERF_COUNTER_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_IDENTITY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_IDENTITY")
            .field("CounterSetGuid", &self.CounterSetGuid)
            .field("BufferSize", &self.BufferSize)
            .field("CounterId", &self.CounterId)
            .field("InstanceId", &self.InstanceId)
            .field("MachineOffset", &self.MachineOffset)
            .field("NameOffset", &self.NameOffset)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_IDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.CounterSetGuid == other.CounterSetGuid && self.BufferSize == other.BufferSize && self.CounterId == other.CounterId && self.InstanceId == other.InstanceId && self.MachineOffset == other.MachineOffset && self.NameOffset == other.NameOffset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_IDENTITY {}
unsafe impl ::windows::core::Abi for PERF_COUNTER_IDENTITY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
impl PERF_COUNTER_INFO {}
impl ::core::default::Default for PERF_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_INFO").field("CounterId", &self.CounterId).field("Type", &self.Type).field("Attrib", &self.Attrib).field("Size", &self.Size).field("DetailLevel", &self.DetailLevel).field("Scale", &self.Scale).field("Offset", &self.Offset).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterId == other.CounterId && self.Type == other.Type && self.Attrib == other.Attrib && self.Size == other.Size && self.DetailLevel == other.DetailLevel && self.Scale == other.Scale && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_INFO {}
unsafe impl ::windows::core::Abi for PERF_COUNTER_INFO {
    type Abi = Self;
}
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
pub const PERF_COUNTER_RATE: u32 = 65536u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: PERF_COUNTER_AGGREGATE_FUNC,
    pub Reserved: u32,
}
impl PERF_COUNTER_REG_INFO {}
impl ::core::default::Default for PERF_COUNTER_REG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_COUNTER_REG_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_COUNTER_REG_INFO")
            .field("CounterId", &self.CounterId)
            .field("Type", &self.Type)
            .field("Attrib", &self.Attrib)
            .field("DetailLevel", &self.DetailLevel)
            .field("DefaultScale", &self.DefaultScale)
            .field("BaseCounterId", &self.BaseCounterId)
            .field("PerfTimeId", &self.PerfTimeId)
            .field("PerfFreqId", &self.PerfFreqId)
            .field("MultiId", &self.MultiId)
            .field("AggregateFunc", &self.AggregateFunc)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_COUNTER_REG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CounterId == other.CounterId && self.Type == other.Type && self.Attrib == other.Attrib && self.DetailLevel == other.DetailLevel && self.DefaultScale == other.DefaultScale && self.BaseCounterId == other.BaseCounterId && self.PerfTimeId == other.PerfTimeId && self.PerfFreqId == other.PerfFreqId && self.MultiId == other.MultiId && self.AggregateFunc == other.AggregateFunc && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PERF_COUNTER_REG_INFO {}
unsafe impl ::windows::core::Abi for PERF_COUNTER_REG_INFO {
    type Abi = Self;
}
pub const PERF_COUNTER_VALUE: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PERF_DATA_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_DATA_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_DATA_BLOCK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_DATA_BLOCK")
            .field("Signature", &self.Signature)
            .field("LittleEndian", &self.LittleEndian)
            .field("Version", &self.Version)
            .field("Revision", &self.Revision)
            .field("TotalByteLength", &self.TotalByteLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("NumObjectTypes", &self.NumObjectTypes)
            .field("DefaultObject", &self.DefaultObject)
            .field("SystemTime", &self.SystemTime)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .field("PerfTime100nSec", &self.PerfTime100nSec)
            .field("SystemNameLength", &self.SystemNameLength)
            .field("SystemNameOffset", &self.SystemNameOffset)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_DATA_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature
            && self.LittleEndian == other.LittleEndian
            && self.Version == other.Version
            && self.Revision == other.Revision
            && self.TotalByteLength == other.TotalByteLength
            && self.HeaderLength == other.HeaderLength
            && self.NumObjectTypes == other.NumObjectTypes
            && self.DefaultObject == other.DefaultObject
            && self.SystemTime == other.SystemTime
            && self.PerfTime == other.PerfTime
            && self.PerfFreq == other.PerfFreq
            && self.PerfTime100nSec == other.PerfTime100nSec
            && self.SystemNameLength == other.SystemNameLength
            && self.SystemNameOffset == other.SystemNameOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_DATA_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_DATA_BLOCK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl PERF_DATA_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_DATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_DATA_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_DATA_HEADER").field("dwTotalSize", &self.dwTotalSize).field("dwNumCounters", &self.dwNumCounters).field("PerfTimeStamp", &self.PerfTimeStamp).field("PerfTime100NSec", &self.PerfTime100NSec).field("PerfFreq", &self.PerfFreq).field("SystemTime", &self.SystemTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_DATA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize && self.dwNumCounters == other.dwNumCounters && self.PerfTimeStamp == other.PerfTimeStamp && self.PerfTime100NSec == other.PerfTime100NSec && self.PerfFreq == other.PerfFreq && self.SystemTime == other.SystemTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_DATA_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_DATA_HEADER {
    type Abi = Self;
}
pub const PERF_DATA_REVISION: u32 = 1u32;
pub const PERF_DATA_VERSION: u32 = 1u32;
pub const PERF_DELTA_BASE: u32 = 8388608u32;
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PERF_DETAIL(pub u32);
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = PERF_DETAIL(100u32);
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = PERF_DETAIL(200u32);
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = PERF_DETAIL(300u32);
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = PERF_DETAIL(400u32);
impl ::core::convert::From<u32> for PERF_DETAIL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PERF_DETAIL {
    type Abi = Self;
}
impl ::core::ops::BitOr for PERF_DETAIL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PERF_DETAIL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PERF_DETAIL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PERF_DETAIL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PERF_DETAIL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
pub const PERF_FILTER: u32 = 9u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
impl PERF_INSTANCE_DEFINITION {}
impl ::core::default::Default for PERF_INSTANCE_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_DEFINITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_INSTANCE_DEFINITION")
            .field("ByteLength", &self.ByteLength)
            .field("ParentObjectTitleIndex", &self.ParentObjectTitleIndex)
            .field("ParentObjectInstance", &self.ParentObjectInstance)
            .field("UniqueID", &self.UniqueID)
            .field("NameOffset", &self.NameOffset)
            .field("NameLength", &self.NameLength)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.ByteLength == other.ByteLength && self.ParentObjectTitleIndex == other.ParentObjectTitleIndex && self.ParentObjectInstance == other.ParentObjectInstance && self.UniqueID == other.UniqueID && self.NameOffset == other.NameOffset && self.NameLength == other.NameLength
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_DEFINITION {}
unsafe impl ::windows::core::Abi for PERF_INSTANCE_DEFINITION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
impl PERF_INSTANCE_HEADER {}
impl ::core::default::Default for PERF_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_INSTANCE_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_INSTANCE_HEADER").field("Size", &self.Size).field("InstanceId", &self.InstanceId).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_INSTANCE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.InstanceId == other.InstanceId
    }
}
impl ::core::cmp::Eq for PERF_INSTANCE_HEADER {}
unsafe impl ::windows::core::Abi for PERF_INSTANCE_HEADER {
    type Abi = Self;
}
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
pub type PERF_MEM_ALLOC = unsafe extern "system" fn(allocsize: usize, pcontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
pub type PERF_MEM_FREE = unsafe extern "system" fn(pbuffer: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void);
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl PERF_MULTI_COUNTERS {}
impl ::core::default::Default for PERF_MULTI_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_MULTI_COUNTERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_MULTI_COUNTERS").field("dwSize", &self.dwSize).field("dwCounters", &self.dwCounters).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_MULTI_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCounters == other.dwCounters
    }
}
impl ::core::cmp::Eq for PERF_MULTI_COUNTERS {}
unsafe impl ::windows::core::Abi for PERF_MULTI_COUNTERS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
impl PERF_MULTI_INSTANCES {}
impl ::core::default::Default for PERF_MULTI_INSTANCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_MULTI_INSTANCES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_MULTI_INSTANCES").field("dwTotalSize", &self.dwTotalSize).field("dwInstances", &self.dwInstances).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_MULTI_INSTANCES {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize && self.dwInstances == other.dwInstances
    }
}
impl ::core::cmp::Eq for PERF_MULTI_INSTANCES {}
unsafe impl ::windows::core::Abi for PERF_MULTI_INSTANCES {
    type Abi = Self;
}
pub const PERF_NO_INSTANCES: i32 = -1i32;
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
pub const PERF_NUMBER_HEX: u32 = 0u32;
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.TotalByteLength == other.TotalByteLength
            && self.DefinitionLength == other.DefinitionLength
            && self.HeaderLength == other.HeaderLength
            && self.ObjectNameTitleIndex == other.ObjectNameTitleIndex
            && self.ObjectNameTitle == other.ObjectNameTitle
            && self.ObjectHelpTitleIndex == other.ObjectHelpTitleIndex
            && self.ObjectHelpTitle == other.ObjectHelpTitle
            && self.DetailLevel == other.DetailLevel
            && self.NumCounters == other.NumCounters
            && self.DefaultCounter == other.DefaultCounter
            && self.NumInstances == other.NumInstances
            && self.CodePage == other.CodePage
            && self.PerfTime == other.PerfTime
            && self.PerfFreq == other.PerfFreq
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_OBJECT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: super::super::Foundation::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: super::super::Foundation::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERF_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERF_OBJECT_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_OBJECT_TYPE")
            .field("TotalByteLength", &self.TotalByteLength)
            .field("DefinitionLength", &self.DefinitionLength)
            .field("HeaderLength", &self.HeaderLength)
            .field("ObjectNameTitleIndex", &self.ObjectNameTitleIndex)
            .field("ObjectNameTitle", &self.ObjectNameTitle)
            .field("ObjectHelpTitleIndex", &self.ObjectHelpTitleIndex)
            .field("ObjectHelpTitle", &self.ObjectHelpTitle)
            .field("DetailLevel", &self.DetailLevel)
            .field("NumCounters", &self.NumCounters)
            .field("DefaultCounter", &self.DefaultCounter)
            .field("NumInstances", &self.NumInstances)
            .field("CodePage", &self.CodePage)
            .field("PerfTime", &self.PerfTime)
            .field("PerfFreq", &self.PerfFreq)
            .finish()
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERF_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.TotalByteLength == other.TotalByteLength
            && self.DefinitionLength == other.DefinitionLength
            && self.HeaderLength == other.HeaderLength
            && self.ObjectNameTitleIndex == other.ObjectNameTitleIndex
            && self.ObjectNameTitle == other.ObjectNameTitle
            && self.ObjectHelpTitleIndex == other.ObjectHelpTitleIndex
            && self.ObjectHelpTitle == other.ObjectHelpTitle
            && self.DetailLevel == other.DetailLevel
            && self.NumCounters == other.NumCounters
            && self.DefaultCounter == other.DefaultCounter
            && self.NumInstances == other.NumInstances
            && self.CodePage == other.CodePage
            && self.PerfTime == other.PerfTime
            && self.PerfFreq == other.PerfFreq
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PERF_OBJECT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: ::core::option::Option<PERFLIBREQUEST>,
    pub MemAllocRoutine: ::core::option::Option<PERF_MEM_ALLOC>,
    pub MemFreeRoutine: ::core::option::Option<PERF_MEM_FREE>,
    pub pMemContext: *mut ::core::ffi::c_void,
}
impl PERF_PROVIDER_CONTEXT {}
impl ::core::default::Default for PERF_PROVIDER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_PROVIDER_CONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_PROVIDER_CONTEXT").field("ContextSize", &self.ContextSize).field("Reserved", &self.Reserved).field("pMemContext", &self.pMemContext).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_PROVIDER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ContextSize == other.ContextSize && self.Reserved == other.Reserved && self.ControlCallback.map(|f| f as usize) == other.ControlCallback.map(|f| f as usize) && self.MemAllocRoutine.map(|f| f as usize) == other.MemAllocRoutine.map(|f| f as usize) && self.MemFreeRoutine.map(|f| f as usize) == other.MemFreeRoutine.map(|f| f as usize) && self.pMemContext == other.pMemContext
    }
}
impl ::core::cmp::Eq for PERF_PROVIDER_CONTEXT {}
unsafe impl ::windows::core::Abi for PERF_PROVIDER_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
pub const PERF_SIZE_DWORD: u32 = 0u32;
pub const PERF_SIZE_LARGE: u32 = 256u32;
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
pub const PERF_SIZE_ZERO: u32 = 512u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl PERF_STRING_BUFFER_HEADER {}
impl ::core::default::Default for PERF_STRING_BUFFER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_STRING_BUFFER_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_STRING_BUFFER_HEADER").field("dwSize", &self.dwSize).field("dwCounters", &self.dwCounters).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_STRING_BUFFER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCounters == other.dwCounters
    }
}
impl ::core::cmp::Eq for PERF_STRING_BUFFER_HEADER {}
unsafe impl ::windows::core::Abi for PERF_STRING_BUFFER_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
impl PERF_STRING_COUNTER_HEADER {}
impl ::core::default::Default for PERF_STRING_COUNTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PERF_STRING_COUNTER_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERF_STRING_COUNTER_HEADER").field("dwCounterId", &self.dwCounterId).field("dwOffset", &self.dwOffset).finish()
    }
}
impl ::core::cmp::PartialEq for PERF_STRING_COUNTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwCounterId == other.dwCounterId && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for PERF_STRING_COUNTER_HEADER {}
unsafe impl ::windows::core::Abi for PERF_STRING_COUNTER_HEADER {
    type Abi = Self;
}
pub const PERF_TEXT_ASCII: u32 = 65536u32;
pub const PERF_TEXT_UNICODE: u32 = 0u32;
pub const PERF_TIMER_100NS: u32 = 1048576u32;
pub const PERF_TIMER_TICK: u32 = 0u32;
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
pub const PERF_TYPE_NUMBER: u32 = 0u32;
pub const PERF_TYPE_TEXT: u32 = 2048u32;
pub const PERF_TYPE_ZERO: u32 = 3072u32;
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub type PLA_CABEXTRACT_CALLBACK = unsafe extern "system" fn(filename: super::super::Foundation::PWSTR, context: *mut ::core::ffi::c_void);
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
pub type PM_CLOSE_PROC = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PM_COLLECT_PROC = unsafe extern "system" fn(pvaluename: super::super::Foundation::PWSTR, ppdata: *mut *mut ::core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PM_OPEN_PROC = unsafe extern "system" fn(pcontext: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddCounterA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddCounterA(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddCounterW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddCounterW(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddEnglishCounterA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddEnglishCounterA(hquery: isize, szfullcounterpath: super::super::Foundation::PSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddEnglishCounterA(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhAddEnglishCounterW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hquery: isize, szfullcounterpath: Param1, dwuserdata: usize, phcounter: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhAddEnglishCounterW(hquery: isize, szfullcounterpath: super::super::Foundation::PWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhAddEnglishCounterW(::core::mem::transmute(hquery), szfullcounterpath.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBindInputDataSourceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(phdatasource: *mut isize, logfilenamelist: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBindInputDataSourceA(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhBindInputDataSourceA(::core::mem::transmute(phdatasource), logfilenamelist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBindInputDataSourceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(phdatasource: *mut isize, logfilenamelist: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBindInputDataSourceW(phdatasource: *mut isize, logfilenamelist: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhBindInputDataSourceW(::core::mem::transmute(phdatasource), logfilenamelist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersA(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_A>) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersA(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersHA(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_HA>) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersHA(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersHW(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_HW>) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersHW(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhBrowseCountersW(pbrowsedlgdata: *const ::core::mem::ManuallyDrop<PDH_BROWSE_DLG_CONFIG_W>) -> i32;
        }
        ::core::mem::transmute(PdhBrowseCountersW(::core::mem::transmute(pbrowsedlgdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhCalculateCounterFromRawValue(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(rawvalue1), ::core::mem::transmute(rawvalue2), ::core::mem::transmute(fmtvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhCloseLog(::core::mem::transmute(hlog), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhCloseQuery(hquery: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCloseQuery(hquery: isize) -> i32;
        }
        ::core::mem::transmute(PdhCloseQuery(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhCollectQueryData(hquery: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryData(hquery: isize) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryData(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCollectQueryDataEx<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hquery: isize, dwintervaltime: u32, hnewdataevent: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryDataEx(hquery: isize, dwintervaltime: u32, hnewdataevent: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryDataEx(::core::mem::transmute(hquery), ::core::mem::transmute(dwintervaltime), hnewdataevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhCollectQueryDataWithTime(::core::mem::transmute(hquery), ::core::mem::transmute(plltimestamp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32;
        }
        ::core::mem::transmute(PdhComputeCounterStatistics(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(dwfirstentry), ::core::mem::transmute(dwnumentries), ::core::mem::transmute(lprawvaluearray), ::core::mem::transmute(data)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhConnectMachineA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szmachinename: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhConnectMachineA(szmachinename: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhConnectMachineA(szmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhConnectMachineW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachinename: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhConnectMachineW(szmachinename: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhConnectMachineW(szmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCreateSQLTablesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCreateSQLTablesA(szdatasource: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhCreateSQLTablesA(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhCreateSQLTablesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhCreateSQLTablesW(szdatasource: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhCreateSQLTablesW(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumLogSetNamesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, mszdatasetnamelist: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumLogSetNamesA(szdatasource: super::super::Foundation::PSTR, mszdatasetnamelist: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumLogSetNamesA(szdatasource.into_param().abi(), ::core::mem::transmute(mszdatasetnamelist), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumLogSetNamesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, mszdatasetnamelist: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumLogSetNamesW(szdatasource: super::super::Foundation::PWSTR, mszdatasetnamelist: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumLogSetNamesW(szdatasource.into_param().abi(), ::core::mem::transmute(mszdatasetnamelist), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesA(szdatasource: super::super::Foundation::PSTR, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesA(szdatasource.into_param().abi(), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesHA(::core::mem::transmute(hdatasource), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesHW(::core::mem::transmute(hdatasource), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumMachinesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumMachinesW(szdatasource: super::super::Foundation::PWSTR, mszmachinelist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumMachinesW(szdatasource.into_param().abi(), ::core::mem::transmute(mszmachinelist), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(
    szdatasource: Param0,
    szmachinename: Param1,
    szobjectname: Param2,
    mszcounterlist: super::super::Foundation::PSTR,
    pcchcounterlistlength: *mut u32,
    mszinstancelist: super::super::Foundation::PSTR,
    pcchinstancelistlength: *mut u32,
    dwdetaillevel: PERF_DETAIL,
    dwflags: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsA(
            szdatasource.into_param().abi(),
            szmachinename.into_param().abi(),
            szobjectname.into_param().abi(),
            ::core::mem::transmute(mszcounterlist),
            ::core::mem::transmute(pcchcounterlistlength),
            ::core::mem::transmute(mszinstancelist),
            ::core::mem::transmute(pcchinstancelistlength),
            ::core::mem::transmute(dwdetaillevel),
            ::core::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, mszcounterlist: super::super::Foundation::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsHA(
            ::core::mem::transmute(hdatasource),
            szmachinename.into_param().abi(),
            szobjectname.into_param().abi(),
            ::core::mem::transmute(mszcounterlist),
            ::core::mem::transmute(pcchcounterlistlength),
            ::core::mem::transmute(mszinstancelist),
            ::core::mem::transmute(pcchinstancelistlength),
            ::core::mem::transmute(dwdetaillevel),
            ::core::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsHW(
            ::core::mem::transmute(hdatasource),
            szmachinename.into_param().abi(),
            szobjectname.into_param().abi(),
            ::core::mem::transmute(mszcounterlist),
            ::core::mem::transmute(pcchcounterlistlength),
            ::core::mem::transmute(mszinstancelist),
            ::core::mem::transmute(pcchinstancelistlength),
            ::core::mem::transmute(dwdetaillevel),
            ::core::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectItemsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
    szdatasource: Param0,
    szmachinename: Param1,
    szobjectname: Param2,
    mszcounterlist: super::super::Foundation::PWSTR,
    pcchcounterlistlength: *mut u32,
    mszinstancelist: super::super::Foundation::PWSTR,
    pcchinstancelistlength: *mut u32,
    dwdetaillevel: PERF_DETAIL,
    dwflags: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectItemsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, mszcounterlist: super::super::Foundation::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: super::super::Foundation::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectItemsW(
            szdatasource.into_param().abi(),
            szmachinename.into_param().abi(),
            szobjectname.into_param().abi(),
            ::core::mem::transmute(mszcounterlist),
            ::core::mem::transmute(pcchcounterlistlength),
            ::core::mem::transmute(mszinstancelist),
            ::core::mem::transmute(pcchinstancelistlength),
            ::core::mem::transmute(dwdetaillevel),
            ::core::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szdatasource: Param0, szmachinename: Param1, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hdatasource: isize, szmachinename: Param1, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, mszobjectlist: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hdatasource: isize, szmachinename: Param1, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhEnumObjectsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szdatasource: Param0, szmachinename: Param1, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: Param5) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhEnumObjectsW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, mszobjectlist: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: super::super::Foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(PdhEnumObjectsW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(mszobjectlist), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwdetaillevel), brefresh.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandCounterPathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szwildcardpath: Param0, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandCounterPathA(szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandCounterPathA(szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandCounterPathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szwildcardpath: Param0, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandCounterPathW(szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandCounterPathW(szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathA(szdatasource: super::super::Foundation::PSTR, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathA(szdatasource.into_param().abi(), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathHA(hdatasource: isize, szwildcardpath: super::super::Foundation::PSTR, mszexpandedpathlist: super::super::Foundation::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathHA(::core::mem::transmute(hdatasource), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathHW(hdatasource: isize, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathHW(::core::mem::transmute(hdatasource), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhExpandWildCardPathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, szwildcardpath: Param1, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhExpandWildCardPathW(szdatasource: super::super::Foundation::PWSTR, szwildcardpath: super::super::Foundation::PWSTR, mszexpandedpathlist: super::super::Foundation::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhExpandWildCardPathW(szdatasource.into_param().abi(), szwildcardpath.into_param().abi(), ::core::mem::transmute(mszexpandedpathlist), ::core::mem::transmute(pcchpathlistlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhFormatFromRawValue(::core::mem::transmute(dwcountertype), ::core::mem::transmute(dwformat), ::core::mem::transmute(ptimebase), ::core::mem::transmute(prawvalue1), ::core::mem::transmute(prawvalue2), ::core::mem::transmute(pfmtvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetCounterInfoA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(hcounter: isize, bretrieveexplaintext: Param1, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterInfoA(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterInfoA(::core::mem::transmute(hcounter), bretrieveexplaintext.into_param().abi(), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetCounterInfoW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(hcounter: isize, bretrieveexplaintext: Param1, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterInfoW(hcounter: isize, bretrieveexplaintext: super::super::Foundation::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterInfoW(::core::mem::transmute(hcounter), bretrieveexplaintext.into_param().abi(), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhGetCounterTimeBase(::core::mem::transmute(hcounter), ::core::mem::transmute(ptimebase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeA(szdatasource: super::super::Foundation::PSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeA(szdatasource.into_param().abi(), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeH(::core::mem::transmute(hdatasource), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDataSourceTimeRangeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDataSourceTimeRangeW(szdatasource: super::super::Foundation::PWSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDataSourceTimeRangeW(szdatasource.into_param().abi(), ::core::mem::transmute(pdwnumentries), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szobjectname: super::super::Foundation::PSTR, szdefaultcountername: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfCounterW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, szmachinename: Param1, szobjectname: Param2, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfCounterW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szobjectname: super::super::Foundation::PWSTR, szdefaultcountername: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfCounterW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), szobjectname.into_param().abi(), ::core::mem::transmute(szdefaultcountername), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectA(szdatasource: super::super::Foundation::PSTR, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectA(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectHA(hdatasource: isize, szmachinename: super::super::Foundation::PSTR, szdefaultobjectname: super::super::Foundation::PSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectHA(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectHW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectHW(hdatasource: isize, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectHW(::core::mem::transmute(hdatasource), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetDefaultPerfObjectW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, szmachinename: Param1, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDefaultPerfObjectW(szdatasource: super::super::Foundation::PWSTR, szmachinename: super::super::Foundation::PWSTR, szdefaultobjectname: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhGetDefaultPerfObjectW(szdatasource.into_param().abi(), szmachinename.into_param().abi(), ::core::mem::transmute(szdefaultobjectname), ::core::mem::transmute(pcchbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32;
        }
        ::core::mem::transmute(PdhGetDllVersion(::core::mem::transmute(lpdwversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterArrayA(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterArrayW(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
        }
        ::core::mem::transmute(PdhGetFormattedCounterValue(::core::mem::transmute(hcounter), ::core::mem::transmute(dwformat), ::core::mem::transmute(lpdwtype), ::core::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32;
        }
        ::core::mem::transmute(PdhGetLogFileSize(::core::mem::transmute(hlog), ::core::mem::transmute(llsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows::core::GUID, prunid: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows::core::GUID, prunid: *mut i32) -> i32;
        }
        ::core::mem::transmute(PdhGetLogSetGUID(::core::mem::transmute(hlog), ::core::mem::transmute(pguid), ::core::mem::transmute(prunid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterArrayA(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterArrayW(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwbuffersize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(itembuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32;
        }
        ::core::mem::transmute(PdhGetRawCounterValue(::core::mem::transmute(hcounter), ::core::mem::transmute(lpdwtype), ::core::mem::transmute(pvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhIsRealTimeQuery(hquery: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PdhIsRealTimeQuery(::core::mem::transmute(hquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szmachinename: Param0, sznamebuffer: Param1, pdwindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfIndexByNameA(szmachinename: super::super::Foundation::PSTR, sznamebuffer: super::super::Foundation::PSTR, pdwindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfIndexByNameA(szmachinename.into_param().abi(), sznamebuffer.into_param().abi(), ::core::mem::transmute(pdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfIndexByNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachinename: Param0, sznamebuffer: Param1, pdwindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfIndexByNameW(szmachinename: super::super::Foundation::PWSTR, sznamebuffer: super::super::Foundation::PWSTR, pdwindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfIndexByNameW(szmachinename.into_param().abi(), sznamebuffer.into_param().abi(), ::core::mem::transmute(pdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szmachinename: Param0, dwnameindex: u32, sznamebuffer: super::super::Foundation::PSTR, pcchnamebuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfNameByIndexA(szmachinename: super::super::Foundation::PSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PSTR, pcchnamebuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfNameByIndexA(szmachinename.into_param().abi(), ::core::mem::transmute(dwnameindex), ::core::mem::transmute(sznamebuffer), ::core::mem::transmute(pcchnamebuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhLookupPerfNameByIndexW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachinename: Param0, dwnameindex: u32, sznamebuffer: super::super::Foundation::PWSTR, pcchnamebuffersize: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhLookupPerfNameByIndexW(szmachinename: super::super::Foundation::PWSTR, dwnameindex: u32, sznamebuffer: super::super::Foundation::PWSTR, pcchnamebuffersize: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhLookupPerfNameByIndexW(szmachinename.into_param().abi(), ::core::mem::transmute(dwnameindex), ::core::mem::transmute(sznamebuffer), ::core::mem::transmute(pcchnamebuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: super::super::Foundation::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhMakeCounterPathA(::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(szfullpathbuffer), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: super::super::Foundation::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhMakeCounterPathW(::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(szfullpathbuffer), ::core::mem::transmute(pcchbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenLogA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szlogfilename: Param0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: Param5, phlog: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenLogA(szlogfilename: super::super::Foundation::PSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PSTR, phlog: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenLogA(szlogfilename.into_param().abi(), ::core::mem::transmute(dwaccessflags), ::core::mem::transmute(lpdwlogtype), ::core::mem::transmute(hquery), ::core::mem::transmute(dwmaxsize), szusercaption.into_param().abi(), ::core::mem::transmute(phlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenLogW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szlogfilename: Param0, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: Param5, phlog: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenLogW(szlogfilename: super::super::Foundation::PWSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: super::super::Foundation::PWSTR, phlog: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenLogW(szlogfilename.into_param().abi(), ::core::mem::transmute(dwaccessflags), ::core::mem::transmute(lpdwlogtype), ::core::mem::transmute(hquery), ::core::mem::transmute(dwmaxsize), szusercaption.into_param().abi(), ::core::mem::transmute(phlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenQueryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryA(szdatasource: super::super::Foundation::PSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryA(szdatasource.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryH(::core::mem::transmute(hdatasource), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhOpenQueryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0, dwuserdata: usize, phquery: *mut isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhOpenQueryW(szdatasource: super::super::Foundation::PWSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
        }
        ::core::mem::transmute(PdhOpenQueryW(szdatasource.into_param().abi(), ::core::mem::transmute(dwuserdata), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseCounterPathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szfullpathbuffer: Param0, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseCounterPathA(szfullpathbuffer: super::super::Foundation::PSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhParseCounterPathA(szfullpathbuffer.into_param().abi(), ::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseCounterPathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfullpathbuffer: Param0, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseCounterPathW(szfullpathbuffer: super::super::Foundation::PWSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
        }
        ::core::mem::transmute(PdhParseCounterPathW(szfullpathbuffer.into_param().abi(), ::core::mem::transmute(pcounterpathelements), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseInstanceNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szinstancestring: Param0, szinstancename: super::super::Foundation::PSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseInstanceNameA(szinstancestring: super::super::Foundation::PSTR, szinstancename: super::super::Foundation::PSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhParseInstanceNameA(szinstancestring.into_param().abi(), ::core::mem::transmute(szinstancename), ::core::mem::transmute(pcchinstancenamelength), ::core::mem::transmute(szparentname), ::core::mem::transmute(pcchparentnamelength), ::core::mem::transmute(lpindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhParseInstanceNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szinstancestring: Param0, szinstancename: super::super::Foundation::PWSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhParseInstanceNameW(szinstancestring: super::super::Foundation::PWSTR, szinstancename: super::super::Foundation::PWSTR, pcchinstancenamelength: *mut u32, szparentname: super::super::Foundation::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhParseInstanceNameW(szinstancestring.into_param().abi(), ::core::mem::transmute(szinstancename), ::core::mem::transmute(pcchinstancenamelength), ::core::mem::transmute(szparentname), ::core::mem::transmute(pcchparentnamelength), ::core::mem::transmute(lpindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhReadRawLogRecord<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(hlog: isize, ftrecord: Param1, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhReadRawLogRecord(hlog: isize, ftrecord: super::super::Foundation::FILETIME, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhReadRawLogRecord(::core::mem::transmute(hlog), ftrecord.into_param().abi(), ::core::mem::transmute(prawlogrecord), ::core::mem::transmute(pdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhRemoveCounter(hcounter: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhRemoveCounter(hcounter: isize) -> i32;
        }
        ::core::mem::transmute(PdhRemoveCounter(::core::mem::transmute(hcounter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhSelectDataSourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hwndowner: Param0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: Param2, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSelectDataSourceA(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhSelectDataSourceA(hwndowner.into_param().abi(), ::core::mem::transmute(dwflags), szdatasource.into_param().abi(), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhSelectDataSourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hwndowner: Param0, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: Param2, pcchbufferlength: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSelectDataSourceW(hwndowner: super::super::Foundation::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: super::super::Foundation::PWSTR, pcchbufferlength: *mut u32) -> i32;
        }
        ::core::mem::transmute(PdhSelectDataSourceW(hwndowner.into_param().abi(), ::core::mem::transmute(dwflags), szdatasource.into_param().abi(), ::core::mem::transmute(pcchbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32;
        }
        ::core::mem::transmute(PdhSetCounterScaleFactor(::core::mem::transmute(hcounter), ::core::mem::transmute(lfactor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32;
        }
        ::core::mem::transmute(PdhSetDefaultRealTimeDataSource(::core::mem::transmute(dwdatasourceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32;
        }
        ::core::mem::transmute(PdhSetLogSetRunID(::core::mem::transmute(hlog), ::core::mem::transmute(runid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32;
        }
        ::core::mem::transmute(PdhSetQueryTimeRange(::core::mem::transmute(hquery), ::core::mem::transmute(pinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhUpdateLogA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hlog: isize, szuserstring: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogA(hlog: isize, szuserstring: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogA(::core::mem::transmute(hlog), szuserstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PdhUpdateLogFileCatalog(hlog: isize) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogFileCatalog(hlog: isize) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogFileCatalog(::core::mem::transmute(hlog)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhUpdateLogW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hlog: isize, szuserstring: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhUpdateLogW(hlog: isize, szuserstring: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhUpdateLogW(::core::mem::transmute(hlog), szuserstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szfullpathbuffer: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathA(szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathA(szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathExA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hdatasource: isize, szfullpathbuffer: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathExA(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathExA(::core::mem::transmute(hdatasource), szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathExW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hdatasource: isize, szfullpathbuffer: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathExW(hdatasource: isize, szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathExW(::core::mem::transmute(hdatasource), szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhValidatePathW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfullpathbuffer: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhValidatePathW(szfullpathbuffer: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhValidatePathW(szfullpathbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhVerifySQLDBA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhVerifySQLDBA(szdatasource: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(PdhVerifySQLDBA(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PdhVerifySQLDBW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szdatasource: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdhVerifySQLDBW(szdatasource: super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PdhVerifySQLDBW(szdatasource.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PerfAddCounters<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfAddCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
        }
        ::core::mem::transmute(PerfAddCounters(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfCloseQueryHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hquery: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfCloseQueryHandle(hquery: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(PerfCloseQueryHandle(hquery.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PerfCounterDataType(pub i32);
pub const PERF_ERROR_RETURN: PerfCounterDataType = PerfCounterDataType(0i32);
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = PerfCounterDataType(1i32);
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = PerfCounterDataType(2i32);
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = PerfCounterDataType(4i32);
pub const PERF_COUNTERSET: PerfCounterDataType = PerfCounterDataType(6i32);
impl ::core::convert::From<i32> for PerfCounterDataType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PerfCounterDataType {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfCreateInstance<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(providerhandle: Param0, countersetguid: *const ::windows::core::GUID, name: Param2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfCreateInstance(providerhandle: PerfProviderHandle, countersetguid: *const ::windows::core::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
        }
        ::core::mem::transmute(PerfCreateInstance(providerhandle.into_param().abi(), ::core::mem::transmute(countersetguid), name.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfDecrementULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDecrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfDecrementULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfDecrementULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDecrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfDecrementULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PerfDeleteCounters<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDeleteCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
        }
        ::core::mem::transmute(PerfDeleteCounters(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PerfDeleteInstance<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>>(provider: Param0, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfDeleteInstance(provider: PerfProviderHandle, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32;
        }
        ::core::mem::transmute(PerfDeleteInstance(provider.into_param().abi(), ::core::mem::transmute(instanceblock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfEnumerateCounterSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, pcountersetids: *mut ::windows::core::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfEnumerateCounterSet(szmachine: super::super::Foundation::PWSTR, pcountersetids: *mut ::windows::core::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfEnumerateCounterSet(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetids), ::core::mem::transmute(ccountersetids), ::core::mem::transmute(pccountersetidsactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfEnumerateCounterSetInstances<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, pcountersetid: *const ::windows::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfEnumerateCounterSetInstances(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows::core::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfEnumerateCounterSetInstances(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetid), ::core::mem::transmute(pinstances), ::core::mem::transmute(cbinstances), ::core::mem::transmute(pcbinstancesactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfIncrementULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfIncrementULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfIncrementULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfIncrementULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfIncrementULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfIncrementULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfOpenQueryHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, phquery: *mut PerfQueryHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfOpenQueryHandle(szmachine: super::super::Foundation::PWSTR, phquery: *mut PerfQueryHandle) -> u32;
        }
        ::core::mem::transmute(PerfOpenQueryHandle(szmachine.into_param().abi(), ::core::mem::transmute(phquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct PerfProviderHandle(pub isize);
impl ::core::default::Default for PerfProviderHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for PerfProviderHandle {}
unsafe impl ::windows::core::Abi for PerfProviderHandle {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryCounterData<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterData(hquery: PerfQueryHandle, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterData(hquery.into_param().abi(), ::core::mem::transmute(pcounterblock), ::core::mem::transmute(cbcounterblock), ::core::mem::transmute(pcbcounterblockactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PerfQueryCounterInfo<'a, Param0: ::windows::core::IntoParam<'a, PerfQueryHandle>>(hquery: Param0, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterInfo(hquery: PerfQueryHandle, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterInfo(hquery.into_param().abi(), ::core::mem::transmute(pcounters), ::core::mem::transmute(cbcounters), ::core::mem::transmute(pcbcountersactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryCounterSetRegistrationInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szmachine: Param0, pcountersetid: *const ::windows::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryCounterSetRegistrationInfo(szmachine: super::super::Foundation::PWSTR, pcountersetid: *const ::windows::core::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32;
        }
        ::core::mem::transmute(PerfQueryCounterSetRegistrationInfo(szmachine.into_param().abi(), ::core::mem::transmute(pcountersetid), ::core::mem::transmute(requestcode), ::core::mem::transmute(requestlangid), ::core::mem::transmute(pbreginfo), ::core::mem::transmute(cbreginfo), ::core::mem::transmute(pcbreginfoactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct PerfQueryHandle(pub isize);
impl ::core::default::Default for PerfQueryHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for PerfQueryHandle {}
unsafe impl ::windows::core::Abi for PerfQueryHandle {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfQueryInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(providerhandle: Param0, countersetguid: *const ::windows::core::GUID, name: Param2, id: u32) -> *mut PERF_COUNTERSET_INSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfQueryInstance(providerhandle: super::super::Foundation::HANDLE, countersetguid: *const ::windows::core::GUID, name: super::super::Foundation::PWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
        }
        ::core::mem::transmute(PerfQueryInstance(providerhandle.into_param().abi(), ::core::mem::transmute(countersetguid), name.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PerfRegInfoType(pub i32);
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = PerfRegInfoType(1i32);
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = PerfRegInfoType(2i32);
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = PerfRegInfoType(3i32);
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = PerfRegInfoType(4i32);
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = PerfRegInfoType(5i32);
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = PerfRegInfoType(6i32);
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = PerfRegInfoType(7i32);
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = PerfRegInfoType(8i32);
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = PerfRegInfoType(9i32);
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = PerfRegInfoType(10i32);
impl ::core::convert::From<i32> for PerfRegInfoType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PerfRegInfoType {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetCounterRefValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetCounterRefValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(PerfSetCounterRefValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(address)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetCounterSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(providerhandle: Param0, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetCounterSetInfo(providerhandle: super::super::Foundation::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32;
        }
        ::core::mem::transmute(PerfSetCounterSetInfo(providerhandle.into_param().abi(), ::core::mem::transmute(template), ::core::mem::transmute(templatesize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetULongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetULongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
        }
        ::core::mem::transmute(PerfSetULongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerfSetULongLongCounterValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(provider: Param0, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfSetULongLongCounterValue(provider: super::super::Foundation::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
        }
        ::core::mem::transmute(PerfSetULongLongCounterValue(provider.into_param().abi(), ::core::mem::transmute(instance), ::core::mem::transmute(counterid), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PerfStartProvider(providerguid: *const ::windows::core::GUID, controlcallback: ::core::option::Option<PERFLIBREQUEST>, phprovider: *mut PerfProviderHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStartProvider(providerguid: *const ::windows::core::GUID, controlcallback: ::windows::core::RawPtr, phprovider: *mut PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStartProvider(::core::mem::transmute(providerguid), ::core::mem::transmute(controlcallback), ::core::mem::transmute(phprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PerfStartProviderEx(providerguid: *const ::windows::core::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStartProviderEx(providerguid: *const ::windows::core::GUID, providercontext: *const ::core::mem::ManuallyDrop<PERF_PROVIDER_CONTEXT>, provider: *mut PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStartProviderEx(::core::mem::transmute(providerguid), ::core::mem::transmute(providercontext), ::core::mem::transmute(provider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PerfStopProvider<'a, Param0: ::windows::core::IntoParam<'a, PerfProviderHandle>>(providerhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerfStopProvider(providerhandle: PerfProviderHandle) -> u32;
        }
        ::core::mem::transmute(PerfStopProvider(providerhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryPerformanceCounter(::core::mem::transmute(lpperformancecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryPerformanceFrequency(::core::mem::transmute(lpfrequency)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REAL_TIME_DATA_SOURCE_ID_FLAGS(pub u32);
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(1u32);
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = REAL_TIME_DATA_SOURCE_ID_FLAGS(4u32);
impl ::core::convert::From<u32> for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for REAL_TIME_DATA_SOURCE_ID_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ReportValueTypeConstants(pub i32);
pub const sysmonDefaultValue: ReportValueTypeConstants = ReportValueTypeConstants(0i32);
pub const sysmonCurrentValue: ReportValueTypeConstants = ReportValueTypeConstants(1i32);
pub const sysmonAverage: ReportValueTypeConstants = ReportValueTypeConstants(2i32);
pub const sysmonMinimum: ReportValueTypeConstants = ReportValueTypeConstants(3i32);
pub const sysmonMaximum: ReportValueTypeConstants = ReportValueTypeConstants(4i32);
impl ::core::convert::From<i32> for ReportValueTypeConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ReportValueTypeConstants {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ResourcePolicy(pub i32);
pub const plaDeleteLargest: ResourcePolicy = ResourcePolicy(0i32);
pub const plaDeleteOldest: ResourcePolicy = ResourcePolicy(1i32);
impl ::core::convert::From<i32> for ResourcePolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ResourcePolicy {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestorePerfRegistryFromFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0, szlangid: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestorePerfRegistryFromFileW(szfilename: super::super::Foundation::PWSTR, szlangid: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(RestorePerfRegistryFromFileW(szfilename.into_param().abi(), szlangid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const S_PDH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04d66358_c4a1_419b_8023_23b73902de2c);
pub const ServerDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceAsTrustedA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szreserved: Param0, szservicename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceAsTrustedA(szreserved: super::super::Foundation::PSTR, szservicename: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(SetServiceAsTrustedA(szreserved.into_param().abi(), szservicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceAsTrustedW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szreserved: Param0, szservicename: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceAsTrustedW(szreserved: super::super::Foundation::PWSTR, szservicename: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(SetServiceAsTrustedW(szreserved.into_param().abi(), szservicename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SourcePropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cf32aa1_7571_11d0_93c4_00aa00a3ddea);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StreamMode(pub i32);
pub const plaFile: StreamMode = StreamMode(1i32);
pub const plaRealTime: StreamMode = StreamMode(2i32);
pub const plaBoth: StreamMode = StreamMode(3i32);
pub const plaBuffering: StreamMode = StreamMode(4i32);
impl ::core::convert::From<i32> for StreamMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StreamMode {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SysmonBatchReason(pub i32);
pub const sysmonBatchNone: SysmonBatchReason = SysmonBatchReason(0i32);
pub const sysmonBatchAddFiles: SysmonBatchReason = SysmonBatchReason(1i32);
pub const sysmonBatchAddCounters: SysmonBatchReason = SysmonBatchReason(2i32);
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = SysmonBatchReason(3i32);
impl ::core::convert::From<i32> for SysmonBatchReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SysmonBatchReason {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SysmonDataType(pub i32);
pub const sysmonDataAvg: SysmonDataType = SysmonDataType(1i32);
pub const sysmonDataMin: SysmonDataType = SysmonDataType(2i32);
pub const sysmonDataMax: SysmonDataType = SysmonDataType(3i32);
pub const sysmonDataTime: SysmonDataType = SysmonDataType(4i32);
pub const sysmonDataCount: SysmonDataType = SysmonDataType(5i32);
impl ::core::convert::From<i32> for SysmonDataType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SysmonDataType {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SysmonFileType(pub i32);
pub const sysmonFileHtml: SysmonFileType = SysmonFileType(1i32);
pub const sysmonFileReport: SysmonFileType = SysmonFileType(2i32);
pub const sysmonFileCsv: SysmonFileType = SysmonFileType(3i32);
pub const sysmonFileTsv: SysmonFileType = SysmonFileType(4i32);
pub const sysmonFileBlg: SysmonFileType = SysmonFileType(5i32);
pub const sysmonFileRetiredBlg: SysmonFileType = SysmonFileType(6i32);
pub const sysmonFileGif: SysmonFileType = SysmonFileType(7i32);
impl ::core::convert::From<i32> for SysmonFileType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SysmonFileType {
    type Abi = Self;
}
pub const SystemDataCollectorSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const SystemMonitor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d2d8e0_d1dd_11ce_940f_008029004347);
pub const SystemMonitor2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f30578c_5f38_4612_acfe_6ed04c7b7af8);
pub const TraceDataProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnloadPerfCounterTextStringsA(lpcommandline: super::super::Foundation::PSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(UnloadPerfCounterTextStringsA(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnloadPerfCounterTextStringsW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpcommandline: Param0, bquietmodearg: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnloadPerfCounterTextStringsW(lpcommandline: super::super::Foundation::PWSTR, bquietmodearg: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(UnloadPerfCounterTextStringsW(lpcommandline.into_param().abi(), bquietmodearg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdatePerfNameFilesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(sznewctrfilepath: Param0, sznewhlpfilepath: Param1, szlanguageid: Param2, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePerfNameFilesA(sznewctrfilepath: super::super::Foundation::PSTR, sznewhlpfilepath: super::super::Foundation::PSTR, szlanguageid: super::super::Foundation::PSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(UpdatePerfNameFilesA(sznewctrfilepath.into_param().abi(), sznewhlpfilepath.into_param().abi(), szlanguageid.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdatePerfNameFilesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(sznewctrfilepath: Param0, sznewhlpfilepath: Param1, szlanguageid: Param2, dwflags: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePerfNameFilesW(sznewctrfilepath: super::super::Foundation::PWSTR, sznewhlpfilepath: super::super::Foundation::PWSTR, szlanguageid: super::super::Foundation::PWSTR, dwflags: usize) -> u32;
        }
        ::core::mem::transmute(UpdatePerfNameFilesW(sznewctrfilepath.into_param().abi(), sznewhlpfilepath.into_param().abi(), szlanguageid.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ValueMapType(pub i32);
pub const plaIndex: ValueMapType = ValueMapType(1i32);
pub const plaFlag: ValueMapType = ValueMapType(2i32);
pub const plaFlagArray: ValueMapType = ValueMapType(3i32);
pub const plaValidation: ValueMapType = ValueMapType(4i32);
impl ::core::convert::From<i32> for ValueMapType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ValueMapType {
    type Abi = Self;
}
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
pub const WINPERF_LOG_NONE: u32 = 0u32;
pub const WINPERF_LOG_USER: u32 = 1u32;
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WeekDays(pub i32);
pub const plaRunOnce: WeekDays = WeekDays(0i32);
pub const plaSunday: WeekDays = WeekDays(1i32);
pub const plaMonday: WeekDays = WeekDays(2i32);
pub const plaTuesday: WeekDays = WeekDays(4i32);
pub const plaWednesday: WeekDays = WeekDays(8i32);
pub const plaThursday: WeekDays = WeekDays(16i32);
pub const plaFriday: WeekDays = WeekDays(32i32);
pub const plaSaturday: WeekDays = WeekDays(64i32);
pub const plaEveryday: WeekDays = WeekDays(127i32);
impl ::core::convert::From<i32> for WeekDays {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WeekDays {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct _ICounterItemUnion(pub ::windows::core::IUnknown);
impl _ICounterItemUnion {
    pub unsafe fn Value(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn Color(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetWidth(&self, iwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(iwidth)).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLineStyle(&self, ilinestyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ilinestyle)).ok()
    }
    pub unsafe fn LineStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetScaleFactor(&self, iscale: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iscale)).ok()
    }
    pub unsafe fn ScaleFactor(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetValue(&self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(value), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn GetStatistics(&self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(max), ::core::mem::transmute(min), ::core::mem::transmute(avg), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn SetSelected(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn Selected(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetVisible(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn Visible(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDataAt(&self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iwhich), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for _ICounterItemUnion {
    type Vtable = _ICounterItemUnion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde1a6b74_9182_4c41_8e2c_24c2cd30ee83);
}
impl ::core::convert::From<_ICounterItemUnion> for ::windows::core::IUnknown {
    fn from(value: _ICounterItemUnion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&_ICounterItemUnion> for ::windows::core::IUnknown {
    fn from(value: &_ICounterItemUnion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ICounterItemUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _ICounterItemUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ICounterItemUnion_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdblvalue: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iwidth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ilinestyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iscale: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstrvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: i32, iwhich: SysmonDataType, pvariant: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct _ISystemMonitorUnion(pub ::windows::core::IUnknown);
impl _ISystemMonitorUnion {
    pub unsafe fn Appearance(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppearance(&self, iappearance: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(iappearance)).ok()
    }
    pub unsafe fn BackColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBackColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn BorderStyle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBorderStyle(&self, iborderstyle: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iborderstyle)).ok()
    }
    pub unsafe fn ForeColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetForeColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn Font(&self) -> ::windows::core::Result<super::Ole::IFontDisp> {
        let mut result__: <super::Ole::IFontDisp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::IFontDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn putref_Font<'a, Param0: ::windows::core::IntoParam<'a, super::Ole::IFontDisp>>(&self, pfont: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pfont.into_param().abi()).ok()
    }
    pub unsafe fn Counters(&self) -> ::windows::core::Result<ICounters> {
        let mut result__: <ICounters as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ICounters>(result__)
    }
    pub unsafe fn SetShowVerticalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowVerticalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowHorizontalGrid(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowHorizontalGrid(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowLegend(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowLegend(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowScaleLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowScaleLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowValueBar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowValueBar(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMaximumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn MaximumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMinimumScale(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn MinimumScale(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetUpdateInterval(&self, fvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(fvalue)).ok()
    }
    pub unsafe fn UpdateInterval(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDisplayType(&self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(edisplaytype)).ok()
    }
    pub unsafe fn DisplayType(&self) -> ::windows::core::Result<DisplayTypeConstants> {
        let mut result__: <DisplayTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DisplayTypeConstants>(result__)
    }
    pub unsafe fn SetManualUpdate(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ManualUpdate(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGraphTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GraphTitle(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYAxisLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), bstitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YAxisLabel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CollectSample(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn UpdateGraph(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BrowseCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisplayProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Counter(&self, iindex: i32) -> ::windows::core::Result<ICounterItem> {
        let mut result__: <ICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), &mut result__).from_abi::<ICounterItem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCounter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bspath: Param0) -> ::windows::core::Result<ICounterItem> {
        let mut result__: <ICounterItem as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), bspath.into_param().abi(), &mut result__).from_abi::<ICounterItem>(result__)
    }
    pub unsafe fn DeleteCounter<'a, Param0: ::windows::core::IntoParam<'a, ICounterItem>>(&self, pctr: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), pctr.into_param().abi()).ok()
    }
    pub unsafe fn BackColorCtl(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBackColorCtl(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bsfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), bsfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogFileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetLogViewStart(&self, starttime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime)).ok()
    }
    pub unsafe fn LogViewStart(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLogViewStop(&self, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(stoptime)).ok()
    }
    pub unsafe fn LogViewStop(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn GridColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetGridColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn TimeBarColor(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetTimeBarColor(&self, color: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    pub unsafe fn Highlight(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetHighlight(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowToolbar(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowToolbar(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn Paste(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Copy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetReadOnly(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ReadOnly(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetReportValueType(&self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), ::core::mem::transmute(ereportvaluetype)).ok()
    }
    pub unsafe fn ReportValueType(&self) -> ::windows::core::Result<ReportValueTypeConstants> {
        let mut result__: <ReportValueTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ReportValueTypeConstants>(result__)
    }
    pub unsafe fn SetMonitorDuplicateInstances(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn MonitorDuplicateInstances(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisplayFilter(&self, ivalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), ::core::mem::transmute(ivalue)).ok()
    }
    pub unsafe fn DisplayFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn LogFiles(&self) -> ::windows::core::Result<ILogFiles> {
        let mut result__: <ILogFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ILogFiles>(result__)
    }
    pub unsafe fn SetDataSourceType(&self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatasourcetype)).ok()
    }
    pub unsafe fn DataSourceType(&self) -> ::windows::core::Result<DataSourceTypeConstants> {
        let mut result__: <DataSourceTypeConstants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataSourceTypeConstants>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlDsnName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqldsnname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), bssqldsnname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlDsnName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSqlLogSetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bssqllogsetname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), bssqllogsetname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SqlLogSetName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetEnableDigitGrouping(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn EnableDigitGrouping(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnableToolTips(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn EnableToolTips(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShowTimeAxisLabels(&self, bstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), ::core::mem::transmute(bstate)).ok()
    }
    pub unsafe fn ShowTimeAxisLabels(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetChartScroll(&self, bscroll: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), ::core::mem::transmute(bscroll)).ok()
    }
    pub unsafe fn ChartScroll(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDataPointCount(&self, inewcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), ::core::mem::transmute(inewcount)).ok()
    }
    pub unsafe fn DataPointCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ScaleToFit(&self, bselectedcountersonly: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), ::core::mem::transmute(bselectedcountersonly)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Relog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfilename: Param0, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(esysmonfiletype), ::core::mem::transmute(ifilter)).ok()
    }
    pub unsafe fn ClearData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn LogSourceStartTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn LogSourceStopTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLogViewRange(&self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    pub unsafe fn GetLogViewRange(&self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(stoptime)).ok()
    }
    pub unsafe fn BatchingLock(&self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), ::core::mem::transmute(flock), ::core::mem::transmute(ebatchreason)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsettingfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).98)(::core::mem::transmute_copy(self), bstrsettingfilename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for _ISystemMonitorUnion {
    type Vtable = _ISystemMonitorUnion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8a77338_265f_4de5_aa25_c7da1ce5a8f4);
}
impl ::core::convert::From<_ISystemMonitorUnion> for ::windows::core::IUnknown {
    fn from(value: _ISystemMonitorUnion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&_ISystemMonitorUnion> for ::windows::core::IUnknown {
    fn from(value: &_ISystemMonitorUnion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _ISystemMonitorUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _ISystemMonitorUnion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _ISystemMonitorUnion_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iappearance: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iappearance: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iborderstyle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iborderstyle: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fvalue: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfvalue: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstitle: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstitle: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bsfilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ivalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pivalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqldsnname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bssqllogsetname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstate: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstate: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bscroll: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbscroll: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inewcount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pidatapointcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bselectedcountersonly: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
