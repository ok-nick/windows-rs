#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CACHE_DESCRIPTOR {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub Size: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
}
impl CACHE_DESCRIPTOR {}
impl ::core::default::Default for CACHE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CACHE_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CACHE_DESCRIPTOR").field("Level", &self.Level).field("Associativity", &self.Associativity).field("LineSize", &self.LineSize).field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::core::cmp::PartialEq for CACHE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Associativity == other.Associativity && self.LineSize == other.LineSize && self.Size == other.Size && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for CACHE_DESCRIPTOR {}
unsafe impl ::windows::core::Abi for CACHE_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CACHE_RELATIONSHIP {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub CacheSize: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: CACHE_RELATIONSHIP_0,
}
impl CACHE_RELATIONSHIP {}
impl ::core::default::Default for CACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CACHE_RELATIONSHIP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for CACHE_RELATIONSHIP {}
unsafe impl ::windows::core::Abi for CACHE_RELATIONSHIP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union CACHE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl CACHE_RELATIONSHIP_0 {}
impl ::core::default::Default for CACHE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CACHE_RELATIONSHIP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for CACHE_RELATIONSHIP_0 {}
unsafe impl ::windows::core::Abi for CACHE_RELATIONSHIP_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPUTER_NAME_FORMAT(pub i32);
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(0i32);
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(1i32);
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(2i32);
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(3i32);
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(4i32);
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(5i32);
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(6i32);
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(7i32);
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(8i32);
impl ::core::convert::From<i32> for COMPUTER_NAME_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COMPUTER_NAME_FORMAT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CPU_SET_INFORMATION_TYPE(pub i32);
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = CPU_SET_INFORMATION_TYPE(0i32);
impl ::core::convert::From<i32> for CPU_SET_INFORMATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CPU_SET_INFORMATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEP_SYSTEM_POLICY_TYPE(pub i32);
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(0i32);
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(1i32);
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(2i32);
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(3i32);
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(4i32);
impl ::core::convert::From<i32> for DEP_SYSTEM_POLICY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DEP_SYSTEM_POLICY_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEVICEFAMILYDEVICEFORM(pub u32);
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(0u32);
pub const DEVICEFAMILYDEVICEFORM_PHONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(1u32);
pub const DEVICEFAMILYDEVICEFORM_TABLET: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(2u32);
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(3u32);
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(4u32);
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(5u32);
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(6u32);
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(7u32);
pub const DEVICEFAMILYDEVICEFORM_STICKPC: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(8u32);
pub const DEVICEFAMILYDEVICEFORM_PUCK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(9u32);
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(10u32);
pub const DEVICEFAMILYDEVICEFORM_HMD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(11u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(12u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(13u32);
pub const DEVICEFAMILYDEVICEFORM_BANKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(14u32);
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(15u32);
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(16u32);
pub const DEVICEFAMILYDEVICEFORM_GAMING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(17u32);
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(18u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(19u32);
pub const DEVICEFAMILYDEVICEFORM_KIOSK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(20u32);
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(21u32);
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(22u32);
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(23u32);
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(24u32);
pub const DEVICEFAMILYDEVICEFORM_PRINTING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(25u32);
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(26u32);
pub const DEVICEFAMILYDEVICEFORM_TOY: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(27u32);
pub const DEVICEFAMILYDEVICEFORM_VENDING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(28u32);
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(29u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(30u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(31u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(32u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(33u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(34u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(35u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_00: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(36u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(37u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(38u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(39u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(40u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(41u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(42u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(43u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(44u32);
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(45u32);
pub const DEVICEFAMILYDEVICEFORM_MAX: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(45u32);
impl ::core::convert::From<u32> for DEVICEFAMILYDEVICEFORM {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DEVICEFAMILYDEVICEFORM {
    type Abi = Self;
}
impl ::core::ops::BitOr for DEVICEFAMILYDEVICEFORM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DEVICEFAMILYDEVICEFORM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DEVICEFAMILYDEVICEFORM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DEVICEFAMILYDEVICEFORM {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DEVICEFAMILYDEVICEFORM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEVICEFAMILYINFOENUM(pub u32);
pub const DEVICEFAMILYINFOENUM_UAP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(0u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(1u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(2u32);
pub const DEVICEFAMILYINFOENUM_DESKTOP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(3u32);
pub const DEVICEFAMILYINFOENUM_MOBILE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(4u32);
pub const DEVICEFAMILYINFOENUM_XBOX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(5u32);
pub const DEVICEFAMILYINFOENUM_TEAM: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(6u32);
pub const DEVICEFAMILYINFOENUM_IOT: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(7u32);
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(8u32);
pub const DEVICEFAMILYINFOENUM_SERVER: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(9u32);
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(10u32);
pub const DEVICEFAMILYINFOENUM_XBOXSRA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(11u32);
pub const DEVICEFAMILYINFOENUM_XBOXERA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(12u32);
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(13u32);
pub const DEVICEFAMILYINFOENUM_8828080: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(14u32);
pub const DEVICEFAMILYINFOENUM_7067329: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(15u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(16u32);
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(17u32);
pub const DEVICEFAMILYINFOENUM_MAX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(17u32);
impl ::core::convert::From<u32> for DEVICEFAMILYINFOENUM {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DEVICEFAMILYINFOENUM {
    type Abi = Self;
}
impl ::core::ops::BitOr for DEVICEFAMILYINFOENUM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DEVICEFAMILYINFOENUM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DEVICEFAMILYINFOENUM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DEVICEFAMILYINFOENUM {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DEVICEFAMILYINFOENUM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DnsHostnameToComputerNameExW(hostname: super::super::Foundation::PWSTR, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DnsHostnameToComputerNameExW(hostname.into_param().abi(), ::core::mem::transmute(computername), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: *mut FIRMWARE_TABLE_ID, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: *mut FIRMWARE_TABLE_ID, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(EnumSystemFirmwareTables(::core::mem::transmute(firmwaretableprovidersignature), ::core::mem::transmute(pfirmwaretableenumbuffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct FIRMWARE_TABLE_ID(pub u32);
impl ::core::default::Default for FIRMWARE_TABLE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for FIRMWARE_TABLE_ID {}
unsafe impl ::windows::core::Abi for FIRMWARE_TABLE_ID {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FIRMWARE_TABLE_PROVIDER(pub u32);
pub const ACPI: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1094930505u32);
pub const FIRM: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1179210317u32);
pub const RSMB: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1381190978u32);
impl ::core::convert::From<u32> for FIRMWARE_TABLE_PROVIDER {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FIRMWARE_TABLE_PROVIDER {
    type Abi = Self;
}
impl ::core::ops::BitOr for FIRMWARE_TABLE_PROVIDER {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for FIRMWARE_TABLE_PROVIDER {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for FIRMWARE_TABLE_PROVIDER {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for FIRMWARE_TABLE_PROVIDER {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for FIRMWARE_TABLE_PROVIDER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FIRMWARE_TYPE(pub i32);
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = FIRMWARE_TYPE(0i32);
pub const FirmwareTypeBios: FIRMWARE_TYPE = FIRMWARE_TYPE(1i32);
pub const FirmwareTypeUefi: FIRMWARE_TYPE = FIRMWARE_TYPE(2i32);
pub const FirmwareTypeMax: FIRMWARE_TYPE = FIRMWARE_TYPE(3i32);
impl ::core::convert::From<i32> for FIRMWARE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FIRMWARE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GROUP_AFFINITY {
    pub Mask: usize,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl GROUP_AFFINITY {}
impl ::core::default::Default for GROUP_AFFINITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GROUP_AFFINITY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GROUP_AFFINITY").field("Mask", &self.Mask).field("Group", &self.Group).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::cmp::PartialEq for GROUP_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Group == other.Group && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for GROUP_AFFINITY {}
unsafe impl ::windows::core::Abi for GROUP_AFFINITY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: u16,
    pub ActiveGroupCount: u16,
    pub Reserved: [u8; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
}
impl GROUP_RELATIONSHIP {}
impl ::core::default::Default for GROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GROUP_RELATIONSHIP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GROUP_RELATIONSHIP").field("MaximumGroupCount", &self.MaximumGroupCount).field("ActiveGroupCount", &self.ActiveGroupCount).field("Reserved", &self.Reserved).field("GroupInfo", &self.GroupInfo).finish()
    }
}
impl ::core::cmp::PartialEq for GROUP_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumGroupCount == other.MaximumGroupCount && self.ActiveGroupCount == other.ActiveGroupCount && self.Reserved == other.Reserved && self.GroupInfo == other.GroupInfo
    }
}
impl ::core::cmp::Eq for GROUP_RELATIONSHIP {}
unsafe impl ::windows::core::Abi for GROUP_RELATIONSHIP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetComputerNameExA(::core::mem::transmute(nametype), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetComputerNameExW(::core::mem::transmute(nametype), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetFirmwareType(::core::mem::transmute(firmwaretype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetIntegratedDisplaySize() -> ::windows::core::Result<f64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIntegratedDisplaySize(sizeininches: *mut f64) -> ::windows::core::HRESULT;
        }
        let mut result__: <f64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetIntegratedDisplaySize(&mut result__).from_abi::<f64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLocalTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLocalTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
        }
        ::core::mem::transmute(GetLocalTime(::core::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalProcessorInformation(buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogicalProcessorInformation(buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetLogicalProcessorInformation(::core::mem::transmute(buffer), ::core::mem::transmute(returnedlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetLogicalProcessorInformationEx(::core::mem::transmute(relationshiptype), ::core::mem::transmute(buffer), ::core::mem::transmute(returnedlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
        }
        ::core::mem::transmute(GetNativeSystemInfo(::core::mem::transmute(lpsysteminfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOsManufacturingMode(::core::mem::transmute(pbenabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOsSafeBootMode(::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetPhysicallyInstalledSystemMemory(::core::mem::transmute(totalmemoryinkilobytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessorSystemCycleTime(group: u16, buffer: *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessorSystemCycleTime(group: u16, buffer: *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessorSystemCycleTime(::core::mem::transmute(group), ::core::mem::transmute(buffer), ::core::mem::transmute(returnedlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProductInfo(::core::mem::transmute(dwosmajorversion), ::core::mem::transmute(dwosminorversion), ::core::mem::transmute(dwspmajorversion), ::core::mem::transmute(dwspminorversion), ::core::mem::transmute(pdwreturnedproducttype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemCpuSetInformation<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(information: *mut SYSTEM_CPU_SET_INFORMATION, bufferlength: u32, returnedlength: *mut u32, process: Param3, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemCpuSetInformation(information: *mut SYSTEM_CPU_SET_INFORMATION, bufferlength: u32, returnedlength: *mut u32, process: super::super::Foundation::HANDLE, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemCpuSetInformation(::core::mem::transmute(information), ::core::mem::transmute(bufferlength), ::core::mem::transmute(returnedlength), process.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE;
        }
        ::core::mem::transmute(GetSystemDEPPolicy())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetSystemDirectoryA(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetSystemDirectoryW(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetSystemFirmwareTable<'a, Param1: ::windows::core::IntoParam<'a, FIRMWARE_TABLE_ID>>(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: Param1, pfirmwaretablebuffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemFirmwareTable(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: FIRMWARE_TABLE_ID, pfirmwaretablebuffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
        }
        ::core::mem::transmute(GetSystemFirmwareTable(::core::mem::transmute(firmwaretableprovidersignature), firmwaretableid.into_param().abi(), ::core::mem::transmute(pfirmwaretablebuffer), ::core::mem::transmute(buffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
        }
        ::core::mem::transmute(GetSystemInfo(::core::mem::transmute(lpsysteminfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemLeapSecondInformation(::core::mem::transmute(enabled), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
        }
        ::core::mem::transmute(GetSystemTime(::core::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemTimeAdjustment(::core::mem::transmute(lptimeadjustment), ::core::mem::transmute(lptimeincrement), ::core::mem::transmute(lptimeadjustmentdisabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemTimeAdjustmentPrecise(::core::mem::transmute(lptimeadjustment), ::core::mem::transmute(lptimeincrement), ::core::mem::transmute(lptimeadjustmentdisabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
        }
        ::core::mem::transmute(GetSystemTimeAsFileTime(::core::mem::transmute(lpsystemtimeasfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
        }
        ::core::mem::transmute(GetSystemTimePreciseAsFileTime(::core::mem::transmute(lpsystemtimeasfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetSystemWindowsDirectoryA(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetSystemWindowsDirectoryW(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64Directory2A(lpbuffer: super::super::Foundation::PSTR, usize: u32, imagefilemachinetype: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64Directory2A(lpbuffer: super::super::Foundation::PSTR, usize: u32, imagefilemachinetype: u16) -> u32;
        }
        ::core::mem::transmute(GetSystemWow64Directory2A(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize), ::core::mem::transmute(imagefilemachinetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64Directory2W(lpbuffer: super::super::Foundation::PWSTR, usize: u32, imagefilemachinetype: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64Directory2W(lpbuffer: super::super::Foundation::PWSTR, usize: u32, imagefilemachinetype: u16) -> u32;
        }
        ::core::mem::transmute(GetSystemWow64Directory2W(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize), ::core::mem::transmute(imagefilemachinetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64DirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64DirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetSystemWow64DirectoryA(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemWow64DirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemWow64DirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetSystemWow64DirectoryW(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTickCount() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTickCount() -> u32;
        }
        ::core::mem::transmute(GetTickCount())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTickCount64() -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTickCount64() -> u64;
        }
        ::core::mem::transmute(GetTickCount64())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersion() -> u32;
        }
        ::core::mem::transmute(GetVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetVersionExA(::core::mem::transmute(lpversioninformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetVersionExW(::core::mem::transmute(lpversioninformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetWindowsDirectoryA(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
        }
        ::core::mem::transmute(GetWindowsDirectoryW(::core::mem::transmute(lpbuffer), ::core::mem::transmute(usize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS);
        }
        ::core::mem::transmute(GlobalMemoryStatus(::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GlobalMemoryStatusEx(::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsUserCetAvailableInEnvironment(::core::mem::transmute(usercetenvironment)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWow64GuestMachineSupported(wowguestmachine: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWow64GuestMachineSupported(wowguestmachine: u16, machineissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IsWow64GuestMachineSupported(::core::mem::transmute(wowguestmachine), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOGICAL_PROCESSOR_RELATIONSHIP(pub i32);
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(0i32);
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(1i32);
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(2i32);
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(3i32);
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(4i32);
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(5i32);
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(6i32);
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(7i32);
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(65535i32);
impl ::core::convert::From<i32> for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LOGICAL_PROCESSOR_RELATIONSHIP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: usize,
    pub dwAvailPhys: usize,
    pub dwTotalPageFile: usize,
    pub dwAvailPageFile: usize,
    pub dwTotalVirtual: usize,
    pub dwAvailVirtual: usize,
}
impl MEMORYSTATUS {}
impl ::core::default::Default for MEMORYSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MEMORYSTATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MEMORYSTATUS")
            .field("dwLength", &self.dwLength)
            .field("dwMemoryLoad", &self.dwMemoryLoad)
            .field("dwTotalPhys", &self.dwTotalPhys)
            .field("dwAvailPhys", &self.dwAvailPhys)
            .field("dwTotalPageFile", &self.dwTotalPageFile)
            .field("dwAvailPageFile", &self.dwAvailPageFile)
            .field("dwTotalVirtual", &self.dwTotalVirtual)
            .field("dwAvailVirtual", &self.dwAvailVirtual)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORYSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.dwTotalPhys == other.dwTotalPhys && self.dwAvailPhys == other.dwAvailPhys && self.dwTotalPageFile == other.dwTotalPageFile && self.dwAvailPageFile == other.dwAvailPageFile && self.dwTotalVirtual == other.dwTotalVirtual && self.dwAvailVirtual == other.dwAvailVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUS {}
unsafe impl ::windows::core::Abi for MEMORYSTATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MEMORYSTATUSEX {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub ullTotalPhys: u64,
    pub ullAvailPhys: u64,
    pub ullTotalPageFile: u64,
    pub ullAvailPageFile: u64,
    pub ullTotalVirtual: u64,
    pub ullAvailVirtual: u64,
    pub ullAvailExtendedVirtual: u64,
}
impl MEMORYSTATUSEX {}
impl ::core::default::Default for MEMORYSTATUSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MEMORYSTATUSEX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MEMORYSTATUSEX")
            .field("dwLength", &self.dwLength)
            .field("dwMemoryLoad", &self.dwMemoryLoad)
            .field("ullTotalPhys", &self.ullTotalPhys)
            .field("ullAvailPhys", &self.ullAvailPhys)
            .field("ullTotalPageFile", &self.ullTotalPageFile)
            .field("ullAvailPageFile", &self.ullAvailPageFile)
            .field("ullTotalVirtual", &self.ullTotalVirtual)
            .field("ullAvailVirtual", &self.ullAvailVirtual)
            .field("ullAvailExtendedVirtual", &self.ullAvailExtendedVirtual)
            .finish()
    }
}
impl ::core::cmp::PartialEq for MEMORYSTATUSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.ullTotalPhys == other.ullTotalPhys && self.ullAvailPhys == other.ullAvailPhys && self.ullTotalPageFile == other.ullTotalPageFile && self.ullAvailPageFile == other.ullAvailPageFile && self.ullTotalVirtual == other.ullTotalVirtual && self.ullAvailVirtual == other.ullAvailVirtual && self.ullAvailExtendedVirtual == other.ullAvailExtendedVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUSEX {}
unsafe impl ::windows::core::Abi for MEMORYSTATUSEX {
    type Abi = Self;
}
pub const NTDDI_LONGHORN: u32 = 100663296u32;
pub const NTDDI_VERSION: u32 = 167772171u32;
pub const NTDDI_VISTA: u32 = 100663296u32;
pub const NTDDI_VISTASP1: u32 = 100663552u32;
pub const NTDDI_VISTASP2: u32 = 100663808u32;
pub const NTDDI_VISTASP3: u32 = 100664064u32;
pub const NTDDI_VISTASP4: u32 = 100664320u32;
pub const NTDDI_WIN10: u32 = 167772160u32;
pub const NTDDI_WIN10_19H1: u32 = 167772167u32;
pub const NTDDI_WIN10_CO: u32 = 167772171u32;
pub const NTDDI_WIN10_FE: u32 = 167772170u32;
pub const NTDDI_WIN10_MN: u32 = 167772169u32;
pub const NTDDI_WIN10_RS1: u32 = 167772162u32;
pub const NTDDI_WIN10_RS2: u32 = 167772163u32;
pub const NTDDI_WIN10_RS3: u32 = 167772164u32;
pub const NTDDI_WIN10_RS4: u32 = 167772165u32;
pub const NTDDI_WIN10_RS5: u32 = 167772166u32;
pub const NTDDI_WIN10_TH2: u32 = 167772161u32;
pub const NTDDI_WIN10_VB: u32 = 167772168u32;
pub const NTDDI_WIN2K: u32 = 83886080u32;
pub const NTDDI_WIN2KSP1: u32 = 83886336u32;
pub const NTDDI_WIN2KSP2: u32 = 83886592u32;
pub const NTDDI_WIN2KSP3: u32 = 83886848u32;
pub const NTDDI_WIN2KSP4: u32 = 83887104u32;
pub const NTDDI_WIN4: u32 = 67108864u32;
pub const NTDDI_WIN6: u32 = 100663296u32;
pub const NTDDI_WIN6SP1: u32 = 100663552u32;
pub const NTDDI_WIN6SP2: u32 = 100663808u32;
pub const NTDDI_WIN6SP3: u32 = 100664064u32;
pub const NTDDI_WIN6SP4: u32 = 100664320u32;
pub const NTDDI_WIN7: u32 = 100728832u32;
pub const NTDDI_WIN8: u32 = 100794368u32;
pub const NTDDI_WINBLUE: u32 = 100859904u32;
pub const NTDDI_WINTHRESHOLD: u32 = 167772160u32;
pub const NTDDI_WINXP: u32 = 83951616u32;
pub const NTDDI_WINXPSP1: u32 = 83951872u32;
pub const NTDDI_WINXPSP2: u32 = 83952128u32;
pub const NTDDI_WINXPSP3: u32 = 83952384u32;
pub const NTDDI_WINXPSP4: u32 = 83952640u32;
pub const NTDDI_WS03: u32 = 84017152u32;
pub const NTDDI_WS03SP1: u32 = 84017408u32;
pub const NTDDI_WS03SP2: u32 = 84017664u32;
pub const NTDDI_WS03SP3: u32 = 84017920u32;
pub const NTDDI_WS03SP4: u32 = 84018176u32;
pub const NTDDI_WS08: u32 = 100663552u32;
pub const NTDDI_WS08SP2: u32 = 100663808u32;
pub const NTDDI_WS08SP3: u32 = 100664064u32;
pub const NTDDI_WS08SP4: u32 = 100664320u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: u32,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: NUMA_NODE_RELATIONSHIP_0,
}
impl NUMA_NODE_RELATIONSHIP {}
impl ::core::default::Default for NUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NUMA_NODE_RELATIONSHIP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for NUMA_NODE_RELATIONSHIP {}
unsafe impl ::windows::core::Abi for NUMA_NODE_RELATIONSHIP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union NUMA_NODE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl NUMA_NODE_RELATIONSHIP_0 {}
impl ::core::default::Default for NUMA_NODE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NUMA_NODE_RELATIONSHIP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for NUMA_NODE_RELATIONSHIP_0 {}
unsafe impl ::windows::core::Abi for NUMA_NODE_RELATIONSHIP_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl OSVERSIONINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OSVERSIONINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OSVERSIONINFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OSVERSIONINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OSVERSIONINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OSVERSIONINFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl OSVERSIONINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OSVERSIONINFOEXA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOEXA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OSVERSIONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
            && self.wServicePackMajor == other.wServicePackMajor
            && self.wServicePackMinor == other.wServicePackMinor
            && self.wSuiteMask == other.wSuiteMask
            && self.wProductType == other.wProductType
            && self.wReserved == other.wReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OSVERSIONINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OSVERSIONINFOEXA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl OSVERSIONINFOEXW {}
impl ::core::default::Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOEXW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOEXW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize
            && self.dwMajorVersion == other.dwMajorVersion
            && self.dwMinorVersion == other.dwMinorVersion
            && self.dwBuildNumber == other.dwBuildNumber
            && self.dwPlatformId == other.dwPlatformId
            && self.szCSDVersion == other.szCSDVersion
            && self.wServicePackMajor == other.wServicePackMajor
            && self.wServicePackMinor == other.wServicePackMinor
            && self.wSuiteMask == other.wSuiteMask
            && self.wProductType == other.wProductType
            && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOEXW {}
unsafe impl ::windows::core::Abi for OSVERSIONINFOEXW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl OSVERSIONINFOW {}
impl ::core::default::Default for OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OSVERSIONINFOW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .finish()
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOW {}
unsafe impl ::windows::core::Abi for OSVERSIONINFOW {
    type Abi = Self;
}
pub const OSVERSION_MASK: u32 = 4294901760u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OS_DEPLOYEMENT_STATE_VALUES(pub i32);
pub const OS_DEPLOYMENT_STANDARD: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(1i32);
pub const OS_DEPLOYMENT_COMPACT: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(2i32);
impl ::core::convert::From<i32> for OS_DEPLOYEMENT_STATE_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OS_DEPLOYEMENT_STATE_VALUES {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OS_PRODUCT_TYPE(pub u32);
pub const PRODUCT_BUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(6u32);
pub const PRODUCT_BUSINESS_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(16u32);
pub const PRODUCT_CLUSTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(18u32);
pub const PRODUCT_CLUSTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(64u32);
pub const PRODUCT_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(101u32);
pub const PRODUCT_CORE_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(99u32);
pub const PRODUCT_CORE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(98u32);
pub const PRODUCT_CORE_SINGLELANGUAGE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(100u32);
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(80u32);
pub const PRODUCT_DATACENTER_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(145u32);
pub const PRODUCT_STANDARD_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(146u32);
pub const PRODUCT_DATACENTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(8u32);
pub const PRODUCT_DATACENTER_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(12u32);
pub const PRODUCT_DATACENTER_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(39u32);
pub const PRODUCT_DATACENTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(37u32);
pub const PRODUCT_EDUCATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(121u32);
pub const PRODUCT_EDUCATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(122u32);
pub const PRODUCT_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(4u32);
pub const PRODUCT_ENTERPRISE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(70u32);
pub const PRODUCT_ENTERPRISE_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(72u32);
pub const PRODUCT_ENTERPRISE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(27u32);
pub const PRODUCT_ENTERPRISE_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(84u32);
pub const PRODUCT_ENTERPRISE_S: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(125u32);
pub const PRODUCT_ENTERPRISE_S_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(129u32);
pub const PRODUCT_ENTERPRISE_S_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(126u32);
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(130u32);
pub const PRODUCT_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(10u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(14u32);
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(41u32);
pub const PRODUCT_ENTERPRISE_SERVER_IA64: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(15u32);
pub const PRODUCT_ENTERPRISE_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(38u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(60u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(62u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(59u32);
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(61u32);
pub const PRODUCT_HOME_BASIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(2u32);
pub const PRODUCT_HOME_BASIC_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(67u32);
pub const PRODUCT_HOME_BASIC_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(5u32);
pub const PRODUCT_HOME_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(3u32);
pub const PRODUCT_HOME_PREMIUM_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(68u32);
pub const PRODUCT_HOME_PREMIUM_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(26u32);
pub const PRODUCT_HOME_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(34u32);
pub const PRODUCT_HOME_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(19u32);
pub const PRODUCT_HYPERV: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(42u32);
pub const PRODUCT_IOTUAP: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(123u32);
pub const PRODUCT_IOTUAPCOMMERCIAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(131u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(30u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(32u32);
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(31u32);
pub const PRODUCT_MOBILE_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(104u32);
pub const PRODUCT_MOBILE_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(133u32);
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(77u32);
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(76u32);
pub const PRODUCT_PRO_WORKSTATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(161u32);
pub const PRODUCT_PRO_WORKSTATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(162u32);
pub const PRODUCT_PROFESSIONAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(48u32);
pub const PRODUCT_PROFESSIONAL_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(69u32);
pub const PRODUCT_PROFESSIONAL_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(49u32);
pub const PRODUCT_PROFESSIONAL_WMC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(103u32);
pub const PRODUCT_SB_SOLUTION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(50u32);
pub const PRODUCT_SB_SOLUTION_SERVER_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(54u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(51u32);
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(55u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(24u32);
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(35u32);
pub const PRODUCT_SERVER_FOUNDATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(33u32);
pub const PRODUCT_SMALLBUSINESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(9u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(25u32);
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(63u32);
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(56u32);
pub const PRODUCT_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(79u32);
pub const PRODUCT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(7u32);
pub const PRODUCT_STANDARD_SERVER_CORE_: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(13u32);
pub const PRODUCT_STANDARD_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(40u32);
pub const PRODUCT_STANDARD_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(36u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(52u32);
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(53u32);
pub const PRODUCT_STARTER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(11u32);
pub const PRODUCT_STARTER_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(66u32);
pub const PRODUCT_STARTER_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(47u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(23u32);
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(46u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(20u32);
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(43u32);
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(96u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(21u32);
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(44u32);
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(95u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(22u32);
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(45u32);
pub const PRODUCT_ULTIMATE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(1u32);
pub const PRODUCT_ULTIMATE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(71u32);
pub const PRODUCT_ULTIMATE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(28u32);
pub const PRODUCT_UNDEFINED: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(0u32);
pub const PRODUCT_WEB_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(17u32);
pub const PRODUCT_WEB_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(29u32);
impl ::core::convert::From<u32> for OS_PRODUCT_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OS_PRODUCT_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for OS_PRODUCT_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for OS_PRODUCT_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for OS_PRODUCT_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for OS_PRODUCT_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for OS_PRODUCT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = unsafe extern "system" fn(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = unsafe extern "system" fn(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROCESSOR_CACHE_TYPE(pub i32);
pub const CacheUnified: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(0i32);
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(1i32);
pub const CacheData: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(2i32);
pub const CacheTrace: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(3i32);
impl ::core::convert::From<i32> for PROCESSOR_CACHE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROCESSOR_CACHE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: u8,
    pub ActiveProcessorCount: u8,
    pub Reserved: [u8; 38],
    pub ActiveProcessorMask: usize,
}
impl PROCESSOR_GROUP_INFO {}
impl ::core::default::Default for PROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROCESSOR_GROUP_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROCESSOR_GROUP_INFO").field("MaximumProcessorCount", &self.MaximumProcessorCount).field("ActiveProcessorCount", &self.ActiveProcessorCount).field("Reserved", &self.Reserved).field("ActiveProcessorMask", &self.ActiveProcessorMask).finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumProcessorCount == other.MaximumProcessorCount && self.ActiveProcessorCount == other.ActiveProcessorCount && self.Reserved == other.Reserved && self.ActiveProcessorMask == other.ActiveProcessorMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_GROUP_INFO {}
unsafe impl ::windows::core::Abi for PROCESSOR_GROUP_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROCESSOR_RELATIONSHIP {
    pub Flags: u8,
    pub EfficiencyClass: u8,
    pub Reserved: [u8; 20],
    pub GroupCount: u16,
    pub GroupMask: [GROUP_AFFINITY; 1],
}
impl PROCESSOR_RELATIONSHIP {}
impl ::core::default::Default for PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROCESSOR_RELATIONSHIP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROCESSOR_RELATIONSHIP").field("Flags", &self.Flags).field("EfficiencyClass", &self.EfficiencyClass).field("Reserved", &self.Reserved).field("GroupCount", &self.GroupCount).field("GroupMask", &self.GroupMask).finish()
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EfficiencyClass == other.EfficiencyClass && self.Reserved == other.Reserved && self.GroupCount == other.GroupCount && self.GroupMask == other.GroupMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_RELATIONSHIP {}
unsafe impl ::windows::core::Abi for PROCESSOR_RELATIONSHIP {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTL_SYSTEM_GLOBAL_DATA_ID(pub i32);
pub const GlobalDataIdUnknown: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(0i32);
pub const GlobalDataIdRngSeedVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(1i32);
pub const GlobalDataIdInterruptTime: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(2i32);
pub const GlobalDataIdTimeZoneBias: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(3i32);
pub const GlobalDataIdImageNumberLow: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(4i32);
pub const GlobalDataIdImageNumberHigh: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(5i32);
pub const GlobalDataIdTimeZoneId: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(6i32);
pub const GlobalDataIdNtMajorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(7i32);
pub const GlobalDataIdNtMinorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(8i32);
pub const GlobalDataIdSystemExpirationDate: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(9i32);
pub const GlobalDataIdKdDebuggerEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(10i32);
pub const GlobalDataIdCyclesPerYield: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(11i32);
pub const GlobalDataIdSafeBootMode: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(12i32);
pub const GlobalDataIdLastSystemRITEventTickCount: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(13i32);
impl ::core::convert::From<i32> for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTL_SYSTEM_GLOBAL_DATA_ID {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: super::super::Foundation::PWSTR, deviceform: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: super::super::Foundation::PWSTR, deviceform: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(RtlConvertDeviceFamilyInfoToString(::core::mem::transmute(puldevicefamilybuffersize), ::core::mem::transmute(puldeviceformbuffersize), ::core::mem::transmute(devicefamily), ::core::mem::transmute(deviceform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: *mut u64, puldevicefamily: *mut DEVICEFAMILYINFOENUM, puldeviceform: *mut DEVICEFAMILYDEVICEFORM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: *mut u64, puldevicefamily: *mut DEVICEFAMILYINFOENUM, puldeviceform: *mut DEVICEFAMILYDEVICEFORM);
        }
        ::core::mem::transmute(RtlGetDeviceFamilyInfoEnum(::core::mem::transmute(pulluapinfo), ::core::mem::transmute(puldevicefamily), ::core::mem::transmute(puldeviceform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(RtlGetProductInfo(::core::mem::transmute(osmajorversion), ::core::mem::transmute(osminorversion), ::core::mem::transmute(spmajorversion), ::core::mem::transmute(spminorversion), ::core::mem::transmute(returnedproducttype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut ::core::ffi::c_void, size: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut ::core::ffi::c_void, size: u32) -> u32;
        }
        ::core::mem::transmute(RtlGetSystemGlobalData(::core::mem::transmute(dataid), ::core::mem::transmute(buffer), ::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES;
        }
        ::core::mem::transmute(RtlOsDeploymentState(::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32;
        }
        ::core::mem::transmute(RtlSwitchedVVI(::core::mem::transmute(versioninfo), ::core::mem::transmute(typemask), ::core::mem::transmute(conditionmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1u32;
pub const SPVERSION_MASK: u32 = 65280u32;
pub const SUBVERSION_MASK: u32 = 255u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_CPU_SET_INFORMATION {
    pub Size: u32,
    pub Type: CPU_SET_INFORMATION_TYPE,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0,
}
impl SYSTEM_CPU_SET_INFORMATION {}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION {}
unsafe impl ::windows::core::Abi for SYSTEM_CPU_SET_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union SYSTEM_CPU_SET_INFORMATION_0 {
    pub CpuSet: SYSTEM_CPU_SET_INFORMATION_0_0,
}
impl SYSTEM_CPU_SET_INFORMATION_0 {}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0 {}
unsafe impl ::windows::core::Abi for SYSTEM_CPU_SET_INFORMATION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0 {
    pub Id: u32,
    pub Group: u16,
    pub LogicalProcessorIndex: u8,
    pub CoreIndex: u8,
    pub LastLevelCacheIndex: u8,
    pub NumaNodeIndex: u8,
    pub EfficiencyClass: u8,
    pub Anonymous1: SYSTEM_CPU_SET_INFORMATION_0_0_0,
    pub Anonymous2: SYSTEM_CPU_SET_INFORMATION_0_0_1,
    pub AllocationTag: u64,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0 {}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0 {}
unsafe impl ::windows::core::Abi for SYSTEM_CPU_SET_INFORMATION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    pub AllFlags: u8,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0_0_0_0,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0_0 {}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_0 {}
unsafe impl ::windows::core::Abi for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    pub _bitfield: u8,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
unsafe impl ::windows::core::Abi for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    pub Reserved: u32,
    pub SchedulingClass: u8,
}
impl SYSTEM_CPU_SET_INFORMATION_0_0_1 {}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_1 {}
unsafe impl ::windows::core::Abi for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    type Abi = Self;
}
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2u32;
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4u32;
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1u32;
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: *mut ::core::ffi::c_void,
    pub lpMaximumApplicationAddress: *mut ::core::ffi::c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl SYSTEM_INFO {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SYSTEM_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SYSTEM_INFO {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SYSTEM_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SYSTEM_INFO_0 {
    pub dwOemId: u32,
    pub Anonymous: SYSTEM_INFO_0_0,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl SYSTEM_INFO_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SYSTEM_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SYSTEM_INFO_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SYSTEM_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: super::Diagnostics::Debug::PROCESSOR_ARCHITECTURE,
    pub wReserved: u16,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl SYSTEM_INFO_0_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SYSTEM_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::fmt::Debug for SYSTEM_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("wProcessorArchitecture", &self.wProcessorArchitecture).field("wReserved", &self.wReserved).finish()
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SYSTEM_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wProcessorArchitecture == other.wProcessorArchitecture && self.wReserved == other.wReserved
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SYSTEM_INFO_0_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
unsafe impl ::windows::core::Abi for SYSTEM_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub ProcessorMask: usize,
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {}
unsafe impl ::windows::core::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    pub ProcessorCore: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1,
    pub NumaNode: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0,
    pub Cache: CACHE_DESCRIPTOR,
    pub Reserved: [u64; 2],
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {}
unsafe impl ::windows::core::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    pub NodeNumber: u32,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_NumaNode_e__Struct").field("NodeNumber", &self.NodeNumber).finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NodeNumber == other.NodeNumber
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
unsafe impl ::windows::core::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    pub Flags: u8,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ProcessorCore_e__Struct").field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
unsafe impl ::windows::core::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: u32,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {}
unsafe impl ::windows::core::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    pub Processor: PROCESSOR_RELATIONSHIP,
    pub NumaNode: NUMA_NODE_RELATIONSHIP,
    pub Cache: CACHE_RELATIONSHIP,
    pub Group: GROUP_RELATIONSHIP,
}
impl SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {}
unsafe impl ::windows::core::Abi for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_POOL_ZEROING_INFORMATION {
    pub PoolZeroingSupportPresent: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_POOL_ZEROING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_POOL_ZEROING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_POOL_ZEROING_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SYSTEM_POOL_ZEROING_INFORMATION").field("PoolZeroingSupportPresent", &self.PoolZeroingSupportPresent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_POOL_ZEROING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolZeroingSupportPresent == other.PoolZeroingSupportPresent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_POOL_ZEROING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYSTEM_POOL_ZEROING_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub CycleTime: u64,
}
impl SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
impl ::core::default::Default for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION").field("CycleTime", &self.CycleTime).finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CycleTime == other.CycleTime
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
unsafe impl ::windows::core::Abi for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
impl SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl ::core::default::Default for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
unsafe impl ::windows::core::Abi for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpcomputername: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameA(lpcomputername: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetComputerNameA(lpcomputername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameEx2W<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameEx2W(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetComputerNameEx2W(::core::mem::transmute(nametype), ::core::mem::transmute(flags), lpbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameExA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetComputerNameExA(::core::mem::transmute(nametype), lpbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameExW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetComputerNameExW(::core::mem::transmute(nametype), lpbuffer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpcomputername: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetComputerNameW(lpcomputername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetComputerNameW(lpcomputername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetLocalTime(::core::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetSystemTime(::core::mem::transmute(lpsystemtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTimeAdjustment<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(dwtimeadjustment: u32, btimeadjustmentdisabled: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemTimeAdjustment(dwtimeadjustment: u32, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetSystemTimeAdjustment(::core::mem::transmute(dwtimeadjustment), btimeadjustmentdisabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTimeAdjustmentPrecise<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(dwtimeadjustment: u64, btimeadjustmentdisabled: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: u64, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetSystemTimeAdjustmentPrecise(::core::mem::transmute(dwtimeadjustment), btimeadjustmentdisabled.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct USER_CET_ENVIRONMENT(pub u32);
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(0u32);
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(2u32);
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(16u32);
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(17u32);
impl ::core::convert::From<u32> for USER_CET_ENVIRONMENT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for USER_CET_ENVIRONMENT {
    type Abi = Self;
}
impl ::core::ops::BitOr for USER_CET_ENVIRONMENT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for USER_CET_ENVIRONMENT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for USER_CET_ENVIRONMENT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for USER_CET_ENVIRONMENT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for USER_CET_ENVIRONMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VER_FLAGS(pub u32);
pub const VER_MINORVERSION: VER_FLAGS = VER_FLAGS(1u32);
pub const VER_MAJORVERSION: VER_FLAGS = VER_FLAGS(2u32);
pub const VER_BUILDNUMBER: VER_FLAGS = VER_FLAGS(4u32);
pub const VER_PLATFORMID: VER_FLAGS = VER_FLAGS(8u32);
pub const VER_SERVICEPACKMINOR: VER_FLAGS = VER_FLAGS(16u32);
pub const VER_SERVICEPACKMAJOR: VER_FLAGS = VER_FLAGS(32u32);
pub const VER_SUITENAME: VER_FLAGS = VER_FLAGS(64u32);
pub const VER_PRODUCT_TYPE: VER_FLAGS = VER_FLAGS(128u32);
impl ::core::convert::From<u32> for VER_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VER_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for VER_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for VER_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for VER_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for VER_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for VER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64;
        }
        ::core::mem::transmute(VerSetConditionMask(::core::mem::transmute(conditionmask), ::core::mem::transmute(typemask), ::core::mem::transmute(condition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VerifyVersionInfoA(::core::mem::transmute(lpversioninformation), ::core::mem::transmute(dwtypemask), ::core::mem::transmute(dwlconditionmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VerifyVersionInfoW(::core::mem::transmute(lpversioninformation), ::core::mem::transmute(dwtypemask), ::core::mem::transmute(dwlconditionmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WDK_NTDDI_VERSION: u32 = 167772171u32;
pub const _WIN32_IE_IE100: u32 = 2560u32;
pub const _WIN32_IE_IE110: u32 = 2560u32;
pub const _WIN32_IE_IE20: u32 = 512u32;
pub const _WIN32_IE_IE30: u32 = 768u32;
pub const _WIN32_IE_IE302: u32 = 770u32;
pub const _WIN32_IE_IE40: u32 = 1024u32;
pub const _WIN32_IE_IE401: u32 = 1025u32;
pub const _WIN32_IE_IE50: u32 = 1280u32;
pub const _WIN32_IE_IE501: u32 = 1281u32;
pub const _WIN32_IE_IE55: u32 = 1360u32;
pub const _WIN32_IE_IE60: u32 = 1536u32;
pub const _WIN32_IE_IE60SP1: u32 = 1537u32;
pub const _WIN32_IE_IE60SP2: u32 = 1539u32;
pub const _WIN32_IE_IE70: u32 = 1792u32;
pub const _WIN32_IE_IE80: u32 = 2048u32;
pub const _WIN32_IE_IE90: u32 = 2304u32;
pub const _WIN32_IE_LONGHORN: u32 = 1792u32;
pub const _WIN32_IE_NT4: u32 = 512u32;
pub const _WIN32_IE_NT4SP1: u32 = 512u32;
pub const _WIN32_IE_NT4SP2: u32 = 512u32;
pub const _WIN32_IE_NT4SP3: u32 = 770u32;
pub const _WIN32_IE_NT4SP4: u32 = 1025u32;
pub const _WIN32_IE_NT4SP5: u32 = 1025u32;
pub const _WIN32_IE_NT4SP6: u32 = 1280u32;
pub const _WIN32_IE_WIN10: u32 = 2560u32;
pub const _WIN32_IE_WIN2K: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP1: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP2: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP3: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP4: u32 = 1281u32;
pub const _WIN32_IE_WIN6: u32 = 1792u32;
pub const _WIN32_IE_WIN7: u32 = 2048u32;
pub const _WIN32_IE_WIN8: u32 = 2560u32;
pub const _WIN32_IE_WIN98: u32 = 1025u32;
pub const _WIN32_IE_WIN98SE: u32 = 1280u32;
pub const _WIN32_IE_WINBLUE: u32 = 2560u32;
pub const _WIN32_IE_WINME: u32 = 1360u32;
pub const _WIN32_IE_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_IE_WS03: u32 = 1538u32;
pub const _WIN32_IE_WS03SP1: u32 = 1539u32;
pub const _WIN32_IE_XP: u32 = 1536u32;
pub const _WIN32_IE_XPSP1: u32 = 1537u32;
pub const _WIN32_IE_XPSP2: u32 = 1539u32;
pub const _WIN32_WINNT_LONGHORN: u32 = 1536u32;
pub const _WIN32_WINNT_NT4: u32 = 1024u32;
pub const _WIN32_WINNT_VISTA: u32 = 1536u32;
pub const _WIN32_WINNT_WIN10: u32 = 2560u32;
pub const _WIN32_WINNT_WIN2K: u32 = 1280u32;
pub const _WIN32_WINNT_WIN6: u32 = 1536u32;
pub const _WIN32_WINNT_WIN7: u32 = 1537u32;
pub const _WIN32_WINNT_WIN8: u32 = 1538u32;
pub const _WIN32_WINNT_WINBLUE: u32 = 1539u32;
pub const _WIN32_WINNT_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_WINNT_WINXP: u32 = 1281u32;
pub const _WIN32_WINNT_WS03: u32 = 1282u32;
pub const _WIN32_WINNT_WS08: u32 = 1536u32;
