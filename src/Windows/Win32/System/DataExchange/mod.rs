#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const APPCLASS_MASK: i32 = 15i32;
pub const APPCMD_MASK: i32 = 4080i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAtomA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
        }
        ::core::mem::transmute(AddAtomA(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddAtomW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
        }
        ::core::mem::transmute(AddAtomW(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddClipboardFormatListener<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AddClipboardFormatListener(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CADV_LATEACK: u32 = 65535u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CONVCONTEXT {
    pub cb: u32,
    pub wFlags: u32,
    pub wCountryID: u32,
    pub iCodePage: i32,
    pub dwLangID: u32,
    pub dwSecurity: u32,
    pub qos: super::super::Security::SECURITY_QUALITY_OF_SERVICE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl CONVCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CONVCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CONVCONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CONVCONTEXT").field("cb", &self.cb).field("wFlags", &self.wFlags).field("wCountryID", &self.wCountryID).field("iCodePage", &self.iCodePage).field("dwLangID", &self.dwLangID).field("dwSecurity", &self.dwSecurity).field("qos", &self.qos).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CONVCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.wFlags == other.wFlags && self.wCountryID == other.wCountryID && self.iCodePage == other.iCodePage && self.dwLangID == other.dwLangID && self.dwSecurity == other.dwSecurity && self.qos == other.qos
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CONVCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for CONVCONTEXT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct CONVINFO {
    pub cb: u32,
    pub hUser: usize,
    pub hConvPartner: HCONV,
    pub hszSvcPartner: HSZ,
    pub hszServiceReq: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub wType: DDE_CLIENT_TRANSACTION_TYPE,
    pub wStatus: CONVINFO_STATUS,
    pub wConvst: CONVINFO_CONVERSATION_STATE,
    pub wLastError: u32,
    pub hConvList: HCONVLIST,
    pub ConvCtxt: CONVCONTEXT,
    pub hwnd: super::super::Foundation::HWND,
    pub hwndPartner: super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl CONVINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CONVINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CONVINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CONVINFO")
            .field("cb", &self.cb)
            .field("hUser", &self.hUser)
            .field("hConvPartner", &self.hConvPartner)
            .field("hszSvcPartner", &self.hszSvcPartner)
            .field("hszServiceReq", &self.hszServiceReq)
            .field("hszTopic", &self.hszTopic)
            .field("hszItem", &self.hszItem)
            .field("wFmt", &self.wFmt)
            .field("wType", &self.wType)
            .field("wStatus", &self.wStatus)
            .field("wConvst", &self.wConvst)
            .field("wLastError", &self.wLastError)
            .field("hConvList", &self.hConvList)
            .field("ConvCtxt", &self.ConvCtxt)
            .field("hwnd", &self.hwnd)
            .field("hwndPartner", &self.hwndPartner)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CONVINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.hUser == other.hUser
            && self.hConvPartner == other.hConvPartner
            && self.hszSvcPartner == other.hszSvcPartner
            && self.hszServiceReq == other.hszServiceReq
            && self.hszTopic == other.hszTopic
            && self.hszItem == other.hszItem
            && self.wFmt == other.wFmt
            && self.wType == other.wType
            && self.wStatus == other.wStatus
            && self.wConvst == other.wConvst
            && self.wLastError == other.wLastError
            && self.hConvList == other.hConvList
            && self.ConvCtxt == other.ConvCtxt
            && self.hwnd == other.hwnd
            && self.hwndPartner == other.hwndPartner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CONVINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for CONVINFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONVINFO_CONVERSATION_STATE(pub u32);
pub const XST_ADVACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(13u32);
pub const XST_ADVDATAACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(16u32);
pub const XST_ADVDATASENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(15u32);
pub const XST_ADVSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(11u32);
pub const XST_CONNECTED: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(2u32);
pub const XST_DATARCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(6u32);
pub const XST_EXECACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(10u32);
pub const XST_EXECSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(9u32);
pub const XST_INCOMPLETE: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(1u32);
pub const XST_INIT1: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(3u32);
pub const XST_INIT2: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(4u32);
pub const XST_NULL: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(0u32);
pub const XST_POKEACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(8u32);
pub const XST_POKESENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(7u32);
pub const XST_REQSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(5u32);
pub const XST_UNADVACKRCVD: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(14u32);
pub const XST_UNADVSENT: CONVINFO_CONVERSATION_STATE = CONVINFO_CONVERSATION_STATE(12u32);
impl ::core::convert::From<u32> for CONVINFO_CONVERSATION_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CONVINFO_CONVERSATION_STATE {
    type Abi = Self;
}
impl ::core::ops::BitOr for CONVINFO_CONVERSATION_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CONVINFO_CONVERSATION_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CONVINFO_CONVERSATION_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CONVINFO_CONVERSATION_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CONVINFO_CONVERSATION_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONVINFO_STATUS(pub u32);
pub const ST_ADVISE: CONVINFO_STATUS = CONVINFO_STATUS(2u32);
pub const ST_BLOCKED: CONVINFO_STATUS = CONVINFO_STATUS(8u32);
pub const ST_BLOCKNEXT: CONVINFO_STATUS = CONVINFO_STATUS(128u32);
pub const ST_CLIENT: CONVINFO_STATUS = CONVINFO_STATUS(16u32);
pub const ST_CONNECTED: CONVINFO_STATUS = CONVINFO_STATUS(1u32);
pub const ST_INLIST: CONVINFO_STATUS = CONVINFO_STATUS(64u32);
pub const ST_ISLOCAL: CONVINFO_STATUS = CONVINFO_STATUS(4u32);
pub const ST_ISSELF: CONVINFO_STATUS = CONVINFO_STATUS(256u32);
pub const ST_TERMINATED: CONVINFO_STATUS = CONVINFO_STATUS(32u32);
impl ::core::convert::From<u32> for CONVINFO_STATUS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CONVINFO_STATUS {
    type Abi = Self;
}
impl ::core::ops::BitOr for CONVINFO_STATUS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CONVINFO_STATUS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CONVINFO_STATUS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CONVINFO_STATUS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CONVINFO_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct COPYDATASTRUCT {
    pub dwData: usize,
    pub cbData: u32,
    pub lpData: *mut ::core::ffi::c_void,
}
impl COPYDATASTRUCT {}
impl ::core::default::Default for COPYDATASTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COPYDATASTRUCT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COPYDATASTRUCT").field("dwData", &self.dwData).field("cbData", &self.cbData).field("lpData", &self.lpData).finish()
    }
}
impl ::core::cmp::PartialEq for COPYDATASTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwData == other.dwData && self.cbData == other.cbData && self.lpData == other.lpData
    }
}
impl ::core::cmp::Eq for COPYDATASTRUCT {}
unsafe impl ::windows::core::Abi for COPYDATASTRUCT {
    type Abi = Self;
}
pub const CP_WINANSI: i32 = 1004i32;
pub const CP_WINNEUTRAL: i32 = 1200i32;
pub const CP_WINUNICODE: i32 = 1200i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeClipboardChain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndremove: Param0, hwndnewnext: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeClipboardChain(hwndremove: super::super::Foundation::HWND, hwndnewnext: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChangeClipboardChain(hwndremove.into_param().abi(), hwndnewnext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseClipboard() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseClipboard() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseClipboard())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CountClipboardFormats() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CountClipboardFormats() -> i32;
        }
        ::core::mem::transmute(CountClipboardFormats())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DDEACK {
    pub _bitfield: u16,
}
impl DDEACK {}
impl ::core::default::Default for DDEACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDEACK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDEACK").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for DDEACK {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DDEACK {}
unsafe impl ::windows::core::Abi for DDEACK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DDEADVISE {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl DDEADVISE {}
impl ::core::default::Default for DDEADVISE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDEADVISE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDEADVISE").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).finish()
    }
}
impl ::core::cmp::PartialEq for DDEADVISE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat
    }
}
impl ::core::cmp::Eq for DDEADVISE {}
unsafe impl ::windows::core::Abi for DDEADVISE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DDEDATA {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl DDEDATA {}
impl ::core::default::Default for DDEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDEDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDEDATA").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for DDEDATA {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DDEDATA {}
unsafe impl ::windows::core::Abi for DDEDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DDELN {
    pub _bitfield: u16,
    pub cfFormat: i16,
}
impl DDELN {}
impl ::core::default::Default for DDELN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDELN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDELN").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).finish()
    }
}
impl ::core::cmp::PartialEq for DDELN {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat
    }
}
impl ::core::cmp::Eq for DDELN {}
unsafe impl ::windows::core::Abi for DDELN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DDEML_MSG_HOOK_DATA {
    pub uiLo: usize,
    pub uiHi: usize,
    pub cbData: u32,
    pub Data: [u32; 8],
}
impl DDEML_MSG_HOOK_DATA {}
impl ::core::default::Default for DDEML_MSG_HOOK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDEML_MSG_HOOK_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDEML_MSG_HOOK_DATA").field("uiLo", &self.uiLo).field("uiHi", &self.uiHi).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
impl ::core::cmp::PartialEq for DDEML_MSG_HOOK_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.uiLo == other.uiLo && self.uiHi == other.uiHi && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DDEML_MSG_HOOK_DATA {}
unsafe impl ::windows::core::Abi for DDEML_MSG_HOOK_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DDEPOKE {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub Value: [u8; 1],
}
impl DDEPOKE {}
impl ::core::default::Default for DDEPOKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDEPOKE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDEPOKE").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for DDEPOKE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DDEPOKE {}
unsafe impl ::windows::core::Abi for DDEPOKE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DDEUP {
    pub _bitfield: u16,
    pub cfFormat: i16,
    pub rgb: [u8; 1],
}
impl DDEUP {}
impl ::core::default::Default for DDEUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDEUP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDEUP").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("rgb", &self.rgb).finish()
    }
}
impl ::core::cmp::PartialEq for DDEUP {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat && self.rgb == other.rgb
    }
}
impl ::core::cmp::Eq for DDEUP {}
unsafe impl ::windows::core::Abi for DDEUP {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DDE_CLIENT_TRANSACTION_TYPE(pub u32);
pub const XTYP_ADVSTART: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(4144u32);
pub const XTYP_ADVSTOP: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32832u32);
pub const XTYP_EXECUTE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16464u32);
pub const XTYP_POKE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16528u32);
pub const XTYP_REQUEST: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8368u32);
pub const XTYP_ADVDATA: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(16400u32);
pub const XTYP_ADVREQ: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8226u32);
pub const XTYP_CONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(4194u32);
pub const XTYP_CONNECT_CONFIRM: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32882u32);
pub const XTYP_DISCONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32962u32);
pub const XTYP_MONITOR: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(33010u32);
pub const XTYP_REGISTER: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32930u32);
pub const XTYP_UNREGISTER: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32978u32);
pub const XTYP_WILDCONNECT: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(8418u32);
pub const XTYP_XACT_COMPLETE: DDE_CLIENT_TRANSACTION_TYPE = DDE_CLIENT_TRANSACTION_TYPE(32896u32);
impl ::core::convert::From<u32> for DDE_CLIENT_TRANSACTION_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DDE_CLIENT_TRANSACTION_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for DDE_CLIENT_TRANSACTION_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DDE_CLIENT_TRANSACTION_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DDE_CLIENT_TRANSACTION_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DDE_CLIENT_TRANSACTION_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DDE_CLIENT_TRANSACTION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DDE_ENABLE_CALLBACK_CMD(pub u32);
pub const EC_ENABLEALL: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(0u32);
pub const EC_ENABLEONE: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(128u32);
pub const EC_DISABLE: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(8u32);
pub const EC_QUERYWAITING: DDE_ENABLE_CALLBACK_CMD = DDE_ENABLE_CALLBACK_CMD(2u32);
impl ::core::convert::From<u32> for DDE_ENABLE_CALLBACK_CMD {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DDE_ENABLE_CALLBACK_CMD {
    type Abi = Self;
}
impl ::core::ops::BitOr for DDE_ENABLE_CALLBACK_CMD {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DDE_ENABLE_CALLBACK_CMD {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DDE_ENABLE_CALLBACK_CMD {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DDE_ENABLE_CALLBACK_CMD {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DDE_ENABLE_CALLBACK_CMD {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DDE_FACK: u32 = 32768u32;
pub const DDE_FACKREQ: u32 = 32768u32;
pub const DDE_FAPPSTATUS: u32 = 255u32;
pub const DDE_FBUSY: u32 = 16384u32;
pub const DDE_FDEFERUPD: u32 = 16384u32;
pub const DDE_FNOTPROCESSED: u32 = 0u32;
pub const DDE_FRELEASE: u32 = 8192u32;
pub const DDE_FREQUESTED: u32 = 4096u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DDE_INITIALIZE_COMMAND(pub u32);
pub const APPCLASS_MONITOR: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1u32);
pub const APPCLASS_STANDARD: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(0u32);
pub const APPCMD_CLIENTONLY: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16u32);
pub const APPCMD_FILTERINITS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(32u32);
pub const CBF_FAIL_ALLSVRXACTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(258048u32);
pub const CBF_FAIL_ADVISES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16384u32);
pub const CBF_FAIL_CONNECTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(8192u32);
pub const CBF_FAIL_EXECUTES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(32768u32);
pub const CBF_FAIL_POKES: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(65536u32);
pub const CBF_FAIL_REQUESTS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(131072u32);
pub const CBF_FAIL_SELFCONNECTIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(4096u32);
pub const CBF_SKIP_ALLNOTIFICATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(3932160u32);
pub const CBF_SKIP_CONNECT_CONFIRMS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(262144u32);
pub const CBF_SKIP_DISCONNECTS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(2097152u32);
pub const CBF_SKIP_REGISTRATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(524288u32);
pub const CBF_SKIP_UNREGISTRATIONS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1048576u32);
pub const MF_CALLBACKS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(134217728u32);
pub const MF_CONV: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(1073741824u32);
pub const MF_ERRORS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(268435456u32);
pub const MF_HSZ_INFO: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(16777216u32);
pub const MF_LINKS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(536870912u32);
pub const MF_POSTMSGS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(67108864u32);
pub const MF_SENDMSGS: DDE_INITIALIZE_COMMAND = DDE_INITIALIZE_COMMAND(33554432u32);
impl ::core::convert::From<u32> for DDE_INITIALIZE_COMMAND {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DDE_INITIALIZE_COMMAND {
    type Abi = Self;
}
impl ::core::ops::BitOr for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DDE_INITIALIZE_COMMAND {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DDE_INITIALIZE_COMMAND {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DDE_NAME_SERVICE_CMD(pub u32);
pub const DNS_REGISTER: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(1u32);
pub const DNS_UNREGISTER: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(2u32);
pub const DNS_FILTERON: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(4u32);
pub const DNS_FILTEROFF: DDE_NAME_SERVICE_CMD = DDE_NAME_SERVICE_CMD(8u32);
impl ::core::convert::From<u32> for DDE_NAME_SERVICE_CMD {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DDE_NAME_SERVICE_CMD {
    type Abi = Self;
}
impl ::core::ops::BitOr for DDE_NAME_SERVICE_CMD {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DDE_NAME_SERVICE_CMD {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DDE_NAME_SERVICE_CMD {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DDE_NAME_SERVICE_CMD {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DDE_NAME_SERVICE_CMD {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DMLERR_ADVACKTIMEOUT: u32 = 16384u32;
pub const DMLERR_BUSY: u32 = 16385u32;
pub const DMLERR_DATAACKTIMEOUT: u32 = 16386u32;
pub const DMLERR_DLL_NOT_INITIALIZED: u32 = 16387u32;
pub const DMLERR_DLL_USAGE: u32 = 16388u32;
pub const DMLERR_EXECACKTIMEOUT: u32 = 16389u32;
pub const DMLERR_FIRST: u32 = 16384u32;
pub const DMLERR_INVALIDPARAMETER: u32 = 16390u32;
pub const DMLERR_LAST: u32 = 16401u32;
pub const DMLERR_LOW_MEMORY: u32 = 16391u32;
pub const DMLERR_MEMORY_ERROR: u32 = 16392u32;
pub const DMLERR_NOTPROCESSED: u32 = 16393u32;
pub const DMLERR_NO_CONV_ESTABLISHED: u32 = 16394u32;
pub const DMLERR_NO_ERROR: u32 = 0u32;
pub const DMLERR_POKEACKTIMEOUT: u32 = 16395u32;
pub const DMLERR_POSTMSG_FAILED: u32 = 16396u32;
pub const DMLERR_REENTRANCY: u32 = 16397u32;
pub const DMLERR_SERVER_DIED: u32 = 16398u32;
pub const DMLERR_SYS_ERROR: u32 = 16399u32;
pub const DMLERR_UNADVACKTIMEOUT: u32 = 16400u32;
pub const DMLERR_UNFOUND_QUEUE_ID: u32 = 16401u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeAbandonTransaction<'a, Param1: ::windows::core::IntoParam<'a, HCONV>>(idinst: u32, hconv: Param1, idtransaction: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeAbandonTransaction(idinst: u32, hconv: HCONV, idtransaction: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeAbandonTransaction(::core::mem::transmute(idinst), hconv.into_param().abi(), ::core::mem::transmute(idtransaction)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeAccessData<'a, Param0: ::windows::core::IntoParam<'a, HDDEDATA>>(hdata: Param0, pcbdatasize: *mut u32) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeAccessData(hdata: HDDEDATA, pcbdatasize: *mut u32) -> *mut u8;
        }
        ::core::mem::transmute(DdeAccessData(hdata.into_param().abi(), ::core::mem::transmute(pcbdatasize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeAddData<'a, Param0: ::windows::core::IntoParam<'a, HDDEDATA>>(hdata: Param0, psrc: *const u8, cb: u32, cboff: u32) -> HDDEDATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeAddData(hdata: HDDEDATA, psrc: *const u8, cb: u32, cboff: u32) -> HDDEDATA;
        }
        ::core::mem::transmute(DdeAddData(hdata.into_param().abi(), ::core::mem::transmute(psrc), ::core::mem::transmute(cb), ::core::mem::transmute(cboff)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeClientTransaction<'a, Param2: ::windows::core::IntoParam<'a, HCONV>, Param3: ::windows::core::IntoParam<'a, HSZ>>(pdata: *const u8, cbdata: u32, hconv: Param2, hszitem: Param3, wfmt: u32, wtype: DDE_CLIENT_TRANSACTION_TYPE, dwtimeout: u32, pdwresult: *mut u32) -> HDDEDATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeClientTransaction(pdata: *const u8, cbdata: u32, hconv: HCONV, hszitem: HSZ, wfmt: u32, wtype: DDE_CLIENT_TRANSACTION_TYPE, dwtimeout: u32, pdwresult: *mut u32) -> HDDEDATA;
        }
        ::core::mem::transmute(DdeClientTransaction(::core::mem::transmute(pdata), ::core::mem::transmute(cbdata), hconv.into_param().abi(), hszitem.into_param().abi(), ::core::mem::transmute(wfmt), ::core::mem::transmute(wtype), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(pdwresult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeCmpStringHandles<'a, Param0: ::windows::core::IntoParam<'a, HSZ>, Param1: ::windows::core::IntoParam<'a, HSZ>>(hsz1: Param0, hsz2: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeCmpStringHandles(hsz1: HSZ, hsz2: HSZ) -> i32;
        }
        ::core::mem::transmute(DdeCmpStringHandles(hsz1.into_param().abi(), hsz2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeConnect<'a, Param1: ::windows::core::IntoParam<'a, HSZ>, Param2: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, hszservice: Param1, hsztopic: Param2, pcc: *const CONVCONTEXT) -> HCONV {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeConnect(idinst: u32, hszservice: HSZ, hsztopic: HSZ, pcc: *const CONVCONTEXT) -> HCONV;
        }
        ::core::mem::transmute(DdeConnect(::core::mem::transmute(idinst), hszservice.into_param().abi(), hsztopic.into_param().abi(), ::core::mem::transmute(pcc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeConnectList<'a, Param1: ::windows::core::IntoParam<'a, HSZ>, Param2: ::windows::core::IntoParam<'a, HSZ>, Param3: ::windows::core::IntoParam<'a, HCONVLIST>>(idinst: u32, hszservice: Param1, hsztopic: Param2, hconvlist: Param3, pcc: *const CONVCONTEXT) -> HCONVLIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeConnectList(idinst: u32, hszservice: HSZ, hsztopic: HSZ, hconvlist: HCONVLIST, pcc: *const CONVCONTEXT) -> HCONVLIST;
        }
        ::core::mem::transmute(DdeConnectList(::core::mem::transmute(idinst), hszservice.into_param().abi(), hsztopic.into_param().abi(), hconvlist.into_param().abi(), ::core::mem::transmute(pcc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeCreateDataHandle<'a, Param4: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, psrc: *const u8, cb: u32, cboff: u32, hszitem: Param4, wfmt: u32, afcmd: u32) -> HDDEDATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeCreateDataHandle(idinst: u32, psrc: *const u8, cb: u32, cboff: u32, hszitem: HSZ, wfmt: u32, afcmd: u32) -> HDDEDATA;
        }
        ::core::mem::transmute(DdeCreateDataHandle(::core::mem::transmute(idinst), ::core::mem::transmute(psrc), ::core::mem::transmute(cb), ::core::mem::transmute(cboff), hszitem.into_param().abi(), ::core::mem::transmute(wfmt), ::core::mem::transmute(afcmd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeCreateStringHandleA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(idinst: u32, psz: Param1, icodepage: i32) -> HSZ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeCreateStringHandleA(idinst: u32, psz: super::super::Foundation::PSTR, icodepage: i32) -> HSZ;
        }
        ::core::mem::transmute(DdeCreateStringHandleA(::core::mem::transmute(idinst), psz.into_param().abi(), ::core::mem::transmute(icodepage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeCreateStringHandleW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(idinst: u32, psz: Param1, icodepage: i32) -> HSZ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeCreateStringHandleW(idinst: u32, psz: super::super::Foundation::PWSTR, icodepage: i32) -> HSZ;
        }
        ::core::mem::transmute(DdeCreateStringHandleW(::core::mem::transmute(idinst), psz.into_param().abi(), ::core::mem::transmute(icodepage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeDisconnect<'a, Param0: ::windows::core::IntoParam<'a, HCONV>>(hconv: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeDisconnect(hconv: HCONV) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeDisconnect(hconv.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeDisconnectList<'a, Param0: ::windows::core::IntoParam<'a, HCONVLIST>>(hconvlist: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeDisconnectList(hconvlist: HCONVLIST) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeDisconnectList(hconvlist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeEnableCallback<'a, Param1: ::windows::core::IntoParam<'a, HCONV>>(idinst: u32, hconv: Param1, wcmd: DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeEnableCallback(idinst: u32, hconv: HCONV, wcmd: DDE_ENABLE_CALLBACK_CMD) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeEnableCallback(::core::mem::transmute(idinst), hconv.into_param().abi(), ::core::mem::transmute(wcmd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeFreeDataHandle<'a, Param0: ::windows::core::IntoParam<'a, HDDEDATA>>(hdata: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeFreeDataHandle(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeFreeDataHandle(hdata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeFreeStringHandle<'a, Param1: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, hsz: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeFreeStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeFreeStringHandle(::core::mem::transmute(idinst), hsz.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeGetData<'a, Param0: ::windows::core::IntoParam<'a, HDDEDATA>>(hdata: Param0, pdst: *mut u8, cbmax: u32, cboff: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeGetData(hdata: HDDEDATA, pdst: *mut u8, cbmax: u32, cboff: u32) -> u32;
        }
        ::core::mem::transmute(DdeGetData(hdata.into_param().abi(), ::core::mem::transmute(pdst), ::core::mem::transmute(cbmax), ::core::mem::transmute(cboff)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeGetLastError(idinst: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeGetLastError(idinst: u32) -> u32;
        }
        ::core::mem::transmute(DdeGetLastError(::core::mem::transmute(idinst)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeImpersonateClient<'a, Param0: ::windows::core::IntoParam<'a, HCONV>>(hconv: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeImpersonateClient(hconv: HCONV) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeImpersonateClient(hconv.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeInitializeA(pidinst: *mut u32, pfncallback: ::core::option::Option<PFNCALLBACK>, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeInitializeA(pidinst: *mut u32, pfncallback: ::windows::core::RawPtr, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
        }
        ::core::mem::transmute(DdeInitializeA(::core::mem::transmute(pidinst), ::core::mem::transmute(pfncallback), ::core::mem::transmute(afcmd), ::core::mem::transmute(ulres)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeInitializeW(pidinst: *mut u32, pfncallback: ::core::option::Option<PFNCALLBACK>, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeInitializeW(pidinst: *mut u32, pfncallback: ::windows::core::RawPtr, afcmd: DDE_INITIALIZE_COMMAND, ulres: u32) -> u32;
        }
        ::core::mem::transmute(DdeInitializeW(::core::mem::transmute(pidinst), ::core::mem::transmute(pfncallback), ::core::mem::transmute(afcmd), ::core::mem::transmute(ulres)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeKeepStringHandle<'a, Param1: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, hsz: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeKeepStringHandle(idinst: u32, hsz: HSZ) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeKeepStringHandle(::core::mem::transmute(idinst), hsz.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeNameService<'a, Param1: ::windows::core::IntoParam<'a, HSZ>, Param2: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, hsz1: Param1, hsz2: Param2, afcmd: DDE_NAME_SERVICE_CMD) -> HDDEDATA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeNameService(idinst: u32, hsz1: HSZ, hsz2: HSZ, afcmd: DDE_NAME_SERVICE_CMD) -> HDDEDATA;
        }
        ::core::mem::transmute(DdeNameService(::core::mem::transmute(idinst), hsz1.into_param().abi(), hsz2.into_param().abi(), ::core::mem::transmute(afcmd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdePostAdvise<'a, Param1: ::windows::core::IntoParam<'a, HSZ>, Param2: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, hsztopic: Param1, hszitem: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdePostAdvise(idinst: u32, hsztopic: HSZ, hszitem: HSZ) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdePostAdvise(::core::mem::transmute(idinst), hsztopic.into_param().abi(), hszitem.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeQueryConvInfo<'a, Param0: ::windows::core::IntoParam<'a, HCONV>>(hconv: Param0, idtransaction: u32, pconvinfo: *mut CONVINFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeQueryConvInfo(hconv: HCONV, idtransaction: u32, pconvinfo: *mut CONVINFO) -> u32;
        }
        ::core::mem::transmute(DdeQueryConvInfo(hconv.into_param().abi(), ::core::mem::transmute(idtransaction), ::core::mem::transmute(pconvinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeQueryNextServer<'a, Param0: ::windows::core::IntoParam<'a, HCONVLIST>, Param1: ::windows::core::IntoParam<'a, HCONV>>(hconvlist: Param0, hconvprev: Param1) -> HCONV {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeQueryNextServer(hconvlist: HCONVLIST, hconvprev: HCONV) -> HCONV;
        }
        ::core::mem::transmute(DdeQueryNextServer(hconvlist.into_param().abi(), hconvprev.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeQueryStringA<'a, Param1: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, hsz: Param1, psz: super::super::Foundation::PSTR, cchmax: u32, icodepage: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeQueryStringA(idinst: u32, hsz: HSZ, psz: super::super::Foundation::PSTR, cchmax: u32, icodepage: i32) -> u32;
        }
        ::core::mem::transmute(DdeQueryStringA(::core::mem::transmute(idinst), hsz.into_param().abi(), ::core::mem::transmute(psz), ::core::mem::transmute(cchmax), ::core::mem::transmute(icodepage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeQueryStringW<'a, Param1: ::windows::core::IntoParam<'a, HSZ>>(idinst: u32, hsz: Param1, psz: super::super::Foundation::PWSTR, cchmax: u32, icodepage: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeQueryStringW(idinst: u32, hsz: HSZ, psz: super::super::Foundation::PWSTR, cchmax: u32, icodepage: i32) -> u32;
        }
        ::core::mem::transmute(DdeQueryStringW(::core::mem::transmute(idinst), hsz.into_param().abi(), ::core::mem::transmute(psz), ::core::mem::transmute(cchmax), ::core::mem::transmute(icodepage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DdeReconnect<'a, Param0: ::windows::core::IntoParam<'a, HCONV>>(hconv: Param0) -> HCONV {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeReconnect(hconv: HCONV) -> HCONV;
        }
        ::core::mem::transmute(DdeReconnect(hconv.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DdeSetQualityOfService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndclient: Param0, pqosnew: *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeSetQualityOfService(hwndclient: super::super::Foundation::HWND, pqosnew: *const super::super::Security::SECURITY_QUALITY_OF_SERVICE, pqosprev: *mut super::super::Security::SECURITY_QUALITY_OF_SERVICE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeSetQualityOfService(hwndclient.into_param().abi(), ::core::mem::transmute(pqosnew), ::core::mem::transmute(pqosprev)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeSetUserHandle<'a, Param0: ::windows::core::IntoParam<'a, HCONV>>(hconv: Param0, id: u32, huser: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeSetUserHandle(hconv: HCONV, id: u32, huser: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeSetUserHandle(hconv.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(huser)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeUnaccessData<'a, Param0: ::windows::core::IntoParam<'a, HDDEDATA>>(hdata: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeUnaccessData(hdata: HDDEDATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeUnaccessData(hdata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdeUninitialize(idinst: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdeUninitialize(idinst: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DdeUninitialize(::core::mem::transmute(idinst)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeleteAtom(natom: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAtom(natom: u16) -> u16;
        }
        ::core::mem::transmute(DeleteAtom(::core::mem::transmute(natom)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EmptyClipboard() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EmptyClipboard() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EmptyClipboard())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnumClipboardFormats(format: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumClipboardFormats(format: u32) -> u32;
        }
        ::core::mem::transmute(EnumClipboardFormats(::core::mem::transmute(format)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindAtomA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
        }
        ::core::mem::transmute(FindAtomA(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindAtomW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
        }
        ::core::mem::transmute(FindAtomW(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeDDElParam<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(msg: u32, lparam: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeDDElParam(::core::mem::transmute(msg), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32;
        }
        ::core::mem::transmute(GetAtomNameA(::core::mem::transmute(natom), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32;
        }
        ::core::mem::transmute(GetAtomNameW(::core::mem::transmute(natom), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardData(uformat: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClipboardData(uformat: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(GetClipboardData(::core::mem::transmute(uformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardFormatNameA(format: u32, lpszformatname: super::super::Foundation::PSTR, cchmaxcount: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClipboardFormatNameA(format: u32, lpszformatname: super::super::Foundation::PSTR, cchmaxcount: i32) -> i32;
        }
        ::core::mem::transmute(GetClipboardFormatNameA(::core::mem::transmute(format), ::core::mem::transmute(lpszformatname), ::core::mem::transmute(cchmaxcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardFormatNameW(format: u32, lpszformatname: super::super::Foundation::PWSTR, cchmaxcount: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClipboardFormatNameW(format: u32, lpszformatname: super::super::Foundation::PWSTR, cchmaxcount: i32) -> i32;
        }
        ::core::mem::transmute(GetClipboardFormatNameW(::core::mem::transmute(format), ::core::mem::transmute(lpszformatname), ::core::mem::transmute(cchmaxcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardOwner() -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClipboardOwner() -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(GetClipboardOwner())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetClipboardSequenceNumber() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClipboardSequenceNumber() -> u32;
        }
        ::core::mem::transmute(GetClipboardSequenceNumber())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClipboardViewer() -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClipboardViewer() -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(GetClipboardViewer())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenClipboardWindow() -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenClipboardWindow() -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(GetOpenClipboardWindow())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPriorityClipboardFormat(paformatprioritylist: *const u32, cformats: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPriorityClipboardFormat(paformatprioritylist: *const u32, cformats: i32) -> i32;
        }
        ::core::mem::transmute(GetPriorityClipboardFormat(::core::mem::transmute(paformatprioritylist), ::core::mem::transmute(cformats)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUpdatedClipboardFormats(lpuiformats: *mut u32, cformats: u32, pcformatsout: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUpdatedClipboardFormats(lpuiformats: *mut u32, cformats: u32, pcformatsout: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUpdatedClipboardFormats(::core::mem::transmute(lpuiformats), ::core::mem::transmute(cformats), ::core::mem::transmute(pcformatsout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalAddAtomA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalAddAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
        }
        ::core::mem::transmute(GlobalAddAtomA(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalAddAtomExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpstring: Param0, flags: u32) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalAddAtomExA(lpstring: super::super::Foundation::PSTR, flags: u32) -> u16;
        }
        ::core::mem::transmute(GlobalAddAtomExA(lpstring.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalAddAtomExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpstring: Param0, flags: u32) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalAddAtomExW(lpstring: super::super::Foundation::PWSTR, flags: u32) -> u16;
        }
        ::core::mem::transmute(GlobalAddAtomExW(lpstring.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalAddAtomW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalAddAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
        }
        ::core::mem::transmute(GlobalAddAtomW(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalDeleteAtom(natom: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalDeleteAtom(natom: u16) -> u16;
        }
        ::core::mem::transmute(GlobalDeleteAtom(::core::mem::transmute(natom)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalFindAtomA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFindAtomA(lpstring: super::super::Foundation::PSTR) -> u16;
        }
        ::core::mem::transmute(GlobalFindAtomA(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalFindAtomW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpstring: Param0) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFindAtomW(lpstring: super::super::Foundation::PWSTR) -> u16;
        }
        ::core::mem::transmute(GlobalFindAtomW(lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalGetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalGetAtomNameA(natom: u16, lpbuffer: super::super::Foundation::PSTR, nsize: i32) -> u32;
        }
        ::core::mem::transmute(GlobalGetAtomNameA(::core::mem::transmute(natom), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalGetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalGetAtomNameW(natom: u16, lpbuffer: super::super::Foundation::PWSTR, nsize: i32) -> u32;
        }
        ::core::mem::transmute(GlobalGetAtomNameW(::core::mem::transmute(natom), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HCONV(pub isize);
impl ::core::default::Default for HCONV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HCONV {}
unsafe impl ::windows::core::Abi for HCONV {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HCONVLIST(pub isize);
impl ::core::default::Default for HCONVLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HCONVLIST {}
unsafe impl ::windows::core::Abi for HCONVLIST {
    type Abi = Self;
}
pub const HDATA_APPOWNED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDDEDATA(pub isize);
impl ::core::default::Default for HDDEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HDDEDATA {}
unsafe impl ::windows::core::Abi for HDDEDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSZ(pub isize);
impl ::core::default::Default for HSZ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HSZ {}
unsafe impl ::windows::core::Abi for HSZ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct HSZPAIR {
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
}
impl HSZPAIR {}
impl ::core::default::Default for HSZPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HSZPAIR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HSZPAIR").field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).finish()
    }
}
impl ::core::cmp::PartialEq for HSZPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.hszSvc == other.hszSvc && self.hszTopic == other.hszTopic
    }
}
impl ::core::cmp::Eq for HSZPAIR {}
unsafe impl ::windows::core::Abi for HSZPAIR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImpersonateDdeClientWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndclient: Param0, hwndserver: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImpersonateDdeClientWindow(hwndclient: super::super::Foundation::HWND, hwndserver: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImpersonateDdeClientWindow(hwndclient.into_param().abi(), hwndserver.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitAtomTable(nsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitAtomTable(nsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitAtomTable(::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsClipboardFormatAvailable(format: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsClipboardFormatAvailable(format: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsClipboardFormatAvailable(::core::mem::transmute(format)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAX_MONITORS: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct METAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: super::super::Graphics::Gdi::HMETAFILE,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl METAFILEPICT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for METAFILEPICT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for METAFILEPICT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("METAFILEPICT").field("mm", &self.mm).field("xExt", &self.xExt).field("yExt", &self.yExt).field("hMF", &self.hMF).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for METAFILEPICT {
    fn eq(&self, other: &Self) -> bool {
        self.mm == other.mm && self.xExt == other.xExt && self.yExt == other.yExt && self.hMF == other.hMF
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for METAFILEPICT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for METAFILEPICT {
    type Abi = Self;
}
pub const MF_MASK: u32 = 4278190080u32;
pub const MH_CLEANUP: u32 = 4u32;
pub const MH_CREATE: u32 = 1u32;
pub const MH_DELETE: u32 = 3u32;
pub const MH_KEEP: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct MONCBSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub dwRet: u32,
    pub wType: u32,
    pub wFmt: u32,
    pub hConv: HCONV,
    pub hsz1: HSZ,
    pub hsz2: HSZ,
    pub hData: HDDEDATA,
    pub dwData1: usize,
    pub dwData2: usize,
    pub cc: CONVCONTEXT,
    pub cbData: u32,
    pub Data: [u32; 8],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl MONCBSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for MONCBSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for MONCBSTRUCT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MONCBSTRUCT")
            .field("cb", &self.cb)
            .field("dwTime", &self.dwTime)
            .field("hTask", &self.hTask)
            .field("dwRet", &self.dwRet)
            .field("wType", &self.wType)
            .field("wFmt", &self.wFmt)
            .field("hConv", &self.hConv)
            .field("hsz1", &self.hsz1)
            .field("hsz2", &self.hsz2)
            .field("hData", &self.hData)
            .field("dwData1", &self.dwData1)
            .field("dwData2", &self.dwData2)
            .field("cc", &self.cc)
            .field("cbData", &self.cbData)
            .field("Data", &self.Data)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for MONCBSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwTime == other.dwTime && self.hTask == other.hTask && self.dwRet == other.dwRet && self.wType == other.wType && self.wFmt == other.wFmt && self.hConv == other.hConv && self.hsz1 == other.hsz1 && self.hsz2 == other.hsz2 && self.hData == other.hData && self.dwData1 == other.dwData1 && self.dwData2 == other.dwData2 && self.cc == other.cc && self.cbData == other.cbData && self.Data == other.Data
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for MONCBSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for MONCBSTRUCT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MONCONVSTRUCT {
    pub cb: u32,
    pub fConnect: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hConvClient: HCONV,
    pub hConvServer: HCONV,
}
#[cfg(feature = "Win32_Foundation")]
impl MONCONVSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONCONVSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONCONVSTRUCT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MONCONVSTRUCT").field("cb", &self.cb).field("fConnect", &self.fConnect).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).field("hConvClient", &self.hConvClient).field("hConvServer", &self.hConvServer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONCONVSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fConnect == other.fConnect && self.dwTime == other.dwTime && self.hTask == other.hTask && self.hszSvc == other.hszSvc && self.hszTopic == other.hszTopic && self.hConvClient == other.hConvClient && self.hConvServer == other.hConvServer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONCONVSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONCONVSTRUCT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MONERRSTRUCT {
    pub cb: u32,
    pub wLastError: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl MONERRSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONERRSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONERRSTRUCT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MONERRSTRUCT").field("cb", &self.cb).field("wLastError", &self.wLastError).field("dwTime", &self.dwTime).field("hTask", &self.hTask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONERRSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.wLastError == other.wLastError && self.dwTime == other.dwTime && self.hTask == other.hTask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONERRSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONERRSTRUCT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MONHSZSTRUCTA {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl MONHSZSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONHSZSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONHSZSTRUCTA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MONHSZSTRUCTA").field("cb", &self.cb).field("fsAction", &self.fsAction).field("dwTime", &self.dwTime).field("hsz", &self.hsz).field("hTask", &self.hTask).field("str", &self.str).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONHSZSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fsAction == other.fsAction && self.dwTime == other.dwTime && self.hsz == other.hsz && self.hTask == other.hTask && self.str == other.str
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONHSZSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONHSZSTRUCTA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MONHSZSTRUCTW {
    pub cb: u32,
    pub fsAction: super::super::Foundation::BOOL,
    pub dwTime: u32,
    pub hsz: HSZ,
    pub hTask: super::super::Foundation::HANDLE,
    pub str: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl MONHSZSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONHSZSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONHSZSTRUCTW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MONHSZSTRUCTW").field("cb", &self.cb).field("fsAction", &self.fsAction).field("dwTime", &self.dwTime).field("hsz", &self.hsz).field("hTask", &self.hTask).field("str", &self.str).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONHSZSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fsAction == other.fsAction && self.dwTime == other.dwTime && self.hsz == other.hsz && self.hTask == other.hTask && self.str == other.str
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONHSZSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONHSZSTRUCTW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MONLINKSTRUCT {
    pub cb: u32,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub fEstablished: super::super::Foundation::BOOL,
    pub fNoData: super::super::Foundation::BOOL,
    pub hszSvc: HSZ,
    pub hszTopic: HSZ,
    pub hszItem: HSZ,
    pub wFmt: u32,
    pub fServer: super::super::Foundation::BOOL,
    pub hConvServer: HCONV,
    pub hConvClient: HCONV,
}
#[cfg(feature = "Win32_Foundation")]
impl MONLINKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONLINKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONLINKSTRUCT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MONLINKSTRUCT")
            .field("cb", &self.cb)
            .field("dwTime", &self.dwTime)
            .field("hTask", &self.hTask)
            .field("fEstablished", &self.fEstablished)
            .field("fNoData", &self.fNoData)
            .field("hszSvc", &self.hszSvc)
            .field("hszTopic", &self.hszTopic)
            .field("hszItem", &self.hszItem)
            .field("wFmt", &self.wFmt)
            .field("fServer", &self.fServer)
            .field("hConvServer", &self.hConvServer)
            .field("hConvClient", &self.hConvClient)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONLINKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwTime == other.dwTime && self.hTask == other.hTask && self.fEstablished == other.fEstablished && self.fNoData == other.fNoData && self.hszSvc == other.hszSvc && self.hszTopic == other.hszTopic && self.hszItem == other.hszItem && self.wFmt == other.wFmt && self.fServer == other.fServer && self.hConvServer == other.hConvServer && self.hConvClient == other.hConvClient
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONLINKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONLINKSTRUCT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MONMSGSTRUCT {
    pub cb: u32,
    pub hwndTo: super::super::Foundation::HWND,
    pub dwTime: u32,
    pub hTask: super::super::Foundation::HANDLE,
    pub wMsg: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub dmhd: DDEML_MSG_HOOK_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl MONMSGSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONMSGSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONMSGSTRUCT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MONMSGSTRUCT").field("cb", &self.cb).field("hwndTo", &self.hwndTo).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("wMsg", &self.wMsg).field("wParam", &self.wParam).field("lParam", &self.lParam).field("dmhd", &self.dmhd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONMSGSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.hwndTo == other.hwndTo && self.dwTime == other.dwTime && self.hTask == other.hTask && self.wMsg == other.wMsg && self.wParam == other.wParam && self.lParam == other.lParam && self.dmhd == other.dmhd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONMSGSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MONMSGSTRUCT {
    type Abi = Self;
}
pub const MSGF_DDEMGR: u32 = 32769u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenClipboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndnewowner: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenClipboard(hwndnewowner: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OpenClipboard(hwndnewowner.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type PFNCALLBACK = unsafe extern "system" fn(wtype: u32, wfmt: u32, hconv: HCONV, hsz1: HSZ, hsz2: HSZ, hdata: HDDEDATA, dwdata1: usize, dwdata2: usize) -> HDDEDATA;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackDDElParam(msg: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
        }
        ::core::mem::transmute(PackDDElParam(::core::mem::transmute(msg), ::core::mem::transmute(uilo), ::core::mem::transmute(uihi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const QID_SYNC: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterClipboardFormatA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszformat: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterClipboardFormatA(lpszformat: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(RegisterClipboardFormatA(lpszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterClipboardFormatW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszformat: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterClipboardFormatW(lpszformat: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(RegisterClipboardFormatW(lpszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveClipboardFormatListener<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveClipboardFormatListener(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RemoveClipboardFormatListener(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReuseDDElParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(lparam: Param0, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReuseDDElParam(lparam: super::super::Foundation::LPARAM, msgin: u32, msgout: u32, uilo: usize, uihi: usize) -> super::super::Foundation::LPARAM;
        }
        ::core::mem::transmute(ReuseDDElParam(lparam.into_param().abi(), ::core::mem::transmute(msgin), ::core::mem::transmute(msgout), ::core::mem::transmute(uilo), ::core::mem::transmute(uihi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClipboardData<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(uformat: u32, hmem: Param1) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetClipboardData(uformat: u32, hmem: super::super::Foundation::HANDLE) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(SetClipboardData(::core::mem::transmute(uformat), hmem.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetClipboardViewer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndnewviewer: Param0) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetClipboardViewer(hwndnewviewer: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(SetClipboardViewer(hwndnewviewer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetWinMetaFileBits<'a, Param2: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(nsize: u32, lpmeta16data: *const u8, hdcref: Param2, lpmfp: *const METAFILEPICT) -> super::super::Graphics::Gdi::HENHMETAFILE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWinMetaFileBits(nsize: u32, lpmeta16data: *const u8, hdcref: super::super::Graphics::Gdi::HDC, lpmfp: *const METAFILEPICT) -> super::super::Graphics::Gdi::HENHMETAFILE;
        }
        ::core::mem::transmute(SetWinMetaFileBits(::core::mem::transmute(nsize), ::core::mem::transmute(lpmeta16data), hdcref.into_param().abi(), ::core::mem::transmute(lpmfp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const TIMEOUT_ASYNC: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnpackDDElParam<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(msg: u32, lparam: Param1, puilo: *mut usize, puihi: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnpackDDElParam(msg: u32, lparam: super::super::Foundation::LPARAM, puilo: *mut usize, puihi: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnpackDDElParam(::core::mem::transmute(msg), lparam.into_param().abi(), ::core::mem::transmute(puilo), ::core::mem::transmute(puihi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WM_DDE_ACK: u32 = 996u32;
pub const WM_DDE_ADVISE: u32 = 994u32;
pub const WM_DDE_DATA: u32 = 997u32;
pub const WM_DDE_EXECUTE: u32 = 1000u32;
pub const WM_DDE_FIRST: u32 = 992u32;
pub const WM_DDE_INITIATE: u32 = 992u32;
pub const WM_DDE_LAST: u32 = 1000u32;
pub const WM_DDE_POKE: u32 = 999u32;
pub const WM_DDE_REQUEST: u32 = 998u32;
pub const WM_DDE_TERMINATE: u32 = 993u32;
pub const WM_DDE_UNADVISE: u32 = 995u32;
pub const XCLASS_BOOL: u32 = 4096u32;
pub const XCLASS_DATA: u32 = 8192u32;
pub const XCLASS_FLAGS: u32 = 16384u32;
pub const XCLASS_MASK: u32 = 64512u32;
pub const XCLASS_NOTIFICATION: u32 = 32768u32;
pub const XTYPF_ACKREQ: u32 = 8u32;
pub const XTYPF_NOBLOCK: u32 = 2u32;
pub const XTYPF_NODATA: u32 = 4u32;
pub const XTYP_MASK: u32 = 240u32;
pub const XTYP_SHIFT: u32 = 4u32;
