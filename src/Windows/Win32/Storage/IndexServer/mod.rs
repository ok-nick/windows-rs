#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn BindIFilterFromStorage<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::StructuredStorage::IStorage>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pstg: Param0, punkouter: Param1, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindIFilterFromStorage(pstg: ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        BindIFilterFromStorage(pstg.into_param().abi(), punkouter.into_param().abi(), ::core::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn BindIFilterFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pstm: Param0, punkouter: Param1, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindIFilterFromStream(pstm: ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        BindIFilterFromStream(pstm.into_param().abi(), punkouter.into_param().abi(), ::core::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHUNKSTATE(pub i32);
pub const CHUNK_TEXT: CHUNKSTATE = CHUNKSTATE(1i32);
pub const CHUNK_VALUE: CHUNKSTATE = CHUNKSTATE(2i32);
pub const CHUNK_FILTER_OWNED_VALUE: CHUNKSTATE = CHUNKSTATE(4i32);
impl ::core::convert::From<i32> for CHUNKSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CHUNKSTATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHUNK_BREAKTYPE(pub i32);
pub const CHUNK_NO_BREAK: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(0i32);
pub const CHUNK_EOW: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(1i32);
pub const CHUNK_EOS: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(2i32);
pub const CHUNK_EOP: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(3i32);
pub const CHUNK_EOC: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(4i32);
impl ::core::convert::From<i32> for CHUNK_BREAKTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CHUNK_BREAKTYPE {
    type Abi = Self;
}
pub const CICAT_ALL_OPENED: u32 = 32u32;
pub const CICAT_GET_STATE: u32 = 16u32;
pub const CICAT_NO_QUERY: u32 = 8u32;
pub const CICAT_READONLY: u32 = 2u32;
pub const CICAT_STOPPED: u32 = 1u32;
pub const CICAT_WRITABLE: u32 = 4u32;
pub const CI_PROVIDER_ALL: u32 = 4294967295u32;
pub const CI_PROVIDER_INDEXING_SERVICE: u32 = 2u32;
pub const CI_PROVIDER_MSSEARCH: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CI_STATE {
    pub cbStruct: u32,
    pub cWordList: u32,
    pub cPersistentIndex: u32,
    pub cQueries: u32,
    pub cDocuments: u32,
    pub cFreshTest: u32,
    pub dwMergeProgress: u32,
    pub eState: u32,
    pub cFilteredDocuments: u32,
    pub cTotalDocuments: u32,
    pub cPendingScans: u32,
    pub dwIndexSize: u32,
    pub cUniqueKeys: u32,
    pub cSecQDocuments: u32,
    pub dwPropCacheSize: u32,
}
impl CI_STATE {}
impl ::core::default::Default for CI_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CI_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CI_STATE")
            .field("cbStruct", &self.cbStruct)
            .field("cWordList", &self.cWordList)
            .field("cPersistentIndex", &self.cPersistentIndex)
            .field("cQueries", &self.cQueries)
            .field("cDocuments", &self.cDocuments)
            .field("cFreshTest", &self.cFreshTest)
            .field("dwMergeProgress", &self.dwMergeProgress)
            .field("eState", &self.eState)
            .field("cFilteredDocuments", &self.cFilteredDocuments)
            .field("cTotalDocuments", &self.cTotalDocuments)
            .field("cPendingScans", &self.cPendingScans)
            .field("dwIndexSize", &self.dwIndexSize)
            .field("cUniqueKeys", &self.cUniqueKeys)
            .field("cSecQDocuments", &self.cSecQDocuments)
            .field("dwPropCacheSize", &self.dwPropCacheSize)
            .finish()
    }
}
impl ::core::cmp::PartialEq for CI_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.cWordList == other.cWordList
            && self.cPersistentIndex == other.cPersistentIndex
            && self.cQueries == other.cQueries
            && self.cDocuments == other.cDocuments
            && self.cFreshTest == other.cFreshTest
            && self.dwMergeProgress == other.dwMergeProgress
            && self.eState == other.eState
            && self.cFilteredDocuments == other.cFilteredDocuments
            && self.cTotalDocuments == other.cTotalDocuments
            && self.cPendingScans == other.cPendingScans
            && self.dwIndexSize == other.dwIndexSize
            && self.cUniqueKeys == other.cUniqueKeys
            && self.cSecQDocuments == other.cSecQDocuments
            && self.dwPropCacheSize == other.dwPropCacheSize
    }
}
impl ::core::cmp::Eq for CI_STATE {}
unsafe impl ::windows::core::Abi for CI_STATE {
    type Abi = Self;
}
pub const CI_STATE_ANNEALING_MERGE: u32 = 8u32;
pub const CI_STATE_BATTERY_POLICY: u32 = 262144u32;
pub const CI_STATE_BATTERY_POWER: u32 = 2048u32;
pub const CI_STATE_CONTENT_SCAN_REQUIRED: u32 = 4u32;
pub const CI_STATE_DELETION_MERGE: u32 = 32768u32;
pub const CI_STATE_HIGH_CPU: u32 = 131072u32;
pub const CI_STATE_HIGH_IO: u32 = 256u32;
pub const CI_STATE_INDEX_MIGRATION_MERGE: u32 = 64u32;
pub const CI_STATE_LOW_DISK: u32 = 65536u32;
pub const CI_STATE_LOW_MEMORY: u32 = 128u32;
pub const CI_STATE_MASTER_MERGE: u32 = 2u32;
pub const CI_STATE_MASTER_MERGE_PAUSED: u32 = 512u32;
pub const CI_STATE_READING_USNS: u32 = 16384u32;
pub const CI_STATE_READ_ONLY: u32 = 1024u32;
pub const CI_STATE_RECOVERING: u32 = 32u32;
pub const CI_STATE_SCANNING: u32 = 16u32;
pub const CI_STATE_SHADOW_MERGE: u32 = 1u32;
pub const CI_STATE_STARTING: u32 = 8192u32;
pub const CI_STATE_USER_ACTIVE: u32 = 4096u32;
pub const CI_VERSION_WDS30: u32 = 258u32;
pub const CI_VERSION_WDS40: u32 = 265u32;
pub const CI_VERSION_WIN70: u32 = 1792u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: u32,
    pub uName: DBID_1,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl DBID {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DBID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DBID {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DBID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub union DBID_0 {
    pub guid: ::windows::core::GUID,
    pub pguid: *mut ::windows::core::GUID,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl DBID_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DBID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DBID_0 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DBID_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub union DBID_1 {
    pub pwszName: super::super::Foundation::PWSTR,
    pub ulPropid: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl DBID_1 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBID_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DBID_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DBID_1 {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DBID_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(2))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: u32,
    pub uName: DBID_1,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl DBID {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DBID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DBID {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DBID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(2))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub union DBID_0 {
    pub guid: ::windows::core::GUID,
    pub pguid: *mut ::windows::core::GUID,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl DBID_0 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DBID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DBID_0 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DBID_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(2))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub union DBID_1 {
    pub pwszName: super::super::Foundation::PWSTR,
    pub ulPropid: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl DBID_1 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBID_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DBID_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DBID_1 {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DBID_1 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DBKINDENUM(pub i32);
pub const DBKIND_GUID_NAME: DBKINDENUM = DBKINDENUM(0i32);
pub const DBKIND_GUID_PROPID: DBKINDENUM = DBKINDENUM(1i32);
pub const DBKIND_NAME: DBKINDENUM = DBKINDENUM(2i32);
pub const DBKIND_PGUID_NAME: DBKINDENUM = DBKINDENUM(3i32);
pub const DBKIND_PGUID_PROPID: DBKINDENUM = DBKINDENUM(4i32);
pub const DBKIND_PROPID: DBKINDENUM = DBKINDENUM(5i32);
pub const DBKIND_GUID: DBKINDENUM = DBKINDENUM(6i32);
impl ::core::convert::From<i32> for DBKINDENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DBKINDENUM {
    type Abi = Self;
}
pub const DBPROP_APPLICATION_NAME: u32 = 11u32;
pub const DBPROP_CATALOGLISTID: u32 = 9u32;
pub const DBPROP_CI_CATALOG_NAME: u32 = 2u32;
pub const DBPROP_CI_DEPTHS: u32 = 4u32;
pub const DBPROP_CI_EXCLUDE_SCOPES: u32 = 5u32;
pub const DBPROP_CI_INCLUDE_SCOPES: u32 = 3u32;
pub const DBPROP_CI_PROVIDER: u32 = 8u32;
pub const DBPROP_CI_QUERY_TYPE: u32 = 7u32;
pub const DBPROP_CI_SCOPE_FLAGS: u32 = 4u32;
pub const DBPROP_CI_SECURITY_ID: u32 = 6u32;
pub const DBPROP_CLIENT_CLSID: u32 = 3u32;
pub const DBPROP_DEFAULT_EQUALS_BEHAVIOR: u32 = 2u32;
pub const DBPROP_DEFERCATALOGVERIFICATION: u32 = 8u32;
pub const DBPROP_DEFERNONINDEXEDTRIMMING: u32 = 3u32;
pub const DBPROP_DONOTCOMPUTEEXPENSIVEPROPS: u32 = 15u32;
pub const DBPROP_ENABLEROWSETEVENTS: u32 = 16u32;
pub const DBPROP_FIRSTROWS: u32 = 7u32;
pub const DBPROP_FREETEXTANYTERM: u32 = 12u32;
pub const DBPROP_FREETEXTUSESTEMMING: u32 = 13u32;
pub const DBPROP_GENERATEPARSETREE: u32 = 10u32;
pub const DBPROP_GENERICOPTIONS_STRING: u32 = 6u32;
pub const DBPROP_IGNORENOISEONLYCLAUSES: u32 = 5u32;
pub const DBPROP_IGNORESBRI: u32 = 14u32;
pub const DBPROP_MACHINE: u32 = 2u32;
pub const DBPROP_USECONTENTINDEX: u32 = 2u32;
pub const DBPROP_USEEXTENDEDDBTYPES: u32 = 4u32;
pub const DBSETFUNC_ALL: u32 = 1u32;
pub const DBSETFUNC_DISTINCT: u32 = 2u32;
pub const DBSETFUNC_NONE: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FILTERREGION {
    pub idChunk: u32,
    pub cwcStart: u32,
    pub cwcExtent: u32,
}
impl FILTERREGION {}
impl ::core::default::Default for FILTERREGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FILTERREGION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FILTERREGION").field("idChunk", &self.idChunk).field("cwcStart", &self.cwcStart).field("cwcExtent", &self.cwcExtent).finish()
    }
}
impl ::core::cmp::PartialEq for FILTERREGION {
    fn eq(&self, other: &Self) -> bool {
        self.idChunk == other.idChunk && self.cwcStart == other.cwcStart && self.cwcExtent == other.cwcExtent
    }
}
impl ::core::cmp::Eq for FILTERREGION {}
unsafe impl ::windows::core::Abi for FILTERREGION {
    type Abi = Self;
}
pub const FILTER_E_ACCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215613i32 as _);
pub const FILTER_E_EMBEDDING_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215609i32 as _);
pub const FILTER_E_END_OF_CHUNKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215616i32 as _);
pub const FILTER_E_LINK_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215608i32 as _);
pub const FILTER_E_NO_MORE_TEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215615i32 as _);
pub const FILTER_E_NO_MORE_VALUES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215614i32 as _);
pub const FILTER_E_NO_TEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215611i32 as _);
pub const FILTER_E_NO_VALUES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215610i32 as _);
pub const FILTER_E_PASSWORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215605i32 as _);
pub const FILTER_E_UNKNOWNFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147215604i32 as _);
pub const FILTER_S_LAST_TEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(268041i32 as _);
pub const FILTER_S_LAST_VALUES: ::windows::core::HRESULT = ::windows::core::HRESULT(268042i32 as _);
pub const FILTER_W_MONIKER_CLIPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(268036i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FULLPROPSPEC {
    pub guidPropSet: ::windows::core::GUID,
    pub psProperty: super::super::System::Com::StructuredStorage::PROPSPEC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl FULLPROPSPEC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for FULLPROPSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for FULLPROPSPEC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for FULLPROPSPEC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for FULLPROPSPEC {
    type Abi = Self;
}
pub const GENERATE_METHOD_EXACT: u32 = 0u32;
pub const GENERATE_METHOD_INFLECT: u32 = 2u32;
pub const GENERATE_METHOD_PREFIX: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IFILTER_FLAGS(pub i32);
pub const IFILTER_FLAGS_OLE_PROPERTIES: IFILTER_FLAGS = IFILTER_FLAGS(1i32);
impl ::core::convert::From<i32> for IFILTER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IFILTER_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IFILTER_INIT(pub i32);
pub const IFILTER_INIT_CANON_PARAGRAPHS: IFILTER_INIT = IFILTER_INIT(1i32);
pub const IFILTER_INIT_HARD_LINE_BREAKS: IFILTER_INIT = IFILTER_INIT(2i32);
pub const IFILTER_INIT_CANON_HYPHENS: IFILTER_INIT = IFILTER_INIT(4i32);
pub const IFILTER_INIT_CANON_SPACES: IFILTER_INIT = IFILTER_INIT(8i32);
pub const IFILTER_INIT_APPLY_INDEX_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(16i32);
pub const IFILTER_INIT_APPLY_OTHER_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(32i32);
pub const IFILTER_INIT_APPLY_CRAWL_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(256i32);
pub const IFILTER_INIT_INDEXING_ONLY: IFILTER_INIT = IFILTER_INIT(64i32);
pub const IFILTER_INIT_SEARCH_LINKS: IFILTER_INIT = IFILTER_INIT(128i32);
pub const IFILTER_INIT_FILTER_OWNED_VALUE_OK: IFILTER_INIT = IFILTER_INIT(512i32);
pub const IFILTER_INIT_FILTER_AGGRESSIVE_BREAK: IFILTER_INIT = IFILTER_INIT(1024i32);
pub const IFILTER_INIT_DISABLE_EMBEDDED: IFILTER_INIT = IFILTER_INIT(2048i32);
pub const IFILTER_INIT_EMIT_FORMATTING: IFILTER_INIT = IFILTER_INIT(4096i32);
impl ::core::convert::From<i32> for IFILTER_INIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IFILTER_INIT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFilter(pub ::windows::core::IUnknown);
impl IFilter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Init(&self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfflags), ::core::mem::transmute(cattributes), ::core::mem::transmute(aattributes), ::core::mem::transmute(pflags)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstat)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcwcbuffer), ::core::mem::transmute(awcbuffer)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppropvalue)))
    }
    pub unsafe fn BindRegion<'a, Param0: ::windows::core::IntoParam<'a, FILTERREGION>>(&self, origpos: Param0, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), origpos.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)))
    }
}
unsafe impl ::windows::core::Interface for IFilter {
    type Vtable = IFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89bcb740_6119_101a_bcb7_00dd010655af);
}
impl ::core::convert::From<IFilter> for ::windows::core::IUnknown {
    fn from(value: IFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFilter> for ::windows::core::IUnknown {
    fn from(value: &IFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstat: *mut STAT_CHUNK) -> i32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, origpos: FILTERREGION, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPhraseSink(pub ::windows::core::IUnknown);
impl IPhraseSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutSmallPhrase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcnoun: Param0, cwcnoun: u32, pwcmodifier: Param2, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwcnoun.into_param().abi(), ::core::mem::transmute(cwcnoun), pwcmodifier.into_param().abi(), ::core::mem::transmute(cwcmodifier), ::core::mem::transmute(ulattachmenttype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutPhrase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcphrase: Param0, cwcphrase: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwcphrase.into_param().abi(), ::core::mem::transmute(cwcphrase)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPhraseSink {
    type Vtable = IPhraseSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc906ff0_c058_101a_b554_08002b33b0e6);
}
impl ::core::convert::From<IPhraseSink> for ::windows::core::IUnknown {
    fn from(value: IPhraseSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPhraseSink> for ::windows::core::IUnknown {
    fn from(value: &IPhraseSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPhraseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPhraseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhraseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwcphrase: super::super::Foundation::PWSTR, cwcphrase: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const LIFF_FORCE_TEXT_FILTER_FALLBACK: u32 = 3u32;
pub const LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY: u32 = 2u32;
pub const LIFF_LOAD_DEFINED_FILTER: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadIFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pwcspath: Param0, punkouter: Param1, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIFilter(pwcspath: super::super::Foundation::PWSTR, punkouter: ::windows::core::RawPtr, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        LoadIFilter(pwcspath.into_param().abi(), punkouter.into_param().abi(), ::core::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadIFilterEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwcspath: Param0, dwflags: u32, riid: *const ::windows::core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIFilterEx(pwcspath: super::super::Foundation::PWSTR, dwflags: u32, riid: *const ::windows::core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        LoadIFilterEx(pwcspath.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(riid), ::core::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MSIDXSPROP_COMMAND_LOCALE_STRING: u32 = 3u32;
pub const MSIDXSPROP_MAX_RANK: u32 = 6u32;
pub const MSIDXSPROP_PARSE_TREE: u32 = 5u32;
pub const MSIDXSPROP_QUERY_RESTRICTION: u32 = 4u32;
pub const MSIDXSPROP_RESULTS_FOUND: u32 = 7u32;
pub const MSIDXSPROP_ROWSETQUERYSTATUS: u32 = 2u32;
pub const MSIDXSPROP_SAME_SORTORDER_USED: u32 = 14u32;
pub const MSIDXSPROP_SERVER_NLSVERSION: u32 = 12u32;
pub const MSIDXSPROP_SERVER_NLSVER_DEFINED: u32 = 13u32;
pub const MSIDXSPROP_SERVER_VERSION: u32 = 9u32;
pub const MSIDXSPROP_SERVER_WINVER_MAJOR: u32 = 10u32;
pub const MSIDXSPROP_SERVER_WINVER_MINOR: u32 = 11u32;
pub const MSIDXSPROP_WHEREID: u32 = 8u32;
pub const NOT_AN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(524288i32 as _);
pub const PID_FILENAME: u32 = 100u32;
pub const PROPID_QUERY_ALL: u32 = 6u32;
pub const PROPID_QUERY_HITCOUNT: u32 = 4u32;
pub const PROPID_QUERY_LASTSEENTIME: u32 = 10u32;
pub const PROPID_QUERY_RANK: u32 = 3u32;
pub const PROPID_QUERY_RANKVECTOR: u32 = 2u32;
pub const PROPID_QUERY_UNFILTERED: u32 = 7u32;
pub const PROPID_QUERY_VIRTUALPATH: u32 = 9u32;
pub const PROPID_QUERY_WORKID: u32 = 5u32;
pub const PROPID_STG_CONTENTS: u32 = 19u32;
pub const PROXIMITY_UNIT_CHAPTER: u32 = 3u32;
pub const PROXIMITY_UNIT_PARAGRAPH: u32 = 2u32;
pub const PROXIMITY_UNIT_SENTENCE: u32 = 1u32;
pub const PROXIMITY_UNIT_WORD: u32 = 0u32;
pub const QUERY_DEEP: u32 = 1u32;
pub const QUERY_PHYSICAL_PATH: u32 = 0u32;
pub const QUERY_SHALLOW: u32 = 0u32;
pub const QUERY_VIRTUAL_PATH: u32 = 2u32;
pub const SCOPE_FLAG_DEEP: u32 = 2u32;
pub const SCOPE_FLAG_INCLUDE: u32 = 1u32;
pub const SCOPE_FLAG_MASK: u32 = 255u32;
pub const SCOPE_TYPE_MASK: u32 = 4294967040u32;
pub const SCOPE_TYPE_VPATH: u32 = 512u32;
pub const SCOPE_TYPE_WINPATH: u32 = 256u32;
pub const STAT_BUSY: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STAT_CHUNK {
    pub idChunk: u32,
    pub breakType: CHUNK_BREAKTYPE,
    pub flags: CHUNKSTATE,
    pub locale: u32,
    pub attribute: FULLPROPSPEC,
    pub idChunkSource: u32,
    pub cwcStartSource: u32,
    pub cwcLenSource: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl STAT_CHUNK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STAT_CHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STAT_CHUNK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STAT_CHUNK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for STAT_CHUNK {
    type Abi = Self;
}
pub const STAT_COALESCE_COMP_ALL_NOISE: u32 = 8192u32;
pub const STAT_CONTENT_OUT_OF_DATE: u32 = 32u32;
pub const STAT_CONTENT_QUERY_INCOMPLETE: u32 = 128u32;
pub const STAT_DONE: u32 = 2u32;
pub const STAT_ERROR: u32 = 1u32;
pub const STAT_MISSING_PROP_IN_RELDOC: u32 = 2048u32;
pub const STAT_MISSING_RELDOC: u32 = 1024u32;
pub const STAT_NOISE_WORDS: u32 = 16u32;
pub const STAT_PARTIAL_SCOPE: u32 = 8u32;
pub const STAT_REFRESH: u32 = 3u32;
pub const STAT_REFRESH_INCOMPLETE: u32 = 64u32;
pub const STAT_RELDOC_ACCESS_DENIED: u32 = 4096u32;
pub const STAT_SHARING_VIOLATION: u32 = 512u32;
pub const STAT_TIME_LIMIT_EXCEEDED: u32 = 256u32;
pub const VECTOR_RANK_DICE: u32 = 3u32;
pub const VECTOR_RANK_INNER: u32 = 2u32;
pub const VECTOR_RANK_JACCARD: u32 = 4u32;
pub const VECTOR_RANK_MAX: u32 = 1u32;
pub const VECTOR_RANK_MIN: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WORDREP_BREAK_TYPE(pub i32);
pub const WORDREP_BREAK_EOW: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(0i32);
pub const WORDREP_BREAK_EOS: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(1i32);
pub const WORDREP_BREAK_EOP: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(2i32);
pub const WORDREP_BREAK_EOC: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(3i32);
impl ::core::convert::From<i32> for WORDREP_BREAK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WORDREP_BREAK_TYPE {
    type Abi = Self;
}
