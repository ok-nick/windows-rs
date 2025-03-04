#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
impl ::core::convert::From<i32> for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(pub i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(0i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(1i32);
impl ::core::convert::From<i32> for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_CAPABILITIES(pub u32);
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = APPX_CAPABILITIES(1u32);
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(2u32);
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(4u32);
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(8u32);
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(16u32);
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(32u32);
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(64u32);
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = APPX_CAPABILITIES(128u32);
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = APPX_CAPABILITIES(256u32);
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = APPX_CAPABILITIES(512u32);
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = APPX_CAPABILITIES(1024u32);
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = APPX_CAPABILITIES(2048u32);
impl ::core::convert::From<u32> for APPX_CAPABILITIES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_CAPABILITIES {
    type Abi = Self;
}
impl ::core::ops::BitOr for APPX_CAPABILITIES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for APPX_CAPABILITIES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for APPX_CAPABILITIES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for APPX_CAPABILITIES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for APPX_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_CAPABILITY_CLASS_TYPE(pub i32);
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(0i32);
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(1i32);
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(2i32);
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(4i32);
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(7i32);
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(8i32);
impl ::core::convert::From<i32> for APPX_CAPABILITY_CLASS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_CAPABILITY_CLASS_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_COMPRESSION_OPTION(pub i32);
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(0i32);
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(1i32);
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(2i32);
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(3i32);
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(4i32);
impl ::core::convert::From<i32> for APPX_COMPRESSION_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_COMPRESSION_OPTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *mut super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl APPX_ENCRYPTED_EXEMPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APPX_ENCRYPTED_EXEMPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APPX_ENCRYPTED_EXEMPTIONS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APPX_ENCRYPTED_EXEMPTIONS").field("count", &self.count).field("plainTextFiles", &self.plainTextFiles).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APPX_ENCRYPTED_EXEMPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.plainTextFiles == other.plainTextFiles
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APPX_ENCRYPTED_EXEMPTIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_EXEMPTIONS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(pub u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(0u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(1u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(2u32);
impl ::core::convert::From<u32> for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: super::super::super::Foundation::PWSTR,
    pub useDiffusion: super::super::super::Foundation::BOOL,
    pub blockMapHashAlgorithm: ::core::option::Option<super::super::super::System::Com::IUri>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APPX_ENCRYPTED_PACKAGE_SETTINGS").field("keyLength", &self.keyLength).field("encryptionAlgorithm", &self.encryptionAlgorithm).field("useDiffusion", &self.useDiffusion).field("blockMapHashAlgorithm", &self.blockMapHashAlgorithm).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.encryptionAlgorithm == other.encryptionAlgorithm && self.useDiffusion == other.useDiffusion && self.blockMapHashAlgorithm == other.blockMapHashAlgorithm
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: super::super::super::Foundation::PWSTR,
    pub blockMapHashAlgorithm: ::core::option::Option<super::super::super::System::Com::IUri>,
    pub options: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APPX_ENCRYPTED_PACKAGE_SETTINGS2").field("keyLength", &self.keyLength).field("encryptionAlgorithm", &self.encryptionAlgorithm).field("blockMapHashAlgorithm", &self.blockMapHashAlgorithm).field("options", &self.options).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.encryptionAlgorithm == other.encryptionAlgorithm && self.blockMapHashAlgorithm == other.blockMapHashAlgorithm && self.options == other.options
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(3i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(4i32);
impl ::core::convert::From<i32> for APPX_FOOTPRINT_FILE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_FOOTPRINT_FILE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl APPX_KEY_INFO {}
impl ::core::default::Default for APPX_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for APPX_KEY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APPX_KEY_INFO").field("keyLength", &self.keyLength).field("keyIdLength", &self.keyIdLength).field("key", &self.key).field("keyId", &self.keyId).finish()
    }
}
impl ::core::cmp::PartialEq for APPX_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.keyIdLength == other.keyIdLength && self.key == other.key && self.keyId == other.keyId
    }
}
impl ::core::cmp::Eq for APPX_KEY_INFO {}
unsafe impl ::windows::core::Abi for APPX_KEY_INFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_ARCHITECTURE(pub i32);
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(0i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(5i32);
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(9i32);
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(11i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(12i32);
impl ::core::convert::From<i32> for APPX_PACKAGE_ARCHITECTURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_ARCHITECTURE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_ARCHITECTURE2(pub i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(0i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(5i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(9i32);
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(11i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(12i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(14i32);
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(65535i32);
impl ::core::convert::From<i32> for APPX_PACKAGE_ARCHITECTURE2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_ARCHITECTURE2 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(pub u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(0u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(1u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(2u32);
impl ::core::convert::From<u32> for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(pub i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(0i32);
impl ::core::convert::From<i32> for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: super::super::super::Foundation::BOOL,
    pub hashMethod: ::core::option::Option<super::super::super::System::Com::IUri>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for APPX_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for APPX_PACKAGE_SETTINGS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APPX_PACKAGE_SETTINGS").field("forceZip32", &self.forceZip32).field("hashMethod", &self.hashMethod).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for APPX_PACKAGE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.forceZip32 == other.forceZip32 && self.hashMethod == other.hashMethod
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for APPX_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for APPX_PACKAGE_SETTINGS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: ::core::option::Option<super::super::super::System::Com::IStream>,
    pub fileName: super::super::super::Foundation::PWSTR,
    pub contentType: super::super::super::Foundation::PWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APPX_PACKAGE_WRITER_PAYLOAD_STREAM").field("inputStream", &self.inputStream).field("fileName", &self.fileName).field("contentType", &self.contentType).field("compressionOption", &self.compressionOption).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.inputStream == other.inputStream && self.fileName == other.fileName && self.contentType == other.contentType && self.compressionOption == other.compressionOption
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(pub i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(0i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(1i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(2i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(3i32);
impl ::core::convert::From<i32> for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    type Abi = Self;
}
#[inline]
pub unsafe fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::core::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, cookie: *mut usize) -> ::windows::core::HRESULT;
        }
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        ActivatePackageVirtualizationContext(::core::mem::transmute(context), &mut result__).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddPackageDependency<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagedependencyid: Param0, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        AddPackageDependency(packagedependencyid.into_param().abi(), ::core::mem::transmute(rank), ::core::mem::transmute(options), ::core::mem::transmute(packagedependencycontext), ::core::mem::transmute(packagefullname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AddPackageDependencyOptions(pub i32);
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = AddPackageDependencyOptions(0i32);
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = AddPackageDependencyOptions(1i32);
impl ::core::convert::From<i32> for AddPackageDependencyOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AddPackageDependencyOptions {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyClrCompat(pub i32);
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = AppPolicyClrCompat(0i32);
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = AppPolicyClrCompat(1i32);
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = AppPolicyClrCompat(2i32);
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = AppPolicyClrCompat(3i32);
impl ::core::convert::From<i32> for AppPolicyClrCompat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyClrCompat {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyCreateFileAccess(pub i32);
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(0i32);
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(1i32);
impl ::core::convert::From<i32> for AppPolicyCreateFileAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyCreateFileAccess {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetClrCompat<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyClrCompat) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetClrCompat(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyClrCompat) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetClrCompat(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetCreateFileAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyCreateFileAccess) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetCreateFileAccess(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetCreateFileAccess(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetLifecycleManagement<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyLifecycleManagement) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetLifecycleManagement(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetLifecycleManagement(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetMediaFoundationCodecLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyMediaFoundationCodecLoading) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetMediaFoundationCodecLoading(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetMediaFoundationCodecLoading(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetProcessTerminationMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyProcessTerminationMethod) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetProcessTerminationMethod(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetProcessTerminationMethod(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetShowDeveloperDiagnostic<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyShowDeveloperDiagnostic) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetShowDeveloperDiagnostic(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetShowDeveloperDiagnostic(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetThreadInitializationType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyThreadInitializationType) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetThreadInitializationType(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetThreadInitializationType(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetWindowingModel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(processtoken: Param0, policy: *mut AppPolicyWindowingModel) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppPolicyGetWindowingModel(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyWindowingModel) -> i32;
        }
        ::core::mem::transmute(AppPolicyGetWindowingModel(processtoken.into_param().abi(), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyLifecycleManagement(pub i32);
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(0i32);
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(1i32);
impl ::core::convert::From<i32> for AppPolicyLifecycleManagement {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyLifecycleManagement {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyMediaFoundationCodecLoading(pub i32);
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(0i32);
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(1i32);
impl ::core::convert::From<i32> for AppPolicyMediaFoundationCodecLoading {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyMediaFoundationCodecLoading {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyProcessTerminationMethod(pub i32);
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(0i32);
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(1i32);
impl ::core::convert::From<i32> for AppPolicyProcessTerminationMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyProcessTerminationMethod {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyShowDeveloperDiagnostic(pub i32);
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(0i32);
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(1i32);
impl ::core::convert::From<i32> for AppPolicyShowDeveloperDiagnostic {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyShowDeveloperDiagnostic {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyThreadInitializationType(pub i32);
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(0i32);
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(1i32);
impl ::core::convert::From<i32> for AppPolicyThreadInitializationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyThreadInitializationType {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppPolicyWindowingModel(pub i32);
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = AppPolicyWindowingModel(0i32);
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = AppPolicyWindowingModel(1i32);
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = AppPolicyWindowingModel(2i32);
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = AppPolicyWindowingModel(3i32);
impl ::core::convert::From<i32> for AppPolicyWindowingModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyWindowingModel {
    type Abi = Self;
}
pub const AppxBundleFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378e0446_5384_43b7_8877_e7dbdd883446);
pub const AppxEncryptionFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc664fdd_d868_46ee_8780_8d196cb739f7);
pub const AppxFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5842a140_ff9f_4166_8f5c_62f5b7b0c781);
pub const AppxPackageEditor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf004f2ca_aebc_4b0d_bf58_e516d5bcc0ab);
pub const AppxPackagingDiagnosticEventSinkManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ca0a46_1588_4161_8ed2_ef9e469ced5d);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckIsMSIXPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckIsMSIXPackage(packagefullname: super::super::super::Foundation::PWSTR, ismsixpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CheckIsMSIXPackage(packagefullname.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> i32;
        }
        ::core::mem::transmute(ClosePackageInfo(::core::mem::transmute(packageinforeference)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CreatePackageDependencyOptions(pub i32);
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = CreatePackageDependencyOptions(0i32);
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = CreatePackageDependencyOptions(1i32);
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = CreatePackageDependencyOptions(2i32);
impl ::core::convert::From<i32> for CreatePackageDependencyOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CreatePackageDependencyOptions {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePackageVirtualizationContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0) -> ::windows::core::Result<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePackageVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, context: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreatePackageVirtualizationContext(packagefamilyname.into_param().abi(), &mut result__).from_abi::<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DX_FEATURE_LEVEL(pub i32);
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(0i32);
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(1i32);
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(2i32);
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(3i32);
impl ::core::convert::From<i32> for DX_FEATURE_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DX_FEATURE_LEVEL {
    type Abi = Self;
}
#[inline]
pub unsafe fn DeactivatePackageVirtualizationContext(cookie: usize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeactivatePackageVirtualizationContext(cookie: usize);
        }
        ::core::mem::transmute(DeactivatePackageVirtualizationContext(::core::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeletePackageDependency<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagedependencyid: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeletePackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        DeletePackageDependency(packagedependencyid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::core::Result<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, destcontext: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::core::HRESULT;
        }
        let mut result__: <*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DuplicatePackageVirtualizationContext(::core::mem::transmute(sourcecontext), &mut result__).from_abi::<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindPackagesByPackageFamily<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, packagefilters: u32, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR, packageproperties: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, packagefilters: u32, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR, packageproperties: *mut u32) -> i32;
        }
        ::core::mem::transmute(FindPackagesByPackageFamily(packagefamilyname.into_param().abi(), ::core::mem::transmute(packagefilters), ::core::mem::transmute(count), ::core::mem::transmute(packagefullnames), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer), ::core::mem::transmute(packageproperties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FormatApplicationUserModelId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, packagerelativeapplicationid: Param1, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FormatApplicationUserModelId(packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationid: super::super::super::Foundation::PWSTR, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(FormatApplicationUserModelId(packagefamilyname.into_param().abi(), packagerelativeapplicationid.into_param().abi(), ::core::mem::transmute(applicationusermodelidlength), ::core::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationUserModelId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationUserModelId(hprocess: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetApplicationUserModelId(hprocess.into_param().abi(), ::core::mem::transmute(applicationusermodelidlength), ::core::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationUserModelIdFromToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(token: Param0, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationUserModelIdFromToken(token: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetApplicationUserModelIdFromToken(token.into_param().abi(), ::core::mem::transmute(applicationusermodelidlength), ::core::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetCurrentApplicationUserModelId(::core::mem::transmute(applicationusermodelidlength), ::core::mem::transmute(applicationusermodelid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetCurrentPackageFamilyName(::core::mem::transmute(packagefamilynamelength), ::core::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetCurrentPackageFullName(::core::mem::transmute(packagefullnamelength), ::core::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> i32;
        }
        ::core::mem::transmute(GetCurrentPackageId(::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetCurrentPackageInfo(::core::mem::transmute(flags), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetCurrentPackageInfo2(::core::mem::transmute(flags), ::core::mem::transmute(packagepathtype), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackagePath(pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackagePath(pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetCurrentPackagePath(::core::mem::transmute(pathlength), ::core::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetCurrentPackagePath2(::core::mem::transmute(packagepathtype), ::core::mem::transmute(pathlength), ::core::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__;
        }
        ::core::mem::transmute(GetCurrentPackageVirtualizationContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetIdForPackageDependencyContext(::core::mem::transmute(packagedependencycontext), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetPackageApplicationIds(::core::mem::transmute(packageinforeference), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFamilyName(hprocess: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackageFamilyName(hprocess.into_param().abi(), ::core::mem::transmute(packagefamilynamelength), ::core::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFamilyNameFromToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(token: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFamilyNameFromToken(token: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackageFamilyNameFromToken(token.into_param().abi(), ::core::mem::transmute(packagefamilynamelength), ::core::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFullName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFullName(hprocess: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackageFullName(hprocess.into_param().abi(), ::core::mem::transmute(packagefullnamelength), ::core::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFullNameFromToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(token: Param0, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageFullNameFromToken(token: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackageFullNameFromToken(token.into_param().abi(), ::core::mem::transmute(packagefullnamelength), ::core::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hprocess: Param0, bufferlength: *mut u32, buffer: *mut u8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageId(hprocess: super::super::super::Foundation::HANDLE, bufferlength: *mut u32, buffer: *mut u8) -> i32;
        }
        ::core::mem::transmute(GetPackageId(hprocess.into_param().abi(), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetPackageInfo(::core::mem::transmute(packageinforeference), ::core::mem::transmute(flags), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetPackageInfo2(::core::mem::transmute(packageinforeference), ::core::mem::transmute(flags), ::core::mem::transmute(packagepathtype), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackagePath(::core::mem::transmute(packageid), ::core::mem::transmute(reserved), ::core::mem::transmute(pathlength), ::core::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePathByFullName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackagePathByFullName(packagefullname.into_param().abi(), ::core::mem::transmute(pathlength), ::core::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePathByFullName2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackagePathByFullName2(packagefullname.into_param().abi(), ::core::mem::transmute(packagepathtype), ::core::mem::transmute(pathlength), ::core::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagesByPackageFamily<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetPackagesByPackageFamily(packagefamilyname.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(packagefullnames), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessesInVirtualizationContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessesInVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        GetProcessesInVirtualizationContext(packagefamilyname.into_param().abi(), ::core::mem::transmute(count), ::core::mem::transmute(processes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetResolvedPackageFullNameForPackageDependency<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagedependencyid: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetResolvedPackageFullNameForPackageDependency(packagedependencyid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackageOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, origin: *mut PackageOrigin) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStagedPackageOrigin(packagefullname: super::super::super::Foundation::PWSTR, origin: *mut PackageOrigin) -> i32;
        }
        ::core::mem::transmute(GetStagedPackageOrigin(packagefullname.into_param().abi(), ::core::mem::transmute(origin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStagedPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetStagedPackagePathByFullName(packagefullname.into_param().abi(), ::core::mem::transmute(pathlength), ::core::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStagedPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(GetStagedPackagePathByFullName2(packagefullname.into_param().abi(), ::core::mem::transmute(packagepathtype), ::core::mem::transmute(pathlength), ::core::mem::transmute(path)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBlockMapBlock(pub ::windows::core::IUnknown);
impl IAppxBlockMapBlock {
    pub unsafe fn GetHash(&self, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffersize), ::core::mem::transmute(buffer)).ok()
    }
    pub unsafe fn GetCompressedSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBlockMapBlock {
    type Vtable = IAppxBlockMapBlock_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75cf3930_3244_4fe0_a8c8_e0bcb270b889);
}
impl ::core::convert::From<IAppxBlockMapBlock> for ::windows::core::IUnknown {
    fn from(value: IAppxBlockMapBlock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBlockMapBlock> for ::windows::core::IUnknown {
    fn from(value: &IAppxBlockMapBlock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBlockMapBlock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBlockMapBlock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlock_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBlockMapBlocksEnumerator(pub ::windows::core::IUnknown);
impl IAppxBlockMapBlocksEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBlockMapBlock> {
        let mut result__: <IAppxBlockMapBlock as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapBlock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBlockMapBlocksEnumerator {
    type Vtable = IAppxBlockMapBlocksEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b429b5b_36ef_479e_b9eb_0c1482b49e16);
}
impl ::core::convert::From<IAppxBlockMapBlocksEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxBlockMapBlocksEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBlockMapBlocksEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxBlockMapBlocksEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBlockMapBlocksEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBlockMapBlocksEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlocksEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, block: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBlockMapFile(pub ::windows::core::IUnknown);
impl IAppxBlockMapFile {
    pub unsafe fn GetBlocks(&self) -> ::windows::core::Result<IAppxBlockMapBlocksEnumerator> {
        let mut result__: <IAppxBlockMapBlocksEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapBlocksEnumerator>(result__)
    }
    pub unsafe fn GetLocalFileHeaderSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetUncompressedSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ValidateFileHash<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filestream: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), filestream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBlockMapFile {
    type Vtable = IAppxBlockMapFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x277672ac_4f63_42c1_8abc_beae3600eb59);
}
impl ::core::convert::From<IAppxBlockMapFile> for ::windows::core::IUnknown {
    fn from(value: IAppxBlockMapFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBlockMapFile> for ::windows::core::IUnknown {
    fn from(value: &IAppxBlockMapFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBlockMapFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBlockMapFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blocks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lfhsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filestream: ::windows::core::RawPtr, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBlockMapFilesEnumerator(pub ::windows::core::IUnknown);
impl IAppxBlockMapFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBlockMapFile> {
        let mut result__: <IAppxBlockMapFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapFile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBlockMapFilesEnumerator {
    type Vtable = IAppxBlockMapFilesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b856a2_4262_4070_bacb_1a8cbbc42305);
}
impl ::core::convert::From<IAppxBlockMapFilesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxBlockMapFilesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBlockMapFilesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxBlockMapFilesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBlockMapFilesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBlockMapFilesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFilesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBlockMapReader(pub ::windows::core::IUnknown);
impl IAppxBlockMapReader {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<IAppxBlockMapFile> {
        let mut result__: <IAppxBlockMapFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<IAppxBlockMapFile>(result__)
    }
    pub unsafe fn GetFiles(&self) -> ::windows::core::Result<IAppxBlockMapFilesEnumerator> {
        let mut result__: <IAppxBlockMapFilesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapFilesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHashMethod(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__: <super::super::super::System::Com::IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IUri>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBlockMapReader {
    type Vtable = IAppxBlockMapReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5efec991_bca3_42d1_9ec2_e92d609ec22a);
}
impl ::core::convert::From<IAppxBlockMapReader> for ::windows::core::IUnknown {
    fn from(value: IAppxBlockMapReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBlockMapReader> for ::windows::core::IUnknown {
    fn from(value: &IAppxBlockMapReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBlockMapReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBlockMapReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hashmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blockmapstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleFactory(pub ::windows::core::IUnknown);
impl IAppxBundleFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, bundleversion: u64) -> ::windows::core::Result<IAppxBundleWriter> {
        let mut result__: <IAppxBundleWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), ::core::mem::transmute(bundleversion), &mut result__).from_abi::<IAppxBundleWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::core::Result<IAppxBundleReader> {
        let mut result__: <IAppxBundleReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxBundleReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleManifestReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::core::Result<IAppxBundleManifestReader> {
        let mut result__: <IAppxBundleManifestReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxBundleManifestReader>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleFactory {
    type Vtable = IAppxBundleFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbba65864_965f_4a5f_855f_f074bdbf3a7b);
}
impl ::core::convert::From<IAppxBundleFactory> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleFactory> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, bundleversion: u64, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestOptionalBundleInfo(pub ::windows::core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfo {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetPackageInfoItems(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__: <IAppxBundleManifestPackageInfoEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestPackageInfoEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestOptionalBundleInfo {
    type Vtable = IAppxBundleManifestOptionalBundleInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x515bf2e8_bcb0_4d69_8c48_e383147b6e12);
}
impl ::core::convert::From<IAppxBundleManifestOptionalBundleInfo> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestOptionalBundleInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestOptionalBundleInfo> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestOptionalBundleInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestOptionalBundleInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestOptionalBundleInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator(pub ::windows::core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfo> {
        let mut result__: <IAppxBundleManifestOptionalBundleInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestOptionalBundleInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestOptionalBundleInfoEnumerator {
    type Vtable = IAppxBundleManifestOptionalBundleInfoEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a178793_f97e_46ac_aaca_dd5ba4c177c8);
}
impl ::core::convert::From<IAppxBundleManifestOptionalBundleInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestOptionalBundleInfoEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestOptionalBundleInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestOptionalBundleInfoEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, optionalbundle: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo(pub ::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo {
    pub unsafe fn GetPackageType(&self) -> ::windows::core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE> {
        let mut result__: <APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE>(result__)
    }
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetOffset(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo {
    type Vtable = IAppxBundleManifestPackageInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54cd06c1_268f_40bb_8ed2_757a9ebaec8d);
}
impl ::core::convert::From<IAppxBundleManifestPackageInfo> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestPackageInfo> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestPackageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestPackageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo2(pub ::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsPackageReference(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsDefaultApplicablePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo2 {
    type Vtable = IAppxBundleManifestPackageInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44c2acbc_b2cf_4ccb_bbdb_9c6da8c3bc9e);
}
impl ::core::convert::From<IAppxBundleManifestPackageInfo2> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestPackageInfo2> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestPackageInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestPackageInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo3(pub ::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo3 {
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__: <IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamiliesEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo3 {
    type Vtable = IAppxBundleManifestPackageInfo3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ba74b98_bb74_4296_80d0_5f4256a99675);
}
impl ::core::convert::From<IAppxBundleManifestPackageInfo3> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestPackageInfo3> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestPackageInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestPackageInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfo4(pub ::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo4 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsStub(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo4 {
    type Vtable = IAppxBundleManifestPackageInfo4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5da6f13d_a8a7_4532_857c_1393d659371d);
}
impl ::core::convert::From<IAppxBundleManifestPackageInfo4> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfo4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestPackageInfo4> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfo4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestPackageInfo4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestPackageInfo4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestPackageInfoEnumerator(pub ::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfo> {
        let mut result__: <IAppxBundleManifestPackageInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestPackageInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfoEnumerator {
    type Vtable = IAppxBundleManifestPackageInfoEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9b856ee_49a6_4e19_b2b0_6a2406d63a32);
}
impl ::core::convert::From<IAppxBundleManifestPackageInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestPackageInfoEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestPackageInfoEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestPackageInfoEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestPackageInfoEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestPackageInfoEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfoEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestReader(pub ::windows::core::IUnknown);
impl IAppxBundleManifestReader {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    pub unsafe fn GetPackageInfoItems(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__: <IAppxBundleManifestPackageInfoEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestPackageInfoEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestReader {
    type Vtable = IAppxBundleManifestReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf0ebbc1_cc99_4106_91eb_e67462e04fb0);
}
impl ::core::convert::From<IAppxBundleManifestReader> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestReader> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageinfoitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleManifestReader2(pub ::windows::core::IUnknown);
impl IAppxBundleManifestReader2 {
    pub unsafe fn GetOptionalBundles(&self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator> {
        let mut result__: <IAppxBundleManifestOptionalBundleInfoEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestOptionalBundleInfoEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestReader2 {
    type Vtable = IAppxBundleManifestReader2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5517df70_033f_4af2_8213_87d766805c02);
}
impl ::core::convert::From<IAppxBundleManifestReader2> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleManifestReader2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleManifestReader2> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleManifestReader2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleManifestReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleManifestReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, optionalbundles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleReader(pub ::windows::core::IUnknown);
impl IAppxBundleReader {
    pub unsafe fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(filetype), &mut result__).from_abi::<IAppxFile>(result__)
    }
    pub unsafe fn GetBlockMap(&self) -> ::windows::core::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
    pub unsafe fn GetManifest(&self) -> ::windows::core::Result<IAppxBundleManifestReader> {
        let mut result__: <IAppxBundleManifestReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBundleManifestReader>(result__)
    }
    pub unsafe fn GetPayloadPackages(&self) -> ::windows::core::Result<IAppxFilesEnumerator> {
        let mut result__: <IAppxFilesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxFilesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPayloadPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<IAppxFile>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleReader {
    type Vtable = IAppxBundleReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd75b8c0_ba76_43b0_ae0f_68656a1dc5c8);
}
impl ::core::convert::From<IAppxBundleReader> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleReader> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, payloadpackages: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, payloadpackage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleWriter(pub ::windows::core::IUnknown);
impl IAppxBundleWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, packagestream: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter {
    type Vtable = IAppxBundleWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec446fe8_bfec_4c64_ab4f_49f038f0c6d2);
}
impl ::core::convert::From<IAppxBundleWriter> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleWriter> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleWriter2(pub ::windows::core::IUnknown);
impl IAppxBundleWriter2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, inputstream: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter2 {
    type Vtable = IAppxBundleWriter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d8fe971_01cc_49a0_b685_233851279962);
}
impl ::core::convert::From<IAppxBundleWriter2> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleWriter2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleWriter2> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleWriter3(pub ::windows::core::IUnknown);
impl IAppxBundleWriter3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPackageReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, inputstream: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hashmethodstring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hashmethodstring.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter3 {
    type Vtable = IAppxBundleWriter3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad711152_f969_4193_82d5_9ddf2786d21a);
}
impl ::core::convert::From<IAppxBundleWriter3> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleWriter3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleWriter3> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleWriter3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hashmethodstring: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxBundleWriter4(pub ::windows::core::IUnknown);
impl IAppxBundleWriter4 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, packagestream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPackageReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, inputstream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, inputstream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter4 {
    type Vtable = IAppxBundleWriter4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cd9d523_5009_4c01_9882_dc029fbd47a3);
}
impl ::core::convert::From<IAppxBundleWriter4> for ::windows::core::IUnknown {
    fn from(value: IAppxBundleWriter4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxBundleWriter4> for ::windows::core::IUnknown {
    fn from(value: &IAppxBundleWriter4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxBundleWriter4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxBundleWriter4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxContentGroup(pub ::windows::core::IUnknown);
impl IAppxContentGroup {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetFiles(&self) -> ::windows::core::Result<IAppxContentGroupFilesEnumerator> {
        let mut result__: <IAppxContentGroupFilesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroupFilesEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxContentGroup {
    type Vtable = IAppxContentGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x328f6468_c04f_4e3c_b6fa_6b8d27f3003a);
}
impl ::core::convert::From<IAppxContentGroup> for ::windows::core::IUnknown {
    fn from(value: IAppxContentGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxContentGroup> for ::windows::core::IUnknown {
    fn from(value: &IAppxContentGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxContentGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxContentGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, groupname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxContentGroupFilesEnumerator(pub ::windows::core::IUnknown);
impl IAppxContentGroupFilesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxContentGroupFilesEnumerator {
    type Vtable = IAppxContentGroupFilesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a09a2fd_7440_44eb_8c84_848205a6a1cc);
}
impl ::core::convert::From<IAppxContentGroupFilesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxContentGroupFilesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxContentGroupFilesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxContentGroupFilesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxContentGroupFilesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxContentGroupFilesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupFilesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxContentGroupMapReader(pub ::windows::core::IUnknown);
impl IAppxContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> ::windows::core::Result<IAppxContentGroup> {
        let mut result__: <IAppxContentGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroup>(result__)
    }
    pub unsafe fn GetAutomaticGroups(&self) -> ::windows::core::Result<IAppxContentGroupsEnumerator> {
        let mut result__: <IAppxContentGroupsEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroupsEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxContentGroupMapReader {
    type Vtable = IAppxContentGroupMapReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x418726d8_dd99_4f5d_9886_157add20de01);
}
impl ::core::convert::From<IAppxContentGroupMapReader> for ::windows::core::IUnknown {
    fn from(value: IAppxContentGroupMapReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxContentGroupMapReader> for ::windows::core::IUnknown {
    fn from(value: &IAppxContentGroupMapReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxContentGroupMapReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxContentGroupMapReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxContentGroupMapWriter(pub ::windows::core::IUnknown);
impl IAppxContentGroupMapWriter {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAutomaticGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, groupname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), groupname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAutomaticFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), filename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxContentGroupMapWriter {
    type Vtable = IAppxContentGroupMapWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd07ab776_a9de_4798_8c14_3db31e687c78);
}
impl ::core::convert::From<IAppxContentGroupMapWriter> for ::windows::core::IUnknown {
    fn from(value: IAppxContentGroupMapWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxContentGroupMapWriter> for ::windows::core::IUnknown {
    fn from(value: &IAppxContentGroupMapWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxContentGroupMapWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxContentGroupMapWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, groupname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxContentGroupsEnumerator(pub ::windows::core::IUnknown);
impl IAppxContentGroupsEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxContentGroup> {
        let mut result__: <IAppxContentGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroup>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxContentGroupsEnumerator {
    type Vtable = IAppxContentGroupsEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3264e477_16d1_4d63_823e_7d2984696634);
}
impl ::core::convert::From<IAppxContentGroupsEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxContentGroupsEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxContentGroupsEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxContentGroupsEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxContentGroupsEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxContentGroupsEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupsEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptedBundleWriter(pub ::windows::core::IUnknown);
impl IAppxEncryptedBundleWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadPackageEncrypted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, packagestream: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptedBundleWriter {
    type Vtable = IAppxEncryptedBundleWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80b0902f_7bf0_4117_b8c6_4279ef81ee77);
}
impl ::core::convert::From<IAppxEncryptedBundleWriter> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptedBundleWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptedBundleWriter> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptedBundleWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptedBundleWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptedBundleWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptedBundleWriter2(pub ::windows::core::IUnknown);
impl IAppxEncryptedBundleWriter2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, inputstream: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptedBundleWriter2 {
    type Vtable = IAppxEncryptedBundleWriter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe644be82_f0fa_42b8_a956_8d1cb48ee379);
}
impl ::core::convert::From<IAppxEncryptedBundleWriter2> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptedBundleWriter2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptedBundleWriter2> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptedBundleWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptedBundleWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptedBundleWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptedBundleWriter3(pub ::windows::core::IUnknown);
impl IAppxEncryptedBundleWriter3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadPackageEncrypted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, packagestream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), packagestream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddExternalPackageReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, filename: Param0, inputstream: Param1, isdefaultapplicablepackage: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), filename.into_param().abi(), inputstream.into_param().abi(), isdefaultapplicablepackage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptedBundleWriter3 {
    type Vtable = IAppxEncryptedBundleWriter3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d34deb3_5cae_4dd3_977c_504932a51d31);
}
impl ::core::convert::From<IAppxEncryptedBundleWriter3> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptedBundleWriter3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptedBundleWriter3> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptedBundleWriter3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptedBundleWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptedBundleWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, packagestream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, inputstream: ::windows::core::RawPtr, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptedPackageWriter(pub ::windows::core::IUnknown);
impl IAppxEncryptedPackageWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadFileEncrypted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, compressionoption: APPX_COMPRESSION_OPTION, inputstream: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), ::core::mem::transmute(compressionoption), inputstream.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptedPackageWriter {
    type Vtable = IAppxEncryptedPackageWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf43d0b0b_1379_40e2_9b29_682ea2bf42af);
}
impl ::core::convert::From<IAppxEncryptedPackageWriter> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptedPackageWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptedPackageWriter> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptedPackageWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptedPackageWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptedPackageWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptedPackageWriter2(pub ::windows::core::IUnknown);
impl IAppxEncryptedPackageWriter2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadFilesEncrypted(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(filecount), ::core::mem::transmute(payloadfiles), ::core::mem::transmute(memorylimit)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptedPackageWriter2 {
    type Vtable = IAppxEncryptedPackageWriter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e475447_3a25_40b5_8ad2_f953ae50c92d);
}
impl ::core::convert::From<IAppxEncryptedPackageWriter2> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptedPackageWriter2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptedPackageWriter2> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptedPackageWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptedPackageWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptedPackageWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filecount: u32, payloadfiles: *const ::core::mem::ManuallyDrop<APPX_PACKAGE_WRITER_PAYLOAD_STREAM>, memorylimit: u64) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptionFactory(pub ::windows::core::IUnknown);
impl IAppxEncryptionFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EncryptPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::core::mem::transmute(keyinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedPackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, manifeststream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter> {
        let mut result__: <IAppxEncryptedPackageWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), manifeststream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedPackageWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxPackageReader> {
        let mut result__: <IAppxPackageReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), ::core::mem::transmute(keyinfo), &mut result__).from_abi::<IAppxPackageReader>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EncryptBundle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptBundle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::core::mem::transmute(keyinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedBundleWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter> {
        let mut result__: <IAppxEncryptedBundleWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), ::core::mem::transmute(bundleversion), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedBundleWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxBundleReader> {
        let mut result__: <IAppxBundleReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), ::core::mem::transmute(keyinfo), &mut result__).from_abi::<IAppxBundleReader>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory {
    type Vtable = IAppxEncryptionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e8e04d_8c88_44ae_a011_7cadf6fb2e72);
}
impl ::core::convert::From<IAppxEncryptionFactory> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptionFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptionFactory> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptionFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptionFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptionFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptionFactory2(pub ::windows::core::IUnknown);
impl IAppxEncryptionFactory2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedPackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        outputstream: Param0,
        manifeststream: Param1,
        contentgroupmapstream: Param2,
        settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS,
        keyinfo: *const APPX_KEY_INFO,
        exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS,
    ) -> ::windows::core::Result<IAppxEncryptedPackageWriter> {
        let mut result__: <IAppxEncryptedPackageWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), manifeststream.into_param().abi(), contentgroupmapstream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedPackageWriter>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory2 {
    type Vtable = IAppxEncryptionFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b11eee_c4ba_4ab2_a55d_d015fe8ff64f);
}
impl ::core::convert::From<IAppxEncryptionFactory2> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptionFactory2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptionFactory2> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptionFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptionFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptionFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptionFactory3(pub ::windows::core::IUnknown);
impl IAppxEncryptionFactory3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EncryptPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedPackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        outputstream: Param0,
        manifeststream: Param1,
        contentgroupmapstream: Param2,
        settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2,
        keyinfo: *const APPX_KEY_INFO,
        exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS,
    ) -> ::windows::core::Result<IAppxEncryptedPackageWriter> {
        let mut result__: <IAppxEncryptedPackageWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), manifeststream.into_param().abi(), contentgroupmapstream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedPackageWriter>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EncryptBundle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedBundleWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter> {
        let mut result__: <IAppxEncryptedBundleWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), ::core::mem::transmute(bundleversion), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles), &mut result__).from_abi::<IAppxEncryptedBundleWriter>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory3 {
    type Vtable = IAppxEncryptionFactory3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09edca37_cd64_47d6_b7e8_1cb11d4f7e05);
}
impl ::core::convert::From<IAppxEncryptionFactory3> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptionFactory3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptionFactory3> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptionFactory3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptionFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptionFactory3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, manifeststream: ::windows::core::RawPtr, contentgroupmapstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, bundleversion: u64, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxEncryptionFactory4(pub ::windows::core::IUnknown);
impl IAppxEncryptionFactory4 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EncryptPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0, outputstream: Param1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), outputstream.into_param().abi(), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo), ::core::mem::transmute(exemptedfiles), ::core::mem::transmute(memorylimit)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory4 {
    type Vtable = IAppxEncryptionFactory4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa879611f_12fd_41fe_85d5_06ae779bbaf5);
}
impl ::core::convert::From<IAppxEncryptionFactory4> for ::windows::core::IUnknown {
    fn from(value: IAppxEncryptionFactory4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxEncryptionFactory4> for ::windows::core::IUnknown {
    fn from(value: &IAppxEncryptionFactory4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxEncryptionFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxEncryptionFactory4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxFactory(pub ::windows::core::IUnknown);
impl IAppxFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, outputstream: Param0, settings: *const APPX_PACKAGE_SETTINGS) -> ::windows::core::Result<IAppxPackageWriter> {
        let mut result__: <IAppxPackageWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), outputstream.into_param().abi(), ::core::mem::transmute(settings), &mut result__).from_abi::<IAppxPackageWriter>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::core::Result<IAppxPackageReader> {
        let mut result__: <IAppxPackageReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxPackageReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateManifestReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::core::Result<IAppxManifestReader> {
        let mut result__: <IAppxManifestReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxManifestReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlockMapReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::core::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateValidatedBlockMapReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, blockmapstream: Param0, signaturefilename: Param1) -> ::windows::core::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), blockmapstream.into_param().abi(), signaturefilename.into_param().abi(), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxFactory {
    type Vtable = IAppxFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb94909_e451_438b_b5a7_d79e767b75d8);
}
impl ::core::convert::From<IAppxFactory> for ::windows::core::IUnknown {
    fn from(value: IAppxFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxFactory> for ::windows::core::IUnknown {
    fn from(value: &IAppxFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, settings: *const ::core::mem::ManuallyDrop<APPX_PACKAGE_SETTINGS>, packagewriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, packagereader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blockmapstream: ::windows::core::RawPtr, signaturefilename: super::super::super::Foundation::PWSTR, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxFactory2(pub ::windows::core::IUnknown);
impl IAppxFactory2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::core::Result<IAppxContentGroupMapReader> {
        let mut result__: <IAppxContentGroupMapReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxContentGroupMapReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSourceContentGroupMapReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, inputstream: Param0) -> ::windows::core::Result<IAppxSourceContentGroupMapReader> {
        let mut result__: <IAppxSourceContentGroupMapReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), inputstream.into_param().abi(), &mut result__).from_abi::<IAppxSourceContentGroupMapReader>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapWriter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, stream: Param0) -> ::windows::core::Result<IAppxContentGroupMapWriter> {
        let mut result__: <IAppxContentGroupMapWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), stream.into_param().abi(), &mut result__).from_abi::<IAppxContentGroupMapWriter>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxFactory2 {
    type Vtable = IAppxFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1346df2_c282_4e22_b918_743a929a8d55);
}
impl ::core::convert::From<IAppxFactory2> for ::windows::core::IUnknown {
    fn from(value: IAppxFactory2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxFactory2> for ::windows::core::IUnknown {
    fn from(value: &IAppxFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, contentgroupmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, reader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, contentgroupmapwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxFile(pub ::windows::core::IUnknown);
impl IAppxFile {
    pub unsafe fn GetCompressionOption(&self) -> ::windows::core::Result<APPX_COMPRESSION_OPTION> {
        let mut result__: <APPX_COMPRESSION_OPTION as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_COMPRESSION_OPTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxFile {
    type Vtable = IAppxFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91df827b_94fd_468f_827b_57f41b2f6f2e);
}
impl ::core::convert::From<IAppxFile> for ::windows::core::IUnknown {
    fn from(value: IAppxFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxFile> for ::windows::core::IUnknown {
    fn from(value: &IAppxFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contenttype: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxFilesEnumerator(pub ::windows::core::IUnknown);
impl IAppxFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxFile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxFilesEnumerator {
    type Vtable = IAppxFilesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf007eeaf_9831_411c_9847_917cdc62d1fe);
}
impl ::core::convert::From<IAppxFilesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxFilesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxFilesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxFilesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxFilesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxFilesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFilesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestApplication(pub ::windows::core::IUnknown);
impl IAppxManifestApplication {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAppUserModelId(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestApplication {
    type Vtable = IAppxManifestApplication_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5da89bf4_3773_46be_b650_7e744863b7e8);
}
impl ::core::convert::From<IAppxManifestApplication> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestApplication) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestApplication> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestApplication) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appusermodelid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestApplicationsEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestApplicationsEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestApplication> {
        let mut result__: <IAppxManifestApplication as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplication>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestApplicationsEnumerator {
    type Vtable = IAppxManifestApplicationsEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eb8a55a_f04b_4d0d_808d_686185d4847a);
}
impl ::core::convert::From<IAppxManifestApplicationsEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestApplicationsEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestApplicationsEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestApplicationsEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestApplicationsEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestApplicationsEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplicationsEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, application: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestCapabilitiesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestCapabilitiesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestCapabilitiesEnumerator {
    type Vtable = IAppxManifestCapabilitiesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d22258_f470_42c1_b291_8361c5437e41);
}
impl ::core::convert::From<IAppxManifestCapabilitiesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestCapabilitiesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestCapabilitiesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestCapabilitiesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestCapabilitiesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestDeviceCapabilitiesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestDeviceCapabilitiesEnumerator {
    type Vtable = IAppxManifestDeviceCapabilitiesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30204541_427b_4a1c_bacf_655bf463a540);
}
impl ::core::convert::From<IAppxManifestDeviceCapabilitiesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestDeviceCapabilitiesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestDeviceCapabilitiesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestDeviceCapabilitiesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestDeviceCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestDeviceCapabilitiesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devicecapability: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestDriverConstraint(pub ::windows::core::IUnknown);
impl IAppxManifestDriverConstraint {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMinDate(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverConstraint {
    type Vtable = IAppxManifestDriverConstraint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc031bee4_bbcc_48ea_a237_c34045c80a07);
}
impl ::core::convert::From<IAppxManifestDriverConstraint> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestDriverConstraint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestDriverConstraint> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestDriverConstraint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestDriverConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestDriverConstraint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraint_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minversion: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mindate: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestDriverConstraintsEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestDriverConstraintsEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestDriverConstraint> {
        let mut result__: <IAppxManifestDriverConstraint as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverConstraint>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverConstraintsEnumerator {
    type Vtable = IAppxManifestDriverConstraintsEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd402b2d1_f600_49e0_95e6_975d8da13d89);
}
impl ::core::convert::From<IAppxManifestDriverConstraintsEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestDriverConstraintsEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestDriverConstraintsEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestDriverConstraintsEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestDriverConstraintsEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestDriverConstraintsEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraintsEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, driverconstraint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestDriverDependenciesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestDriverDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestDriverDependency> {
        let mut result__: <IAppxManifestDriverDependency as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverDependenciesEnumerator {
    type Vtable = IAppxManifestDriverDependenciesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe039db2_467f_4755_8404_8f5eb6865b33);
}
impl ::core::convert::From<IAppxManifestDriverDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestDriverDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestDriverDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestDriverDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestDriverDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestDriverDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, driverdependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestDriverDependency(pub ::windows::core::IUnknown);
impl IAppxManifestDriverDependency {
    pub unsafe fn GetDriverConstraints(&self) -> ::windows::core::Result<IAppxManifestDriverConstraintsEnumerator> {
        let mut result__: <IAppxManifestDriverConstraintsEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverConstraintsEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverDependency {
    type Vtable = IAppxManifestDriverDependency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1210cb94_5a92_4602_be24_79f318af4af9);
}
impl ::core::convert::From<IAppxManifestDriverDependency> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestDriverDependency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestDriverDependency> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestDriverDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestDriverDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestDriverDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, driverconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestHostRuntimeDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependency> {
        let mut result__: <IAppxManifestHostRuntimeDependency as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestHostRuntimeDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestHostRuntimeDependenciesEnumerator {
    type Vtable = IAppxManifestHostRuntimeDependenciesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6427a646_7f49_433e_b1a6_0da309f6885a);
}
impl ::core::convert::From<IAppxManifestHostRuntimeDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestHostRuntimeDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestHostRuntimeDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestHostRuntimeDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestHostRuntimeDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestHostRuntimeDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostruntimedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestHostRuntimeDependency(pub ::windows::core::IUnknown);
impl IAppxManifestHostRuntimeDependency {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestHostRuntimeDependency {
    type Vtable = IAppxManifestHostRuntimeDependency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3455d234_8414_410d_95c7_7b35255b8391);
}
impl ::core::convert::From<IAppxManifestHostRuntimeDependency> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestHostRuntimeDependency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestHostRuntimeDependency> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestHostRuntimeDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestHostRuntimeDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestHostRuntimeDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minversion: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestHostRuntimeDependency2(pub ::windows::core::IUnknown);
impl IAppxManifestHostRuntimeDependency2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestHostRuntimeDependency2 {
    type Vtable = IAppxManifestHostRuntimeDependency2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc26f23a8_ee10_4ad6_b898_2b4d7aebfe6a);
}
impl ::core::convert::From<IAppxManifestHostRuntimeDependency2> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestHostRuntimeDependency2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestHostRuntimeDependency2> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestHostRuntimeDependency2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestHostRuntimeDependency2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestHostRuntimeDependency2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestMainPackageDependenciesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestMainPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestMainPackageDependency> {
        let mut result__: <IAppxManifestMainPackageDependency as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestMainPackageDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestMainPackageDependenciesEnumerator {
    type Vtable = IAppxManifestMainPackageDependenciesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa99c4f00_51d2_4f0f_ba46_7ed5255ebdff);
}
impl ::core::convert::From<IAppxManifestMainPackageDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestMainPackageDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestMainPackageDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestMainPackageDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestMainPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestMainPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mainpackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestMainPackageDependency(pub ::windows::core::IUnknown);
impl IAppxManifestMainPackageDependency {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestMainPackageDependency {
    type Vtable = IAppxManifestMainPackageDependency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05d0611c_bc29_46d5_97e2_84b9c79bd8ae);
}
impl ::core::convert::From<IAppxManifestMainPackageDependency> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestMainPackageDependency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestMainPackageDependency> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestMainPackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestMainPackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestMainPackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestOSPackageDependenciesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestOSPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestOSPackageDependency> {
        let mut result__: <IAppxManifestOSPackageDependency as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestOSPackageDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestOSPackageDependenciesEnumerator {
    type Vtable = IAppxManifestOSPackageDependenciesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84e2fc3_f8ec_4bc1_8ae2_156346f5ffea);
}
impl ::core::convert::From<IAppxManifestOSPackageDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestOSPackageDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestOSPackageDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestOSPackageDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestOSPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestOSPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ospackagedependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestOSPackageDependency(pub ::windows::core::IUnknown);
impl IAppxManifestOSPackageDependency {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestOSPackageDependency {
    type Vtable = IAppxManifestOSPackageDependency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x154995ee_54a6_4f14_ac97_d8cf0519644b);
}
impl ::core::convert::From<IAppxManifestOSPackageDependency> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestOSPackageDependency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestOSPackageDependency> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestOSPackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestOSPackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestOSPackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, version: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestOptionalPackageInfo(pub ::windows::core::IUnknown);
impl IAppxManifestOptionalPackageInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsOptionalPackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainPackageName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestOptionalPackageInfo {
    type Vtable = IAppxManifestOptionalPackageInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2634847d_5b5d_4fe5_a243_002ff95edc7e);
}
impl ::core::convert::From<IAppxManifestOptionalPackageInfo> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestOptionalPackageInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestOptionalPackageInfo> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestOptionalPackageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestOptionalPackageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestOptionalPackageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOptionalPackageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mainpackagename: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestPackageDependenciesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestPackageDependency> {
        let mut result__: <IAppxManifestPackageDependency as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependency>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependenciesEnumerator {
    type Vtable = IAppxManifestPackageDependenciesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb43bbcf9_65a6_42dd_bac0_8c6741e7f5a4);
}
impl ::core::convert::From<IAppxManifestPackageDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestPackageDependenciesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestPackageDependenciesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestPackageDependenciesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestPackageDependenciesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependenciesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependency: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestPackageDependency(pub ::windows::core::IUnknown);
impl IAppxManifestPackageDependency {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependency {
    type Vtable = IAppxManifestPackageDependency_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4946b59_733e_43f0_a724_3bde4c1285a0);
}
impl ::core::convert::From<IAppxManifestPackageDependency> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestPackageDependency) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestPackageDependency> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestPackageDependency) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestPackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestPackageDependency {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minversion: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestPackageDependency2(pub ::windows::core::IUnknown);
impl IAppxManifestPackageDependency2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetMaxMajorVersionTested(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependency2 {
    type Vtable = IAppxManifestPackageDependency2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdda0b713_f3ff_49d3_898a_2786780c5d98);
}
impl ::core::convert::From<IAppxManifestPackageDependency2> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestPackageDependency2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestPackageDependency2> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestPackageDependency2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAppxManifestPackageDependency2> for IAppxManifestPackageDependency {
    fn from(value: IAppxManifestPackageDependency2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestPackageDependency2> for IAppxManifestPackageDependency {
    fn from(value: &IAppxManifestPackageDependency2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestPackageDependency> for IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestPackageDependency> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestPackageDependency> for &IAppxManifestPackageDependency2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestPackageDependency> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minversion: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxmajorversiontested: *mut u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestPackageDependency3(pub ::windows::core::IUnknown);
impl IAppxManifestPackageDependency3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsOptional(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependency3 {
    type Vtable = IAppxManifestPackageDependency3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ac56374_6198_4d6b_92e4_749d5ab8a895);
}
impl ::core::convert::From<IAppxManifestPackageDependency3> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestPackageDependency3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestPackageDependency3> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestPackageDependency3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestPackageDependency3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestPackageDependency3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestPackageId(pub ::windows::core::IUnknown);
impl IAppxManifestPackageId {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetArchitecture(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE> {
        let mut result__: <APPX_PACKAGE_ARCHITECTURE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_PACKAGE_ARCHITECTURE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResourceId(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComparePublisher<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, other: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), other.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPackageFullName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageId {
    type Vtable = IAppxManifestPackageId_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x283ce2d7_7153_4a91_9649_7a0f7240945f);
}
impl ::core::convert::From<IAppxManifestPackageId> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestPackageId) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestPackageId> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestPackageId) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestPackageId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestPackageId {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageversion: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resourceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, other: super::super::super::Foundation::PWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestPackageId2(pub ::windows::core::IUnknown);
impl IAppxManifestPackageId2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetArchitecture(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE> {
        let mut result__: <APPX_PACKAGE_ARCHITECTURE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_PACKAGE_ARCHITECTURE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResourceId(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComparePublisher<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, other: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), other.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPackageFullName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetArchitecture2(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE2> {
        let mut result__: <APPX_PACKAGE_ARCHITECTURE2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_PACKAGE_ARCHITECTURE2>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageId2 {
    type Vtable = IAppxManifestPackageId2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2256999d_d617_42f1_880e_0ba4542319d5);
}
impl ::core::convert::From<IAppxManifestPackageId2> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestPackageId2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestPackageId2> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestPackageId2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAppxManifestPackageId2> for IAppxManifestPackageId {
    fn from(value: IAppxManifestPackageId2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestPackageId2> for IAppxManifestPackageId {
    fn from(value: &IAppxManifestPackageId2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestPackageId> for IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestPackageId> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestPackageId> for &IAppxManifestPackageId2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestPackageId> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisher: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageversion: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resourceid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, other: super::super::super::Foundation::PWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestProperties(pub ::windows::core::IUnknown);
impl IAppxManifestProperties {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestProperties {
    type Vtable = IAppxManifestProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03faf64d_f26f_4b2c_aaf7_8fe7789b8bca);
}
impl ::core::convert::From<IAppxManifestProperties> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestProperties> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestQualifiedResource(pub ::windows::core::IUnknown);
impl IAppxManifestQualifiedResource {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetScale(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetDXFeatureLevel(&self) -> ::windows::core::Result<DX_FEATURE_LEVEL> {
        let mut result__: <DX_FEATURE_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DX_FEATURE_LEVEL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestQualifiedResource {
    type Vtable = IAppxManifestQualifiedResource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b53a497_3c5c_48d1_9ea3_bb7eac8cd7d4);
}
impl ::core::convert::From<IAppxManifestQualifiedResource> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestQualifiedResource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestQualifiedResource> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestQualifiedResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestQualifiedResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestQualifiedResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, language: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scale: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestQualifiedResourcesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestQualifiedResourcesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestQualifiedResource> {
        let mut result__: <IAppxManifestQualifiedResource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResource>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestQualifiedResourcesEnumerator {
    type Vtable = IAppxManifestQualifiedResourcesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ef6adfe_3762_4a8f_9373_2fc5d444c8d2);
}
impl ::core::convert::From<IAppxManifestQualifiedResourcesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestQualifiedResourcesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestQualifiedResourcesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestQualifiedResourcesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestQualifiedResourcesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestQualifiedResourcesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResourcesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestReader(pub ::windows::core::IUnknown);
impl IAppxManifestReader {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestReader {
    type Vtable = IAppxManifestReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e1bd148_55a0_4480_a3d1_15544710637c);
}
impl ::core::convert::From<IAppxManifestReader> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestReader> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestReader2(pub ::windows::core::IUnknown);
impl IAppxManifestReader2 {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestReader2 {
    type Vtable = IAppxManifestReader2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06f67bc_b31d_4eba_a8af_638e73e77b4d);
}
impl ::core::convert::From<IAppxManifestReader2> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestReader2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestReader2> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestReader2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAppxManifestReader2> for IAppxManifestReader {
    fn from(value: IAppxManifestReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestReader2> for IAppxManifestReader {
    fn from(value: &IAppxManifestReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader> for IAppxManifestReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader> for &IAppxManifestReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestReader3(pub ::windows::core::IUnknown);
impl IAppxManifestReader3 {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
    pub unsafe fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::core::Result<IAppxManifestCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestCapabilitiesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(capabilityclass), &mut result__).from_abi::<IAppxManifestCapabilitiesEnumerator>(result__)
    }
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__: <IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamiliesEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestReader3 {
    type Vtable = IAppxManifestReader3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc43825ab_69b7_400a_9709_cc37f5a72d24);
}
impl ::core::convert::From<IAppxManifestReader3> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestReader3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestReader3> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestReader3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAppxManifestReader3> for IAppxManifestReader2 {
    fn from(value: IAppxManifestReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestReader3> for IAppxManifestReader2 {
    fn from(value: &IAppxManifestReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader2> for IAppxManifestReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader2> for &IAppxManifestReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppxManifestReader3> for IAppxManifestReader {
    fn from(value: IAppxManifestReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestReader3> for IAppxManifestReader {
    fn from(value: &IAppxManifestReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader> for IAppxManifestReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader> for &IAppxManifestReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestReader4(pub ::windows::core::IUnknown);
impl IAppxManifestReader4 {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__: <IAppxManifestPackageId as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageId>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__: <IAppxManifestProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestProperties>(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestPackageDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestPackageDependenciesEnumerator>(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__: <APPX_CAPABILITIES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APPX_CAPABILITIES>(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__: <IAppxManifestResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestResourcesEnumerator>(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestDeviceCapabilitiesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDeviceCapabilitiesEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrerequisite<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__: <IAppxManifestApplicationsEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestApplicationsEnumerator>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__: <IAppxManifestQualifiedResourcesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestQualifiedResourcesEnumerator>(result__)
    }
    pub unsafe fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::core::Result<IAppxManifestCapabilitiesEnumerator> {
        let mut result__: <IAppxManifestCapabilitiesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(capabilityclass), &mut result__).from_abi::<IAppxManifestCapabilitiesEnumerator>(result__)
    }
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__: <IAppxManifestTargetDeviceFamiliesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamiliesEnumerator>(result__)
    }
    pub unsafe fn GetOptionalPackageInfo(&self) -> ::windows::core::Result<IAppxManifestOptionalPackageInfo> {
        let mut result__: <IAppxManifestOptionalPackageInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestOptionalPackageInfo>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestReader4 {
    type Vtable = IAppxManifestReader4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4579bb7c_741d_4161_b5a1_47bd3b78ad9b);
}
impl ::core::convert::From<IAppxManifestReader4> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestReader4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestReader4> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestReader4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAppxManifestReader4> for IAppxManifestReader3 {
    fn from(value: IAppxManifestReader4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestReader4> for IAppxManifestReader3 {
    fn from(value: &IAppxManifestReader4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader3> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader3> for &IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppxManifestReader4> for IAppxManifestReader2 {
    fn from(value: IAppxManifestReader4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestReader4> for IAppxManifestReader2 {
    fn from(value: &IAppxManifestReader4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader2> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader2> for &IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppxManifestReader4> for IAppxManifestReader {
    fn from(value: IAppxManifestReader4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppxManifestReader4> for IAppxManifestReader {
    fn from(value: &IAppxManifestReader4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader> for IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppxManifestReader> for &IAppxManifestReader4 {
    fn into_param(self) -> ::windows::core::Param<'a, IAppxManifestReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packageproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devicecapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::Foundation::PWSTR, value: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifeststream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resources: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetdevicefamilies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, optionalpackageinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestReader5(pub ::windows::core::IUnknown);
impl IAppxManifestReader5 {
    pub unsafe fn GetMainPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestMainPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestMainPackageDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestMainPackageDependenciesEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestReader5 {
    type Vtable = IAppxManifestReader5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d7ae132_a690_4c00_b75a_6aae1feaac80);
}
impl ::core::convert::From<IAppxManifestReader5> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestReader5) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestReader5> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestReader5) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestReader5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestReader5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mainpackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestReader6(pub ::windows::core::IUnknown);
impl IAppxManifestReader6 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestReader6 {
    type Vtable = IAppxManifestReader6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34deaca4_d3c0_4e3e_b312_e42625e3807e);
}
impl ::core::convert::From<IAppxManifestReader6> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestReader6) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestReader6> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestReader6) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestReader6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestReader6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestReader7(pub ::windows::core::IUnknown);
impl IAppxManifestReader7 {
    pub unsafe fn GetDriverDependencies(&self) -> ::windows::core::Result<IAppxManifestDriverDependenciesEnumerator> {
        let mut result__: <IAppxManifestDriverDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestDriverDependenciesEnumerator>(result__)
    }
    pub unsafe fn GetOSPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestOSPackageDependenciesEnumerator> {
        let mut result__: <IAppxManifestOSPackageDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestOSPackageDependenciesEnumerator>(result__)
    }
    pub unsafe fn GetHostRuntimeDependencies(&self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependenciesEnumerator> {
        let mut result__: <IAppxManifestHostRuntimeDependenciesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestHostRuntimeDependenciesEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestReader7 {
    type Vtable = IAppxManifestReader7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efe6f27_0ce0_4988_b32d_738eb63db3b7);
}
impl ::core::convert::From<IAppxManifestReader7> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestReader7) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestReader7> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestReader7) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestReader7 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestReader7 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader7_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, driverdependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ospackagedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostruntimedependencies: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestResourcesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestResourcesEnumerator {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestResourcesEnumerator {
    type Vtable = IAppxManifestResourcesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde4dfbbd_881a_48bb_858c_d6f2baeae6ed);
}
impl ::core::convert::From<IAppxManifestResourcesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestResourcesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestResourcesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestResourcesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestResourcesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestResourcesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestResourcesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resource: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator(pub ::windows::core::IUnknown);
impl IAppxManifestTargetDeviceFamiliesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamily> {
        let mut result__: <IAppxManifestTargetDeviceFamily as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestTargetDeviceFamily>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestTargetDeviceFamiliesEnumerator {
    type Vtable = IAppxManifestTargetDeviceFamiliesEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36537f36_27a4_4788_88c0_733819575017);
}
impl ::core::convert::From<IAppxManifestTargetDeviceFamiliesEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestTargetDeviceFamiliesEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestTargetDeviceFamiliesEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestTargetDeviceFamiliesEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestTargetDeviceFamiliesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestTargetDeviceFamiliesEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targetdevicefamily: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxManifestTargetDeviceFamily(pub ::windows::core::IUnknown);
impl IAppxManifestTargetDeviceFamily {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetMaxVersionTested(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxManifestTargetDeviceFamily {
    type Vtable = IAppxManifestTargetDeviceFamily_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9091b09b_c8d5_4f31_8687_a338259faefb);
}
impl ::core::convert::From<IAppxManifestTargetDeviceFamily> for ::windows::core::IUnknown {
    fn from(value: IAppxManifestTargetDeviceFamily) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxManifestTargetDeviceFamily> for ::windows::core::IUnknown {
    fn from(value: &IAppxManifestTargetDeviceFamily) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxManifestTargetDeviceFamily {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxManifestTargetDeviceFamily {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamily_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, minversion: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxversiontested: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxPackageEditor(pub ::windows::core::IUnknown);
impl IAppxPackageEditor {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, workingdirectory: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), workingdirectory.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDeltaPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, updatedpackagestream: Param0, baselinepackagestream: Param1, deltapackagestream: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), updatedpackagestream.into_param().abi(), baselinepackagestream.into_param().abi(), deltapackagestream.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDeltaPackageUsingBaselineBlockMap<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(
        &self,
        updatedpackagestream: Param0,
        baselineblockmapstream: Param1,
        baselinepackagefullname: Param2,
        deltapackagestream: Param3,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), updatedpackagestream.into_param().abi(), baselineblockmapstream.into_param().abi(), baselinepackagefullname.into_param().abi(), deltapackagestream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdatePackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, baselinepackagestream: Param0, deltapackagestream: Param1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), baselinepackagestream.into_param().abi(), deltapackagestream.into_param().abi(), ::core::mem::transmute(updateoption)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateEncryptedPackage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, baselineencryptedpackagestream: Param0, deltapackagestream: Param1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), baselineencryptedpackagestream.into_param().abi(), deltapackagestream.into_param().abi(), ::core::mem::transmute(updateoption), ::core::mem::transmute(settings), ::core::mem::transmute(keyinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdatePackageManifest<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, packagestream: Param0, updatedmanifeststream: Param1, ispackageencrypted: Param2, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), packagestream.into_param().abi(), updatedmanifeststream.into_param().abi(), ispackageencrypted.into_param().abi(), ::core::mem::transmute(options)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxPackageEditor {
    type Vtable = IAppxPackageEditor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2adb6dc_5e71_4416_86b6_86e5f5291a6b);
}
impl ::core::convert::From<IAppxPackageEditor> for ::windows::core::IUnknown {
    fn from(value: IAppxPackageEditor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxPackageEditor> for ::windows::core::IUnknown {
    fn from(value: &IAppxPackageEditor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxPackageEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxPackageEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageEditor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, workingdirectory: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updatedpackagestream: ::windows::core::RawPtr, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updatedpackagestream: ::windows::core::RawPtr, baselineblockmapstream: ::windows::core::RawPtr, baselinepackagefullname: super::super::super::Foundation::PWSTR, deltapackagestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baselinepackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baselineencryptedpackagestream: ::windows::core::RawPtr, deltapackagestream: ::windows::core::RawPtr, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const ::core::mem::ManuallyDrop<APPX_ENCRYPTED_PACKAGE_SETTINGS2>, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagestream: ::windows::core::RawPtr, updatedmanifeststream: ::windows::core::RawPtr, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxPackageReader(pub ::windows::core::IUnknown);
impl IAppxPackageReader {
    pub unsafe fn GetBlockMap(&self) -> ::windows::core::Result<IAppxBlockMapReader> {
        let mut result__: <IAppxBlockMapReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxBlockMapReader>(result__)
    }
    pub unsafe fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<IAppxFile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPayloadFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, filename: Param0) -> ::windows::core::Result<IAppxFile> {
        let mut result__: <IAppxFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), filename.into_param().abi(), &mut result__).from_abi::<IAppxFile>(result__)
    }
    pub unsafe fn GetPayloadFiles(&self) -> ::windows::core::Result<IAppxFilesEnumerator> {
        let mut result__: <IAppxFilesEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxFilesEnumerator>(result__)
    }
    pub unsafe fn GetManifest(&self) -> ::windows::core::Result<IAppxManifestReader> {
        let mut result__: <IAppxManifestReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxManifestReader>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxPackageReader {
    type Vtable = IAppxPackageReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5c49650_99bc_481c_9a34_3d53a4106708);
}
impl ::core::convert::From<IAppxPackageReader> for ::windows::core::IUnknown {
    fn from(value: IAppxPackageReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxPackageReader> for ::windows::core::IUnknown {
    fn from(value: &IAppxPackageReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxPackageReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxPackageReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blockmapreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, file: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filesenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifestreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxPackageWriter(pub ::windows::core::IUnknown);
impl IAppxPackageWriter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, filename: Param0, contenttype: Param1, compressionoption: APPX_COMPRESSION_OPTION, inputstream: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), contenttype.into_param().abi(), ::core::mem::transmute(compressionoption), inputstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, manifest: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), manifest.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxPackageWriter {
    type Vtable = IAppxPackageWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9099e33b_246f_41e4_881a_008eb613f858);
}
impl ::core::convert::From<IAppxPackageWriter> for ::windows::core::IUnknown {
    fn from(value: IAppxPackageWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxPackageWriter> for ::windows::core::IUnknown {
    fn from(value: &IAppxPackageWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxPackageWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxPackageWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: super::super::super::Foundation::PWSTR, contenttype: super::super::super::Foundation::PWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifest: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxPackageWriter2(pub ::windows::core::IUnknown);
impl IAppxPackageWriter2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, manifest: Param0, contentgroupmap: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), manifest.into_param().abi(), contentgroupmap.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxPackageWriter2 {
    type Vtable = IAppxPackageWriter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cf5c4fd_e54c_4ea5_ba4e_f8c4b105a8c8);
}
impl ::core::convert::From<IAppxPackageWriter2> for ::windows::core::IUnknown {
    fn from(value: IAppxPackageWriter2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxPackageWriter2> for ::windows::core::IUnknown {
    fn from(value: &IAppxPackageWriter2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxPackageWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxPackageWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, manifest: ::windows::core::RawPtr, contentgroupmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxPackageWriter3(pub ::windows::core::IUnknown);
impl IAppxPackageWriter3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadFiles(&self, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(filecount), ::core::mem::transmute(payloadfiles), ::core::mem::transmute(memorylimit)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxPackageWriter3 {
    type Vtable = IAppxPackageWriter3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa83aacd3_41c0_4501_b8a3_74164f50b2fd);
}
impl ::core::convert::From<IAppxPackageWriter3> for ::windows::core::IUnknown {
    fn from(value: IAppxPackageWriter3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxPackageWriter3> for ::windows::core::IUnknown {
    fn from(value: &IAppxPackageWriter3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxPackageWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxPackageWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filecount: u32, payloadfiles: *const ::core::mem::ManuallyDrop<APPX_PACKAGE_WRITER_PAYLOAD_STREAM>, memorylimit: u64) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxPackagingDiagnosticEventSink(pub ::windows::core::IUnknown);
impl IAppxPackagingDiagnosticEventSink {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportContextChange<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: Param2, contextmessage: Param3, detailsmessage: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), ::core::mem::transmute(contextid), contextname.into_param().abi(), contextmessage.into_param().abi(), detailsmessage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, errormessage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), errormessage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxPackagingDiagnosticEventSink {
    type Vtable = IAppxPackagingDiagnosticEventSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17239d47_6adb_45d2_80f6_f9cbc3bf059d);
}
impl ::core::convert::From<IAppxPackagingDiagnosticEventSink> for ::windows::core::IUnknown {
    fn from(value: IAppxPackagingDiagnosticEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxPackagingDiagnosticEventSink> for ::windows::core::IUnknown {
    fn from(value: &IAppxPackagingDiagnosticEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxPackagingDiagnosticEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxPackagingDiagnosticEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: super::super::super::Foundation::PSTR, contextmessage: super::super::super::Foundation::PWSTR, detailsmessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errormessage: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxPackagingDiagnosticEventSinkManager(pub ::windows::core::IUnknown);
impl IAppxPackagingDiagnosticEventSinkManager {
    pub unsafe fn SetSinkForProcess<'a, Param0: ::windows::core::IntoParam<'a, IAppxPackagingDiagnosticEventSink>>(&self, sink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sink.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAppxPackagingDiagnosticEventSinkManager {
    type Vtable = IAppxPackagingDiagnosticEventSinkManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x369648fa_a7eb_4909_a15d_6954a078f18a);
}
impl ::core::convert::From<IAppxPackagingDiagnosticEventSinkManager> for ::windows::core::IUnknown {
    fn from(value: IAppxPackagingDiagnosticEventSinkManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxPackagingDiagnosticEventSinkManager> for ::windows::core::IUnknown {
    fn from(value: &IAppxPackagingDiagnosticEventSinkManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxPackagingDiagnosticEventSinkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxPackagingDiagnosticEventSinkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSinkManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAppxSourceContentGroupMapReader(pub ::windows::core::IUnknown);
impl IAppxSourceContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> ::windows::core::Result<IAppxContentGroup> {
        let mut result__: <IAppxContentGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroup>(result__)
    }
    pub unsafe fn GetAutomaticGroups(&self) -> ::windows::core::Result<IAppxContentGroupsEnumerator> {
        let mut result__: <IAppxContentGroupsEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAppxContentGroupsEnumerator>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAppxSourceContentGroupMapReader {
    type Vtable = IAppxSourceContentGroupMapReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf329791d_540b_4a9f_bc75_3282b7d73193);
}
impl ::core::convert::From<IAppxSourceContentGroupMapReader> for ::windows::core::IUnknown {
    fn from(value: IAppxSourceContentGroupMapReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppxSourceContentGroupMapReader> for ::windows::core::IUnknown {
    fn from(value: &IAppxSourceContentGroupMapReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppxSourceContentGroupMapReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppxSourceContentGroupMapReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxSourceContentGroupMapReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requiredgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, automaticgroupsenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPackageInfoByFullName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPackageInfoByFullName(packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
        }
        ::core::mem::transmute(OpenPackageInfoByFullName(packagefullname.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(packageinforeference)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPackageInfoByFullNameForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSID>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(usersid: Param0, packagefullname: Param1, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPackageInfoByFullNameForUser(usersid: super::super::super::Foundation::PSID, packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
        }
        ::core::mem::transmute(OpenPackageInfoByFullNameForUser(usersid.into_param().abi(), packagefullname.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(packageinforeference)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PACKAGEDEPENDENCY_CONTEXT__ {
    pub unused: i32,
}
impl PACKAGEDEPENDENCY_CONTEXT__ {}
impl ::core::default::Default for PACKAGEDEPENDENCY_CONTEXT__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PACKAGEDEPENDENCY_CONTEXT__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PACKAGEDEPENDENCY_CONTEXT__").field("unused", &self.unused).finish()
    }
}
impl ::core::cmp::PartialEq for PACKAGEDEPENDENCY_CONTEXT__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for PACKAGEDEPENDENCY_CONTEXT__ {}
unsafe impl ::windows::core::Abi for PACKAGEDEPENDENCY_CONTEXT__ {
    type Abi = Self;
}
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: super::super::super::Foundation::PWSTR,
    pub publisher: super::super::super::Foundation::PWSTR,
    pub resourceId: super::super::super::Foundation::PWSTR,
    pub publisherId: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PACKAGE_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PACKAGE_ID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PACKAGE_ID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PACKAGE_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: super::super::super::Foundation::PWSTR,
    pub packageFullName: super::super::super::Foundation::PWSTR,
    pub packageFamilyName: super::super::super::Foundation::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(feature = "Win32_Foundation")]
impl PACKAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PACKAGE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PACKAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PACKAGE_INFO {
    type Abi = Self;
}
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl PACKAGE_VERSION {}
impl ::core::default::Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PACKAGE_VERSION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PACKAGE_VERSION {}
unsafe impl ::windows::core::Abi for PACKAGE_VERSION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl PACKAGE_VERSION_0 {}
impl ::core::default::Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PACKAGE_VERSION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PACKAGE_VERSION_0 {}
unsafe impl ::windows::core::Abi for PACKAGE_VERSION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
impl PACKAGE_VERSION_0_0 {}
impl ::core::default::Default for PACKAGE_VERSION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PACKAGE_VERSION_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Revision", &self.Revision).field("Build", &self.Build).field("Minor", &self.Minor).field("Major", &self.Major).finish()
    }
}
impl ::core::cmp::PartialEq for PACKAGE_VERSION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.Build == other.Build && self.Minor == other.Minor && self.Major == other.Major
    }
}
impl ::core::cmp::Eq for PACKAGE_VERSION_0_0 {}
unsafe impl ::windows::core::Abi for PACKAGE_VERSION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    pub unused: i32,
}
impl PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
impl ::core::default::Default for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__").field("unused", &self.unused).finish()
    }
}
impl ::core::cmp::PartialEq for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
unsafe impl ::windows::core::Abi for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageDependencyLifetimeKind(pub i32);
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(0i32);
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(1i32);
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(2i32);
impl ::core::convert::From<i32> for PackageDependencyLifetimeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageDependencyLifetimeKind {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageDependencyProcessorArchitectures(pub i32);
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(0i32);
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(1i32);
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(2i32);
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(4i32);
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(8i32);
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(16i32);
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(32i32);
impl ::core::convert::From<i32> for PackageDependencyProcessorArchitectures {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageDependencyProcessorArchitectures {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFamilyNameFromFullName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageFamilyNameFromFullName(packagefullname: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PackageFamilyNameFromFullName(packagefullname.into_param().abi(), ::core::mem::transmute(packagefamilynamelength), ::core::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PackageFamilyNameFromId(::core::mem::transmute(packageid), ::core::mem::transmute(packagefamilynamelength), ::core::mem::transmute(packagefamilyname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PackageFullNameFromId(::core::mem::transmute(packageid), ::core::mem::transmute(packagefullnamelength), ::core::mem::transmute(packagefullname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageIdFromFullName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageIdFromFullName(packagefullname: super::super::super::Foundation::PWSTR, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> i32;
        }
        ::core::mem::transmute(PackageIdFromFullName(packagefullname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(bufferlength), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageNameAndPublisherIdFromFamilyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0, packagenamelength: *mut u32, packagename: super::super::super::Foundation::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR, packagenamelength: *mut u32, packagename: super::super::super::Foundation::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(PackageNameAndPublisherIdFromFamilyName(packagefamilyname.into_param().abi(), ::core::mem::transmute(packagenamelength), ::core::mem::transmute(packagename), ::core::mem::transmute(packagepublisheridlength), ::core::mem::transmute(packagepublisherid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackageOrigin(pub i32);
pub const PackageOrigin_Unknown: PackageOrigin = PackageOrigin(0i32);
pub const PackageOrigin_Unsigned: PackageOrigin = PackageOrigin(1i32);
pub const PackageOrigin_Inbox: PackageOrigin = PackageOrigin(2i32);
pub const PackageOrigin_Store: PackageOrigin = PackageOrigin(3i32);
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = PackageOrigin(4i32);
pub const PackageOrigin_DeveloperSigned: PackageOrigin = PackageOrigin(5i32);
pub const PackageOrigin_LineOfBusiness: PackageOrigin = PackageOrigin(6i32);
impl ::core::convert::From<i32> for PackageOrigin {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackageOrigin {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PackagePathType(pub i32);
pub const PackagePathType_Install: PackagePathType = PackagePathType(0i32);
pub const PackagePathType_Mutable: PackagePathType = PackagePathType(1i32);
pub const PackagePathType_Effective: PackagePathType = PackagePathType(2i32);
pub const PackagePathType_MachineExternal: PackagePathType = PackagePathType(3i32);
pub const PackagePathType_UserExternal: PackagePathType = PackagePathType(4i32);
pub const PackagePathType_EffectiveExternal: PackagePathType = PackagePathType(5i32);
impl ::core::convert::From<i32> for PackagePathType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PackagePathType {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ParseApplicationUserModelId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(applicationusermodelid: Param0, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ParseApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(ParseApplicationUserModelId(applicationusermodelid.into_param().abi(), ::core::mem::transmute(packagefamilynamelength), ::core::mem::transmute(packagefamilyname), ::core::mem::transmute(packagerelativeapplicationidlength), ::core::mem::transmute(packagerelativeapplicationid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__);
        }
        ::core::mem::transmute(ReleasePackageVirtualizationContext(::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::core::HRESULT;
        }
        RemovePackageDependency(::core::mem::transmute(packagedependencycontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCreatePackageDependency<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSID>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, PACKAGE_VERSION>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    user: Param0,
    packagefamilyname: Param1,
    minversion: Param2,
    packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures,
    lifetimekind: PackageDependencyLifetimeKind,
    lifetimeartifact: Param5,
    options: CreatePackageDependencyOptions,
) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryCreatePackageDependency(user: super::super::super::Foundation::PSID, packagefamilyname: super::super::super::Foundation::PWSTR, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: super::super::super::Foundation::PWSTR, options: CreatePackageDependencyOptions, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        TryCreatePackageDependency(user.into_param().abi(), packagefamilyname.into_param().abi(), minversion.into_param().abi(), ::core::mem::transmute(packagedependencyprocessorarchitectures), ::core::mem::transmute(lifetimekind), lifetimeartifact.into_param().abi(), ::core::mem::transmute(options), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyApplicationUserModelId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(applicationusermodelid: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(VerifyApplicationUserModelId(applicationusermodelid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefamilyname: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(VerifyPackageFamilyName(packagefamilyname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageFullName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagefullname: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageFullName(packagefullname: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(VerifyPackageFullName(packagefullname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageId(packageid: *const PACKAGE_ID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageId(packageid: *const PACKAGE_ID) -> i32;
        }
        ::core::mem::transmute(VerifyPackageId(::core::mem::transmute(packageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageRelativeApplicationId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(packagerelativeapplicationid: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
        }
        ::core::mem::transmute(VerifyPackageRelativeApplicationId(packagerelativeapplicationid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut ::core::ffi::c_void,
}
impl _PACKAGE_INFO_REFERENCE {}
impl ::core::default::Default for _PACKAGE_INFO_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for _PACKAGE_INFO_REFERENCE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_PACKAGE_INFO_REFERENCE").field("reserved", &self.reserved).finish()
    }
}
impl ::core::cmp::PartialEq for _PACKAGE_INFO_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for _PACKAGE_INFO_REFERENCE {}
unsafe impl ::windows::core::Abi for _PACKAGE_INFO_REFERENCE {
    type Abi = Self;
}
