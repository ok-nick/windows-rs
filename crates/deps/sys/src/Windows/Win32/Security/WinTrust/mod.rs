#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialog(hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPersonalTrustDBDialogEx(hwndparent: super::super::Foundation::HWND, dwflags: u32, pvreserved: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperCertCheckValidSignature(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperCertIsSelfSigned(dwencoding: u32, pcert: *mut super::Cryptography::CERT_INFO) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WTHelperGetProvCertFromChain(psgnr: *mut CRYPT_PROVIDER_SGNR, idxcert: u32) -> *mut CRYPT_PROVIDER_CERT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvPrivateDataFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, pgproviderid: *mut ::windows_sys::core::GUID) -> *mut CRYPT_PROVIDER_PRIVDATA;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperGetProvSignerFromChain(pprovdata: *mut CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32) -> *mut CRYPT_PROVIDER_SGNR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WTHelperProvDataFromStateData(hstatedata: super::super::Foundation::HANDLE) -> *mut CRYPT_PROVIDER_DATA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinVerifyTrust(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows_sys::core::GUID, pwvtdata: *mut ::core::ffi::c_void) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn WinVerifyTrustEx(hwnd: super::super::Foundation::HWND, pgactionid: *mut ::windows_sys::core::GUID, pwintrustdata: *mut WINTRUST_DATA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddActionID(pgactionid: *const ::windows_sys::core::GUID, fdwflags: u32, psprovinfo: *const CRYPT_REGISTER_ACTIONID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustAddDefaultForUsage(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_REGDEFUSAGE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustGetDefaultForUsage(dwaction: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION, pszusageoid: super::super::Foundation::PSTR, psusage: *mut CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL;
    pub fn WintrustGetRegPolicyFlags(pdwpolicyflags: *mut WINTRUST_POLICY_FLAGS);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
    pub fn WintrustLoadFunctionPointers(pgactionid: *mut ::windows_sys::core::GUID, ppfns: *mut CRYPT_PROVIDER_FUNCTIONS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustRemoveActionID(pgactionid: *const ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetDefaultIncludePEPageHashes(fincludepepagehashes: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WintrustSetRegPolicyFlags(dwpolicyflags: WINTRUST_POLICY_FLAGS) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAT_MEMBERINFO {
    pub pwszSubjGuid: super::super::Foundation::PWSTR,
    pub dwCertVersion: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAT_MEMBERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAT_MEMBERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CAT_MEMBERINFO2 {
    pub SubjectGuid: ::windows_sys::core::GUID,
    pub dwCertVersion: u32,
}
impl ::core::marker::Copy for CAT_MEMBERINFO2 {}
impl ::core::clone::Clone for CAT_MEMBERINFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CAT_NAMEVALUE {
    pub pwszTag: super::super::Foundation::PWSTR,
    pub fdwFlags: u32,
    pub Value: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for CAT_NAMEVALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CAT_NAMEVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CONFIG_CI_PROV_INFO {
    pub cbSize: u32,
    pub dwPolicies: u32,
    pub pPolicies: *mut super::Cryptography::CRYPTOAPI_BLOB,
    pub result: CONFIG_CI_PROV_INFO_RESULT,
    pub dwScenario: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for CONFIG_CI_PROV_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CONFIG_CI_PROV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFIG_CI_PROV_INFO_RESULT {
    pub hr: ::windows_sys::core::HRESULT,
    pub dwResult: u32,
    pub dwPolicyIndex: u32,
    pub fIsExplicitDeny: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONFIG_CI_PROV_INFO_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONFIG_CI_PROV_INFO_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CRYPT_PROVIDER_CERT {
    pub cbStruct: u32,
    pub pCert: *mut super::Cryptography::CERT_CONTEXT,
    pub fCommercial: super::super::Foundation::BOOL,
    pub fTrustedRoot: super::super::Foundation::BOOL,
    pub fSelfSigned: super::super::Foundation::BOOL,
    pub fTestCert: super::super::Foundation::BOOL,
    pub dwRevokedReason: u32,
    pub dwConfidence: u32,
    pub dwError: u32,
    pub pTrustListContext: *mut super::Cryptography::CTL_CONTEXT,
    pub fTrustListSignerCert: super::super::Foundation::BOOL,
    pub pCtlContext: *mut super::Cryptography::CTL_CONTEXT,
    pub dwCtlError: u32,
    pub fIsCyclic: super::super::Foundation::BOOL,
    pub pChainElement: *mut super::Cryptography::CERT_CHAIN_ELEMENT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_CERT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_CERT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct CRYPT_PROVIDER_DATA {
    pub cbStruct: u32,
    pub pWintrustData: *mut WINTRUST_DATA,
    pub fOpenedFile: super::super::Foundation::BOOL,
    pub hWndParent: super::super::Foundation::HWND,
    pub pgActionID: *mut ::windows_sys::core::GUID,
    pub hProv: usize,
    pub dwError: u32,
    pub dwRegSecuritySettings: u32,
    pub dwRegPolicySettings: u32,
    pub psPfns: *mut CRYPT_PROVIDER_FUNCTIONS,
    pub cdwTrustStepErrors: u32,
    pub padwTrustStepErrors: *mut u32,
    pub chStores: u32,
    pub pahStores: *mut *mut ::core::ffi::c_void,
    pub dwEncoding: u32,
    pub hMsg: *mut ::core::ffi::c_void,
    pub csSigners: u32,
    pub pasSigners: *mut CRYPT_PROVIDER_SGNR,
    pub csProvPrivData: u32,
    pub pasProvPrivData: *mut CRYPT_PROVIDER_PRIVDATA,
    pub dwSubjectChoice: u32,
    pub Anonymous: CRYPT_PROVIDER_DATA_0,
    pub pszUsageOID: super::super::Foundation::PSTR,
    pub fRecallWithState: super::super::Foundation::BOOL,
    pub sftSystemTime: super::super::Foundation::FILETIME,
    pub pszCTLSignerUsageOID: super::super::Foundation::PSTR,
    pub dwProvFlags: u32,
    pub dwFinalError: u32,
    pub pRequestUsage: *mut super::Cryptography::CERT_USAGE_MATCH,
    pub dwTrustPubSettings: u32,
    pub dwUIStateFlags: u32,
    pub pSigState: *mut CRYPT_PROVIDER_SIGSTATE,
    pub pSigSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub union CRYPT_PROVIDER_DATA_0 {
    pub pPDSip: *mut PROVDATA_SIP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CRYPT_PROVIDER_DEFUSAGE {
    pub cbStruct: u32,
    pub gActionID: ::windows_sys::core::GUID,
    pub pDefPolicyCallbackData: *mut ::core::ffi::c_void,
    pub pDefSIPClientData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_DEFUSAGE {}
impl ::core::clone::Clone for CRYPT_PROVIDER_DEFUSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct CRYPT_PROVIDER_FUNCTIONS {
    pub cbStruct: u32,
    pub pfnAlloc: PFN_CPD_MEM_ALLOC,
    pub pfnFree: PFN_CPD_MEM_FREE,
    pub pfnAddStore2Chain: PFN_CPD_ADD_STORE,
    pub pfnAddSgnr2Chain: PFN_CPD_ADD_SGNR,
    pub pfnAddCert2Chain: PFN_CPD_ADD_CERT,
    pub pfnAddPrivData2Chain: PFN_CPD_ADD_PRIVDATA,
    pub pfnInitialize: PFN_PROVIDER_INIT_CALL,
    pub pfnObjectTrust: PFN_PROVIDER_OBJTRUST_CALL,
    pub pfnSignatureTrust: PFN_PROVIDER_SIGTRUST_CALL,
    pub pfnCertificateTrust: PFN_PROVIDER_CERTTRUST_CALL,
    pub pfnFinalPolicy: PFN_PROVIDER_FINALPOLICY_CALL,
    pub pfnCertCheckPolicy: PFN_PROVIDER_CERTCHKPOLICY_CALL,
    pub pfnTestFinalPolicy: PFN_PROVIDER_TESTFINALPOLICY_CALL,
    pub psUIpfns: *mut CRYPT_PROVUI_FUNCS,
    pub pfnCleanupPolicy: PFN_PROVIDER_CLEANUP_CALL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CRYPT_PROVIDER_PRIVDATA {
    pub cbStruct: u32,
    pub gProviderID: ::windows_sys::core::GUID,
    pub cbProvData: u32,
    pub pvProvData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CRYPT_PROVIDER_PRIVDATA {}
impl ::core::clone::Clone for CRYPT_PROVIDER_PRIVDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVIDER_REGDEFUSAGE {
    pub cbStruct: u32,
    pub pgActionID: *mut ::windows_sys::core::GUID,
    pub pwszDllName: super::super::Foundation::PWSTR,
    pub pwszLoadCallbackDataFunctionName: super::super::Foundation::PSTR,
    pub pwszFreeCallbackDataFunctionName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_PROVIDER_REGDEFUSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_PROVIDER_REGDEFUSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CRYPT_PROVIDER_SGNR {
    pub cbStruct: u32,
    pub sftVerifyAsOf: super::super::Foundation::FILETIME,
    pub csCertChain: u32,
    pub pasCertChain: *mut CRYPT_PROVIDER_CERT,
    pub dwSignerType: u32,
    pub psSigner: *mut super::Cryptography::CMSG_SIGNER_INFO,
    pub dwError: u32,
    pub csCounterSigners: u32,
    pub pasCounterSigners: *mut CRYPT_PROVIDER_SGNR,
    pub pChainContext: *mut super::Cryptography::CERT_CHAIN_CONTEXT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_SGNR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_SGNR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CRYPT_PROVIDER_SIGSTATE {
    pub cbStruct: u32,
    pub rhSecondarySigs: *mut *mut ::core::ffi::c_void,
    pub hPrimarySig: *mut ::core::ffi::c_void,
    pub fFirstAttemptMade: super::super::Foundation::BOOL,
    pub fNoMoreSigs: super::super::Foundation::BOOL,
    pub cSecondarySigs: u32,
    pub dwCurrentIndex: u32,
    pub fSupportMultiSig: super::super::Foundation::BOOL,
    pub dwCryptoPolicySupport: u32,
    pub iAttemptCount: u32,
    pub fCheckedSealing: super::super::Foundation::BOOL,
    pub pSealingSignature: *mut SEALING_SIGNATURE_ATTRIBUTE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for CRYPT_PROVIDER_SIGSTATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CRYPT_PROVIDER_SIGSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_PROVUI_DATA {
    pub cbStruct: u32,
    pub dwFinalError: u32,
    pub pYesButtonText: super::super::Foundation::PWSTR,
    pub pNoButtonText: super::super::Foundation::PWSTR,
    pub pMoreInfoButtonText: super::super::Foundation::PWSTR,
    pub pAdvancedLinkText: super::super::Foundation::PWSTR,
    pub pCopyActionText: super::super::Foundation::PWSTR,
    pub pCopyActionTextNoTS: super::super::Foundation::PWSTR,
    pub pCopyActionTextNotSigned: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_PROVUI_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_PROVUI_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct CRYPT_PROVUI_FUNCS {
    pub cbStruct: u32,
    pub psUIData: *mut CRYPT_PROVUI_DATA,
    pub pfnOnMoreInfoClick: PFN_PROVUI_CALL,
    pub pfnOnMoreInfoClickDefault: PFN_PROVUI_CALL,
    pub pfnOnAdvancedClick: PFN_PROVUI_CALL,
    pub pfnOnAdvancedClickDefault: PFN_PROVUI_CALL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for CRYPT_PROVUI_FUNCS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for CRYPT_PROVUI_FUNCS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_REGISTER_ACTIONID {
    pub cbStruct: u32,
    pub sInitProvider: CRYPT_TRUST_REG_ENTRY,
    pub sObjectProvider: CRYPT_TRUST_REG_ENTRY,
    pub sSignatureProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCertificateProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCertificatePolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sFinalPolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sTestPolicyProvider: CRYPT_TRUST_REG_ENTRY,
    pub sCleanupProvider: CRYPT_TRUST_REG_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_REGISTER_ACTIONID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_REGISTER_ACTIONID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CRYPT_TRUST_REG_ENTRY {
    pub cbStruct: u32,
    pub pwszDLLName: super::super::Foundation::PWSTR,
    pub pwszFunctionName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CRYPT_TRUST_REG_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CRYPT_TRUST_REG_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct DRIVER_VER_INFO {
    pub cbStruct: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub dwPlatform: u32,
    pub dwVersion: u32,
    pub wszVersion: [u16; 260],
    pub wszSignedBy: [u16; 260],
    pub pcSignerCertContext: *mut super::Cryptography::CERT_CONTEXT,
    pub sOSVersionLow: DRIVER_VER_MAJORMINOR,
    pub sOSVersionHigh: DRIVER_VER_MAJORMINOR,
    pub dwBuildNumberLow: u32,
    pub dwBuildNumberHigh: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for DRIVER_VER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for DRIVER_VER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DRIVER_VER_MAJORMINOR {
    pub dwMajor: u32,
    pub dwMinor: u32,
}
impl ::core::marker::Copy for DRIVER_VER_MAJORMINOR {}
impl ::core::clone::Clone for DRIVER_VER_MAJORMINOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTENT_TO_SEAL_ATTRIBUTE {
    pub version: u32,
    pub seal: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INTENT_TO_SEAL_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INTENT_TO_SEAL_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PFN_ALLOCANDFILLDEFUSAGE = ::core::option::Option<unsafe extern "system" fn(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_CERT = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersigner: super::super::Foundation::BOOL, idxcountersigner: u32, pcert2add: *const super::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_PRIVDATA = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, pprivdata2add: *const CRYPT_PROVIDER_PRIVDATA) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_SGNR = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, fcountersigner: super::super::Foundation::BOOL, idxsigner: u32, psgnr2add: *const CRYPT_PROVIDER_SGNR) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_CPD_ADD_STORE = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, hstore2add: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PFN_CPD_MEM_ALLOC = ::core::option::Option<unsafe extern "system" fn(cbsize: u32) -> *mut ::core::ffi::c_void>;
pub type PFN_CPD_MEM_FREE = ::core::option::Option<unsafe extern "system" fn(pvmem2free: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FREEDEFUSAGE = ::core::option::Option<unsafe extern "system" fn(pszusageoid: super::super::Foundation::PSTR, psdefusage: *const CRYPT_PROVIDER_DEFUSAGE) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CERTCHKPOLICY_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *const CRYPT_PROVIDER_DATA, idxsigner: u32, fcountersignerchain: super::super::Foundation::BOOL, idxcountersigner: u32) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CERTTRUST_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_CLEANUP_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_FINALPOLICY_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_INIT_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_OBJTRUST_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_SIGTRUST_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVIDER_TESTFINALPOLICY_CALL = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA) -> ::windows_sys::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_PROVUI_CALL = ::core::option::Option<unsafe extern "system" fn(hwndsecuritydialog: super::super::Foundation::HWND, pprovdata: *const CRYPT_PROVIDER_DATA) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub type PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pprovdata: *mut CRYPT_PROVIDER_DATA, dwsteperror: u32, dwregpolicysettings: u32, csigner: u32, rgpsigner: *mut *mut WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO, pvpolicyarg: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct PROVDATA_SIP {
    pub cbStruct: u32,
    pub gSubject: ::windows_sys::core::GUID,
    pub pSip: *mut super::Cryptography::Sip::SIP_DISPATCH_INFO,
    pub pCATSip: *mut super::Cryptography::Sip::SIP_DISPATCH_INFO,
    pub psSipSubjectInfo: *mut super::Cryptography::Sip::SIP_SUBJECTINFO,
    pub psSipCATSubjectInfo: *mut super::Cryptography::Sip::SIP_SUBJECTINFO,
    pub psIndirectData: *mut super::Cryptography::Sip::SIP_INDIRECT_DATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for PROVDATA_SIP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for PROVDATA_SIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SEALING_SIGNATURE_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub signatureAlgorithm: super::Cryptography::CRYPT_ALGORITHM_IDENTIFIER,
    pub encryptedDigest: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SEALING_SIGNATURE_ATTRIBUTE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SEALING_SIGNATURE_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SEALING_TIMESTAMP_ATTRIBUTE {
    pub version: u32,
    pub signerIndex: u32,
    pub sealTimeStampToken: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SEALING_TIMESTAMP_ATTRIBUTE {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SEALING_TIMESTAMP_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPC_FINANCIAL_CRITERIA {
    pub fFinancialInfoAvailable: super::super::Foundation::BOOL,
    pub fMeetsCriteria: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPC_FINANCIAL_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPC_FINANCIAL_CRITERIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SPC_IMAGE {
    pub pImageLink: *mut SPC_LINK,
    pub Bitmap: super::Cryptography::CRYPTOAPI_BLOB,
    pub Metafile: super::Cryptography::CRYPTOAPI_BLOB,
    pub EnhancedMetafile: super::Cryptography::CRYPTOAPI_BLOB,
    pub GifFile: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SPC_IMAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SPC_IMAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SPC_INDIRECT_DATA_CONTENT {
    pub Data: super::Cryptography::CRYPT_ATTRIBUTE_TYPE_VALUE,
    pub DigestAlgorithm: super::Cryptography::CRYPT_ALGORITHM_IDENTIFIER,
    pub Digest: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SPC_INDIRECT_DATA_CONTENT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SPC_INDIRECT_DATA_CONTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SPC_LINK {
    pub dwLinkChoice: u32,
    pub Anonymous: SPC_LINK_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SPC_LINK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SPC_LINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub union SPC_LINK_0 {
    pub pwszUrl: super::super::Foundation::PWSTR,
    pub Moniker: SPC_SERIALIZED_OBJECT,
    pub pwszFile: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SPC_LINK_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SPC_LINK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SPC_PE_IMAGE_DATA {
    pub Flags: super::Cryptography::CRYPT_BIT_BLOB,
    pub pFile: *mut SPC_LINK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SPC_PE_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SPC_PE_IMAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct SPC_SERIALIZED_OBJECT {
    pub ClassId: [u8; 16],
    pub SerializedData: super::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for SPC_SERIALIZED_OBJECT {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for SPC_SERIALIZED_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SPC_SIGINFO {
    pub dwSipVersion: u32,
    pub gSIPGuid: ::windows_sys::core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
    pub dwReserved5: u32,
}
impl ::core::marker::Copy for SPC_SIGINFO {}
impl ::core::clone::Clone for SPC_SIGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SPC_SP_AGENCY_INFO {
    pub pPolicyInformation: *mut SPC_LINK,
    pub pwszPolicyDisplayText: super::super::Foundation::PWSTR,
    pub pLogoImage: *mut SPC_IMAGE,
    pub pLogoLink: *mut SPC_LINK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SPC_SP_AGENCY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SPC_SP_AGENCY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SPC_SP_OPUS_INFO {
    pub pwszProgramName: super::super::Foundation::PWSTR,
    pub pMoreInfo: *mut SPC_LINK,
    pub pPublisherInfo: *mut SPC_LINK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SPC_SP_OPUS_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SPC_SP_OPUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SPC_STATEMENT_TYPE {
    pub cKeyPurposeId: u32,
    pub rgpszKeyPurposeId: *mut super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPC_STATEMENT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPC_STATEMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SPC_UUID_LENGTH: u32 = 16u32;
pub const TRUSTERROR_MAX_STEPS: u32 = 38u32;
pub const TRUSTERROR_STEP_CATALOGFILE: u32 = 6u32;
pub const TRUSTERROR_STEP_CERTSTORE: u32 = 7u32;
pub const TRUSTERROR_STEP_FILEIO: u32 = 2u32;
pub const TRUSTERROR_STEP_FINAL_CERTCHKPROV: u32 = 35u32;
pub const TRUSTERROR_STEP_FINAL_CERTPROV: u32 = 34u32;
pub const TRUSTERROR_STEP_FINAL_INITPROV: u32 = 31u32;
pub const TRUSTERROR_STEP_FINAL_OBJPROV: u32 = 32u32;
pub const TRUSTERROR_STEP_FINAL_POLICYPROV: u32 = 36u32;
pub const TRUSTERROR_STEP_FINAL_SIGPROV: u32 = 33u32;
pub const TRUSTERROR_STEP_FINAL_UIPROV: u32 = 37u32;
pub const TRUSTERROR_STEP_FINAL_WVTINIT: u32 = 30u32;
pub const TRUSTERROR_STEP_MESSAGE: u32 = 8u32;
pub const TRUSTERROR_STEP_MSG_CERTCHAIN: u32 = 15u32;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGCERT: u32 = 17u32;
pub const TRUSTERROR_STEP_MSG_COUNTERSIGINFO: u32 = 16u32;
pub const TRUSTERROR_STEP_MSG_INNERCNT: u32 = 11u32;
pub const TRUSTERROR_STEP_MSG_INNERCNTTYPE: u32 = 10u32;
pub const TRUSTERROR_STEP_MSG_SIGNERCERT: u32 = 14u32;
pub const TRUSTERROR_STEP_MSG_SIGNERCOUNT: u32 = 9u32;
pub const TRUSTERROR_STEP_MSG_SIGNERINFO: u32 = 13u32;
pub const TRUSTERROR_STEP_MSG_STORE: u32 = 12u32;
pub const TRUSTERROR_STEP_SIP: u32 = 3u32;
pub const TRUSTERROR_STEP_SIPSUBJINFO: u32 = 5u32;
pub const TRUSTERROR_STEP_VERIFY_MSGHASH: u32 = 18u32;
pub const TRUSTERROR_STEP_VERIFY_MSGINDIRECTDATA: u32 = 19u32;
pub const TRUSTERROR_STEP_WVTPARAMS: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WINTRUST_BLOB_INFO {
    pub cbStruct: u32,
    pub gSubject: ::windows_sys::core::GUID,
    pub pcwszDisplayName: super::super::Foundation::PWSTR,
    pub cbMemObject: u32,
    pub pbMemObject: *mut u8,
    pub cbMemSignedMsg: u32,
    pub pbMemSignedMsg: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINTRUST_BLOB_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINTRUST_BLOB_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_CATALOG_INFO {
    pub cbStruct: u32,
    pub dwCatalogVersion: u32,
    pub pcwszCatalogFilePath: super::super::Foundation::PWSTR,
    pub pcwszMemberTag: super::super::Foundation::PWSTR,
    pub pcwszMemberFilePath: super::super::Foundation::PWSTR,
    pub hMemberFile: super::super::Foundation::HANDLE,
    pub pbCalculatedFileHash: *mut u8,
    pub cbCalculatedFileHash: u32,
    pub pcCatalogContext: *mut super::Cryptography::CTL_CONTEXT,
    pub hCatAdmin: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_CATALOG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_CATALOG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_CERT_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: super::super::Foundation::PWSTR,
    pub psCertContext: *mut super::Cryptography::CERT_CONTEXT,
    pub chStores: u32,
    pub pahStores: *mut *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub psftVerifyAsOf: *mut super::super::Foundation::FILETIME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_CERT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_DATA {
    pub cbStruct: u32,
    pub pPolicyCallbackData: *mut ::core::ffi::c_void,
    pub pSIPClientData: *mut ::core::ffi::c_void,
    pub dwUIChoice: WINTRUST_DATA_UICHOICE,
    pub fdwRevocationChecks: WINTRUST_DATA_REVOCATION_CHECKS,
    pub dwUnionChoice: WINTRUST_DATA_UNION_CHOICE,
    pub Anonymous: WINTRUST_DATA_0,
    pub dwStateAction: WINTRUST_DATA_STATE_ACTION,
    pub hWVTStateData: super::super::Foundation::HANDLE,
    pub pwszURLReference: super::super::Foundation::PWSTR,
    pub dwProvFlags: u32,
    pub dwUIContext: WINTRUST_DATA_UICONTEXT,
    pub pSignatureSettings: *mut WINTRUST_SIGNATURE_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub union WINTRUST_DATA_0 {
    pub pFile: *mut WINTRUST_FILE_INFO,
    pub pCatalog: *mut WINTRUST_CATALOG_INFO,
    pub pBlob: *mut WINTRUST_BLOB_INFO,
    pub pSgnr: *mut WINTRUST_SGNR_INFO,
    pub pCert: *mut WINTRUST_CERT_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WINTRUST_DATA_REVOCATION_CHECKS = u32;
pub const WTD_REVOKE_NONE: WINTRUST_DATA_REVOCATION_CHECKS = 0u32;
pub const WTD_REVOKE_WHOLECHAIN: WINTRUST_DATA_REVOCATION_CHECKS = 1u32;
pub type WINTRUST_DATA_STATE_ACTION = u32;
pub const WTD_STATEACTION_IGNORE: WINTRUST_DATA_STATE_ACTION = 0u32;
pub const WTD_STATEACTION_VERIFY: WINTRUST_DATA_STATE_ACTION = 1u32;
pub const WTD_STATEACTION_CLOSE: WINTRUST_DATA_STATE_ACTION = 2u32;
pub const WTD_STATEACTION_AUTO_CACHE: WINTRUST_DATA_STATE_ACTION = 3u32;
pub const WTD_STATEACTION_AUTO_CACHE_FLUSH: WINTRUST_DATA_STATE_ACTION = 4u32;
pub type WINTRUST_DATA_UICHOICE = u32;
pub const WTD_UI_ALL: WINTRUST_DATA_UICHOICE = 1u32;
pub const WTD_UI_NONE: WINTRUST_DATA_UICHOICE = 2u32;
pub const WTD_UI_NOBAD: WINTRUST_DATA_UICHOICE = 3u32;
pub const WTD_UI_NOGOOD: WINTRUST_DATA_UICHOICE = 4u32;
pub type WINTRUST_DATA_UICONTEXT = u32;
pub const WTD_UICONTEXT_EXECUTE: WINTRUST_DATA_UICONTEXT = 0u32;
pub const WTD_UICONTEXT_INSTALL: WINTRUST_DATA_UICONTEXT = 1u32;
pub type WINTRUST_DATA_UNION_CHOICE = u32;
pub const WTD_CHOICE_FILE: WINTRUST_DATA_UNION_CHOICE = 1u32;
pub const WTD_CHOICE_CATALOG: WINTRUST_DATA_UNION_CHOICE = 2u32;
pub const WTD_CHOICE_BLOB: WINTRUST_DATA_UNION_CHOICE = 3u32;
pub const WTD_CHOICE_SIGNER: WINTRUST_DATA_UNION_CHOICE = 4u32;
pub const WTD_CHOICE_CERT: WINTRUST_DATA_UNION_CHOICE = 5u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WINTRUST_FILE_INFO {
    pub cbStruct: u32,
    pub pcwszFilePath: super::super::Foundation::PWSTR,
    pub hFile: super::super::Foundation::HANDLE,
    pub pgKnownSubject: *mut ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WINTRUST_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WINTRUST_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = u32;
pub const DWACTION_ALLOCANDFILL: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = 1u32;
pub const DWACTION_FREE: WINTRUST_GET_DEFAULT_FOR_USAGE_ACTION = 2u32;
pub const WINTRUST_MAX_HASH_BYTES_TO_MAP_DEFAULT: u32 = 1048576u32;
pub const WINTRUST_MAX_HEADER_BYTES_TO_MAP_DEFAULT: u32 = 10485760u32;
pub type WINTRUST_POLICY_FLAGS = u32;
pub const WTPF_TRUSTTEST: WINTRUST_POLICY_FLAGS = 32u32;
pub const WTPF_TESTCANBEVALID: WINTRUST_POLICY_FLAGS = 128u32;
pub const WTPF_IGNOREEXPIRATION: WINTRUST_POLICY_FLAGS = 256u32;
pub const WTPF_IGNOREREVOKATION: WINTRUST_POLICY_FLAGS = 512u32;
pub const WTPF_OFFLINEOK_IND: WINTRUST_POLICY_FLAGS = 1024u32;
pub const WTPF_OFFLINEOK_COM: WINTRUST_POLICY_FLAGS = 2048u32;
pub const WTPF_OFFLINEOKNBU_IND: WINTRUST_POLICY_FLAGS = 4096u32;
pub const WTPF_OFFLINEOKNBU_COM: WINTRUST_POLICY_FLAGS = 8192u32;
pub const WTPF_VERIFY_V1_OFF: WINTRUST_POLICY_FLAGS = 65536u32;
pub const WTPF_IGNOREREVOCATIONONTS: WINTRUST_POLICY_FLAGS = 131072u32;
pub const WTPF_ALLOWONLYPERTRUST: WINTRUST_POLICY_FLAGS = 262144u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_SGNR_INFO {
    pub cbStruct: u32,
    pub pcwszDisplayName: super::super::Foundation::PWSTR,
    pub psSignerInfo: *mut super::Cryptography::CMSG_SIGNER_INFO,
    pub chStores: u32,
    pub pahStores: *mut *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_SGNR_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_SGNR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WINTRUST_SIGNATURE_SETTINGS {
    pub cbStruct: u32,
    pub dwIndex: u32,
    pub dwFlags: WINTRUST_SIGNATURE_SETTINGS_FLAGS,
    pub cSecondarySigs: u32,
    pub dwVerifiedSigIndex: u32,
    pub pCryptoPolicy: *mut super::Cryptography::CERT_STRONG_SIGN_PARA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WINTRUST_SIGNATURE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WINTRUST_SIGNATURE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WINTRUST_SIGNATURE_SETTINGS_FLAGS = u32;
pub const WSS_VERIFY_SPECIFIC: WINTRUST_SIGNATURE_SETTINGS_FLAGS = 1u32;
pub const WSS_GET_SECONDARY_SIG_COUNT: WINTRUST_SIGNATURE_SETTINGS_FLAGS = 2u32;
#[repr(C)]
pub struct WIN_CERTIFICATE {
    pub dwLength: u32,
    pub wRevision: u16,
    pub wCertificateType: u16,
    pub bCertificate: [u8; 1],
}
impl ::core::marker::Copy for WIN_CERTIFICATE {}
impl ::core::clone::Clone for WIN_CERTIFICATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WIN_CERT_REVISION_1_0: u32 = 256u32;
pub const WIN_CERT_REVISION_2_0: u32 = 512u32;
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u32 = 2u32;
pub const WIN_CERT_TYPE_RESERVED_1: u32 = 3u32;
pub const WIN_CERT_TYPE_TS_STACK_SIGNED: u32 = 4u32;
pub const WIN_CERT_TYPE_X509: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    pub hClientToken: super::super::Foundation::HANDLE,
    pub lpCertificate: *mut WIN_CERTIFICATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_SPUB_TRUSTED_PUBLISHER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_SPUB_TRUSTED_PUBLISHER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    pub hClientToken: super::super::Foundation::HANDLE,
    pub SubjectType: *mut ::windows_sys::core::GUID,
    pub Subject: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_TRUST_ACTDATA_CONTEXT_WITH_SUBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    pub SubjectType: *mut ::windows_sys::core::GUID,
    pub Subject: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WIN_TRUST_ACTDATA_SUBJECT_ONLY {}
impl ::core::clone::Clone for WIN_TRUST_ACTDATA_SUBJECT_ONLY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_SUBJECT_FILE {
    pub hFile: super::super::Foundation::HANDLE,
    pub lpPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_TRUST_SUBJECT_FILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_TRUST_SUBJECT_FILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    pub hFile: super::super::Foundation::HANDLE,
    pub lpPath: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WIN_TRUST_SUBJECT_FILE_AND_DISPLAY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WSS_CERTTRUST_SUPPORT: u32 = 4u32;
pub const WSS_INPUT_FLAG_MASK: u32 = 7u32;
pub const WSS_OBJTRUST_SUPPORT: u32 = 1u32;
pub const WSS_OUTPUT_FLAG_MASK: u32 = 3758096384u32;
pub const WSS_OUT_FILE_SUPPORTS_SEAL: u32 = 536870912u32;
pub const WSS_OUT_HAS_SEALING_INTENT: u32 = 1073741824u32;
pub const WSS_OUT_SEALING_STATUS_VERIFIED: u32 = 2147483648u32;
pub const WSS_SIGTRUST_SUPPORT: u32 = 2u32;
pub const WSS_VERIFY_SEALING: u32 = 4u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0,
    pub hChainEngine: super::Cryptography::HCERTCHAINENGINE,
    pub pChainPara: *mut super::Cryptography::CERT_CHAIN_PARA,
    pub dwFlags: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub union WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_CREATE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub struct WTD_GENERIC_CHAIN_POLICY_DATA {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_DATA_0,
    pub pSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pCounterSignerChainInfo: *mut WTD_GENERIC_CHAIN_POLICY_CREATE_INFO,
    pub pfnPolicyCallback: PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK,
    pub pvPolicyArg: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
pub union WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Security_Cryptography_Catalog", feature = "Win32_Security_Cryptography_Sip"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    pub Anonymous: WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0,
    pub pChainContext: *mut super::Cryptography::CERT_CHAIN_CONTEXT,
    pub dwSignerType: u32,
    pub pMsgSignerInfo: *mut super::Cryptography::CMSG_SIGNER_INFO,
    pub dwError: u32,
    pub cCounterSigner: u32,
    pub rgpCounterSigner: *mut *mut WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub union WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    pub cbStruct: u32,
    pub cbSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for WTD_GENERIC_CHAIN_POLICY_SIGNER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WT_ADD_ACTION_ID_RET_RESULT_FLAG: u32 = 1u32;
pub const WT_CURRENT_VERSION: u32 = 512u32;
pub const WT_TRUSTDBDIALOG_NO_UI_FLAG: u32 = 1u32;
pub const WT_TRUSTDBDIALOG_ONLY_PUB_TAB_FLAG: u32 = 2u32;
pub const WT_TRUSTDBDIALOG_WRITE_IEAK_STORE_FLAG: u32 = 512u32;
pub const WT_TRUSTDBDIALOG_WRITE_LEGACY_REG_FLAG: u32 = 256u32;
