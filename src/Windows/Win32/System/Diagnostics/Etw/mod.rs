#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CLASSIC_EVENT_ID {
    pub EventGuid: ::windows::core::GUID,
    pub Type: u8,
    pub Reserved: [u8; 7],
}
impl CLASSIC_EVENT_ID {}
impl ::core::default::Default for CLASSIC_EVENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CLASSIC_EVENT_ID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CLASSIC_EVENT_ID").field("EventGuid", &self.EventGuid).field("Type", &self.Type).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for CLASSIC_EVENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.EventGuid == other.EventGuid && self.Type == other.Type && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for CLASSIC_EVENT_ID {}
unsafe impl ::windows::core::Abi for CLASSIC_EVENT_ID {
    type Abi = Self;
}
pub const CLSID_TraceRelogger: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b40792d_05ff_44c4_9058_f440c71f17d4);
pub const CTraceRelogger: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b40792d_05ff_44c4_9058_f440c71f17d4);
#[inline]
pub unsafe fn CloseTrace(tracehandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseTrace(tracehandle: u64) -> u32;
        }
        ::core::mem::transmute(CloseTrace(::core::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ControlTraceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
        }
        ::core::mem::transmute(ControlTraceA(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties), ::core::mem::transmute(controlcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ControlTraceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES, controlcode: EVENT_TRACE_CONTROL) -> u32;
        }
        ::core::mem::transmute(ControlTraceW(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties), ::core::mem::transmute(controlcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTraceInstanceId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(reghandle: Param0, instinfo: *mut EVENT_INSTANCE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTraceInstanceId(reghandle: super::super::super::Foundation::HANDLE, instinfo: *mut EVENT_INSTANCE_INFO) -> u32;
        }
        ::core::mem::transmute(CreateTraceInstanceId(reghandle.into_param().abi(), ::core::mem::transmute(instinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CveEventWrite<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(cveid: Param0, additionaldetails: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CveEventWrite(cveid: super::super::super::Foundation::PWSTR, additionaldetails: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(CveEventWrite(cveid.into_param().abi(), additionaldetails.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DECODING_SOURCE(pub i32);
pub const DecodingSourceXMLFile: DECODING_SOURCE = DECODING_SOURCE(0i32);
pub const DecodingSourceWbem: DECODING_SOURCE = DECODING_SOURCE(1i32);
pub const DecodingSourceWPP: DECODING_SOURCE = DECODING_SOURCE(2i32);
pub const DecodingSourceTlg: DECODING_SOURCE = DECODING_SOURCE(3i32);
pub const DecodingSourceMax: DECODING_SOURCE = DECODING_SOURCE(4i32);
impl ::core::convert::From<i32> for DECODING_SOURCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DECODING_SOURCE {
    type Abi = Self;
}
pub const DefaultTraceSecurityGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0811c1af_7a07_4a06_82ed_869455cdf713);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENABLECALLBACK_ENABLED_STATE(pub u32);
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(0u32);
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(1u32);
pub const EVENT_CONTROL_CODE_CAPTURE_STATE: ENABLECALLBACK_ENABLED_STATE = ENABLECALLBACK_ENABLED_STATE(2u32);
impl ::core::convert::From<u32> for ENABLECALLBACK_ENABLED_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ENABLECALLBACK_ENABLED_STATE {
    type Abi = Self;
}
impl ::core::ops::BitOr for ENABLECALLBACK_ENABLED_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ENABLECALLBACK_ENABLED_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ENABLECALLBACK_ENABLED_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ENABLECALLBACK_ENABLED_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ENABLECALLBACK_ENABLED_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows::core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: u32,
}
impl ENABLE_TRACE_PARAMETERS {}
impl ::core::default::Default for ENABLE_TRACE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ENABLE_TRACE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENABLE_TRACE_PARAMETERS")
            .field("Version", &self.Version)
            .field("EnableProperty", &self.EnableProperty)
            .field("ControlFlags", &self.ControlFlags)
            .field("SourceId", &self.SourceId)
            .field("EnableFilterDesc", &self.EnableFilterDesc)
            .field("FilterDescCount", &self.FilterDescCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENABLE_TRACE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.EnableProperty == other.EnableProperty && self.ControlFlags == other.ControlFlags && self.SourceId == other.SourceId && self.EnableFilterDesc == other.EnableFilterDesc && self.FilterDescCount == other.FilterDescCount
    }
}
impl ::core::cmp::Eq for ENABLE_TRACE_PARAMETERS {}
unsafe impl ::windows::core::Abi for ENABLE_TRACE_PARAMETERS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ENABLE_TRACE_PARAMETERS_V1 {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: ::windows::core::GUID,
    pub EnableFilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
}
impl ENABLE_TRACE_PARAMETERS_V1 {}
impl ::core::default::Default for ENABLE_TRACE_PARAMETERS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ENABLE_TRACE_PARAMETERS_V1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENABLE_TRACE_PARAMETERS_V1").field("Version", &self.Version).field("EnableProperty", &self.EnableProperty).field("ControlFlags", &self.ControlFlags).field("SourceId", &self.SourceId).field("EnableFilterDesc", &self.EnableFilterDesc).finish()
    }
}
impl ::core::cmp::PartialEq for ENABLE_TRACE_PARAMETERS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.EnableProperty == other.EnableProperty && self.ControlFlags == other.ControlFlags && self.SourceId == other.SourceId && self.EnableFilterDesc == other.EnableFilterDesc
    }
}
impl ::core::cmp::Eq for ENABLE_TRACE_PARAMETERS_V1 {}
unsafe impl ::windows::core::Abi for ENABLE_TRACE_PARAMETERS_V1 {
    type Abi = Self;
}
pub const ENABLE_TRACE_PARAMETERS_VERSION: u32 = 1u32;
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: u32 = 2u32;
pub const ETW_ASCIICHAR_TYPE_VALUE: u32 = 102u32;
pub const ETW_ASCIISTRING_TYPE_VALUE: u32 = 103u32;
pub const ETW_BOOLEAN_TYPE_VALUE: u32 = 14u32;
pub const ETW_BOOL_TYPE_VALUE: u32 = 108u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ETW_BUFFER_CONTEXT {
    pub Anonymous: ETW_BUFFER_CONTEXT_0,
    pub LoggerId: u16,
}
impl ETW_BUFFER_CONTEXT {}
impl ::core::default::Default for ETW_BUFFER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ETW_BUFFER_CONTEXT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ETW_BUFFER_CONTEXT {}
unsafe impl ::windows::core::Abi for ETW_BUFFER_CONTEXT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union ETW_BUFFER_CONTEXT_0 {
    pub Anonymous: ETW_BUFFER_CONTEXT_0_0,
    pub ProcessorIndex: u16,
}
impl ETW_BUFFER_CONTEXT_0 {}
impl ::core::default::Default for ETW_BUFFER_CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ETW_BUFFER_CONTEXT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ETW_BUFFER_CONTEXT_0 {}
unsafe impl ::windows::core::Abi for ETW_BUFFER_CONTEXT_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ETW_BUFFER_CONTEXT_0_0 {
    pub ProcessorNumber: u8,
    pub Alignment: u8,
}
impl ETW_BUFFER_CONTEXT_0_0 {}
impl ::core::default::Default for ETW_BUFFER_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ETW_BUFFER_CONTEXT_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("ProcessorNumber", &self.ProcessorNumber).field("Alignment", &self.Alignment).finish()
    }
}
impl ::core::cmp::PartialEq for ETW_BUFFER_CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessorNumber == other.ProcessorNumber && self.Alignment == other.Alignment
    }
}
impl ::core::cmp::Eq for ETW_BUFFER_CONTEXT_0_0 {}
unsafe impl ::windows::core::Abi for ETW_BUFFER_CONTEXT_0_0 {
    type Abi = Self;
}
pub const ETW_BYTE_TYPE_VALUE: u32 = 4u32;
pub const ETW_CHAR_TYPE_VALUE: u32 = 11u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_COMPRESSION_RESUMPTION_MODE(pub i32);
pub const EtwCompressionModeRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(0i32);
pub const EtwCompressionModeNoDisable: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(1i32);
pub const EtwCompressionModeNoRestart: ETW_COMPRESSION_RESUMPTION_MODE = ETW_COMPRESSION_RESUMPTION_MODE(2i32);
impl ::core::convert::From<i32> for ETW_COMPRESSION_RESUMPTION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ETW_COMPRESSION_RESUMPTION_MODE {
    type Abi = Self;
}
pub const ETW_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 109u32;
pub const ETW_COUNTED_STRING_TYPE_VALUE: u32 = 104u32;
pub const ETW_DATETIME_TYPE_VALUE: u32 = 119u32;
pub const ETW_DECIMAL_TYPE_VALUE: u32 = 15u32;
pub const ETW_DOUBLE_TYPE_VALUE: u32 = 13u32;
pub const ETW_GUID_TYPE_VALUE: u32 = 101u32;
pub const ETW_HIDDEN_TYPE_VALUE: u32 = 107u32;
pub const ETW_INT16_TYPE_VALUE: u32 = 5u32;
pub const ETW_INT32_TYPE_VALUE: u32 = 7u32;
pub const ETW_INT64_TYPE_VALUE: u32 = 9u32;
pub const ETW_NON_NULL_TERMINATED_STRING_TYPE_VALUE: u32 = 112u32;
pub const ETW_NULL_TYPE_VALUE: u32 = 0u32;
pub const ETW_OBJECT_TYPE_VALUE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ETW_PMC_COUNTER_OWNER {
    pub OwnerType: ETW_PMC_COUNTER_OWNER_TYPE,
    pub ProfileSource: u32,
    pub OwnerTag: u32,
}
impl ETW_PMC_COUNTER_OWNER {}
impl ::core::default::Default for ETW_PMC_COUNTER_OWNER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ETW_PMC_COUNTER_OWNER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ETW_PMC_COUNTER_OWNER").field("OwnerType", &self.OwnerType).field("ProfileSource", &self.ProfileSource).field("OwnerTag", &self.OwnerTag).finish()
    }
}
impl ::core::cmp::PartialEq for ETW_PMC_COUNTER_OWNER {
    fn eq(&self, other: &Self) -> bool {
        self.OwnerType == other.OwnerType && self.ProfileSource == other.ProfileSource && self.OwnerTag == other.OwnerTag
    }
}
impl ::core::cmp::Eq for ETW_PMC_COUNTER_OWNER {}
unsafe impl ::windows::core::Abi for ETW_PMC_COUNTER_OWNER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    pub ProcessorNumber: u32,
    pub NumberOfCounters: u32,
    pub CounterOwners: [ETW_PMC_COUNTER_OWNER; 1],
}
impl ETW_PMC_COUNTER_OWNERSHIP_STATUS {}
impl ::core::default::Default for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ETW_PMC_COUNTER_OWNERSHIP_STATUS").field("ProcessorNumber", &self.ProcessorNumber).field("NumberOfCounters", &self.NumberOfCounters).field("CounterOwners", &self.CounterOwners).finish()
    }
}
impl ::core::cmp::PartialEq for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessorNumber == other.ProcessorNumber && self.NumberOfCounters == other.NumberOfCounters && self.CounterOwners == other.CounterOwners
    }
}
impl ::core::cmp::Eq for ETW_PMC_COUNTER_OWNERSHIP_STATUS {}
unsafe impl ::windows::core::Abi for ETW_PMC_COUNTER_OWNERSHIP_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_PMC_COUNTER_OWNER_TYPE(pub i32);
pub const EtwPmcOwnerFree: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(0i32);
pub const EtwPmcOwnerUntagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(1i32);
pub const EtwPmcOwnerTagged: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(2i32);
pub const EtwPmcOwnerTaggedWithSource: ETW_PMC_COUNTER_OWNER_TYPE = ETW_PMC_COUNTER_OWNER_TYPE(3i32);
impl ::core::convert::From<i32> for ETW_PMC_COUNTER_OWNER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ETW_PMC_COUNTER_OWNER_TYPE {
    type Abi = Self;
}
pub const ETW_POINTER_TYPE_VALUE: u32 = 105u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_PROCESS_HANDLE_INFO_TYPE(pub i32);
pub const EtwQueryPartitionInformation: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(1i32);
pub const EtwQueryPartitionInformationV2: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(2i32);
pub const EtwQueryLastDroppedTimes: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(3i32);
pub const EtwQueryProcessHandleInfoMax: ETW_PROCESS_HANDLE_INFO_TYPE = ETW_PROCESS_HANDLE_INFO_TYPE(4i32);
impl ::core::convert::From<i32> for ETW_PROCESS_HANDLE_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ETW_PROCESS_HANDLE_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ETW_PROVIDER_TRAIT_TYPE(pub i32);
pub const EtwProviderTraitTypeGroup: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(1i32);
pub const EtwProviderTraitDecodeGuid: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(2i32);
pub const EtwProviderTraitTypeMax: ETW_PROVIDER_TRAIT_TYPE = ETW_PROVIDER_TRAIT_TYPE(3i32);
impl ::core::convert::From<i32> for ETW_PROVIDER_TRAIT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ETW_PROVIDER_TRAIT_TYPE {
    type Abi = Self;
}
pub const ETW_PTVECTOR_TYPE_VALUE: u32 = 117u32;
pub const ETW_REDUCED_ANSISTRING_TYPE_VALUE: u32 = 113u32;
pub const ETW_REDUCED_STRING_TYPE_VALUE: u32 = 114u32;
pub const ETW_REFRENCE_TYPE_VALUE: u32 = 120u32;
pub const ETW_REVERSED_COUNTED_ANSISTRING_TYPE_VALUE: u32 = 111u32;
pub const ETW_REVERSED_COUNTED_STRING_TYPE_VALUE: u32 = 110u32;
pub const ETW_SBYTE_TYPE_VALUE: u32 = 3u32;
pub const ETW_SID_TYPE_VALUE: u32 = 115u32;
pub const ETW_SINGLE_TYPE_VALUE: u32 = 12u32;
pub const ETW_SIZET_TYPE_VALUE: u32 = 106u32;
pub const ETW_STRING_TYPE_VALUE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ETW_TRACE_PARTITION_INFORMATION {
    pub PartitionId: ::windows::core::GUID,
    pub ParentId: ::windows::core::GUID,
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
}
impl ETW_TRACE_PARTITION_INFORMATION {}
impl ::core::default::Default for ETW_TRACE_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ETW_TRACE_PARTITION_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ETW_TRACE_PARTITION_INFORMATION").field("PartitionId", &self.PartitionId).field("ParentId", &self.ParentId).field("QpcOffsetFromRoot", &self.QpcOffsetFromRoot).field("PartitionType", &self.PartitionType).finish()
    }
}
impl ::core::cmp::PartialEq for ETW_TRACE_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionId == other.PartitionId && self.ParentId == other.ParentId && self.QpcOffsetFromRoot == other.QpcOffsetFromRoot && self.PartitionType == other.PartitionType
    }
}
impl ::core::cmp::Eq for ETW_TRACE_PARTITION_INFORMATION {}
unsafe impl ::windows::core::Abi for ETW_TRACE_PARTITION_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ETW_TRACE_PARTITION_INFORMATION_V2 {
    pub QpcOffsetFromRoot: i64,
    pub PartitionType: u32,
    pub PartitionId: super::super::super::Foundation::PWSTR,
    pub ParentId: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ETW_TRACE_PARTITION_INFORMATION_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ETW_TRACE_PARTITION_INFORMATION_V2").field("QpcOffsetFromRoot", &self.QpcOffsetFromRoot).field("PartitionType", &self.PartitionType).field("PartitionId", &self.PartitionId).field("ParentId", &self.ParentId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ETW_TRACE_PARTITION_INFORMATION_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.QpcOffsetFromRoot == other.QpcOffsetFromRoot && self.PartitionType == other.PartitionType && self.PartitionId == other.PartitionId && self.ParentId == other.ParentId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ETW_TRACE_PARTITION_INFORMATION_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ETW_TRACE_PARTITION_INFORMATION_V2 {
    type Abi = Self;
}
pub const ETW_UINT16_TYPE_VALUE: u32 = 6u32;
pub const ETW_UINT32_TYPE_VALUE: u32 = 8u32;
pub const ETW_UINT64_TYPE_VALUE: u32 = 10u32;
pub const ETW_VARIANT_TYPE_VALUE: u32 = 116u32;
pub const ETW_WMITIME_TYPE_VALUE: u32 = 118u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENTSECURITYOPERATION(pub i32);
pub const EventSecuritySetDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(0i32);
pub const EventSecuritySetSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(1i32);
pub const EventSecurityAddDACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(2i32);
pub const EventSecurityAddSACL: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(3i32);
pub const EventSecurityMax: EVENTSECURITYOPERATION = EVENTSECURITYOPERATION(4i32);
impl ::core::convert::From<i32> for EVENTSECURITYOPERATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVENTSECURITYOPERATION {
    type Abi = Self;
}
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: u32 = 3u32;
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: u32 = 5u32;
pub const EVENT_ACTIVITY_CTRL_GET_ID: u32 = 1u32;
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: u32 = 4u32;
pub const EVENT_ACTIVITY_CTRL_SET_ID: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0,
}
impl EVENT_DATA_DESCRIPTOR {}
impl ::core::default::Default for EVENT_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_DATA_DESCRIPTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_DATA_DESCRIPTOR {}
unsafe impl ::windows::core::Abi for EVENT_DATA_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_DATA_DESCRIPTOR_0 {
    pub Reserved: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0_0,
}
impl EVENT_DATA_DESCRIPTOR_0 {}
impl ::core::default::Default for EVENT_DATA_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_DATA_DESCRIPTOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_DATA_DESCRIPTOR_0 {}
unsafe impl ::windows::core::Abi for EVENT_DATA_DESCRIPTOR_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
}
impl EVENT_DATA_DESCRIPTOR_0_0 {}
impl ::core::default::Default for EVENT_DATA_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_DATA_DESCRIPTOR_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Type", &self.Type).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_DATA_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for EVENT_DATA_DESCRIPTOR_0_0 {}
unsafe impl ::windows::core::Abi for EVENT_DATA_DESCRIPTOR_0_0 {
    type Abi = Self;
}
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: u32 = 1u32;
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: u32 = 0u32;
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: u32 = 2u32;
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: u32 = 3u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_DESCRIPTOR {
    pub Id: u16,
    pub Version: u8,
    pub Channel: u8,
    pub Level: u8,
    pub Opcode: u8,
    pub Task: u16,
    pub Keyword: u64,
}
impl EVENT_DESCRIPTOR {}
impl ::core::default::Default for EVENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_DESCRIPTOR").field("Id", &self.Id).field("Version", &self.Version).field("Channel", &self.Channel).field("Level", &self.Level).field("Opcode", &self.Opcode).field("Task", &self.Task).field("Keyword", &self.Keyword).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Version == other.Version && self.Channel == other.Channel && self.Level == other.Level && self.Opcode == other.Opcode && self.Task == other.Task && self.Keyword == other.Keyword
    }
}
impl ::core::cmp::Eq for EVENT_DESCRIPTOR {}
unsafe impl ::windows::core::Abi for EVENT_DESCRIPTOR {
    type Abi = Self;
}
pub const EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0: u32 = 64u32;
pub const EVENT_ENABLE_PROPERTY_ENABLE_SILOS: u32 = 1024u32;
pub const EVENT_ENABLE_PROPERTY_EVENT_KEY: u32 = 256u32;
pub const EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE: u32 = 512u32;
pub const EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0: u32 = 16u32;
pub const EVENT_ENABLE_PROPERTY_PROCESS_START_KEY: u32 = 128u32;
pub const EVENT_ENABLE_PROPERTY_PROVIDER_GROUP: u32 = 32u32;
pub const EVENT_ENABLE_PROPERTY_PSM_KEY: u32 = 8u32;
pub const EVENT_ENABLE_PROPERTY_SID: u32 = 1u32;
pub const EVENT_ENABLE_PROPERTY_SOURCE_CONTAINER_TRACKING: u32 = 2048u32;
pub const EVENT_ENABLE_PROPERTY_STACK_TRACE: u32 = 4u32;
pub const EVENT_ENABLE_PROPERTY_TS_ID: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_EVENT_KEY {
    pub Key: u64,
}
impl EVENT_EXTENDED_ITEM_EVENT_KEY {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_EVENT_KEY").field("Key", &self.Key).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_EVENT_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_EVENT_KEY {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_EVENT_KEY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_INSTANCE {
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows::core::GUID,
}
impl EVENT_EXTENDED_ITEM_INSTANCE {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_INSTANCE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_INSTANCE").field("InstanceId", &self.InstanceId).field("ParentInstanceId", &self.ParentInstanceId).field("ParentGuid", &self.ParentGuid).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceId == other.InstanceId && self.ParentInstanceId == other.ParentInstanceId && self.ParentGuid == other.ParentGuid
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_INSTANCE {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_INSTANCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_PEBS_INDEX {
    pub PebsIndex: u64,
}
impl EVENT_EXTENDED_ITEM_PEBS_INDEX {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_PEBS_INDEX").field("PebsIndex", &self.PebsIndex).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    fn eq(&self, other: &Self) -> bool {
        self.PebsIndex == other.PebsIndex
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_PEBS_INDEX {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_PEBS_INDEX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    pub Counter: [u64; 1],
}
impl EVENT_EXTENDED_ITEM_PMC_COUNTERS {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_PMC_COUNTERS").field("Counter", &self.Counter).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.Counter == other.Counter
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_PMC_COUNTERS {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    pub ProcessStartKey: u64,
}
impl EVENT_EXTENDED_ITEM_PROCESS_START_KEY {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_PROCESS_START_KEY").field("ProcessStartKey", &self.ProcessStartKey).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessStartKey == other.ProcessStartKey
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    pub RelatedActivityId: ::windows::core::GUID,
}
impl EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID").field("RelatedActivityId", &self.RelatedActivityId).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    fn eq(&self, other: &Self) -> bool {
        self.RelatedActivityId == other.RelatedActivityId
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY32 {
    pub MatchId: u64,
    pub StackKey: u32,
    pub Padding: u32,
}
impl EVENT_EXTENDED_ITEM_STACK_KEY32 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_KEY32").field("MatchId", &self.MatchId).field("StackKey", &self.StackKey).field("Padding", &self.Padding).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.StackKey == other.StackKey && self.Padding == other.Padding
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_KEY32 {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_KEY32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY64 {
    pub MatchId: u64,
    pub StackKey: u64,
}
impl EVENT_EXTENDED_ITEM_STACK_KEY64 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_KEY64").field("MatchId", &self.MatchId).field("StackKey", &self.StackKey).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.StackKey == other.StackKey
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_KEY64 {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_KEY64 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    pub MatchId: u64,
    pub Address: [u32; 1],
}
impl EVENT_EXTENDED_ITEM_STACK_TRACE32 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_TRACE32").field("MatchId", &self.MatchId).field("Address", &self.Address).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_TRACE32 {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    pub MatchId: u64,
    pub Address: [u64; 1],
}
impl EVENT_EXTENDED_ITEM_STACK_TRACE64 {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_STACK_TRACE64").field("MatchId", &self.MatchId).field("Address", &self.Address).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn eq(&self, other: &Self) -> bool {
        self.MatchId == other.MatchId && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_STACK_TRACE64 {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_EXTENDED_ITEM_TS_ID {
    pub SessionId: u32,
}
impl EVENT_EXTENDED_ITEM_TS_ID {}
impl ::core::default::Default for EVENT_EXTENDED_ITEM_TS_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_EXTENDED_ITEM_TS_ID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_EXTENDED_ITEM_TS_ID").field("SessionId", &self.SessionId).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_EXTENDED_ITEM_TS_ID {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId
    }
}
impl ::core::cmp::Eq for EVENT_EXTENDED_ITEM_TS_ID {}
unsafe impl ::windows::core::Abi for EVENT_EXTENDED_ITEM_TS_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_FIELD_TYPE(pub i32);
pub const EventKeywordInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(0i32);
pub const EventLevelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(1i32);
pub const EventChannelInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(2i32);
pub const EventTaskInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(3i32);
pub const EventOpcodeInformation: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(4i32);
pub const EventInformationMax: EVENT_FIELD_TYPE = EVENT_FIELD_TYPE(5i32);
impl ::core::convert::From<i32> for EVENT_FIELD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVENT_FIELD_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
impl EVENT_FILTER_DESCRIPTOR {}
impl ::core::default::Default for EVENT_FILTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_FILTER_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_DESCRIPTOR").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_FILTER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr && self.Size == other.Size && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for EVENT_FILTER_DESCRIPTOR {}
unsafe impl ::windows::core::Abi for EVENT_FILTER_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_EVENT_ID {
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub Count: u16,
    pub Events: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_FILTER_EVENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_FILTER_EVENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_FILTER_EVENT_ID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_EVENT_ID").field("FilterIn", &self.FilterIn).field("Reserved", &self.Reserved).field("Count", &self.Count).field("Events", &self.Events).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_FILTER_EVENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.FilterIn == other.FilterIn && self.Reserved == other.Reserved && self.Count == other.Count && self.Events == other.Events
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_FILTER_EVENT_ID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_FILTER_EVENT_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_EVENT_NAME {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
    pub NameCount: u16,
    pub Names: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_FILTER_EVENT_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_FILTER_EVENT_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_FILTER_EVENT_NAME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_EVENT_NAME").field("MatchAnyKeyword", &self.MatchAnyKeyword).field("MatchAllKeyword", &self.MatchAllKeyword).field("Level", &self.Level).field("FilterIn", &self.FilterIn).field("NameCount", &self.NameCount).field("Names", &self.Names).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_FILTER_EVENT_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.MatchAnyKeyword == other.MatchAnyKeyword && self.MatchAllKeyword == other.MatchAllKeyword && self.Level == other.Level && self.FilterIn == other.FilterIn && self.NameCount == other.NameCount && self.Names == other.Names
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_FILTER_EVENT_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_FILTER_EVENT_NAME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_FILTER_HEADER {
    pub Id: u16,
    pub Version: u8,
    pub Reserved: [u8; 5],
    pub InstanceId: u64,
    pub Size: u32,
    pub NextOffset: u32,
}
impl EVENT_FILTER_HEADER {}
impl ::core::default::Default for EVENT_FILTER_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_FILTER_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_HEADER").field("Id", &self.Id).field("Version", &self.Version).field("Reserved", &self.Reserved).field("InstanceId", &self.InstanceId).field("Size", &self.Size).field("NextOffset", &self.NextOffset).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_FILTER_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Version == other.Version && self.Reserved == other.Reserved && self.InstanceId == other.InstanceId && self.Size == other.Size && self.NextOffset == other.NextOffset
    }
}
impl ::core::cmp::Eq for EVENT_FILTER_HEADER {}
unsafe impl ::windows::core::Abi for EVENT_FILTER_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_FILTER_LEVEL_KW {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_FILTER_LEVEL_KW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_FILTER_LEVEL_KW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_FILTER_LEVEL_KW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_FILTER_LEVEL_KW").field("MatchAnyKeyword", &self.MatchAnyKeyword).field("MatchAllKeyword", &self.MatchAllKeyword).field("Level", &self.Level).field("FilterIn", &self.FilterIn).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_FILTER_LEVEL_KW {
    fn eq(&self, other: &Self) -> bool {
        self.MatchAnyKeyword == other.MatchAnyKeyword && self.MatchAllKeyword == other.MatchAllKeyword && self.Level == other.Level && self.FilterIn == other.FilterIn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_FILTER_LEVEL_KW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_FILTER_LEVEL_KW {
    type Abi = Self;
}
pub const EVENT_FILTER_TYPE_CONTAINER: u32 = 2147516416u32;
pub const EVENT_FILTER_TYPE_EVENT_ID: u32 = 2147484160u32;
pub const EVENT_FILTER_TYPE_EVENT_NAME: u32 = 2147484672u32;
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: u32 = 2147483656u32;
pub const EVENT_FILTER_TYPE_NONE: u32 = 0u32;
pub const EVENT_FILTER_TYPE_PACKAGE_APP_ID: u32 = 2147483680u32;
pub const EVENT_FILTER_TYPE_PACKAGE_ID: u32 = 2147483664u32;
pub const EVENT_FILTER_TYPE_PAYLOAD: u32 = 2147483904u32;
pub const EVENT_FILTER_TYPE_PID: u32 = 2147483652u32;
pub const EVENT_FILTER_TYPE_SCHEMATIZED: u32 = 2147483648u32;
pub const EVENT_FILTER_TYPE_STACKWALK: u32 = 2147487744u32;
pub const EVENT_FILTER_TYPE_STACKWALK_LEVEL_KW: u32 = 2147500032u32;
pub const EVENT_FILTER_TYPE_STACKWALK_NAME: u32 = 2147491840u32;
pub const EVENT_FILTER_TYPE_SYSTEM_FLAGS: u32 = 2147483649u32;
pub const EVENT_FILTER_TYPE_TRACEHANDLE: u32 = 2147483650u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_HEADER {
    pub Size: u16,
    pub HeaderType: u16,
    pub Flags: u16,
    pub EventProperty: u16,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub ProviderId: ::windows::core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub Anonymous: EVENT_HEADER_0,
    pub ActivityId: ::windows::core::GUID,
}
impl EVENT_HEADER {}
impl ::core::default::Default for EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_HEADER {}
unsafe impl ::windows::core::Abi for EVENT_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_HEADER_0 {
    pub Anonymous: EVENT_HEADER_0_0,
    pub ProcessorTime: u64,
}
impl EVENT_HEADER_0 {}
impl ::core::default::Default for EVENT_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_0 {}
unsafe impl ::windows::core::Abi for EVENT_HEADER_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_HEADER_0_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl EVENT_HEADER_0_0 {}
impl ::core::default::Default for EVENT_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.KernelTime == other.KernelTime && self.UserTime == other.UserTime
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_0_0 {}
unsafe impl ::windows::core::Abi for EVENT_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: u16,
    pub ExtType: u16,
    pub Anonymous: EVENT_HEADER_EXTENDED_DATA_ITEM_0,
    pub DataSize: u16,
    pub DataPtr: u64,
}
impl EVENT_HEADER_EXTENDED_DATA_ITEM {}
impl ::core::default::Default for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_HEADER_EXTENDED_DATA_ITEM").field("Reserved1", &self.Reserved1).field("ExtType", &self.ExtType).field("Anonymous", &self.Anonymous).field("DataSize", &self.DataSize).field("DataPtr", &self.DataPtr).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_HEADER_EXTENDED_DATA_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.ExtType == other.ExtType && self.Anonymous == other.Anonymous && self.DataSize == other.DataSize && self.DataPtr == other.DataPtr
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_EXTENDED_DATA_ITEM {}
unsafe impl ::windows::core::Abi for EVENT_HEADER_EXTENDED_DATA_ITEM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub _bitfield: u16,
}
impl EVENT_HEADER_EXTENDED_DATA_ITEM_0 {}
impl ::core::default::Default for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {}
unsafe impl ::windows::core::Abi for EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    type Abi = Self;
}
pub const EVENT_HEADER_EXT_TYPE_CONTAINER_ID: u32 = 16u32;
pub const EVENT_HEADER_EXT_TYPE_CONTROL_GUID: u32 = 14u32;
pub const EVENT_HEADER_EXT_TYPE_EVENT_KEY: u32 = 10u32;
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: u32 = 11u32;
pub const EVENT_HEADER_EXT_TYPE_INSTANCE_INFO: u32 = 4u32;
pub const EVENT_HEADER_EXT_TYPE_MAX: u32 = 19u32;
pub const EVENT_HEADER_EXT_TYPE_PEBS_INDEX: u32 = 7u32;
pub const EVENT_HEADER_EXT_TYPE_PMC_COUNTERS: u32 = 8u32;
pub const EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY: u32 = 13u32;
pub const EVENT_HEADER_EXT_TYPE_PROV_TRAITS: u32 = 12u32;
pub const EVENT_HEADER_EXT_TYPE_PSM_KEY: u32 = 9u32;
pub const EVENT_HEADER_EXT_TYPE_QPC_DELTA: u32 = 15u32;
pub const EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID: u32 = 1u32;
pub const EVENT_HEADER_EXT_TYPE_SID: u32 = 2u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY32: u32 = 17u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY64: u32 = 18u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE32: u32 = 5u32;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE64: u32 = 6u32;
pub const EVENT_HEADER_EXT_TYPE_TS_ID: u32 = 3u32;
pub const EVENT_HEADER_FLAG_32_BIT_HEADER: u32 = 32u32;
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: u32 = 64u32;
pub const EVENT_HEADER_FLAG_CLASSIC_HEADER: u32 = 256u32;
pub const EVENT_HEADER_FLAG_DECODE_GUID: u32 = 128u32;
pub const EVENT_HEADER_FLAG_EXTENDED_INFO: u32 = 1u32;
pub const EVENT_HEADER_FLAG_NO_CPUTIME: u32 = 16u32;
pub const EVENT_HEADER_FLAG_PRIVATE_SESSION: u32 = 2u32;
pub const EVENT_HEADER_FLAG_PROCESSOR_INDEX: u32 = 512u32;
pub const EVENT_HEADER_FLAG_STRING_ONLY: u32 = 4u32;
pub const EVENT_HEADER_FLAG_TRACE_MESSAGE: u32 = 8u32;
pub const EVENT_HEADER_PROPERTY_FORWARDED_XML: u32 = 2u32;
pub const EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG: u32 = 4u32;
pub const EVENT_HEADER_PROPERTY_RELOGGABLE: u32 = 8u32;
pub const EVENT_HEADER_PROPERTY_XML: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_INFO_CLASS(pub i32);
pub const EventProviderBinaryTrackInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(0i32);
pub const EventProviderSetReserved1: EVENT_INFO_CLASS = EVENT_INFO_CLASS(1i32);
pub const EventProviderSetTraits: EVENT_INFO_CLASS = EVENT_INFO_CLASS(2i32);
pub const EventProviderUseDescriptorType: EVENT_INFO_CLASS = EVENT_INFO_CLASS(3i32);
pub const MaxEventInfo: EVENT_INFO_CLASS = EVENT_INFO_CLASS(4i32);
impl ::core::convert::From<i32> for EVENT_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVENT_INFO_CLASS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_INSTANCE_HEADER {
    pub Size: u16,
    pub Anonymous1: EVENT_INSTANCE_HEADER_0,
    pub Anonymous2: EVENT_INSTANCE_HEADER_1,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub RegHandle: u64,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub Anonymous3: EVENT_INSTANCE_HEADER_2,
    pub ParentRegHandle: u64,
}
impl EVENT_INSTANCE_HEADER {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_INSTANCE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_INSTANCE_HEADER_0_0,
}
impl EVENT_INSTANCE_HEADER_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_0 {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_INSTANCE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl EVENT_INSTANCE_HEADER_0_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("HeaderType", &self.HeaderType).field("MarkerFlags", &self.MarkerFlags).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderType == other.HeaderType && self.MarkerFlags == other.MarkerFlags
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_0_0 {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_INSTANCE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_INSTANCE_HEADER_1_0,
}
impl EVENT_INSTANCE_HEADER_1 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_1 {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_INSTANCE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl EVENT_INSTANCE_HEADER_1_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Class_e__Struct").field("Type", &self.Type).field("Level", &self.Level).field("Version", &self.Version).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Level == other.Level && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_1_0 {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_1_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_INSTANCE_HEADER_2 {
    pub Anonymous1: EVENT_INSTANCE_HEADER_2_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_INSTANCE_HEADER_2_1,
}
impl EVENT_INSTANCE_HEADER_2 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_2 {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_INSTANCE_HEADER_2_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl EVENT_INSTANCE_HEADER_2_0 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_2_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.KernelTime == other.KernelTime && self.UserTime == other.UserTime
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_2_0 {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_2_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_INSTANCE_HEADER_2_1 {
    pub EventId: u32,
    pub Flags: u32,
}
impl EVENT_INSTANCE_HEADER_2_1 {}
impl ::core::default::Default for EVENT_INSTANCE_HEADER_2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_INSTANCE_HEADER_2_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("EventId", &self.EventId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_INSTANCE_HEADER_2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.EventId == other.EventId && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for EVENT_INSTANCE_HEADER_2_1 {}
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_HEADER_2_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_INSTANCE_INFO {
    pub RegHandle: super::super::super::Foundation::HANDLE,
    pub InstanceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_INSTANCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_INSTANCE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EVENT_INSTANCE_INFO").field("RegHandle", &self.RegHandle).field("InstanceId", &self.InstanceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RegHandle == other.RegHandle && self.InstanceId == other.InstanceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_INSTANCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_INSTANCE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_MAP_ENTRY {
    pub OutputOffset: u32,
    pub Anonymous: EVENT_MAP_ENTRY_0,
}
impl EVENT_MAP_ENTRY {}
impl ::core::default::Default for EVENT_MAP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_MAP_ENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_MAP_ENTRY {}
unsafe impl ::windows::core::Abi for EVENT_MAP_ENTRY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_MAP_ENTRY_0 {
    pub Value: u32,
    pub InputOffset: u32,
}
impl EVENT_MAP_ENTRY_0 {}
impl ::core::default::Default for EVENT_MAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_MAP_ENTRY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_MAP_ENTRY_0 {}
unsafe impl ::windows::core::Abi for EVENT_MAP_ENTRY_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_MAP_INFO {
    pub NameOffset: u32,
    pub Flag: MAP_FLAGS,
    pub EntryCount: u32,
    pub Anonymous: EVENT_MAP_INFO_0,
    pub MapEntryArray: [EVENT_MAP_ENTRY; 1],
}
impl EVENT_MAP_INFO {}
impl ::core::default::Default for EVENT_MAP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_MAP_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_MAP_INFO {}
unsafe impl ::windows::core::Abi for EVENT_MAP_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_MAP_INFO_0 {
    pub MapEntryValueType: MAP_VALUETYPE,
    pub FormatStringOffset: u32,
}
impl EVENT_MAP_INFO_0 {}
impl ::core::default::Default for EVENT_MAP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_MAP_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_MAP_INFO_0 {}
unsafe impl ::windows::core::Abi for EVENT_MAP_INFO_0 {
    type Abi = Self;
}
pub const EVENT_MAX_LEVEL: u32 = 255u32;
pub const EVENT_MIN_LEVEL: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: u32,
    pub Anonymous1: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
impl EVENT_PROPERTY_INFO {}
impl ::core::default::Default for EVENT_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0_1,
    pub structType: EVENT_PROPERTY_INFO_0_2,
    pub customSchemaType: EVENT_PROPERTY_INFO_0_0,
}
impl EVENT_PROPERTY_INFO_0 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_PROPERTY_INFO_0_0 {
    pub InType: u16,
    pub OutType: u16,
    pub CustomSchemaOffset: u32,
}
impl EVENT_PROPERTY_INFO_0_0 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_customSchemaType").field("InType", &self.InType).field("OutType", &self.OutType).field("CustomSchemaOffset", &self.CustomSchemaOffset).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.InType == other.InType && self.OutType == other.OutType && self.CustomSchemaOffset == other.CustomSchemaOffset
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0_0 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_PROPERTY_INFO_0_1 {
    pub InType: u16,
    pub OutType: u16,
    pub MapNameOffset: u32,
}
impl EVENT_PROPERTY_INFO_0_1 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_nonStructType").field("InType", &self.InType).field("OutType", &self.OutType).field("MapNameOffset", &self.MapNameOffset).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.InType == other.InType && self.OutType == other.OutType && self.MapNameOffset == other.MapNameOffset
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0_1 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_PROPERTY_INFO_0_2 {
    pub StructStartIndex: u16,
    pub NumOfStructMembers: u16,
    pub padding: u32,
}
impl EVENT_PROPERTY_INFO_0_2 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_0_2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_structType").field("StructStartIndex", &self.StructStartIndex).field("NumOfStructMembers", &self.NumOfStructMembers).field("padding", &self.padding).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.StructStartIndex == other.StructStartIndex && self.NumOfStructMembers == other.NumOfStructMembers && self.padding == other.padding
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_0_2 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_0_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: u16,
    pub countPropertyIndex: u16,
}
impl EVENT_PROPERTY_INFO_1 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_1 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: u16,
    pub lengthPropertyIndex: u16,
}
impl EVENT_PROPERTY_INFO_2 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_2 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
impl EVENT_PROPERTY_INFO_3 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_3 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_3 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
impl EVENT_PROPERTY_INFO_3_0 {}
impl ::core::default::Default for EVENT_PROPERTY_INFO_3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_PROPERTY_INFO_3_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_PROPERTY_INFO_3_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for EVENT_PROPERTY_INFO_3_0 {}
unsafe impl ::windows::core::Abi for EVENT_PROPERTY_INFO_3_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: *mut EVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut ::core::ffi::c_void,
    pub UserContext: *mut ::core::ffi::c_void,
}
impl EVENT_RECORD {}
impl ::core::default::Default for EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_RECORD {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_RECORD {}
unsafe impl ::windows::core::Abi for EVENT_RECORD {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_TRACE {
    pub Header: EVENT_TRACE_HEADER,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: ::windows::core::GUID,
    pub MofData: *mut ::core::ffi::c_void,
    pub MofLength: u32,
    pub Anonymous: EVENT_TRACE_0,
}
impl EVENT_TRACE {}
impl ::core::default::Default for EVENT_TRACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_TRACE {}
unsafe impl ::windows::core::Abi for EVENT_TRACE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_TRACE_0 {
    pub ClientContext: u32,
    pub BufferContext: ETW_BUFFER_CONTEXT,
}
impl EVENT_TRACE_0 {}
impl ::core::default::Default for EVENT_TRACE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_0 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_0 {
    type Abi = Self;
}
pub const EVENT_TRACE_ADDTO_TRIAGE_DUMP: u32 = 2147483648u32;
pub const EVENT_TRACE_ADD_HEADER_MODE: u32 = 4096u32;
pub const EVENT_TRACE_BUFFERING_MODE: u32 = 1024u32;
pub const EVENT_TRACE_COMPRESSED_MODE: u32 = 67108864u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_TRACE_CONTROL(pub u32);
pub const EVENT_TRACE_CONTROL_FLUSH: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(3u32);
pub const EVENT_TRACE_CONTROL_QUERY: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(0u32);
pub const EVENT_TRACE_CONTROL_STOP: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(1u32);
pub const EVENT_TRACE_CONTROL_UPDATE: EVENT_TRACE_CONTROL = EVENT_TRACE_CONTROL(2u32);
impl ::core::convert::From<u32> for EVENT_TRACE_CONTROL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_CONTROL {
    type Abi = Self;
}
impl ::core::ops::BitOr for EVENT_TRACE_CONTROL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for EVENT_TRACE_CONTROL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for EVENT_TRACE_CONTROL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for EVENT_TRACE_CONTROL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for EVENT_TRACE_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const EVENT_TRACE_CONTROL_CONVERT_TO_REALTIME: u32 = 5u32;
pub const EVENT_TRACE_CONTROL_INCREMENT_FILE: u32 = 4u32;
pub const EVENT_TRACE_DELAY_OPEN_FILE_MODE: u32 = 512u32;
pub const EVENT_TRACE_FILE_MODE_APPEND: u32 = 4u32;
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: u32 = 2u32;
pub const EVENT_TRACE_FILE_MODE_NEWFILE: u32 = 8u32;
pub const EVENT_TRACE_FILE_MODE_NONE: u32 = 0u32;
pub const EVENT_TRACE_FILE_MODE_PREALLOCATE: u32 = 32u32;
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EVENT_TRACE_FLAG(pub u32);
pub const EVENT_TRACE_FLAG_ALPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1048576u32);
pub const EVENT_TRACE_FLAG_CSWITCH: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16u32);
pub const EVENT_TRACE_FLAG_DBGPRINT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(262144u32);
pub const EVENT_TRACE_FLAG_DISK_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(512u32);
pub const EVENT_TRACE_FLAG_DISK_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(256u32);
pub const EVENT_TRACE_FLAG_DISK_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1024u32);
pub const EVENT_TRACE_FLAG_DISPATCHER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2048u32);
pub const EVENT_TRACE_FLAG_DPC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32u32);
pub const EVENT_TRACE_FLAG_DRIVER: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8388608u32);
pub const EVENT_TRACE_FLAG_FILE_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(33554432u32);
pub const EVENT_TRACE_FLAG_FILE_IO_INIT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(67108864u32);
pub const EVENT_TRACE_FLAG_IMAGE_LOAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4u32);
pub const EVENT_TRACE_FLAG_INTERRUPT: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(64u32);
pub const EVENT_TRACE_FLAG_JOB: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(524288u32);
pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8192u32);
pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(4096u32);
pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(65536u32);
pub const EVENT_TRACE_FLAG_NO_SYSCONFIG: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(268435456u32);
pub const EVENT_TRACE_FLAG_PROCESS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(1u32);
pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(8u32);
pub const EVENT_TRACE_FLAG_PROFILE: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16777216u32);
pub const EVENT_TRACE_FLAG_REGISTRY: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(131072u32);
pub const EVENT_TRACE_FLAG_SPLIT_IO: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2097152u32);
pub const EVENT_TRACE_FLAG_SYSTEMCALL: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(128u32);
pub const EVENT_TRACE_FLAG_THREAD: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(2u32);
pub const EVENT_TRACE_FLAG_VAMAP: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(32768u32);
pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: EVENT_TRACE_FLAG = EVENT_TRACE_FLAG(16384u32);
impl ::core::convert::From<u32> for EVENT_TRACE_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EVENT_TRACE_FLAG {
    type Abi = Self;
}
impl ::core::ops::BitOr for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for EVENT_TRACE_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for EVENT_TRACE_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for EVENT_TRACE_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for EVENT_TRACE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const EVENT_TRACE_FLAG_DEBUG_EVENTS: u32 = 4194304u32;
pub const EVENT_TRACE_FLAG_ENABLE_RESERVE: u32 = 536870912u32;
pub const EVENT_TRACE_FLAG_EXTENSION: u32 = 2147483648u32;
pub const EVENT_TRACE_FLAG_FORWARD_WMI: u32 = 1073741824u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_TRACE_HEADER {
    pub Size: u16,
    pub Anonymous1: EVENT_TRACE_HEADER_0,
    pub Anonymous2: EVENT_TRACE_HEADER_1,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub Anonymous3: EVENT_TRACE_HEADER_2,
    pub Anonymous4: EVENT_TRACE_HEADER_3,
}
impl EVENT_TRACE_HEADER {}
impl ::core::default::Default for EVENT_TRACE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_TRACE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0_0,
}
impl EVENT_TRACE_HEADER_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_0 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_TRACE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
impl EVENT_TRACE_HEADER_0_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("HeaderType", &self.HeaderType).field("MarkerFlags", &self.MarkerFlags).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderType == other.HeaderType && self.MarkerFlags == other.MarkerFlags
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_0_0 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_TRACE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_TRACE_HEADER_1_0,
}
impl EVENT_TRACE_HEADER_1 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_1 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_TRACE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
impl EVENT_TRACE_HEADER_1_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Class_e__Struct").field("Type", &self.Type).field("Level", &self.Level).field("Version", &self.Version).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Level == other.Level && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_1_0 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_1_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_TRACE_HEADER_2 {
    pub Guid: ::windows::core::GUID,
    pub GuidPtr: u64,
}
impl EVENT_TRACE_HEADER_2 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_2 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union EVENT_TRACE_HEADER_3 {
    pub Anonymous1: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
impl EVENT_TRACE_HEADER_3 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_3 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_3 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
impl EVENT_TRACE_HEADER_3_0 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_3_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_3_0 {
    fn eq(&self, other: &Self) -> bool {
        self.KernelTime == other.KernelTime && self.UserTime == other.UserTime
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_3_0 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_3_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: u32,
    pub Flags: u32,
}
impl EVENT_TRACE_HEADER_3_1 {}
impl ::core::default::Default for EVENT_TRACE_HEADER_3_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EVENT_TRACE_HEADER_3_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("ClientContext", &self.ClientContext).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for EVENT_TRACE_HEADER_3_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ClientContext == other.ClientContext && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for EVENT_TRACE_HEADER_3_1 {}
unsafe impl ::windows::core::Abi for EVENT_TRACE_HEADER_3_1 {
    type Abi = Self;
}
pub const EVENT_TRACE_INDEPENDENT_SESSION_MODE: u32 = 134217728u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct EVENT_TRACE_LOGFILEA {
    pub LogFileName: super::super::super::Foundation::PSTR,
    pub LoggerName: super::super::super::Foundation::PSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEA_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: ::core::option::Option<PEVENT_TRACE_BUFFER_CALLBACKA>,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEA_1,
    pub IsKernelTrace: u32,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEA_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEA_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEA_1 {
    pub EventCallback: ::windows::core::RawPtr,
    pub EventRecordCallback: ::windows::core::RawPtr,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEA_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEA_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: super::super::super::Foundation::PWSTR,
    pub LoggerName: super::super::super::Foundation::PWSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous1: EVENT_TRACE_LOGFILEW_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: ::core::option::Option<PEVENT_TRACE_BUFFER_CALLBACKW>,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEW_1,
    pub IsKernelTrace: u32,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEW_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for EVENT_TRACE_LOGFILEW_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: ::windows::core::RawPtr,
    pub EventRecordCallback: ::windows::core::RawPtr,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl EVENT_TRACE_LOGFILEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for EVENT_TRACE_LOGFILEW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for EVENT_TRACE_LOGFILEW_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for EVENT_TRACE_LOGFILEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for EVENT_TRACE_LOGFILEW_1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const EVENT_TRACE_MODE_RESERVED: u32 = 1048576u32;
pub const EVENT_TRACE_NONSTOPPABLE_MODE: u32 = 64u32;
pub const EVENT_TRACE_NO_PER_PROCESSOR_BUFFERING: u32 = 268435456u32;
pub const EVENT_TRACE_PERSIST_ON_HYBRID_SHUTDOWN: u32 = 8388608u32;
pub const EVENT_TRACE_PRIVATE_IN_PROC: u32 = 131072u32;
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: u32 = 2048u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES {
    pub Wnode: WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: EVENT_TRACE_FLAG,
    pub Anonymous: EVENT_TRACE_PROPERTIES_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::super::super::Foundation::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES_V2 {
    pub Wnode: WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: EVENT_TRACE_FLAG,
    pub Anonymous1: EVENT_TRACE_PROPERTIES_V2_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: super::super::super::Foundation::HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
    pub Anonymous2: EVENT_TRACE_PROPERTIES_V2_1,
    pub FilterDescCount: u32,
    pub FilterDesc: *mut EVENT_FILTER_DESCRIPTOR,
    pub Anonymous3: EVENT_TRACE_PROPERTIES_V2_2,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_1 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_1_0,
    pub V2Control: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES_V2_1_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_V2_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_1_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EVENT_TRACE_PROPERTIES_V2_2 {
    pub Anonymous: EVENT_TRACE_PROPERTIES_V2_2_0,
    pub V2Options: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_V2_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EVENT_TRACE_PROPERTIES_V2_2_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EVENT_TRACE_PROPERTIES_V2_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENT_TRACE_PROPERTIES_V2_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENT_TRACE_PROPERTIES_V2_2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EVENT_TRACE_PROPERTIES_V2_2_0 {
    type Abi = Self;
}
pub const EVENT_TRACE_REAL_TIME_MODE: u32 = 256u32;
pub const EVENT_TRACE_RELOG_MODE: u32 = 65536u32;
pub const EVENT_TRACE_SECURE_MODE: u32 = 128u32;
pub const EVENT_TRACE_STOP_ON_HYBRID_SHUTDOWN: u32 = 4194304u32;
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: u32 = 33554432u32;
pub const EVENT_TRACE_TYPE_ACCEPT: u32 = 15u32;
pub const EVENT_TRACE_TYPE_ACKDUP: u32 = 22u32;
pub const EVENT_TRACE_TYPE_ACKFULL: u32 = 20u32;
pub const EVENT_TRACE_TYPE_ACKPART: u32 = 21u32;
pub const EVENT_TRACE_TYPE_CHECKPOINT: u32 = 8u32;
pub const EVENT_TRACE_TYPE_CONFIG: u32 = 11u32;
pub const EVENT_TRACE_TYPE_CONFIG_BOOT: u32 = 37u32;
pub const EVENT_TRACE_TYPE_CONFIG_CI_INFO: u32 = 29u32;
pub const EVENT_TRACE_TYPE_CONFIG_CPU: u32 = 10u32;
pub const EVENT_TRACE_TYPE_CONFIG_DEFRAG: u32 = 31u32;
pub const EVENT_TRACE_TYPE_CONFIG_DEVICEFAMILY: u32 = 33u32;
pub const EVENT_TRACE_TYPE_CONFIG_DPI: u32 = 28u32;
pub const EVENT_TRACE_TYPE_CONFIG_FLIGHTID: u32 = 34u32;
pub const EVENT_TRACE_TYPE_CONFIG_IDECHANNEL: u32 = 23u32;
pub const EVENT_TRACE_TYPE_CONFIG_IRQ: u32 = 21u32;
pub const EVENT_TRACE_TYPE_CONFIG_LOGICALDISK: u32 = 12u32;
pub const EVENT_TRACE_TYPE_CONFIG_MACHINEID: u32 = 30u32;
pub const EVENT_TRACE_TYPE_CONFIG_MOBILEPLATFORM: u32 = 32u32;
pub const EVENT_TRACE_TYPE_CONFIG_NETINFO: u32 = 17u32;
pub const EVENT_TRACE_TYPE_CONFIG_NIC: u32 = 13u32;
pub const EVENT_TRACE_TYPE_CONFIG_NUMANODE: u32 = 24u32;
pub const EVENT_TRACE_TYPE_CONFIG_OPTICALMEDIA: u32 = 18u32;
pub const EVENT_TRACE_TYPE_CONFIG_PHYSICALDISK: u32 = 11u32;
pub const EVENT_TRACE_TYPE_CONFIG_PLATFORM: u32 = 25u32;
pub const EVENT_TRACE_TYPE_CONFIG_PNP: u32 = 22u32;
pub const EVENT_TRACE_TYPE_CONFIG_POWER: u32 = 16u32;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSOR: u32 = 35u32;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORGROUP: u32 = 26u32;
pub const EVENT_TRACE_TYPE_CONFIG_PROCESSORNUMBER: u32 = 27u32;
pub const EVENT_TRACE_TYPE_CONFIG_SERVICES: u32 = 15u32;
pub const EVENT_TRACE_TYPE_CONFIG_VIDEO: u32 = 14u32;
pub const EVENT_TRACE_TYPE_CONFIG_VIRTUALIZATION: u32 = 36u32;
pub const EVENT_TRACE_TYPE_CONNECT: u32 = 12u32;
pub const EVENT_TRACE_TYPE_CONNFAIL: u32 = 17u32;
pub const EVENT_TRACE_TYPE_COPY_ARP: u32 = 19u32;
pub const EVENT_TRACE_TYPE_COPY_TCP: u32 = 18u32;
pub const EVENT_TRACE_TYPE_DBGID_RSDS: u32 = 64u32;
pub const EVENT_TRACE_TYPE_DC_END: u32 = 4u32;
pub const EVENT_TRACE_TYPE_DC_START: u32 = 3u32;
pub const EVENT_TRACE_TYPE_DEQUEUE: u32 = 7u32;
pub const EVENT_TRACE_TYPE_DISCONNECT: u32 = 13u32;
pub const EVENT_TRACE_TYPE_END: u32 = 2u32;
pub const EVENT_TRACE_TYPE_EXTENSION: u32 = 5u32;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_COMPLETION: u32 = 99u32;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_FAILURE: u32 = 101u32;
pub const EVENT_TRACE_TYPE_FLT_POSTOP_INIT: u32 = 97u32;
pub const EVENT_TRACE_TYPE_FLT_PREOP_COMPLETION: u32 = 98u32;
pub const EVENT_TRACE_TYPE_FLT_PREOP_FAILURE: u32 = 100u32;
pub const EVENT_TRACE_TYPE_FLT_PREOP_INIT: u32 = 96u32;
pub const EVENT_TRACE_TYPE_GUIDMAP: u32 = 10u32;
pub const EVENT_TRACE_TYPE_INFO: u32 = 0u32;
pub const EVENT_TRACE_TYPE_IO_FLUSH: u32 = 14u32;
pub const EVENT_TRACE_TYPE_IO_FLUSH_INIT: u32 = 15u32;
pub const EVENT_TRACE_TYPE_IO_READ: u32 = 10u32;
pub const EVENT_TRACE_TYPE_IO_READ_INIT: u32 = 12u32;
pub const EVENT_TRACE_TYPE_IO_REDIRECTED_INIT: u32 = 16u32;
pub const EVENT_TRACE_TYPE_IO_WRITE: u32 = 11u32;
pub const EVENT_TRACE_TYPE_IO_WRITE_INIT: u32 = 13u32;
pub const EVENT_TRACE_TYPE_LOAD: u32 = 10u32;
pub const EVENT_TRACE_TYPE_MM_AV: u32 = 15u32;
pub const EVENT_TRACE_TYPE_MM_COW: u32 = 12u32;
pub const EVENT_TRACE_TYPE_MM_DZF: u32 = 11u32;
pub const EVENT_TRACE_TYPE_MM_GPF: u32 = 13u32;
pub const EVENT_TRACE_TYPE_MM_HPF: u32 = 14u32;
pub const EVENT_TRACE_TYPE_MM_TF: u32 = 10u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH: u32 = 57u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_FLUSH_INIT: u32 = 60u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ: u32 = 55u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_READ_INIT: u32 = 58u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE: u32 = 56u32;
pub const EVENT_TRACE_TYPE_OPTICAL_IO_WRITE_INIT: u32 = 59u32;
pub const EVENT_TRACE_TYPE_RECEIVE: u32 = 11u32;
pub const EVENT_TRACE_TYPE_RECONNECT: u32 = 16u32;
pub const EVENT_TRACE_TYPE_REGCLOSE: u32 = 27u32;
pub const EVENT_TRACE_TYPE_REGCOMMIT: u32 = 30u32;
pub const EVENT_TRACE_TYPE_REGCREATE: u32 = 10u32;
pub const EVENT_TRACE_TYPE_REGDELETE: u32 = 12u32;
pub const EVENT_TRACE_TYPE_REGDELETEVALUE: u32 = 15u32;
pub const EVENT_TRACE_TYPE_REGENUMERATEKEY: u32 = 17u32;
pub const EVENT_TRACE_TYPE_REGENUMERATEVALUEKEY: u32 = 18u32;
pub const EVENT_TRACE_TYPE_REGFLUSH: u32 = 21u32;
pub const EVENT_TRACE_TYPE_REGKCBCREATE: u32 = 22u32;
pub const EVENT_TRACE_TYPE_REGKCBDELETE: u32 = 23u32;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNBEGIN: u32 = 24u32;
pub const EVENT_TRACE_TYPE_REGKCBRUNDOWNEND: u32 = 25u32;
pub const EVENT_TRACE_TYPE_REGMOUNTHIVE: u32 = 33u32;
pub const EVENT_TRACE_TYPE_REGOPEN: u32 = 11u32;
pub const EVENT_TRACE_TYPE_REGPREPARE: u32 = 31u32;
pub const EVENT_TRACE_TYPE_REGQUERY: u32 = 13u32;
pub const EVENT_TRACE_TYPE_REGQUERYMULTIPLEVALUE: u32 = 19u32;
pub const EVENT_TRACE_TYPE_REGQUERYSECURITY: u32 = 29u32;
pub const EVENT_TRACE_TYPE_REGQUERYVALUE: u32 = 16u32;
pub const EVENT_TRACE_TYPE_REGROLLBACK: u32 = 32u32;
pub const EVENT_TRACE_TYPE_REGSETINFORMATION: u32 = 20u32;
pub const EVENT_TRACE_TYPE_REGSETSECURITY: u32 = 28u32;
pub const EVENT_TRACE_TYPE_REGSETVALUE: u32 = 14u32;
pub const EVENT_TRACE_TYPE_REGVIRTUALIZE: u32 = 26u32;
pub const EVENT_TRACE_TYPE_REPLY: u32 = 6u32;
pub const EVENT_TRACE_TYPE_RESUME: u32 = 7u32;
pub const EVENT_TRACE_TYPE_RETRANSMIT: u32 = 14u32;
pub const EVENT_TRACE_TYPE_SECURITY: u32 = 13u32;
pub const EVENT_TRACE_TYPE_SEND: u32 = 10u32;
pub const EVENT_TRACE_TYPE_SIDINFO: u32 = 12u32;
pub const EVENT_TRACE_TYPE_START: u32 = 1u32;
pub const EVENT_TRACE_TYPE_STOP: u32 = 2u32;
pub const EVENT_TRACE_TYPE_SUSPEND: u32 = 8u32;
pub const EVENT_TRACE_TYPE_TERMINATE: u32 = 11u32;
pub const EVENT_TRACE_TYPE_WINEVT_RECEIVE: u32 = 240u32;
pub const EVENT_TRACE_TYPE_WINEVT_SEND: u32 = 9u32;
pub const EVENT_TRACE_USE_GLOBAL_SEQUENCE: u32 = 16384u32;
pub const EVENT_TRACE_USE_KBYTES_FOR_SIZE: u32 = 8192u32;
pub const EVENT_TRACE_USE_LOCAL_SEQUENCE: u32 = 32768u32;
pub const EVENT_TRACE_USE_NOCPUTIME: u32 = 2u32;
pub const EVENT_TRACE_USE_PAGED_MEMORY: u32 = 16777216u32;
pub const EVENT_TRACE_USE_PROCTIME: u32 = 1u32;
pub const EVENT_WRITE_FLAG_INPRIVATE: u32 = 2u32;
pub const EVENT_WRITE_FLAG_NO_FAULTING: u32 = 1u32;
#[inline]
pub unsafe fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows::core::GUID, tracehandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTrace(enable: u32, enableflag: u32, enablelevel: u32, controlguid: *const ::windows::core::GUID, tracehandle: u64) -> u32;
        }
        ::core::mem::transmute(EnableTrace(::core::mem::transmute(enable), ::core::mem::transmute(enableflag), ::core::mem::transmute(enablelevel), ::core::mem::transmute(controlguid), ::core::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnableTraceEx(providerid: *const ::windows::core::GUID, sourceid: *const ::windows::core::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTraceEx(providerid: *const ::windows::core::GUID, sourceid: *const ::windows::core::GUID, tracehandle: u64, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, enableproperty: u32, enablefilterdesc: *const EVENT_FILTER_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(EnableTraceEx(
            ::core::mem::transmute(providerid),
            ::core::mem::transmute(sourceid),
            ::core::mem::transmute(tracehandle),
            ::core::mem::transmute(isenabled),
            ::core::mem::transmute(level),
            ::core::mem::transmute(matchanykeyword),
            ::core::mem::transmute(matchallkeyword),
            ::core::mem::transmute(enableproperty),
            ::core::mem::transmute(enablefilterdesc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows::core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTraceEx2(tracehandle: u64, providerid: *const ::windows::core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, timeout: u32, enableparameters: *const ENABLE_TRACE_PARAMETERS) -> u32;
        }
        ::core::mem::transmute(EnableTraceEx2(
            ::core::mem::transmute(tracehandle),
            ::core::mem::transmute(providerid),
            ::core::mem::transmute(controlcode),
            ::core::mem::transmute(level),
            ::core::mem::transmute(matchanykeyword),
            ::core::mem::transmute(matchallkeyword),
            ::core::mem::transmute(timeout),
            ::core::mem::transmute(enableparameters),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateTraceGuids(guidpropertiesarray: *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount: u32, guidcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumerateTraceGuids(guidpropertiesarray: *mut *mut TRACE_GUID_PROPERTIES, propertyarraycount: u32, guidcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(EnumerateTraceGuids(::core::mem::transmute(guidpropertiesarray), ::core::mem::transmute(propertyarraycount), ::core::mem::transmute(guidcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumerateTraceGuidsEx(tracequeryinfoclass: TRACE_QUERY_INFO_CLASS, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(EnumerateTraceGuidsEx(::core::mem::transmute(tracequeryinfoclass), ::core::mem::transmute(inbuffer), ::core::mem::transmute(inbuffersize), ::core::mem::transmute(outbuffer), ::core::mem::transmute(outbuffersize), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventAccessControl<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSID>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(guid: *const ::windows::core::GUID, operation: u32, sid: Param2, rights: u32, allowordeny: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventAccessControl(guid: *const ::windows::core::GUID, operation: u32, sid: super::super::super::Foundation::PSID, rights: u32, allowordeny: super::super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(EventAccessControl(::core::mem::transmute(guid), ::core::mem::transmute(operation), sid.into_param().abi(), ::core::mem::transmute(rights), allowordeny.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EventAccessQuery(guid: *const ::windows::core::GUID, buffer: *mut super::super::super::Security::SECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventAccessQuery(guid: *const ::windows::core::GUID, buffer: *mut super::super::super::Security::SECURITY_DESCRIPTOR, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(EventAccessQuery(::core::mem::transmute(guid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EventAccessRemove(guid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventAccessRemove(guid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(EventAccessRemove(::core::mem::transmute(guid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventActivityIdControl(controlcode: u32, activityid: *mut ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(EventActivityIdControl(::core::mem::transmute(controlcode), ::core::mem::transmute(activityid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventEnabled(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR) -> super::super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(EventEnabled(::core::mem::transmute(reghandle), ::core::mem::transmute(eventdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventProviderEnabled(reghandle: u64, level: u8, keyword: u64) -> super::super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(EventProviderEnabled(::core::mem::transmute(reghandle), ::core::mem::transmute(level), ::core::mem::transmute(keyword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EventRegister(providerid: *const ::windows::core::GUID, enablecallback: ::core::option::Option<PENABLECALLBACK>, callbackcontext: *const ::core::ffi::c_void, reghandle: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventRegister(providerid: *const ::windows::core::GUID, enablecallback: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, reghandle: *mut u64) -> u32;
        }
        ::core::mem::transmute(EventRegister(::core::mem::transmute(providerid), ::core::mem::transmute(enablecallback), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(reghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventSetInformation(reghandle: u64, informationclass: EVENT_INFO_CLASS, eventinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
        }
        ::core::mem::transmute(EventSetInformation(::core::mem::transmute(reghandle), ::core::mem::transmute(informationclass), ::core::mem::transmute(eventinformation), ::core::mem::transmute(informationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const EventTraceConfigGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01853a65_418f_4f36_aefc_dc0f1d2fd235);
pub const EventTraceGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68fdd900_4a3e_11d1_84f4_0000f80464e3);
#[inline]
pub unsafe fn EventUnregister(reghandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventUnregister(reghandle: u64) -> u32;
        }
        ::core::mem::transmute(EventUnregister(::core::mem::transmute(reghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWrite(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(EventWrite(::core::mem::transmute(reghandle), ::core::mem::transmute(eventdescriptor), ::core::mem::transmute(userdatacount), ::core::mem::transmute(userdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWriteEx(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(EventWriteEx(
            ::core::mem::transmute(reghandle),
            ::core::mem::transmute(eventdescriptor),
            ::core::mem::transmute(filter),
            ::core::mem::transmute(flags),
            ::core::mem::transmute(activityid),
            ::core::mem::transmute(relatedactivityid),
            ::core::mem::transmute(userdatacount),
            ::core::mem::transmute(userdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EventWriteString<'a, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(reghandle: u64, level: u8, keyword: u64, string: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWriteString(reghandle: u64, level: u8, keyword: u64, string: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(EventWriteString(::core::mem::transmute(reghandle), ::core::mem::transmute(level), ::core::mem::transmute(keyword), string.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EventWriteTransfer(reghandle: u64, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: *const ::windows::core::GUID, relatedactivityid: *const ::windows::core::GUID, userdatacount: u32, userdata: *const EVENT_DATA_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(EventWriteTransfer(::core::mem::transmute(reghandle), ::core::mem::transmute(eventdescriptor), ::core::mem::transmute(activityid), ::core::mem::transmute(relatedactivityid), ::core::mem::transmute(userdatacount), ::core::mem::transmute(userdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushTraceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(FlushTraceA(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushTraceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(FlushTraceW(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTraceEnableFlags(tracehandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTraceEnableFlags(tracehandle: u64) -> u32;
        }
        ::core::mem::transmute(GetTraceEnableFlags(::core::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTraceEnableLevel(tracehandle: u64) -> u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTraceEnableLevel(tracehandle: u64) -> u8;
        }
        ::core::mem::transmute(GetTraceEnableLevel(::core::mem::transmute(tracehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTraceLoggerHandle(buffer: *const ::core::ffi::c_void) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTraceLoggerHandle(buffer: *const ::core::ffi::c_void) -> u64;
        }
        ::core::mem::transmute(GetTraceLoggerHandle(::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITraceEvent(pub ::windows::core::IUnknown);
impl ITraceEvent {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITraceEvent> {
        let mut result__: <ITraceEvent as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITraceEvent>(result__)
    }
    pub unsafe fn GetUserContext(&self, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(usercontext)).ok()
    }
    pub unsafe fn GetEventRecord(&self) -> ::windows::core::Result<*mut EVENT_RECORD> {
        let mut result__: <*mut EVENT_RECORD as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut EVENT_RECORD>(result__)
    }
    pub unsafe fn SetPayload(&self, payload: *const u8, payloadsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(payload), ::core::mem::transmute(payloadsize)).ok()
    }
    pub unsafe fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventdescriptor)).ok()
    }
    pub unsafe fn SetProcessId(&self, processid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(processid)).ok()
    }
    pub unsafe fn SetProcessorIndex(&self, processorindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(processorindex)).ok()
    }
    pub unsafe fn SetThreadId(&self, threadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(threadid)).ok()
    }
    pub unsafe fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(kerneltime), ::core::mem::transmute(usertime)).ok()
    }
    pub unsafe fn SetActivityId(&self, activityid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(activityid)).ok()
    }
    pub unsafe fn SetTimeStamp(&self, timestamp: *const i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(timestamp)).ok()
    }
    pub unsafe fn SetProviderId(&self, providerid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(providerid)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITraceEvent {
    type Vtable = ITraceEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cc97f40_9028_4ff3_9b62_7d1f79ca7bcb);
}
impl ::core::convert::From<ITraceEvent> for ::windows::core::IUnknown {
    fn from(value: ITraceEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITraceEvent> for ::windows::core::IUnknown {
    fn from(value: &ITraceEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventrecord: *mut *mut EVENT_RECORD) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, payload: *const u8, payloadsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, processid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, processorindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, threadid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kerneltime: u32, usertime: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activityid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: *const i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITraceEventCallback(pub ::windows::core::IUnknown);
impl ITraceEventCallback {
    pub unsafe fn OnBeginProcessTrace<'a, Param0: ::windows::core::IntoParam<'a, ITraceEvent>, Param1: ::windows::core::IntoParam<'a, ITraceRelogger>>(&self, headerevent: Param0, relogger: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), headerevent.into_param().abi(), relogger.into_param().abi()).ok()
    }
    pub unsafe fn OnFinalizeProcessTrace<'a, Param0: ::windows::core::IntoParam<'a, ITraceRelogger>>(&self, relogger: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), relogger.into_param().abi()).ok()
    }
    pub unsafe fn OnEvent<'a, Param0: ::windows::core::IntoParam<'a, ITraceEvent>, Param1: ::windows::core::IntoParam<'a, ITraceRelogger>>(&self, event: Param0, relogger: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), event.into_param().abi(), relogger.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ITraceEventCallback {
    type Vtable = ITraceEventCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ed25501_593f_43e9_8f38_3ab46f5a4a52);
}
impl ::core::convert::From<ITraceEventCallback> for ::windows::core::IUnknown {
    fn from(value: ITraceEventCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITraceEventCallback> for ::windows::core::IUnknown {
    fn from(value: &ITraceEventCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceEventCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceEventCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, headerevent: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, event: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITraceRelogger(pub ::windows::core::IUnknown);
impl ITraceRelogger {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLogfileTraceStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, logfilename: Param0, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), logfilename.into_param().abi(), ::core::mem::transmute(usercontext), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddRealtimeTraceStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, loggername: Param0, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), loggername.into_param().abi(), ::core::mem::transmute(usercontext), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn RegisterCallback<'a, Param0: ::windows::core::IntoParam<'a, ITraceEventCallback>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    pub unsafe fn Inject<'a, Param0: ::windows::core::IntoParam<'a, ITraceEvent>>(&self, event: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), event.into_param().abi()).ok()
    }
    pub unsafe fn CreateEventInstance(&self, tracehandle: u64, flags: u32) -> ::windows::core::Result<ITraceEvent> {
        let mut result__: <ITraceEvent as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(tracehandle), ::core::mem::transmute(flags), &mut result__).from_abi::<ITraceEvent>(result__)
    }
    pub unsafe fn ProcessTrace(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputFilename<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, logfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), logfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompressionMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(&self, compressionmode: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), compressionmode.into_param().abi()).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITraceRelogger {
    type Vtable = ITraceRelogger_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf754ad43_3bcc_4286_8009_9c5da214e84e);
}
impl ::core::convert::From<ITraceRelogger> for ::windows::core::IUnknown {
    fn from(value: ITraceRelogger) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITraceRelogger> for ::windows::core::IUnknown {
    fn from(value: &ITraceRelogger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITraceRelogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITraceRelogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITraceRelogger_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, loggername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, event: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tracehandle: u64, flags: u32, event: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MAP_FLAGS(pub i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_VALUEMAP: MAP_FLAGS = MAP_FLAGS(1i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_BITMAP: MAP_FLAGS = MAP_FLAGS(2i32);
pub const EVENTMAP_INFO_FLAG_MANIFEST_PATTERNMAP: MAP_FLAGS = MAP_FLAGS(4i32);
pub const EVENTMAP_INFO_FLAG_WBEM_VALUEMAP: MAP_FLAGS = MAP_FLAGS(8i32);
pub const EVENTMAP_INFO_FLAG_WBEM_BITMAP: MAP_FLAGS = MAP_FLAGS(16i32);
pub const EVENTMAP_INFO_FLAG_WBEM_FLAG: MAP_FLAGS = MAP_FLAGS(32i32);
pub const EVENTMAP_INFO_FLAG_WBEM_NO_MAP: MAP_FLAGS = MAP_FLAGS(64i32);
impl ::core::convert::From<i32> for MAP_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MAP_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MAP_VALUETYPE(pub i32);
pub const EVENTMAP_ENTRY_VALUETYPE_ULONG: MAP_VALUETYPE = MAP_VALUETYPE(0i32);
pub const EVENTMAP_ENTRY_VALUETYPE_STRING: MAP_VALUETYPE = MAP_VALUETYPE(1i32);
impl ::core::convert::From<i32> for MAP_VALUETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MAP_VALUETYPE {
    type Abi = Self;
}
pub const MAX_EVENT_DATA_DESCRIPTORS: u32 = 128u32;
pub const MAX_EVENT_FILTERS_COUNT: u32 = 13u32;
pub const MAX_EVENT_FILTER_DATA_SIZE: u32 = 1024u32;
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: u32 = 64u32;
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: u32 = 4096u32;
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: u32 = 4096u32;
pub const MAX_EVENT_FILTER_PID_COUNT: u32 = 8u32;
pub const MAX_MOF_FIELDS: u32 = 16u32;
pub const MAX_PAYLOAD_PREDICATES: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MOF_FIELD {
    pub DataPtr: u64,
    pub Length: u32,
    pub DataType: u32,
}
impl MOF_FIELD {}
impl ::core::default::Default for MOF_FIELD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MOF_FIELD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MOF_FIELD").field("DataPtr", &self.DataPtr).field("Length", &self.Length).field("DataType", &self.DataType).finish()
    }
}
impl ::core::cmp::PartialEq for MOF_FIELD {
    fn eq(&self, other: &Self) -> bool {
        self.DataPtr == other.DataPtr && self.Length == other.Length && self.DataType == other.DataType
    }
}
impl ::core::cmp::Eq for MOF_FIELD {}
unsafe impl ::windows::core::Abi for MOF_FIELD {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct OFFSETINSTANCEDATAANDLENGTH {
    pub OffsetInstanceData: u32,
    pub LengthInstanceData: u32,
}
impl OFFSETINSTANCEDATAANDLENGTH {}
impl ::core::default::Default for OFFSETINSTANCEDATAANDLENGTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OFFSETINSTANCEDATAANDLENGTH {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OFFSETINSTANCEDATAANDLENGTH").field("OffsetInstanceData", &self.OffsetInstanceData).field("LengthInstanceData", &self.LengthInstanceData).finish()
    }
}
impl ::core::cmp::PartialEq for OFFSETINSTANCEDATAANDLENGTH {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetInstanceData == other.OffsetInstanceData && self.LengthInstanceData == other.LengthInstanceData
    }
}
impl ::core::cmp::Eq for OFFSETINSTANCEDATAANDLENGTH {}
unsafe impl ::windows::core::Abi for OFFSETINSTANCEDATAANDLENGTH {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[inline]
pub unsafe fn OpenTraceA(logfile: *mut EVENT_TRACE_LOGFILEA) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenTraceA(logfile: *mut ::core::mem::ManuallyDrop<EVENT_TRACE_LOGFILEA>) -> u64;
        }
        ::core::mem::transmute(OpenTraceA(::core::mem::transmute(logfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[inline]
pub unsafe fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenTraceW(logfile: *mut ::core::mem::ManuallyDrop<EVENT_TRACE_LOGFILEW>) -> u64;
        }
        ::core::mem::transmute(OpenTraceW(::core::mem::transmute(logfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PAYLOAD_FILTER_PREDICATE {
    pub FieldName: super::super::super::Foundation::PWSTR,
    pub CompareOp: u16,
    pub Value: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PAYLOAD_FILTER_PREDICATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAYLOAD_FILTER_PREDICATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PAYLOAD_FILTER_PREDICATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PAYLOAD_FILTER_PREDICATE").field("FieldName", &self.FieldName).field("CompareOp", &self.CompareOp).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAYLOAD_FILTER_PREDICATE {
    fn eq(&self, other: &Self) -> bool {
        self.FieldName == other.FieldName && self.CompareOp == other.CompareOp && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAYLOAD_FILTER_PREDICATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PAYLOAD_FILTER_PREDICATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PAYLOAD_OPERATOR(pub i32);
pub const PAYLOADFIELD_EQ: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(0i32);
pub const PAYLOADFIELD_NE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(1i32);
pub const PAYLOADFIELD_LE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(2i32);
pub const PAYLOADFIELD_GT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(3i32);
pub const PAYLOADFIELD_LT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(4i32);
pub const PAYLOADFIELD_GE: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(5i32);
pub const PAYLOADFIELD_BETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(6i32);
pub const PAYLOADFIELD_NOTBETWEEN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(7i32);
pub const PAYLOADFIELD_MODULO: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(8i32);
pub const PAYLOADFIELD_CONTAINS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(20i32);
pub const PAYLOADFIELD_DOESNTCONTAIN: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(21i32);
pub const PAYLOADFIELD_IS: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(30i32);
pub const PAYLOADFIELD_ISNOT: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(31i32);
pub const PAYLOADFIELD_INVALID: PAYLOAD_OPERATOR = PAYLOAD_OPERATOR(32i32);
impl ::core::convert::From<i32> for PAYLOAD_OPERATOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PAYLOAD_OPERATOR {
    type Abi = Self;
}
pub type PENABLECALLBACK = unsafe extern "system" fn(sourceid: *const ::windows::core::GUID, isenabled: ENABLECALLBACK_ENABLED_STATE, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut ::core::ffi::c_void);
pub type PEVENT_CALLBACK = unsafe extern "system" fn(pevent: *mut EVENT_TRACE);
pub type PEVENT_RECORD_CALLBACK = unsafe extern "system" fn(eventrecord: *mut EVENT_RECORD);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKA = unsafe extern "system" fn(logfile: *mut ::core::mem::ManuallyDrop<EVENT_TRACE_LOGFILEA>) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub type PEVENT_TRACE_BUFFER_CALLBACKW = unsafe extern "system" fn(logfile: *mut ::core::mem::ManuallyDrop<EVENT_TRACE_LOGFILEW>) -> u32;
pub const PROCESS_TRACE_MODE_EVENT_RECORD: u32 = 268435456u32;
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: u32 = 4096u32;
pub const PROCESS_TRACE_MODE_REAL_TIME: u32 = 256u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROFILE_SOURCE_INFO {
    pub NextEntryOffset: u32,
    pub Source: u32,
    pub MinInterval: u32,
    pub MaxInterval: u32,
    pub Reserved: u64,
    pub Description: [u16; 1],
}
impl PROFILE_SOURCE_INFO {}
impl ::core::default::Default for PROFILE_SOURCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROFILE_SOURCE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROFILE_SOURCE_INFO").field("NextEntryOffset", &self.NextEntryOffset).field("Source", &self.Source).field("MinInterval", &self.MinInterval).field("MaxInterval", &self.MaxInterval).field("Reserved", &self.Reserved).field("Description", &self.Description).finish()
    }
}
impl ::core::cmp::PartialEq for PROFILE_SOURCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.Source == other.Source && self.MinInterval == other.MinInterval && self.MaxInterval == other.MaxInterval && self.Reserved == other.Reserved && self.Description == other.Description
    }
}
impl ::core::cmp::Eq for PROFILE_SOURCE_INFO {}
unsafe impl ::windows::core::Abi for PROFILE_SOURCE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: u64,
    pub ArrayIndex: u32,
    pub Reserved: u32,
}
impl PROPERTY_DATA_DESCRIPTOR {}
impl ::core::default::Default for PROPERTY_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROPERTY_DATA_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROPERTY_DATA_DESCRIPTOR").field("PropertyName", &self.PropertyName).field("ArrayIndex", &self.ArrayIndex).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for PROPERTY_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyName == other.PropertyName && self.ArrayIndex == other.ArrayIndex && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PROPERTY_DATA_DESCRIPTOR {}
unsafe impl ::windows::core::Abi for PROPERTY_DATA_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPERTY_FLAGS(pub i32);
pub const PropertyStruct: PROPERTY_FLAGS = PROPERTY_FLAGS(1i32);
pub const PropertyParamLength: PROPERTY_FLAGS = PROPERTY_FLAGS(2i32);
pub const PropertyParamCount: PROPERTY_FLAGS = PROPERTY_FLAGS(4i32);
pub const PropertyWBEMXmlFragment: PROPERTY_FLAGS = PROPERTY_FLAGS(8i32);
pub const PropertyParamFixedLength: PROPERTY_FLAGS = PROPERTY_FLAGS(16i32);
pub const PropertyParamFixedCount: PROPERTY_FLAGS = PROPERTY_FLAGS(32i32);
pub const PropertyHasTags: PROPERTY_FLAGS = PROPERTY_FLAGS(64i32);
pub const PropertyHasCustomSchema: PROPERTY_FLAGS = PROPERTY_FLAGS(128i32);
impl ::core::convert::From<i32> for PROPERTY_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROPERTY_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROVIDER_ENUMERATION_INFO {
    pub NumberOfProviders: u32,
    pub Reserved: u32,
    pub TraceProviderInfoArray: [TRACE_PROVIDER_INFO; 1],
}
impl PROVIDER_ENUMERATION_INFO {}
impl ::core::default::Default for PROVIDER_ENUMERATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROVIDER_ENUMERATION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROVIDER_ENUMERATION_INFO").field("NumberOfProviders", &self.NumberOfProviders).field("Reserved", &self.Reserved).field("TraceProviderInfoArray", &self.TraceProviderInfoArray).finish()
    }
}
impl ::core::cmp::PartialEq for PROVIDER_ENUMERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfProviders == other.NumberOfProviders && self.Reserved == other.Reserved && self.TraceProviderInfoArray == other.TraceProviderInfoArray
    }
}
impl ::core::cmp::Eq for PROVIDER_ENUMERATION_INFO {}
unsafe impl ::windows::core::Abi for PROVIDER_ENUMERATION_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROVIDER_EVENT_INFO {
    pub NumberOfEvents: u32,
    pub Reserved: u32,
    pub EventDescriptorsArray: [EVENT_DESCRIPTOR; 1],
}
impl PROVIDER_EVENT_INFO {}
impl ::core::default::Default for PROVIDER_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROVIDER_EVENT_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROVIDER_EVENT_INFO").field("NumberOfEvents", &self.NumberOfEvents).field("Reserved", &self.Reserved).field("EventDescriptorsArray", &self.EventDescriptorsArray).finish()
    }
}
impl ::core::cmp::PartialEq for PROVIDER_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfEvents == other.NumberOfEvents && self.Reserved == other.Reserved && self.EventDescriptorsArray == other.EventDescriptorsArray
    }
}
impl ::core::cmp::Eq for PROVIDER_EVENT_INFO {}
unsafe impl ::windows::core::Abi for PROVIDER_EVENT_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROVIDER_FIELD_INFO {
    pub NameOffset: u32,
    pub DescriptionOffset: u32,
    pub Value: u64,
}
impl PROVIDER_FIELD_INFO {}
impl ::core::default::Default for PROVIDER_FIELD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROVIDER_FIELD_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROVIDER_FIELD_INFO").field("NameOffset", &self.NameOffset).field("DescriptionOffset", &self.DescriptionOffset).field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for PROVIDER_FIELD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NameOffset == other.NameOffset && self.DescriptionOffset == other.DescriptionOffset && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for PROVIDER_FIELD_INFO {}
unsafe impl ::windows::core::Abi for PROVIDER_FIELD_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROVIDER_FIELD_INFOARRAY {
    pub NumberOfElements: u32,
    pub FieldType: EVENT_FIELD_TYPE,
    pub FieldInfoArray: [PROVIDER_FIELD_INFO; 1],
}
impl PROVIDER_FIELD_INFOARRAY {}
impl ::core::default::Default for PROVIDER_FIELD_INFOARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROVIDER_FIELD_INFOARRAY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROVIDER_FIELD_INFOARRAY").field("NumberOfElements", &self.NumberOfElements).field("FieldType", &self.FieldType).field("FieldInfoArray", &self.FieldInfoArray).finish()
    }
}
impl ::core::cmp::PartialEq for PROVIDER_FIELD_INFOARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfElements == other.NumberOfElements && self.FieldType == other.FieldType && self.FieldInfoArray == other.FieldInfoArray
    }
}
impl ::core::cmp::Eq for PROVIDER_FIELD_INFOARRAY {}
unsafe impl ::windows::core::Abi for PROVIDER_FIELD_INFOARRAY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROVIDER_FILTER_INFO {
    pub Id: u8,
    pub Version: u8,
    pub MessageOffset: u32,
    pub Reserved: u32,
    pub PropertyCount: u32,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl PROVIDER_FILTER_INFO {}
impl ::core::default::Default for PROVIDER_FILTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROVIDER_FILTER_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PROVIDER_FILTER_INFO {}
unsafe impl ::windows::core::Abi for PROVIDER_FILTER_INFO {
    type Abi = Self;
}
pub const PrivateLoggerNotificationGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3595ab5c_042a_4c8e_b942_2d059bfeb1b1);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessTrace(handlearray: *const u64, handlecount: u32, starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessTrace(handlearray: *const u64, handlecount: u32, starttime: *const super::super::super::Foundation::FILETIME, endtime: *const super::super::super::Foundation::FILETIME) -> u32;
        }
        ::core::mem::transmute(ProcessTrace(::core::mem::transmute(handlearray), ::core::mem::transmute(handlecount), ::core::mem::transmute(starttime), ::core::mem::transmute(endtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryAllTracesA(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryAllTracesA(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
        }
        ::core::mem::transmute(QueryAllTracesA(::core::mem::transmute(propertyarray), ::core::mem::transmute(propertyarraycount), ::core::mem::transmute(loggercount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryAllTracesW(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryAllTracesW(propertyarray: *mut *mut EVENT_TRACE_PROPERTIES, propertyarraycount: u32, loggercount: *mut u32) -> u32;
        }
        ::core::mem::transmute(QueryAllTracesW(::core::mem::transmute(propertyarray), ::core::mem::transmute(propertyarraycount), ::core::mem::transmute(loggercount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryTraceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(QueryTraceA(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryTraceProcessingHandle(processinghandle: u64, informationclass: ETW_PROCESS_HANDLE_INFO_TYPE, inbuffer: *const ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, returnlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(QueryTraceProcessingHandle(::core::mem::transmute(processinghandle), ::core::mem::transmute(informationclass), ::core::mem::transmute(inbuffer), ::core::mem::transmute(inbuffersize), ::core::mem::transmute(outbuffer), ::core::mem::transmute(outbuffersize), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryTraceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(QueryTraceW(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTraceGuidsA<'a, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(requestaddress: ::core::option::Option<WMIDPREQUEST>, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: Param5, mofresourcename: Param6, registrationhandle: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTraceGuidsA(requestaddress: ::windows::core::RawPtr, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: super::super::super::Foundation::PSTR, mofresourcename: super::super::super::Foundation::PSTR, registrationhandle: *mut u64) -> u32;
        }
        ::core::mem::transmute(RegisterTraceGuidsA(
            ::core::mem::transmute(requestaddress),
            ::core::mem::transmute(requestcontext),
            ::core::mem::transmute(controlguid),
            ::core::mem::transmute(guidcount),
            ::core::mem::transmute(traceguidreg),
            mofimagepath.into_param().abi(),
            mofresourcename.into_param().abi(),
            ::core::mem::transmute(registrationhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTraceGuidsW<'a, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(requestaddress: ::core::option::Option<WMIDPREQUEST>, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: Param5, mofresourcename: Param6, registrationhandle: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTraceGuidsW(requestaddress: ::windows::core::RawPtr, requestcontext: *const ::core::ffi::c_void, controlguid: *const ::windows::core::GUID, guidcount: u32, traceguidreg: *const TRACE_GUID_REGISTRATION, mofimagepath: super::super::super::Foundation::PWSTR, mofresourcename: super::super::super::Foundation::PWSTR, registrationhandle: *mut u64) -> u32;
        }
        ::core::mem::transmute(RegisterTraceGuidsW(
            ::core::mem::transmute(requestaddress),
            ::core::mem::transmute(requestcontext),
            ::core::mem::transmute(controlguid),
            ::core::mem::transmute(guidcount),
            ::core::mem::transmute(traceguidreg),
            mofimagepath.into_param().abi(),
            mofresourcename.into_param().abi(),
            ::core::mem::transmute(registrationhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RemoveTraceCallback(pguid: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveTraceCallback(pguid: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(RemoveTraceCallback(::core::mem::transmute(pguid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SYSTEM_ALPC_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_CONFIG_KW_GRAPHICS: u64 = 2u64;
pub const SYSTEM_CONFIG_KW_NETWORK: u64 = 8u64;
pub const SYSTEM_CONFIG_KW_OPTICAL: u64 = 64u64;
pub const SYSTEM_CONFIG_KW_PNP: u64 = 32u64;
pub const SYSTEM_CONFIG_KW_SERVICES: u64 = 16u64;
pub const SYSTEM_CONFIG_KW_STORAGE: u64 = 4u64;
pub const SYSTEM_CONFIG_KW_SYSTEM: u64 = 1u64;
pub const SYSTEM_CPU_KW_CACHE_FLUSH: u64 = 2u64;
pub const SYSTEM_CPU_KW_CONFIG: u64 = 1u64;
pub const SYSTEM_CPU_KW_SPEC_CONTROL: u64 = 4u64;
pub const SYSTEM_EVENT_TYPE: u32 = 1u32;
pub const SYSTEM_HYPERVISOR_KW_CALLOUTS: u64 = 2u64;
pub const SYSTEM_HYPERVISOR_KW_PROFILE: u64 = 1u64;
pub const SYSTEM_HYPERVISOR_KW_VTL_CHANGE: u64 = 4u64;
pub const SYSTEM_INTERRUPT_KW_CLOCK_INTERRUPT: u64 = 2u64;
pub const SYSTEM_INTERRUPT_KW_DPC: u64 = 4u64;
pub const SYSTEM_INTERRUPT_KW_DPC_QUEUE: u64 = 8u64;
pub const SYSTEM_INTERRUPT_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_INTERRUPT_KW_IPI: u64 = 64u64;
pub const SYSTEM_INTERRUPT_KW_WDF_DPC: u64 = 16u64;
pub const SYSTEM_INTERRUPT_KW_WDF_INTERRUPT: u64 = 32u64;
pub const SYSTEM_IOFILTER_KW_FAILURE: u64 = 8u64;
pub const SYSTEM_IOFILTER_KW_FASTIO: u64 = 4u64;
pub const SYSTEM_IOFILTER_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_IOFILTER_KW_INIT: u64 = 2u64;
pub const SYSTEM_IO_KW_CC: u64 = 256u64;
pub const SYSTEM_IO_KW_DISK: u64 = 1u64;
pub const SYSTEM_IO_KW_DISK_INIT: u64 = 2u64;
pub const SYSTEM_IO_KW_DRIVERS: u64 = 128u64;
pub const SYSTEM_IO_KW_FILE: u64 = 16u64;
pub const SYSTEM_IO_KW_FILENAME: u64 = 4u64;
pub const SYSTEM_IO_KW_NETWORK: u64 = 512u64;
pub const SYSTEM_IO_KW_OPTICAL: u64 = 32u64;
pub const SYSTEM_IO_KW_OPTICAL_INIT: u64 = 64u64;
pub const SYSTEM_IO_KW_SPLIT: u64 = 8u64;
pub const SYSTEM_LOCK_KW_SPINLOCK: u64 = 1u64;
pub const SYSTEM_LOCK_KW_SPINLOCK_COUNTERS: u64 = 2u64;
pub const SYSTEM_LOCK_KW_SYNC_OBJECTS: u64 = 4u64;
pub const SYSTEM_MEMORY_KW_ALL_FAULTS: u64 = 4u64;
pub const SYSTEM_MEMORY_KW_CONTMEM_GEN: u64 = 512u64;
pub const SYSTEM_MEMORY_KW_FOOTPRINT: u64 = 2048u64;
pub const SYSTEM_MEMORY_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_MEMORY_KW_HARD_FAULTS: u64 = 2u64;
pub const SYSTEM_MEMORY_KW_HEAP: u64 = 128u64;
pub const SYSTEM_MEMORY_KW_MEMINFO: u64 = 16u64;
pub const SYSTEM_MEMORY_KW_MEMINFO_WS: u64 = 64u64;
pub const SYSTEM_MEMORY_KW_NONTRADEABLE: u64 = 32768u64;
pub const SYSTEM_MEMORY_KW_PFSECTION: u64 = 32u64;
pub const SYSTEM_MEMORY_KW_POOL: u64 = 8u64;
pub const SYSTEM_MEMORY_KW_REFSET: u64 = 8192u64;
pub const SYSTEM_MEMORY_KW_SESSION: u64 = 4096u64;
pub const SYSTEM_MEMORY_KW_VAMAP: u64 = 16384u64;
pub const SYSTEM_MEMORY_KW_VIRTUAL_ALLOC: u64 = 1024u64;
pub const SYSTEM_MEMORY_KW_WS: u64 = 256u64;
pub const SYSTEM_MEMORY_POOL_FILTER_ID: u32 = 1u32;
pub const SYSTEM_OBJECT_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_OBJECT_KW_HANDLE: u64 = 2u64;
pub const SYSTEM_POWER_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_POWER_KW_HIBER_RUNDOWN: u64 = 2u64;
pub const SYSTEM_POWER_KW_IDLE_SELECTION: u64 = 8u64;
pub const SYSTEM_POWER_KW_PPM_EXIT_LATENCY: u64 = 16u64;
pub const SYSTEM_POWER_KW_PROCESSOR_IDLE: u64 = 4u64;
pub const SYSTEM_PROCESS_KW_DBGPRINT: u64 = 256u64;
pub const SYSTEM_PROCESS_KW_DEBUG_EVENTS: u64 = 128u64;
pub const SYSTEM_PROCESS_KW_FREEZE: u64 = 4u64;
pub const SYSTEM_PROCESS_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_PROCESS_KW_INSWAP: u64 = 2u64;
pub const SYSTEM_PROCESS_KW_JOB: u64 = 512u64;
pub const SYSTEM_PROCESS_KW_LOADER: u64 = 4096u64;
pub const SYSTEM_PROCESS_KW_PERF_COUNTER: u64 = 8u64;
pub const SYSTEM_PROCESS_KW_THREAD: u64 = 2048u64;
pub const SYSTEM_PROCESS_KW_WAKE_COUNTER: u64 = 16u64;
pub const SYSTEM_PROCESS_KW_WAKE_DROP: u64 = 32u64;
pub const SYSTEM_PROCESS_KW_WAKE_EVENT: u64 = 64u64;
pub const SYSTEM_PROCESS_KW_WORKER_THREAD: u64 = 1024u64;
pub const SYSTEM_PROFILE_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_PROFILE_KW_PMC_PROFILE: u64 = 2u64;
pub const SYSTEM_REGISTRY_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_REGISTRY_KW_HIVE: u64 = 2u64;
pub const SYSTEM_REGISTRY_KW_NOTIFICATION: u64 = 4u64;
pub const SYSTEM_SCHEDULER_KW_AFFINITY: u64 = 64u64;
pub const SYSTEM_SCHEDULER_KW_ANTI_STARVATION: u64 = 16u64;
pub const SYSTEM_SCHEDULER_KW_COMPACT_CSWITCH: u64 = 1024u64;
pub const SYSTEM_SCHEDULER_KW_CONTEXT_SWITCH: u64 = 512u64;
pub const SYSTEM_SCHEDULER_KW_DISPATCHER: u64 = 2u64;
pub const SYSTEM_SCHEDULER_KW_IDEAL_PROCESSOR: u64 = 256u64;
pub const SYSTEM_SCHEDULER_KW_KERNEL_QUEUE: u64 = 4u64;
pub const SYSTEM_SCHEDULER_KW_LOAD_BALANCER: u64 = 32u64;
pub const SYSTEM_SCHEDULER_KW_PRIORITY: u64 = 128u64;
pub const SYSTEM_SCHEDULER_KW_SHOULD_YIELD: u64 = 8u64;
pub const SYSTEM_SCHEDULER_KW_XSCHEDULER: u64 = 1u64;
pub const SYSTEM_SYSCALL_KW_GENERAL: u64 = 1u64;
pub const SYSTEM_TIMER_KW_CLOCK_TIMER: u64 = 2u64;
pub const SYSTEM_TIMER_KW_GENERAL: u64 = 1u64;
#[inline]
pub unsafe fn SetTraceCallback(pguid: *const ::windows::core::GUID, eventcallback: ::core::option::Option<PEVENT_CALLBACK>) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTraceCallback(pguid: *const ::windows::core::GUID, eventcallback: ::windows::core::RawPtr) -> u32;
        }
        ::core::mem::transmute(SetTraceCallback(::core::mem::transmute(pguid), ::core::mem::transmute(eventcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartTraceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: *mut u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartTraceA(tracehandle: *mut u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(StartTraceA(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartTraceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: *mut u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartTraceW(tracehandle: *mut u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(StartTraceW(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StopTraceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StopTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(StopTraceA(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StopTraceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StopTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(StopTraceW(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SystemAlpcProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcb9baaf_e529_4980_92e9_ced1a6aadfdf);
pub const SystemConfigProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfef3a8b6_318d_4b67_a96a_3b0f6b8f18fe);
pub const SystemCpuProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6c5265f_eae8_4650_aae4_9d48603d8510);
pub const SystemHypervisorProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafa072a_918a_4bed_b622_bc152097098f);
pub const SystemInterruptProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4bbee17_b545_4888_858b_744169015b25);
pub const SystemIoFilterProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbd09363_9e22_4661_b8bf_e7a34b535b8c);
pub const SystemIoProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d5c43e3_0f1c_4202_b817_174c0070dc79);
pub const SystemLockProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x721ddfd3_dacc_4e1e_b26a_a2cb31d4705a);
pub const SystemMemoryProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82958ca9_b6cd_47f8_a3a8_03ae85a4bc24);
pub const SystemObjectProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfebd7460_3d1d_47eb_af49_c9eeb1e146f2);
pub const SystemPowerProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc134884a_32d5_4488_80e5_14ed7abb8269);
pub const SystemProcessProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x151f55dc_467d_471f_83b5_5f889d46ff66);
pub const SystemProfileProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfeb0324_1cee_496f_a409_2ac2b48a6322);
pub const SystemRegistryProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16156bd9_fab4_4cfa_a232_89d1099058e3);
pub const SystemSchedulerProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x599a2a76_4d91_4910_9ac7_7d33f2e97a6c);
pub const SystemSyscallProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x434286f7_6f1b_45bb_b37e_95f623046c7c);
pub const SystemTimerProviderGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f061568_e215_499f_ab2e_eda0ae890a5b);
pub const SystemTraceControlGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e814aad_3204_11d2_9a82_006008a86939);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TDH_CONTEXT {
    pub ParameterValue: u64,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: u32,
}
impl TDH_CONTEXT {}
impl ::core::default::Default for TDH_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TDH_CONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TDH_CONTEXT").field("ParameterValue", &self.ParameterValue).field("ParameterType", &self.ParameterType).field("ParameterSize", &self.ParameterSize).finish()
    }
}
impl ::core::cmp::PartialEq for TDH_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ParameterValue == other.ParameterValue && self.ParameterType == other.ParameterType && self.ParameterSize == other.ParameterSize
    }
}
impl ::core::cmp::Eq for TDH_CONTEXT {}
unsafe impl ::windows::core::Abi for TDH_CONTEXT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TDH_CONTEXT_TYPE(pub i32);
pub const TDH_CONTEXT_WPP_TMFFILE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(0i32);
pub const TDH_CONTEXT_WPP_TMFSEARCHPATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(1i32);
pub const TDH_CONTEXT_WPP_GMT: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(2i32);
pub const TDH_CONTEXT_POINTERSIZE: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(3i32);
pub const TDH_CONTEXT_PDB_PATH: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(4i32);
pub const TDH_CONTEXT_MAXIMUM: TDH_CONTEXT_TYPE = TDH_CONTEXT_TYPE(5i32);
impl ::core::convert::From<i32> for TDH_CONTEXT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TDH_CONTEXT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct TDH_HANDLE(pub isize);
impl ::core::default::Default for TDH_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for TDH_HANDLE {}
unsafe impl ::windows::core::Abi for TDH_HANDLE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TEMPLATE_FLAGS(pub i32);
pub const TEMPLATE_EVENT_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(1i32);
pub const TEMPLATE_USER_DATA: TEMPLATE_FLAGS = TEMPLATE_FLAGS(2i32);
pub const TEMPLATE_CONTROL_GUID: TEMPLATE_FLAGS = TEMPLATE_FLAGS(4i32);
impl ::core::convert::From<i32> for TEMPLATE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TEMPLATE_FLAGS {
    type Abi = Self;
}
pub const TRACELOG_ACCESS_KERNEL_LOGGER: u32 = 256u32;
pub const TRACELOG_ACCESS_REALTIME: u32 = 1024u32;
pub const TRACELOG_CREATE_INPROC: u32 = 512u32;
pub const TRACELOG_CREATE_ONDISK: u32 = 64u32;
pub const TRACELOG_CREATE_REALTIME: u32 = 32u32;
pub const TRACELOG_GUID_ENABLE: u32 = 128u32;
pub const TRACELOG_JOIN_GROUP: u32 = 4096u32;
pub const TRACELOG_LOG_EVENT: u32 = 512u32;
pub const TRACELOG_REGISTER_GUIDS: u32 = 2048u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_ENABLE_INFO {
    pub IsEnabled: u32,
    pub Level: u8,
    pub Reserved1: u8,
    pub LoggerId: u16,
    pub EnableProperty: u32,
    pub Reserved2: u32,
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
}
impl TRACE_ENABLE_INFO {}
impl ::core::default::Default for TRACE_ENABLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_ENABLE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_ENABLE_INFO")
            .field("IsEnabled", &self.IsEnabled)
            .field("Level", &self.Level)
            .field("Reserved1", &self.Reserved1)
            .field("LoggerId", &self.LoggerId)
            .field("EnableProperty", &self.EnableProperty)
            .field("Reserved2", &self.Reserved2)
            .field("MatchAnyKeyword", &self.MatchAnyKeyword)
            .field("MatchAllKeyword", &self.MatchAllKeyword)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_ENABLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.IsEnabled == other.IsEnabled && self.Level == other.Level && self.Reserved1 == other.Reserved1 && self.LoggerId == other.LoggerId && self.EnableProperty == other.EnableProperty && self.Reserved2 == other.Reserved2 && self.MatchAnyKeyword == other.MatchAnyKeyword && self.MatchAllKeyword == other.MatchAllKeyword
    }
}
impl ::core::cmp::Eq for TRACE_ENABLE_INFO {}
unsafe impl ::windows::core::Abi for TRACE_ENABLE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: ::windows::core::GUID,
    pub EventGuid: ::windows::core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub DecodingSource: DECODING_SOURCE,
    pub ProviderNameOffset: u32,
    pub LevelNameOffset: u32,
    pub ChannelNameOffset: u32,
    pub KeywordsNameOffset: u32,
    pub TaskNameOffset: u32,
    pub OpcodeNameOffset: u32,
    pub EventMessageOffset: u32,
    pub ProviderMessageOffset: u32,
    pub BinaryXMLOffset: u32,
    pub BinaryXMLSize: u32,
    pub Anonymous1: TRACE_EVENT_INFO_0,
    pub Anonymous2: TRACE_EVENT_INFO_1,
    pub PropertyCount: u32,
    pub TopLevelPropertyCount: u32,
    pub Anonymous3: TRACE_EVENT_INFO_2,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl TRACE_EVENT_INFO {}
impl ::core::default::Default for TRACE_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO {}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: u32,
    pub ActivityIDNameOffset: u32,
}
impl TRACE_EVENT_INFO_0 {}
impl ::core::default::Default for TRACE_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_0 {}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: u32,
    pub RelatedActivityIDNameOffset: u32,
}
impl TRACE_EVENT_INFO_1 {}
impl ::core::default::Default for TRACE_EVENT_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_1 {}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
impl TRACE_EVENT_INFO_2 {}
impl ::core::default::Default for TRACE_EVENT_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_2 {}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
impl TRACE_EVENT_INFO_2_0 {}
impl ::core::default::Default for TRACE_EVENT_INFO_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_EVENT_INFO_2_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_EVENT_INFO_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for TRACE_EVENT_INFO_2_0 {}
unsafe impl ::windows::core::Abi for TRACE_EVENT_INFO_2_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_GUID_INFO {
    pub InstanceCount: u32,
    pub Reserved: u32,
}
impl TRACE_GUID_INFO {}
impl ::core::default::Default for TRACE_GUID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_GUID_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_GUID_INFO").field("InstanceCount", &self.InstanceCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_GUID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceCount == other.InstanceCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for TRACE_GUID_INFO {}
unsafe impl ::windows::core::Abi for TRACE_GUID_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_GUID_PROPERTIES {
    pub Guid: ::windows::core::GUID,
    pub GuidType: u32,
    pub LoggerId: u32,
    pub EnableLevel: u32,
    pub EnableFlags: u32,
    pub IsEnable: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl TRACE_GUID_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRACE_GUID_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRACE_GUID_PROPERTIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_GUID_PROPERTIES").field("Guid", &self.Guid).field("GuidType", &self.GuidType).field("LoggerId", &self.LoggerId).field("EnableLevel", &self.EnableLevel).field("EnableFlags", &self.EnableFlags).field("IsEnable", &self.IsEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRACE_GUID_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid && self.GuidType == other.GuidType && self.LoggerId == other.LoggerId && self.EnableLevel == other.EnableLevel && self.EnableFlags == other.EnableFlags && self.IsEnable == other.IsEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRACE_GUID_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRACE_GUID_PROPERTIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_GUID_REGISTRATION {
    pub Guid: *mut ::windows::core::GUID,
    pub RegHandle: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl TRACE_GUID_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRACE_GUID_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRACE_GUID_REGISTRATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_GUID_REGISTRATION").field("Guid", &self.Guid).field("RegHandle", &self.RegHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRACE_GUID_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid && self.RegHandle == other.RegHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRACE_GUID_REGISTRATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRACE_GUID_REGISTRATION {
    type Abi = Self;
}
pub const TRACE_HEADER_FLAG_LOG_WNODE: u32 = 262144u32;
pub const TRACE_HEADER_FLAG_TRACED_GUID: u32 = 131072u32;
pub const TRACE_HEADER_FLAG_USE_GUID_PTR: u32 = 524288u32;
pub const TRACE_HEADER_FLAG_USE_MOF_PTR: u32 = 1048576u32;
pub const TRACE_HEADER_FLAG_USE_TIMESTAMP: u32 = 512u32;
pub const TRACE_LEVEL_CRITICAL: u32 = 1u32;
pub const TRACE_LEVEL_ERROR: u32 = 2u32;
pub const TRACE_LEVEL_FATAL: u32 = 1u32;
pub const TRACE_LEVEL_INFORMATION: u32 = 4u32;
pub const TRACE_LEVEL_NONE: u32 = 0u32;
pub const TRACE_LEVEL_RESERVED6: u32 = 6u32;
pub const TRACE_LEVEL_RESERVED7: u32 = 7u32;
pub const TRACE_LEVEL_RESERVED8: u32 = 8u32;
pub const TRACE_LEVEL_RESERVED9: u32 = 9u32;
pub const TRACE_LEVEL_VERBOSE: u32 = 5u32;
pub const TRACE_LEVEL_WARNING: u32 = 3u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER_1,
    pub LoggerName: super::super::super::Foundation::PWSTR,
    pub LogFileName: super::super::super::Foundation::PWSTR,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_VersionDetail_e__Struct").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.SubVersion == other.SubVersion && self.SubMinorVersion == other.SubMinorVersion
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: ::windows::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartBuffers == other.StartBuffers && self.PointerSize == other.PointerSize && self.EventsLost == other.EventsLost && self.CpuSpeedInMHz == other.CpuSpeedInMHz
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER_1_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER32 {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER32_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER32_1,
    pub LoggerName: u32,
    pub LogFileName: u32,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER32_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER32_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER32_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER32_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER32_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER32_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_VersionDetail_e__Struct").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.SubVersion == other.SubVersion && self.SubMinorVersion == other.SubMinorVersion
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER32_1 {
    pub LogInstanceGuid: ::windows::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER32_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER32_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER32_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER32_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER32_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER32_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER32_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartBuffers == other.StartBuffers && self.PointerSize == other.PointerSize && self.EventsLost == other.EventsLost && self.CpuSpeedInMHz == other.CpuSpeedInMHz
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER32_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER32_1_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER64 {
    pub BufferSize: u32,
    pub Anonymous1: TRACE_LOGFILE_HEADER64_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER64_1,
    pub LoggerName: u64,
    pub LogFileName: u64,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER64_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER64_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER64_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER64_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER64_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER64_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_VersionDetail_e__Struct").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("SubVersion", &self.SubVersion).field("SubMinorVersion", &self.SubMinorVersion).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.SubVersion == other.SubVersion && self.SubMinorVersion == other.SubMinorVersion
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub union TRACE_LOGFILE_HEADER64_1 {
    pub LogInstanceGuid: ::windows::core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER64_1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER64_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct TRACE_LOGFILE_HEADER64_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl TRACE_LOGFILE_HEADER64_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for TRACE_LOGFILE_HEADER64_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::fmt::Debug for TRACE_LOGFILE_HEADER64_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("StartBuffers", &self.StartBuffers).field("PointerSize", &self.PointerSize).field("EventsLost", &self.EventsLost).field("CpuSpeedInMHz", &self.CpuSpeedInMHz).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::PartialEq for TRACE_LOGFILE_HEADER64_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartBuffers == other.StartBuffers && self.PointerSize == other.PointerSize && self.EventsLost == other.EventsLost && self.CpuSpeedInMHz == other.CpuSpeedInMHz
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::cmp::Eq for TRACE_LOGFILE_HEADER64_1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
unsafe impl ::windows::core::Abi for TRACE_LOGFILE_HEADER64_1_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRACE_MESSAGE_FLAGS(pub u32);
pub const TRACE_MESSAGE_COMPONENTID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(4u32);
pub const TRACE_MESSAGE_GUID: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(2u32);
pub const TRACE_MESSAGE_SEQUENCE: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(1u32);
pub const TRACE_MESSAGE_SYSTEMINFO: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(32u32);
pub const TRACE_MESSAGE_TIMESTAMP: TRACE_MESSAGE_FLAGS = TRACE_MESSAGE_FLAGS(8u32);
impl ::core::convert::From<u32> for TRACE_MESSAGE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TRACE_MESSAGE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TRACE_MESSAGE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TRACE_MESSAGE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TRACE_MESSAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TRACE_MESSAGE_FLAG_MASK: u32 = 65535u32;
pub const TRACE_MESSAGE_PERFORMANCE_TIMESTAMP: u32 = 16u32;
pub const TRACE_MESSAGE_POINTER32: u32 = 64u32;
pub const TRACE_MESSAGE_POINTER64: u32 = 128u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_PERIODIC_CAPTURE_STATE_INFO {
    pub CaptureStateFrequencyInSeconds: u32,
    pub ProviderCount: u16,
    pub Reserved: u16,
}
impl TRACE_PERIODIC_CAPTURE_STATE_INFO {}
impl ::core::default::Default for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_PERIODIC_CAPTURE_STATE_INFO").field("CaptureStateFrequencyInSeconds", &self.CaptureStateFrequencyInSeconds).field("ProviderCount", &self.ProviderCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CaptureStateFrequencyInSeconds == other.CaptureStateFrequencyInSeconds && self.ProviderCount == other.ProviderCount && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for TRACE_PERIODIC_CAPTURE_STATE_INFO {}
unsafe impl ::windows::core::Abi for TRACE_PERIODIC_CAPTURE_STATE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_PROFILE_INTERVAL {
    pub Source: u32,
    pub Interval: u32,
}
impl TRACE_PROFILE_INTERVAL {}
impl ::core::default::Default for TRACE_PROFILE_INTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_PROFILE_INTERVAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_PROFILE_INTERVAL").field("Source", &self.Source).field("Interval", &self.Interval).finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_PROFILE_INTERVAL {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source && self.Interval == other.Interval
    }
}
impl ::core::cmp::Eq for TRACE_PROFILE_INTERVAL {}
unsafe impl ::windows::core::Abi for TRACE_PROFILE_INTERVAL {
    type Abi = Self;
}
pub const TRACE_PROVIDER_FLAG_LEGACY: u32 = 1u32;
pub const TRACE_PROVIDER_FLAG_PRE_ENABLE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_PROVIDER_INFO {
    pub ProviderGuid: ::windows::core::GUID,
    pub SchemaSource: u32,
    pub ProviderNameOffset: u32,
}
impl TRACE_PROVIDER_INFO {}
impl ::core::default::Default for TRACE_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_PROVIDER_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_PROVIDER_INFO").field("ProviderGuid", &self.ProviderGuid).field("SchemaSource", &self.SchemaSource).field("ProviderNameOffset", &self.ProviderNameOffset).finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_PROVIDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProviderGuid == other.ProviderGuid && self.SchemaSource == other.SchemaSource && self.ProviderNameOffset == other.ProviderNameOffset
    }
}
impl ::core::cmp::Eq for TRACE_PROVIDER_INFO {}
unsafe impl ::windows::core::Abi for TRACE_PROVIDER_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_PROVIDER_INSTANCE_INFO {
    pub NextOffset: u32,
    pub EnableCount: u32,
    pub Pid: u32,
    pub Flags: u32,
}
impl TRACE_PROVIDER_INSTANCE_INFO {}
impl ::core::default::Default for TRACE_PROVIDER_INSTANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_PROVIDER_INSTANCE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_PROVIDER_INSTANCE_INFO").field("NextOffset", &self.NextOffset).field("EnableCount", &self.EnableCount).field("Pid", &self.Pid).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_PROVIDER_INSTANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextOffset == other.NextOffset && self.EnableCount == other.EnableCount && self.Pid == other.Pid && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for TRACE_PROVIDER_INSTANCE_INFO {}
unsafe impl ::windows::core::Abi for TRACE_PROVIDER_INSTANCE_INFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRACE_QUERY_INFO_CLASS(pub i32);
pub const TraceGuidQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(0i32);
pub const TraceGuidQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(1i32);
pub const TraceGuidQueryProcess: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(2i32);
pub const TraceStackTracingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(3i32);
pub const TraceSystemTraceEnableFlagsInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(4i32);
pub const TraceSampledProfileIntervalInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(5i32);
pub const TraceProfileSourceConfigInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(6i32);
pub const TraceProfileSourceListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(7i32);
pub const TracePmcEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(8i32);
pub const TracePmcCounterListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(9i32);
pub const TraceSetDisallowList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(10i32);
pub const TraceVersionInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(11i32);
pub const TraceGroupQueryList: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(12i32);
pub const TraceGroupQueryInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(13i32);
pub const TraceDisallowListQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(14i32);
pub const TraceInfoReserved15: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(15i32);
pub const TracePeriodicCaptureStateListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(16i32);
pub const TracePeriodicCaptureStateInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(17i32);
pub const TraceProviderBinaryTracking: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(18i32);
pub const TraceMaxLoggersQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(19i32);
pub const TraceLbrConfigurationInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(20i32);
pub const TraceLbrEventListInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(21i32);
pub const TraceMaxPmcCounterQuery: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(22i32);
pub const TraceStreamCount: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(23i32);
pub const TraceStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(24i32);
pub const TracePmcCounterOwners: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(25i32);
pub const TraceUnifiedStackCachingInfo: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(26i32);
pub const MaxTraceSetInfoClass: TRACE_QUERY_INFO_CLASS = TRACE_QUERY_INFO_CLASS(27i32);
impl ::core::convert::From<i32> for TRACE_QUERY_INFO_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TRACE_QUERY_INFO_CLASS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACE_STACK_CACHING_INFO {
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub CacheSize: u32,
    pub BucketCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl TRACE_STACK_CACHING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRACE_STACK_CACHING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRACE_STACK_CACHING_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_STACK_CACHING_INFO").field("Enabled", &self.Enabled).field("CacheSize", &self.CacheSize).field("BucketCount", &self.BucketCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRACE_STACK_CACHING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled && self.CacheSize == other.CacheSize && self.BucketCount == other.BucketCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRACE_STACK_CACHING_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TRACE_STACK_CACHING_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TRACE_VERSION_INFO {
    pub EtwTraceProcessingVersion: u32,
    pub Reserved: u32,
}
impl TRACE_VERSION_INFO {}
impl ::core::default::Default for TRACE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TRACE_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRACE_VERSION_INFO").field("EtwTraceProcessingVersion", &self.EtwTraceProcessingVersion).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for TRACE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EtwTraceProcessingVersion == other.EtwTraceProcessingVersion && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for TRACE_VERSION_INFO {}
unsafe impl ::windows::core::Abi for TRACE_VERSION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::core::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhAggregatePayloadFilters(payloadfiltercount: u32, payloadfilterptrs: *const *const ::core::ffi::c_void, eventmatchallflags: *const super::super::super::Foundation::BOOLEAN, eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(TdhAggregatePayloadFilters(::core::mem::transmute(payloadfiltercount), ::core::mem::transmute(payloadfilterptrs), ::core::mem::transmute(eventmatchallflags), ::core::mem::transmute(eventfilterdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhCleanupPayloadEventFilterDescriptor(eventfilterdescriptor: *mut EVENT_FILTER_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(TdhCleanupPayloadEventFilterDescriptor(::core::mem::transmute(eventfilterdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhCloseDecodingHandle<'a, Param0: ::windows::core::IntoParam<'a, TDH_HANDLE>>(handle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhCloseDecodingHandle(handle: TDH_HANDLE) -> u32;
        }
        ::core::mem::transmute(TdhCloseDecodingHandle(handle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhCreatePayloadFilter<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOLEAN>>(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: Param2, payloadpredicatecount: u32, payloadpredicates: *const PAYLOAD_FILTER_PREDICATE, payloadfilter: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhCreatePayloadFilter(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, eventmatchany: super::super::super::Foundation::BOOLEAN, payloadpredicatecount: u32, payloadpredicates: *const PAYLOAD_FILTER_PREDICATE, payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(TdhCreatePayloadFilter(::core::mem::transmute(providerguid), ::core::mem::transmute(eventdescriptor), eventmatchany.into_param().abi(), ::core::mem::transmute(payloadpredicatecount), ::core::mem::transmute(payloadpredicates), ::core::mem::transmute(payloadfilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhDeletePayloadFilter(payloadfilter: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(TdhDeletePayloadFilter(::core::mem::transmute(payloadfilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows::core::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateManifestProviderEvents(providerguid: *const ::windows::core::GUID, buffer: *mut PROVIDER_EVENT_INFO, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhEnumerateManifestProviderEvents(::core::mem::transmute(providerguid), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhEnumerateProviderFieldInformation(::core::mem::transmute(pguid), ::core::mem::transmute(eventfieldtype), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhEnumerateProviderFilters(guid: *const ::windows::core::GUID, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProviderFilters(guid: *const ::windows::core::GUID, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, filtercount: *mut u32, buffer: *mut *mut PROVIDER_FILTER_INFO, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhEnumerateProviderFilters(::core::mem::transmute(guid), ::core::mem::transmute(tdhcontextcount), ::core::mem::transmute(tdhcontext), ::core::mem::transmute(filtercount), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProviders(pbuffer: *mut PROVIDER_ENUMERATION_INFO, pbuffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhEnumerateProviders(::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhEnumerateProvidersForDecodingSource(filter: DECODING_SOURCE, buffer: *mut PROVIDER_ENUMERATION_INFO, buffersize: u32, bufferrequired: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhEnumerateProvidersForDecodingSource(::core::mem::transmute(filter), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(bufferrequired)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: super::super::super::Foundation::PWSTR, userdataconsumed: *mut u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhFormatProperty(eventinfo: *const TRACE_EVENT_INFO, mapinfo: *const EVENT_MAP_INFO, pointersize: u32, propertyintype: u16, propertyouttype: u16, propertylength: u16, userdatalength: u16, userdata: *const u8, buffersize: *mut u32, buffer: super::super::super::Foundation::PWSTR, userdataconsumed: *mut u16) -> u32;
        }
        ::core::mem::transmute(TdhFormatProperty(
            ::core::mem::transmute(eventinfo),
            ::core::mem::transmute(mapinfo),
            ::core::mem::transmute(pointersize),
            ::core::mem::transmute(propertyintype),
            ::core::mem::transmute(propertyouttype),
            ::core::mem::transmute(propertylength),
            ::core::mem::transmute(userdatalength),
            ::core::mem::transmute(userdata),
            ::core::mem::transmute(buffersize),
            ::core::mem::transmute(buffer),
            ::core::mem::transmute(userdataconsumed),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhGetDecodingParameter<'a, Param0: ::windows::core::IntoParam<'a, TDH_HANDLE>>(handle: Param0, tdhcontext: *mut TDH_CONTEXT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *mut TDH_CONTEXT) -> u32;
        }
        ::core::mem::transmute(TdhGetDecodingParameter(handle.into_param().abi(), ::core::mem::transmute(tdhcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetEventInformation(event: *const EVENT_RECORD, tdhcontextcount: u32, tdhcontext: *const TDH_CONTEXT, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhGetEventInformation(::core::mem::transmute(event), ::core::mem::transmute(tdhcontextcount), ::core::mem::transmute(tdhcontext), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhGetEventMapInformation<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pevent: *const EVENT_RECORD, pmapname: Param1, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetEventMapInformation(pevent: *const EVENT_RECORD, pmapname: super::super::super::Foundation::PWSTR, pbuffer: *mut EVENT_MAP_INFO, pbuffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhGetEventMapInformation(::core::mem::transmute(pevent), pmapname.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhGetManifestEventInformation(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetManifestEventInformation(providerguid: *const ::windows::core::GUID, eventdescriptor: *const EVENT_DESCRIPTOR, buffer: *mut TRACE_EVENT_INFO, buffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhGetManifestEventInformation(::core::mem::transmute(providerguid), ::core::mem::transmute(eventdescriptor), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhGetProperty(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, buffersize: u32, pbuffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetProperty(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, buffersize: u32, pbuffer: *mut u8) -> u32;
        }
        ::core::mem::transmute(TdhGetProperty(::core::mem::transmute(pevent), ::core::mem::transmute(tdhcontextcount), ::core::mem::transmute(ptdhcontext), ::core::mem::transmute(propertydatacount), ::core::mem::transmute(ppropertydata), ::core::mem::transmute(buffersize), ::core::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhGetPropertySize(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, ppropertysize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetPropertySize(pevent: *const EVENT_RECORD, tdhcontextcount: u32, ptdhcontext: *const TDH_CONTEXT, propertydatacount: u32, ppropertydata: *const PROPERTY_DATA_DESCRIPTOR, ppropertysize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhGetPropertySize(::core::mem::transmute(pevent), ::core::mem::transmute(tdhcontextcount), ::core::mem::transmute(ptdhcontext), ::core::mem::transmute(propertydatacount), ::core::mem::transmute(ppropertydata), ::core::mem::transmute(ppropertysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhGetWppMessage<'a, Param0: ::windows::core::IntoParam<'a, TDH_HANDLE>>(handle: Param0, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetWppMessage(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, buffersize: *mut u32, buffer: *mut u8) -> u32;
        }
        ::core::mem::transmute(TdhGetWppMessage(handle.into_param().abi(), ::core::mem::transmute(eventrecord), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhGetWppProperty<'a, Param0: ::windows::core::IntoParam<'a, TDH_HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(handle: Param0, eventrecord: *const EVENT_RECORD, propertyname: Param2, buffersize: *mut u32, buffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhGetWppProperty(handle: TDH_HANDLE, eventrecord: *const EVENT_RECORD, propertyname: super::super::super::Foundation::PWSTR, buffersize: *mut u32, buffer: *mut u8) -> u32;
        }
        ::core::mem::transmute(TdhGetWppProperty(handle.into_param().abi(), ::core::mem::transmute(eventrecord), propertyname.into_param().abi(), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhLoadManifest<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(manifest: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhLoadManifest(manifest: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(TdhLoadManifest(manifest.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhLoadManifestFromBinary<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(binarypath: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhLoadManifestFromBinary(binarypath: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(TdhLoadManifestFromBinary(binarypath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhLoadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhLoadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
        }
        ::core::mem::transmute(TdhLoadManifestFromMemory(::core::mem::transmute(pdata), ::core::mem::transmute(cbdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhOpenDecodingHandle(handle: *mut TDH_HANDLE) -> u32;
        }
        ::core::mem::transmute(TdhOpenDecodingHandle(::core::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhQueryProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhQueryProviderFieldInformation(pguid: *const ::windows::core::GUID, eventfieldvalue: u64, eventfieldtype: EVENT_FIELD_TYPE, pbuffer: *mut PROVIDER_FIELD_INFOARRAY, pbuffersize: *mut u32) -> u32;
        }
        ::core::mem::transmute(TdhQueryProviderFieldInformation(::core::mem::transmute(pguid), ::core::mem::transmute(eventfieldvalue), ::core::mem::transmute(eventfieldtype), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhSetDecodingParameter<'a, Param0: ::windows::core::IntoParam<'a, TDH_HANDLE>>(handle: Param0, tdhcontext: *const TDH_CONTEXT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhSetDecodingParameter(handle: TDH_HANDLE, tdhcontext: *const TDH_CONTEXT) -> u32;
        }
        ::core::mem::transmute(TdhSetDecodingParameter(handle.into_param().abi(), ::core::mem::transmute(tdhcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TdhUnloadManifest<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(manifest: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhUnloadManifest(manifest: super::super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(TdhUnloadManifest(manifest.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TdhUnloadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TdhUnloadManifestFromMemory(pdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
        }
        ::core::mem::transmute(TdhUnloadManifestFromMemory(::core::mem::transmute(pdata), ::core::mem::transmute(cbdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceEvent(tracehandle: u64, eventtrace: *const EVENT_TRACE_HEADER) -> u32;
        }
        ::core::mem::transmute(TraceEvent(::core::mem::transmute(tracehandle), ::core::mem::transmute(eventtrace)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceEventInstance(tracehandle: u64, eventtrace: *const EVENT_INSTANCE_HEADER, instinfo: *const EVENT_INSTANCE_INFO, parentinstinfo: *const EVENT_INSTANCE_INFO) -> u32;
        }
        ::core::mem::transmute(TraceEventInstance(::core::mem::transmute(tracehandle), ::core::mem::transmute(eventtrace), ::core::mem::transmute(instinfo), ::core::mem::transmute(parentinstinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceMessage(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16) -> u32;
        }
        ::core::mem::transmute(TraceMessage(::core::mem::transmute(loggerhandle), ::core::mem::transmute(messageflags), ::core::mem::transmute(messageguid), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16, messagearglist: *const i8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceMessageVa(loggerhandle: u64, messageflags: TRACE_MESSAGE_FLAGS, messageguid: *const ::windows::core::GUID, messagenumber: u16, messagearglist: *const i8) -> u32;
        }
        ::core::mem::transmute(TraceMessageVa(::core::mem::transmute(loggerhandle), ::core::mem::transmute(messageflags), ::core::mem::transmute(messageguid), ::core::mem::transmute(messagenumber), ::core::mem::transmute(messagearglist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::core::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceQueryInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *mut ::core::ffi::c_void, informationlength: u32, returnlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(TraceQueryInformation(::core::mem::transmute(sessionhandle), ::core::mem::transmute(informationclass), ::core::mem::transmute(traceinformation), ::core::mem::transmute(informationlength), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TraceSetInformation(sessionhandle: u64, informationclass: TRACE_QUERY_INFO_CLASS, traceinformation: *const ::core::ffi::c_void, informationlength: u32) -> u32;
        }
        ::core::mem::transmute(TraceSetInformation(::core::mem::transmute(sessionhandle), ::core::mem::transmute(informationclass), ::core::mem::transmute(traceinformation), ::core::mem::transmute(informationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterTraceGuids(registrationhandle: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterTraceGuids(registrationhandle: u64) -> u32;
        }
        ::core::mem::transmute(UnregisterTraceGuids(::core::mem::transmute(registrationhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateTraceA<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateTraceA(tracehandle: u64, instancename: super::super::super::Foundation::PSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(UpdateTraceA(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateTraceW<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(tracehandle: u64, instancename: Param1, properties: *mut EVENT_TRACE_PROPERTIES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateTraceW(tracehandle: u64, instancename: super::super::super::Foundation::PWSTR, properties: *mut EVENT_TRACE_PROPERTIES) -> u32;
        }
        ::core::mem::transmute(UpdateTraceW(::core::mem::transmute(tracehandle), instancename.into_param().abi(), ::core::mem::transmute(properties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type WMIDPREQUEST = unsafe extern "system" fn(requestcode: WMIDPREQUESTCODE, requestcontext: *const ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WMIDPREQUESTCODE(pub i32);
pub const WMI_GET_ALL_DATA: WMIDPREQUESTCODE = WMIDPREQUESTCODE(0i32);
pub const WMI_GET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(1i32);
pub const WMI_SET_SINGLE_INSTANCE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(2i32);
pub const WMI_SET_SINGLE_ITEM: WMIDPREQUESTCODE = WMIDPREQUESTCODE(3i32);
pub const WMI_ENABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(4i32);
pub const WMI_DISABLE_EVENTS: WMIDPREQUESTCODE = WMIDPREQUESTCODE(5i32);
pub const WMI_ENABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(6i32);
pub const WMI_DISABLE_COLLECTION: WMIDPREQUESTCODE = WMIDPREQUESTCODE(7i32);
pub const WMI_REGINFO: WMIDPREQUESTCODE = WMIDPREQUESTCODE(8i32);
pub const WMI_EXECUTE_METHOD: WMIDPREQUESTCODE = WMIDPREQUESTCODE(9i32);
pub const WMI_CAPTURE_STATE: WMIDPREQUESTCODE = WMIDPREQUESTCODE(10i32);
impl ::core::convert::From<i32> for WMIDPREQUESTCODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WMIDPREQUESTCODE {
    type Abi = Self;
}
pub const WMIGUID_EXECUTE: u32 = 16u32;
pub const WMIGUID_NOTIFICATION: u32 = 4u32;
pub const WMIGUID_QUERY: u32 = 1u32;
pub const WMIGUID_READ_DESCRIPTION: u32 = 8u32;
pub const WMIGUID_SET: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WMIREGGUIDW {
    pub Guid: ::windows::core::GUID,
    pub Flags: u32,
    pub InstanceCount: u32,
    pub Anonymous: WMIREGGUIDW_0,
}
impl WMIREGGUIDW {}
impl ::core::default::Default for WMIREGGUIDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMIREGGUIDW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WMIREGGUIDW {}
unsafe impl ::windows::core::Abi for WMIREGGUIDW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union WMIREGGUIDW_0 {
    pub InstanceNameList: u32,
    pub BaseNameOffset: u32,
    pub Pdo: usize,
    pub InstanceInfo: usize,
}
impl WMIREGGUIDW_0 {}
impl ::core::default::Default for WMIREGGUIDW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMIREGGUIDW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WMIREGGUIDW_0 {}
unsafe impl ::windows::core::Abi for WMIREGGUIDW_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WMIREGINFOW {
    pub BufferSize: u32,
    pub NextWmiRegInfo: u32,
    pub RegistryPath: u32,
    pub MofResourceName: u32,
    pub GuidCount: u32,
    pub WmiRegGuid: [WMIREGGUIDW; 1],
}
impl WMIREGINFOW {}
impl ::core::default::Default for WMIREGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMIREGINFOW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WMIREGINFOW {}
unsafe impl ::windows::core::Abi for WMIREGINFOW {
    type Abi = Self;
}
pub const WMIREG_FLAG_EVENT_ONLY_GUID: u32 = 64u32;
pub const WMIREG_FLAG_EXPENSIVE: u32 = 1u32;
pub const WMIREG_FLAG_INSTANCE_BASENAME: u32 = 8u32;
pub const WMIREG_FLAG_INSTANCE_LIST: u32 = 4u32;
pub const WMIREG_FLAG_INSTANCE_PDO: u32 = 32u32;
pub const WMIREG_FLAG_REMOVE_GUID: u32 = 65536u32;
pub const WMIREG_FLAG_RESERVED1: u32 = 131072u32;
pub const WMIREG_FLAG_RESERVED2: u32 = 262144u32;
pub const WMIREG_FLAG_TRACED_GUID: u32 = 524288u32;
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: u32 = 4096u32;
pub const WMI_GLOBAL_LOGGER_ID: u32 = 1u32;
pub const WMI_GUIDTYPE_DATA: u32 = 2u32;
pub const WMI_GUIDTYPE_EVENT: u32 = 3u32;
pub const WMI_GUIDTYPE_TRACE: u32 = 1u32;
pub const WMI_GUIDTYPE_TRACECONTROL: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_ALL_DATA {
    pub WnodeHeader: WNODE_HEADER,
    pub DataBlockOffset: u32,
    pub InstanceCount: u32,
    pub OffsetInstanceNameOffsets: u32,
    pub Anonymous: WNODE_ALL_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_ALL_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_ALL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_ALL_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_ALL_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_ALL_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_ALL_DATA_0 {
    pub FixedInstanceSize: u32,
    pub OffsetInstanceDataAndLength: [OFFSETINSTANCEDATAANDLENGTH; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_ALL_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_ALL_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_ALL_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_ALL_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_ALL_DATA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_EVENT_ITEM {
    pub WnodeHeader: WNODE_HEADER,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_EVENT_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_EVENT_ITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_EVENT_ITEM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_EVENT_REFERENCE {
    pub WnodeHeader: WNODE_HEADER,
    pub TargetGuid: ::windows::core::GUID,
    pub TargetDataBlockSize: u32,
    pub Anonymous: WNODE_EVENT_REFERENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_EVENT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_EVENT_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_EVENT_REFERENCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_EVENT_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_EVENT_REFERENCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_EVENT_REFERENCE_0 {
    pub TargetInstanceIndex: u32,
    pub TargetInstanceName: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_EVENT_REFERENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_EVENT_REFERENCE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_EVENT_REFERENCE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_EVENT_REFERENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_EVENT_REFERENCE_0 {
    type Abi = Self;
}
pub const WNODE_FLAG_ALL_DATA: u32 = 1u32;
pub const WNODE_FLAG_ANSI_INSTANCENAMES: u32 = 16384u32;
pub const WNODE_FLAG_EVENT_ITEM: u32 = 8u32;
pub const WNODE_FLAG_EVENT_REFERENCE: u32 = 8192u32;
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: u32 = 16u32;
pub const WNODE_FLAG_INSTANCES_SAME: u32 = 64u32;
pub const WNODE_FLAG_INTERNAL: u32 = 256u32;
pub const WNODE_FLAG_LOG_WNODE: u32 = 262144u32;
pub const WNODE_FLAG_METHOD_ITEM: u32 = 32768u32;
pub const WNODE_FLAG_NO_HEADER: u32 = 2097152u32;
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: u32 = 65536u32;
pub const WNODE_FLAG_PERSIST_EVENT: u32 = 1024u32;
pub const WNODE_FLAG_SEND_DATA_BLOCK: u32 = 4194304u32;
pub const WNODE_FLAG_SEVERITY_MASK: u32 = 4278190080u32;
pub const WNODE_FLAG_SINGLE_INSTANCE: u32 = 2u32;
pub const WNODE_FLAG_SINGLE_ITEM: u32 = 4u32;
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: u32 = 128u32;
pub const WNODE_FLAG_TOO_SMALL: u32 = 32u32;
pub const WNODE_FLAG_TRACED_GUID: u32 = 131072u32;
pub const WNODE_FLAG_USE_GUID_PTR: u32 = 524288u32;
pub const WNODE_FLAG_USE_MOF_PTR: u32 = 1048576u32;
pub const WNODE_FLAG_USE_TIMESTAMP: u32 = 512u32;
pub const WNODE_FLAG_VERSIONED_PROPERTIES: u32 = 8388608u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous1: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: ::windows::core::GUID,
    pub ClientContext: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_HEADER_0 {
    pub HistoricalContext: u64,
    pub Anonymous: WNODE_HEADER_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_HEADER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WNODE_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Version", &self.Version).field("Linkage", &self.Linkage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Linkage == other.Linkage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: super::super::super::Foundation::HANDLE,
    pub TimeStamp: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_HEADER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_HEADER_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_HEADER_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_METHOD_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub MethodId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_METHOD_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_METHOD_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_METHOD_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_METHOD_ITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_METHOD_ITEM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_SINGLE_INSTANCE {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_SINGLE_INSTANCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_SINGLE_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_SINGLE_INSTANCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_SINGLE_INSTANCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_SINGLE_INSTANCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_SINGLE_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub ItemId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataItem: u32,
    pub VariableData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_SINGLE_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_SINGLE_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_SINGLE_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_SINGLE_ITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_SINGLE_ITEM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WNODE_TOO_SMALL {
    pub WnodeHeader: WNODE_HEADER,
    pub SizeNeeded: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WNODE_TOO_SMALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNODE_TOO_SMALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNODE_TOO_SMALL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNODE_TOO_SMALL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WNODE_TOO_SMALL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _TDH_IN_TYPE(pub i32);
pub const TDH_INTYPE_NULL: _TDH_IN_TYPE = _TDH_IN_TYPE(0i32);
pub const TDH_INTYPE_UNICODESTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(1i32);
pub const TDH_INTYPE_ANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(2i32);
pub const TDH_INTYPE_INT8: _TDH_IN_TYPE = _TDH_IN_TYPE(3i32);
pub const TDH_INTYPE_UINT8: _TDH_IN_TYPE = _TDH_IN_TYPE(4i32);
pub const TDH_INTYPE_INT16: _TDH_IN_TYPE = _TDH_IN_TYPE(5i32);
pub const TDH_INTYPE_UINT16: _TDH_IN_TYPE = _TDH_IN_TYPE(6i32);
pub const TDH_INTYPE_INT32: _TDH_IN_TYPE = _TDH_IN_TYPE(7i32);
pub const TDH_INTYPE_UINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(8i32);
pub const TDH_INTYPE_INT64: _TDH_IN_TYPE = _TDH_IN_TYPE(9i32);
pub const TDH_INTYPE_UINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(10i32);
pub const TDH_INTYPE_FLOAT: _TDH_IN_TYPE = _TDH_IN_TYPE(11i32);
pub const TDH_INTYPE_DOUBLE: _TDH_IN_TYPE = _TDH_IN_TYPE(12i32);
pub const TDH_INTYPE_BOOLEAN: _TDH_IN_TYPE = _TDH_IN_TYPE(13i32);
pub const TDH_INTYPE_BINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(14i32);
pub const TDH_INTYPE_GUID: _TDH_IN_TYPE = _TDH_IN_TYPE(15i32);
pub const TDH_INTYPE_POINTER: _TDH_IN_TYPE = _TDH_IN_TYPE(16i32);
pub const TDH_INTYPE_FILETIME: _TDH_IN_TYPE = _TDH_IN_TYPE(17i32);
pub const TDH_INTYPE_SYSTEMTIME: _TDH_IN_TYPE = _TDH_IN_TYPE(18i32);
pub const TDH_INTYPE_SID: _TDH_IN_TYPE = _TDH_IN_TYPE(19i32);
pub const TDH_INTYPE_HEXINT32: _TDH_IN_TYPE = _TDH_IN_TYPE(20i32);
pub const TDH_INTYPE_HEXINT64: _TDH_IN_TYPE = _TDH_IN_TYPE(21i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(22i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(23i32);
pub const TDH_INTYPE_RESERVED24: _TDH_IN_TYPE = _TDH_IN_TYPE(24i32);
pub const TDH_INTYPE_MANIFEST_COUNTEDBINARY: _TDH_IN_TYPE = _TDH_IN_TYPE(25i32);
pub const TDH_INTYPE_COUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(300i32);
pub const TDH_INTYPE_COUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(301i32);
pub const TDH_INTYPE_REVERSEDCOUNTEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(302i32);
pub const TDH_INTYPE_REVERSEDCOUNTEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(303i32);
pub const TDH_INTYPE_NONNULLTERMINATEDSTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(304i32);
pub const TDH_INTYPE_NONNULLTERMINATEDANSISTRING: _TDH_IN_TYPE = _TDH_IN_TYPE(305i32);
pub const TDH_INTYPE_UNICODECHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(306i32);
pub const TDH_INTYPE_ANSICHAR: _TDH_IN_TYPE = _TDH_IN_TYPE(307i32);
pub const TDH_INTYPE_SIZET: _TDH_IN_TYPE = _TDH_IN_TYPE(308i32);
pub const TDH_INTYPE_HEXDUMP: _TDH_IN_TYPE = _TDH_IN_TYPE(309i32);
pub const TDH_INTYPE_WBEMSID: _TDH_IN_TYPE = _TDH_IN_TYPE(310i32);
impl ::core::convert::From<i32> for _TDH_IN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _TDH_IN_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _TDH_OUT_TYPE(pub i32);
pub const TDH_OUTTYPE_NULL: _TDH_OUT_TYPE = _TDH_OUT_TYPE(0i32);
pub const TDH_OUTTYPE_STRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(1i32);
pub const TDH_OUTTYPE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(2i32);
pub const TDH_OUTTYPE_BYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(3i32);
pub const TDH_OUTTYPE_UNSIGNEDBYTE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(4i32);
pub const TDH_OUTTYPE_SHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(5i32);
pub const TDH_OUTTYPE_UNSIGNEDSHORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(6i32);
pub const TDH_OUTTYPE_INT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(7i32);
pub const TDH_OUTTYPE_UNSIGNEDINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(8i32);
pub const TDH_OUTTYPE_LONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(9i32);
pub const TDH_OUTTYPE_UNSIGNEDLONG: _TDH_OUT_TYPE = _TDH_OUT_TYPE(10i32);
pub const TDH_OUTTYPE_FLOAT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(11i32);
pub const TDH_OUTTYPE_DOUBLE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(12i32);
pub const TDH_OUTTYPE_BOOLEAN: _TDH_OUT_TYPE = _TDH_OUT_TYPE(13i32);
pub const TDH_OUTTYPE_GUID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(14i32);
pub const TDH_OUTTYPE_HEXBINARY: _TDH_OUT_TYPE = _TDH_OUT_TYPE(15i32);
pub const TDH_OUTTYPE_HEXINT8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(16i32);
pub const TDH_OUTTYPE_HEXINT16: _TDH_OUT_TYPE = _TDH_OUT_TYPE(17i32);
pub const TDH_OUTTYPE_HEXINT32: _TDH_OUT_TYPE = _TDH_OUT_TYPE(18i32);
pub const TDH_OUTTYPE_HEXINT64: _TDH_OUT_TYPE = _TDH_OUT_TYPE(19i32);
pub const TDH_OUTTYPE_PID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(20i32);
pub const TDH_OUTTYPE_TID: _TDH_OUT_TYPE = _TDH_OUT_TYPE(21i32);
pub const TDH_OUTTYPE_PORT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(22i32);
pub const TDH_OUTTYPE_IPV4: _TDH_OUT_TYPE = _TDH_OUT_TYPE(23i32);
pub const TDH_OUTTYPE_IPV6: _TDH_OUT_TYPE = _TDH_OUT_TYPE(24i32);
pub const TDH_OUTTYPE_SOCKETADDRESS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(25i32);
pub const TDH_OUTTYPE_CIMDATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(26i32);
pub const TDH_OUTTYPE_ETWTIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(27i32);
pub const TDH_OUTTYPE_XML: _TDH_OUT_TYPE = _TDH_OUT_TYPE(28i32);
pub const TDH_OUTTYPE_ERRORCODE: _TDH_OUT_TYPE = _TDH_OUT_TYPE(29i32);
pub const TDH_OUTTYPE_WIN32ERROR: _TDH_OUT_TYPE = _TDH_OUT_TYPE(30i32);
pub const TDH_OUTTYPE_NTSTATUS: _TDH_OUT_TYPE = _TDH_OUT_TYPE(31i32);
pub const TDH_OUTTYPE_HRESULT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(32i32);
pub const TDH_OUTTYPE_CULTURE_INSENSITIVE_DATETIME: _TDH_OUT_TYPE = _TDH_OUT_TYPE(33i32);
pub const TDH_OUTTYPE_JSON: _TDH_OUT_TYPE = _TDH_OUT_TYPE(34i32);
pub const TDH_OUTTYPE_UTF8: _TDH_OUT_TYPE = _TDH_OUT_TYPE(35i32);
pub const TDH_OUTTYPE_PKCS7_WITH_TYPE_INFO: _TDH_OUT_TYPE = _TDH_OUT_TYPE(36i32);
pub const TDH_OUTTYPE_CODE_POINTER: _TDH_OUT_TYPE = _TDH_OUT_TYPE(37i32);
pub const TDH_OUTTYPE_DATETIME_UTC: _TDH_OUT_TYPE = _TDH_OUT_TYPE(38i32);
pub const TDH_OUTTYPE_REDUCEDSTRING: _TDH_OUT_TYPE = _TDH_OUT_TYPE(300i32);
pub const TDH_OUTTYPE_NOPRINT: _TDH_OUT_TYPE = _TDH_OUT_TYPE(301i32);
impl ::core::convert::From<i32> for _TDH_OUT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _TDH_OUT_TYPE {
    type Abi = Self;
}
