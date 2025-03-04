#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateA(filter: super::super::Foundation::PSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateW(filter: super::super::Foundation::PWSTR, flags: CRED_ENUMERATE_FLAGS, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    pub fn CredFree(buffer: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetSessionTypes(maximumpersistcount: u32, maximumpersist: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoA(targetname: super::super::Foundation::PSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoW(targetname: super::super::Foundation::PWSTR, flags: u32, targetinfo: *mut *mut CREDENTIAL_TARGET_INFORMATIONW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialA(marshaledcredential: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialW(marshaledcredential: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedA(pszprotectedcredentials: super::super::Foundation::PSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedW(pszprotectedcredentials: super::super::Foundation::PWSTR, pprotectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialA(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialW(credtype: CRED_MARSHAL_TYPE, credential: *const ::core::ffi::c_void, marshaledcredential: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pszusername: super::super::Foundation::PSTR, pszpassword: super::super::Foundation::PSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, ppackedcredentials: *mut u8, pcbpackedcredentials: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectA(fasself: super::super::Foundation::BOOL, pszcredentials: super::super::Foundation::PSTR, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectW(fasself: super::super::Foundation::BOOL, pszcredentials: super::super::Foundation::PWSTR, cchcredentials: u32, pszprotectedcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32, protectiontype: *mut CRED_PROTECTION_TYPE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadA(targetname: super::super::Foundation::PSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, flags: u32, count: *mut u32, credential: *mut *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadW(targetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32, credential: *mut *mut CREDENTIALW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameA(oldtargetname: super::super::Foundation::PSTR, newtargetname: super::super::Foundation::PSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameW(oldtargetname: super::super::Foundation::PWSTR, newtargetname: super::super::Foundation::PWSTR, r#type: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsA(psztargetname: super::super::Foundation::PSTR, pcontext: *mut SecHandle, dwautherror: u32, username: super::super::Foundation::PSTR, uluserbuffersize: u32, pszpassword: super::super::Foundation::PSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsW(psztargetname: super::super::Foundation::PWSTR, pcontext: *mut SecHandle, dwautherror: u32, username: super::super::Foundation::PWSTR, uluserbuffersize: u32, pszpassword: super::super::Foundation::PWSTR, ulpasswordbuffersize: u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsA(psztargetname: super::super::Foundation::PSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsW(psztargetname: super::super::Foundation::PWSTR, bconfirm: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIParseUserNameA(username: super::super::Foundation::PSTR, user: super::super::Foundation::PSTR, userbuffersize: u32, domain: super::super::Foundation::PSTR, domainbuffersize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIParseUserNameW(username: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, userbuffersize: u32, domain: super::super::Foundation::PWSTR, domainbuffersize: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsA(puiinfo: *const CREDUI_INFOA, psztargetname: super::super::Foundation::PSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: super::super::Foundation::PSTR, ulusernamebuffersize: u32, pszpassword: super::super::Foundation::PSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsW(puiinfo: *const CREDUI_INFOW, psztargetname: super::super::Foundation::PWSTR, pcontext: *mut SecHandle, dwautherror: u32, pszusername: super::super::Foundation::PWSTR, ulusernamebuffersize: u32, pszpassword: super::super::Foundation::PWSTR, ulpasswordbuffersize: u32, save: *mut super::super::Foundation::BOOL, dwflags: CREDUI_FLAGS) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsA(puiinfo: *const CREDUI_INFOA, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsW(puiinfo: *const CREDUI_INFOW, dwautherror: u32, pulauthpackage: *mut u32, pvinauthbuffer: *const ::core::ffi::c_void, ulinauthbuffersize: u32, ppvoutauthbuffer: *mut *mut ::core::ffi::c_void, puloutauthbuffersize: *mut u32, pfsave: *mut super::super::Foundation::BOOL, dwflags: CREDUIWIN_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIReadSSOCredW(pszrealm: super::super::Foundation::PWSTR, ppszusername: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIStoreSSOCredW(pszrealm: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, bpersist: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferA(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PSTR, pcchlmaxusername: *mut u32, pszdomainname: super::super::Foundation::PSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferW(dwflags: CRED_PACK_FLAGS, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, pszusername: super::super::Foundation::PWSTR, pcchmaxusername: *mut u32, pszdomainname: super::super::Foundation::PWSTR, pcchmaxdomainname: *mut u32, pszpassword: super::super::Foundation::PWSTR, pcchmaxpassword: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialA(marshaledcredential: super::super::Foundation::PSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialW(marshaledcredential: super::super::Foundation::PWSTR, credtype: *mut CRED_MARSHAL_TYPE, credential: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectA(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: super::super::Foundation::PSTR, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectW(fasself: super::super::Foundation::BOOL, pszprotectedcredentials: super::super::Foundation::PWSTR, cchprotectedcredentials: u32, pszcredentials: super::super::Foundation::PWSTR, pcchmaxchars: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteA(credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsA(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONA, credential: *const CREDENTIALA, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsW(targetinfo: *const CREDENTIAL_TARGET_INFORMATIONW, credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteW(credential: *const CREDENTIALW, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameA(param0: *mut OPENCARDNAMEA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameW(param0: *mut OPENCARDNAMEW) -> i32;
    pub fn KeyCredentialManagerFreeInformation(keycredentialmanagerinfo: *const KeyCredentialManagerInfo);
    pub fn KeyCredentialManagerGetInformation(keycredentialmanagerinfo: *mut *mut KeyCredentialManagerInfo) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerGetOperationErrorStates(keycredentialmanageroperationtype: KeyCredentialManagerOperationType, isready: *mut super::super::Foundation::BOOL, keycredentialmanageroperationerrorstates: *mut KeyCredentialManagerOperationErrorStates) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerShowUIOperation(hwndowner: super::super::Foundation::HWND, keycredentialmanageroperationtype: KeyCredentialManagerOperationType) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAccessStartedEvent() -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAddReaderToGroupA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAddReaderToGroupW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szgroupname: super::super::Foundation::PWSTR) -> i32;
    pub fn SCardAudit(hcontext: usize, dwevent: u32) -> i32;
    pub fn SCardBeginTransaction(hcard: usize) -> i32;
    pub fn SCardCancel(hcontext: usize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardConnectA(hcontext: usize, szreader: super::super::Foundation::PSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardConnectW(hcontext: usize, szreader: super::super::Foundation::PWSTR, dwsharemode: u32, dwpreferredprotocols: u32, phcard: *mut usize, pdwactiveprotocol: *mut u32) -> i32;
    pub fn SCardControl(hcard: usize, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> i32;
    pub fn SCardDisconnect(hcard: usize, dwdisposition: u32) -> i32;
    pub fn SCardDlgExtendedError() -> i32;
    pub fn SCardEndTransaction(hcard: usize, dwdisposition: u32) -> i32;
    pub fn SCardEstablishContext(dwscope: SCARD_SCOPE, pvreserved1: *const ::core::ffi::c_void, pvreserved2: *const ::core::ffi::c_void, phcontext: *mut usize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetCardTypeA(hcontext: usize, szcardname: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetCardTypeW(hcontext: usize, szcardname: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderA(hcontext: usize, szreadername: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderGroupA(hcontext: usize, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderGroupW(hcontext: usize, szgroupname: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderW(hcontext: usize, szreadername: super::super::Foundation::PWSTR) -> i32;
    pub fn SCardFreeMemory(hcontext: usize, pvmem: *const ::core::ffi::c_void) -> i32;
    pub fn SCardGetAttrib(hcard: usize, dwattrid: u32, pbattr: *mut u8, pcbattrlen: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetCardTypeProviderNameA(hcontext: usize, szcardname: super::super::Foundation::PSTR, dwproviderid: u32, szprovider: super::super::Foundation::PSTR, pcchprovider: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetCardTypeProviderNameW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, dwproviderid: u32, szprovider: super::super::Foundation::PWSTR, pcchprovider: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetDeviceTypeIdA(hcontext: usize, szreadername: super::super::Foundation::PSTR, pdwdevicetypeid: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetDeviceTypeIdW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, pdwdevicetypeid: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetProviderIdA(hcontext: usize, szcard: super::super::Foundation::PSTR, pguidproviderid: *mut ::windows_sys::core::GUID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetProviderIdW(hcontext: usize, szcard: super::super::Foundation::PWSTR, pguidproviderid: *mut ::windows_sys::core::GUID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderDeviceInstanceIdA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szdeviceinstanceid: super::super::Foundation::PSTR, pcchdeviceinstanceid: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderDeviceInstanceIdW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szdeviceinstanceid: super::super::Foundation::PWSTR, pcchdeviceinstanceid: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderIconA(hcontext: usize, szreadername: super::super::Foundation::PSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderIconW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, pbicon: *mut u8, pcbicon: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetStatusChangeA(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetStatusChangeW(hcontext: usize, dwtimeout: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    pub fn SCardGetTransmitCount(hcard: usize, pctransmitcount: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceCardTypeA(hcontext: usize, szcardname: super::super::Foundation::PSTR, pguidprimaryprovider: *const ::windows_sys::core::GUID, rgguidinterfaces: *const ::windows_sys::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceCardTypeW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, pguidprimaryprovider: *const ::windows_sys::core::GUID, rgguidinterfaces: *const ::windows_sys::core::GUID, dwinterfacecount: u32, pbatr: *const u8, pbatrmask: *const u8, cbatrlen: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szdevicename: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderGroupA(hcontext: usize, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderGroupW(hcontext: usize, szgroupname: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szdevicename: super::super::Foundation::PWSTR) -> i32;
    pub fn SCardIsValidContext(hcontext: usize) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListCardsA(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows_sys::core::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PSTR, pcchcards: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListCardsW(hcontext: usize, pbatr: *const u8, rgquidinterfaces: *const ::windows_sys::core::GUID, cguidinterfacecount: u32, mszcards: super::super::Foundation::PWSTR, pcchcards: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListInterfacesA(hcontext: usize, szcard: super::super::Foundation::PSTR, pguidinterfaces: *mut ::windows_sys::core::GUID, pcguidinterfaces: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListInterfacesW(hcontext: usize, szcard: super::super::Foundation::PWSTR, pguidinterfaces: *mut ::windows_sys::core::GUID, pcguidinterfaces: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReaderGroupsA(hcontext: usize, mszgroups: super::super::Foundation::PSTR, pcchgroups: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReaderGroupsW(hcontext: usize, mszgroups: super::super::Foundation::PWSTR, pcchgroups: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersA(hcontext: usize, mszgroups: super::super::Foundation::PSTR, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersW(hcontext: usize, mszgroups: super::super::Foundation::PWSTR, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersWithDeviceInstanceIdA(hcontext: usize, szdeviceinstanceid: super::super::Foundation::PSTR, mszreaders: super::super::Foundation::PSTR, pcchreaders: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersWithDeviceInstanceIdW(hcontext: usize, szdeviceinstanceid: super::super::Foundation::PWSTR, mszreaders: super::super::Foundation::PWSTR, pcchreaders: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsA(hcontext: usize, mszcards: super::super::Foundation::PSTR, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsByATRA(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEA, creaders: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsByATRW(hcontext: usize, rgatrmasks: *const SCARD_ATRMASK, catrs: u32, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsW(hcontext: usize, mszcards: super::super::Foundation::PWSTR, rgreaderstates: *mut SCARD_READERSTATEW, creaders: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardReadCacheA(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PSTR, data: *mut u8, datalen: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardReadCacheW(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PWSTR, data: *mut u8, datalen: *mut u32) -> i32;
    pub fn SCardReconnect(hcard: usize, dwsharemode: u32, dwpreferredprotocols: u32, dwinitialization: u32, pdwactiveprotocol: *mut u32) -> i32;
    pub fn SCardReleaseContext(hcontext: usize) -> i32;
    pub fn SCardReleaseStartedEvent();
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardRemoveReaderFromGroupA(hcontext: usize, szreadername: super::super::Foundation::PSTR, szgroupname: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardRemoveReaderFromGroupW(hcontext: usize, szreadername: super::super::Foundation::PWSTR, szgroupname: super::super::Foundation::PWSTR) -> i32;
    pub fn SCardSetAttrib(hcard: usize, dwattrid: u32, pbattr: *const u8, cbattrlen: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardSetCardTypeProviderNameA(hcontext: usize, szcardname: super::super::Foundation::PSTR, dwproviderid: u32, szprovider: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardSetCardTypeProviderNameW(hcontext: usize, szcardname: super::super::Foundation::PWSTR, dwproviderid: u32, szprovider: super::super::Foundation::PWSTR) -> i32;
    pub fn SCardState(hcard: usize, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardStatusA(hcard: usize, mszreadernames: super::super::Foundation::PSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardStatusW(hcard: usize, mszreadernames: super::super::Foundation::PWSTR, pcchreaderlen: *mut u32, pdwstate: *mut u32, pdwprotocol: *mut u32, pbatr: *mut u8, pcbatrlen: *mut u32) -> i32;
    pub fn SCardTransmit(hcard: usize, piosendpci: *const SCARD_IO_REQUEST, pbsendbuffer: *const u8, cbsendlength: u32, piorecvpci: *mut SCARD_IO_REQUEST, pbrecvbuffer: *mut u8, pcbrecvlength: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardA(param0: *mut OPENCARDNAME_EXA) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardW(param0: *mut OPENCARDNAME_EXW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardWriteCacheA(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PSTR, data: *const u8, datalen: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardWriteCacheW(hcontext: usize, cardidentifier: *const ::windows_sys::core::GUID, freshnesscounter: u32, lookupname: super::super::Foundation::PWSTR, data: *const u8, datalen: u32) -> i32;
}
#[repr(C)]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl ::core::marker::Copy for BINARY_BLOB_CREDENTIAL_INFO {}
impl ::core::clone::Clone for BINARY_BLOB_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: u32,
    pub rgbHashOfCert: [u8; 20],
}
impl ::core::marker::Copy for CERT_CREDENTIAL_INFO {}
impl ::core::clone::Clone for CERT_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CERT_HASH_LENGTH: u32 = 20u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALA {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: super::super::Foundation::PSTR,
    pub Comment: super::super::Foundation::PSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: super::super::Foundation::PSTR,
    pub UserName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIALW {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: super::super::Foundation::PWSTR,
    pub UserName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIALW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIALW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_ATTRIBUTEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_ATTRIBUTEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_ATTRIBUTEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_ATTRIBUTEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: super::super::Foundation::PSTR,
    pub NetbiosServerName: super::super::Foundation::PSTR,
    pub DnsServerName: super::super::Foundation::PSTR,
    pub NetbiosDomainName: super::super::Foundation::PSTR,
    pub DnsDomainName: super::super::Foundation::PSTR,
    pub DnsTreeName: super::super::Foundation::PSTR,
    pub PackageName: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_TARGET_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_TARGET_INFORMATIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: super::super::Foundation::PWSTR,
    pub NetbiosServerName: super::super::Foundation::PWSTR,
    pub DnsServerName: super::super::Foundation::PWSTR,
    pub NetbiosDomainName: super::super::Foundation::PWSTR,
    pub DnsDomainName: super::super::Foundation::PWSTR,
    pub DnsTreeName: super::super::Foundation::PWSTR,
    pub PackageName: super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREDENTIAL_TARGET_INFORMATIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREDENTIAL_TARGET_INFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CREDSPP_SUBMIT_TYPE = i32;
pub const CredsspPasswordCreds: CREDSPP_SUBMIT_TYPE = 2i32;
pub const CredsspSchannelCreds: CREDSPP_SUBMIT_TYPE = 4i32;
pub const CredsspCertificateCreds: CREDSPP_SUBMIT_TYPE = 13i32;
pub const CredsspSubmitBufferBoth: CREDSPP_SUBMIT_TYPE = 50i32;
pub const CredsspSubmitBufferBothOld: CREDSPP_SUBMIT_TYPE = 51i32;
pub const CredsspCredEx: CREDSPP_SUBMIT_TYPE = 100i32;
#[repr(C)]
pub struct CREDSSP_CRED {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub pSchannelCred: *mut ::core::ffi::c_void,
    pub pSpnegoCred: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CREDSSP_CRED {}
impl ::core::clone::Clone for CREDSSP_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CREDSSP_CRED_EX {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Cred: CREDSSP_CRED,
}
impl ::core::marker::Copy for CREDSSP_CRED_EX {}
impl ::core::clone::Clone for CREDSSP_CRED_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CREDSSP_CRED_EX_VERSION: u32 = 0u32;
pub const CREDSSP_FLAG_REDIRECT: u32 = 1u32;
pub const CREDSSP_SERVER_AUTH_CERTIFICATE: u32 = 2u32;
pub const CREDSSP_SERVER_AUTH_LOOPBACK: u32 = 4u32;
pub const CREDSSP_SERVER_AUTH_NEGOTIATE: u32 = 1u32;
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648u32;
pub type CREDUIWIN_FLAGS = u32;
pub const CREDUIWIN_GENERIC: CREDUIWIN_FLAGS = 1u32;
pub const CREDUIWIN_CHECKBOX: CREDUIWIN_FLAGS = 2u32;
pub const CREDUIWIN_AUTHPACKAGE_ONLY: CREDUIWIN_FLAGS = 16u32;
pub const CREDUIWIN_IN_CRED_ONLY: CREDUIWIN_FLAGS = 32u32;
pub const CREDUIWIN_ENUMERATE_ADMINS: CREDUIWIN_FLAGS = 256u32;
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: CREDUIWIN_FLAGS = 512u32;
pub const CREDUIWIN_SECURE_PROMPT: CREDUIWIN_FLAGS = 4096u32;
pub const CREDUIWIN_PREPROMPTING: CREDUIWIN_FLAGS = 8192u32;
pub const CREDUIWIN_PACK_32_WOW: CREDUIWIN_FLAGS = 268435456u32;
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144u32;
pub type CREDUI_FLAGS = u32;
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: CREDUI_FLAGS = 128u32;
pub const CREDUI_FLAGS_COMPLETE_USERNAME: CREDUI_FLAGS = 2048u32;
pub const CREDUI_FLAGS_DO_NOT_PERSIST: CREDUI_FLAGS = 2u32;
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: CREDUI_FLAGS = 8u32;
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: CREDUI_FLAGS = 131072u32;
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: CREDUI_FLAGS = 262144u32;
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: CREDUI_FLAGS = 1u32;
pub const CREDUI_FLAGS_KEEP_USERNAME: CREDUI_FLAGS = 1048576u32;
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: CREDUI_FLAGS = 512u32;
pub const CREDUI_FLAGS_PERSIST: CREDUI_FLAGS = 4096u32;
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: CREDUI_FLAGS = 4u32;
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: CREDUI_FLAGS = 16u32;
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: CREDUI_FLAGS = 256u32;
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: CREDUI_FLAGS = 16384u32;
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: CREDUI_FLAGS = 64u32;
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: CREDUI_FLAGS = 524288u32;
pub const CREDUI_FLAGS_VALIDATE_USERNAME: CREDUI_FLAGS = 1024u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: super::super::Foundation::PSTR,
    pub pszCaptionText: super::super::Foundation::PSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CREDUI_INFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CREDUI_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: super::super::Foundation::PWSTR,
    pub pszCaptionText: super::super::Foundation::PWSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CREDUI_INFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CREDUI_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128u32;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767u32;
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024u32;
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1u32;
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1u32;
pub type CRED_ENUMERATE_FLAGS = u32;
pub const CRED_ENUMERATE_ALL_CREDENTIALS: CRED_ENUMERATE_FLAGS = 1u32;
pub type CRED_FLAGS = u32;
pub const CRED_FLAGS_PASSWORD_FOR_CERT: CRED_FLAGS = 1u32;
pub const CRED_FLAGS_PROMPT_NOW: CRED_FLAGS = 2u32;
pub const CRED_FLAGS_USERNAME_TARGET: CRED_FLAGS = 4u32;
pub const CRED_FLAGS_OWF_CRED_BLOB: CRED_FLAGS = 8u32;
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: CRED_FLAGS = 16u32;
pub const CRED_FLAGS_WILDCARD_MATCH: CRED_FLAGS = 32u32;
pub const CRED_FLAGS_VSM_PROTECTED: CRED_FLAGS = 64u32;
pub const CRED_FLAGS_NGC_CERT: CRED_FLAGS = 128u32;
pub const CRED_FLAGS_VALID_FLAGS: CRED_FLAGS = 61695u32;
pub const CRED_FLAGS_VALID_INPUT_FLAGS: CRED_FLAGS = 61599u32;
pub const CRED_LOGON_TYPES_MASK: u32 = 61440u32;
pub type CRED_MARSHAL_TYPE = i32;
pub const CertCredential: CRED_MARSHAL_TYPE = 1i32;
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = 2i32;
pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = 3i32;
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = 4i32;
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = 5i32;
pub const CRED_MAX_ATTRIBUTES: u32 = 64u32;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767u32;
pub const CRED_MAX_STRING_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256u32;
pub const CRED_MAX_VALUE_SIZE: u32 = 256u32;
pub type CRED_PACK_FLAGS = u32;
pub const CRED_PACK_PROTECTED_CREDENTIALS: CRED_PACK_FLAGS = 1u32;
pub const CRED_PACK_WOW_BUFFER: CRED_PACK_FLAGS = 2u32;
pub const CRED_PACK_GENERIC_CREDENTIALS: CRED_PACK_FLAGS = 4u32;
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: CRED_PACK_FLAGS = 8u32;
pub type CRED_PERSIST = u32;
pub const CRED_PERSIST_NONE: CRED_PERSIST = 0u32;
pub const CRED_PERSIST_SESSION: CRED_PERSIST = 1u32;
pub const CRED_PERSIST_LOCAL_MACHINE: CRED_PERSIST = 2u32;
pub const CRED_PERSIST_ENTERPRISE: CRED_PERSIST = 3u32;
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1u32;
pub type CRED_PROTECTION_TYPE = i32;
pub const CredUnprotected: CRED_PROTECTION_TYPE = 0i32;
pub const CredUserProtection: CRED_PROTECTION_TYPE = 1i32;
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = 2i32;
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = 3i32;
pub const CRED_PROTECT_AS_SELF: u32 = 1u32;
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2u32;
pub const CRED_TI_CREATE_EXPLICIT_CRED: u32 = 16u32;
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: u32 = 64u32;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: u32 = 2u32;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: u32 = 4u32;
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: u32 = 1u32;
pub const CRED_TI_USERNAME_TARGET: u32 = 8u32;
pub const CRED_TI_VALID_FLAGS: u32 = 61567u32;
pub const CRED_TI_WORKGROUP_MEMBER: u32 = 32u32;
pub type CRED_TYPE = u32;
pub const CRED_TYPE_GENERIC: CRED_TYPE = 1u32;
pub const CRED_TYPE_DOMAIN_PASSWORD: CRED_TYPE = 2u32;
pub const CRED_TYPE_DOMAIN_CERTIFICATE: CRED_TYPE = 3u32;
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: CRED_TYPE = 4u32;
pub const CRED_TYPE_GENERIC_CERTIFICATE: CRED_TYPE = 5u32;
pub const CRED_TYPE_DOMAIN_EXTENDED: CRED_TYPE = 6u32;
pub const CRED_TYPE_MAXIMUM: CRED_TYPE = 7u32;
pub const CRED_TYPE_MAXIMUM_EX: CRED_TYPE = 1007u32;
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2u32;
pub const CRED_UNPROTECT_AS_SELF: u32 = 1u32;
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
pub const GUID_DEVINTERFACE_SMARTCARD_READER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1356681776, data2: 47754, data3: 4561, data4: [191, 93, 0, 0, 248, 5, 245, 48] };
#[repr(C)]
pub struct KeyCredentialManagerInfo {
    pub containerId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for KeyCredentialManagerInfo {}
impl ::core::clone::Clone for KeyCredentialManagerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KeyCredentialManagerOperationErrorStates = u32;
pub const KeyCredentialManagerOperationErrorStateNone: KeyCredentialManagerOperationErrorStates = 0u32;
pub const KeyCredentialManagerOperationErrorStateDeviceJoinFailure: KeyCredentialManagerOperationErrorStates = 1u32;
pub const KeyCredentialManagerOperationErrorStateTokenFailure: KeyCredentialManagerOperationErrorStates = 2u32;
pub const KeyCredentialManagerOperationErrorStateCertificateFailure: KeyCredentialManagerOperationErrorStates = 4u32;
pub const KeyCredentialManagerOperationErrorStateRemoteSessionFailure: KeyCredentialManagerOperationErrorStates = 8u32;
pub const KeyCredentialManagerOperationErrorStatePolicyFailure: KeyCredentialManagerOperationErrorStates = 16u32;
pub const KeyCredentialManagerOperationErrorStateHardwareFailure: KeyCredentialManagerOperationErrorStates = 32u32;
pub const KeyCredentialManagerOperationErrorStatePinExistsFailure: KeyCredentialManagerOperationErrorStates = 64u32;
pub type KeyCredentialManagerOperationType = i32;
pub const KeyCredentialManagerProvisioning: KeyCredentialManagerOperationType = 0i32;
pub const KeyCredentialManagerPinChange: KeyCredentialManagerOperationType = 1i32;
pub const KeyCredentialManagerPinReset: KeyCredentialManagerOperationType = 2i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCHKPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCONNPROCA = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR, param3: *const ::core::ffi::c_void) -> usize>;
#[cfg(feature = "Win32_Foundation")]
pub type LPOCNCONNPROCW = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: *const ::core::ffi::c_void) -> usize>;
pub type LPOCNDSCPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const ::core::ffi::c_void)>;
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32u32;
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: super::super::Foundation::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: super::super::Foundation::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *mut ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: super::super::Foundation::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: super::super::Foundation::PSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARDNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARDNAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: super::super::Foundation::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: super::super::Foundation::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *mut ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: super::super::Foundation::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARDNAMEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARDNAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: super::super::Foundation::PSTR,
    pub lpstrSearchDesc: super::super::Foundation::PSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: super::super::Foundation::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPENCARDNAME_EXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPENCARDNAME_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: super::super::Foundation::PWSTR,
    pub lpstrSearchDesc: super::super::Foundation::PWSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: super::super::Foundation::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: super::super::Foundation::PWSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OPENCARDNAME_EXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OPENCARDNAME_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: super::super::Foundation::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *mut ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: super::super::Foundation::PSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARD_SEARCH_CRITERIAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARD_SEARCH_CRITERIAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: super::super::Foundation::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *mut ::windows_sys::core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: super::super::Foundation::PWSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENCARD_SEARCH_CRITERIAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENCARD_SEARCH_CRITERIAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
impl ::core::marker::Copy for READER_SEL_REQUEST {}
impl ::core::clone::Clone for READER_SEL_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0_0 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
impl ::core::marker::Copy for READER_SEL_REQUEST_0_1 {}
impl ::core::clone::Clone for READER_SEL_REQUEST_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type READER_SEL_REQUEST_MATCH_TYPE = i32;
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = 1i32;
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = 2i32;
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = 3i32;
#[repr(C)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
impl ::core::marker::Copy for READER_SEL_RESPONSE {}
impl ::core::clone::Clone for READER_SEL_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCARD_ABSENT: u32 = 1u32;
#[repr(C)]
pub struct SCARD_ATRMASK {
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
    pub rgbMask: [u8; 36],
}
impl ::core::marker::Copy for SCARD_ATRMASK {}
impl ::core::clone::Clone for SCARD_ATRMASK {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCARD_ATR_LENGTH: u32 = 33u32;
pub const SCARD_AUDIT_CHV_FAILURE: u32 = 0u32;
pub const SCARD_AUDIT_CHV_SUCCESS: u32 = 1u32;
pub const SCARD_CLASS_COMMUNICATIONS: u32 = 2u32;
pub const SCARD_CLASS_ICC_STATE: u32 = 9u32;
pub const SCARD_CLASS_IFD_PROTOCOL: u32 = 8u32;
pub const SCARD_CLASS_MECHANICAL: u32 = 6u32;
pub const SCARD_CLASS_PERF: u32 = 32766u32;
pub const SCARD_CLASS_POWER_MGMT: u32 = 4u32;
pub const SCARD_CLASS_PROTOCOL: u32 = 3u32;
pub const SCARD_CLASS_SECURITY: u32 = 5u32;
pub const SCARD_CLASS_SYSTEM: u32 = 32767u32;
pub const SCARD_CLASS_VENDOR_DEFINED: u32 = 7u32;
pub const SCARD_CLASS_VENDOR_INFO: u32 = 1u32;
pub const SCARD_COLD_RESET: u32 = 1u32;
pub const SCARD_EJECT_CARD: u32 = 3u32;
#[repr(C)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
impl ::core::marker::Copy for SCARD_IO_REQUEST {}
impl ::core::clone::Clone for SCARD_IO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCARD_LEAVE_CARD: u32 = 0u32;
pub const SCARD_NEGOTIABLE: u32 = 5u32;
pub const SCARD_POWERED: u32 = 4u32;
pub const SCARD_POWER_DOWN: u32 = 0u32;
pub const SCARD_PRESENT: u32 = 2u32;
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648u32;
pub const SCARD_PROTOCOL_OPTIMAL: u32 = 0u32;
pub const SCARD_PROTOCOL_RAW: u32 = 65536u32;
pub const SCARD_PROTOCOL_T0: u32 = 1u32;
pub const SCARD_PROTOCOL_T1: u32 = 2u32;
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0u32;
pub const SCARD_PROVIDER_CSP: u32 = 2u32;
pub const SCARD_PROVIDER_KSP: u32 = 3u32;
pub const SCARD_PROVIDER_PRIMARY: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCARD_READERSTATEA {
    pub szReader: super::super::Foundation::PSTR,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCARD_READERSTATEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCARD_READERSTATEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCARD_READERSTATEW {
    pub szReader: super::super::Foundation::PWSTR,
    pub pvUserData: *mut ::core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCARD_READERSTATEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCARD_READERSTATEW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCARD_READER_CONFISCATES: u32 = 4u32;
pub const SCARD_READER_CONTACTLESS: u32 = 8u32;
pub const SCARD_READER_EJECTS: u32 = 2u32;
pub const SCARD_READER_SWALLOWS: u32 = 1u32;
pub const SCARD_READER_TYPE_EMBEDDEDSE: u32 = 2048u32;
pub const SCARD_READER_TYPE_IDE: u32 = 16u32;
pub const SCARD_READER_TYPE_KEYBOARD: u32 = 4u32;
pub const SCARD_READER_TYPE_NFC: u32 = 256u32;
pub const SCARD_READER_TYPE_NGC: u32 = 1024u32;
pub const SCARD_READER_TYPE_PARALELL: u32 = 2u32;
pub const SCARD_READER_TYPE_PCMCIA: u32 = 64u32;
pub const SCARD_READER_TYPE_SCSI: u32 = 8u32;
pub const SCARD_READER_TYPE_SERIAL: u32 = 1u32;
pub const SCARD_READER_TYPE_TPM: u32 = 128u32;
pub const SCARD_READER_TYPE_UICC: u32 = 512u32;
pub const SCARD_READER_TYPE_USB: u32 = 32u32;
pub const SCARD_READER_TYPE_VENDOR: u32 = 240u32;
pub const SCARD_RESET_CARD: u32 = 1u32;
pub type SCARD_SCOPE = u32;
pub const SCARD_SCOPE_USER: SCARD_SCOPE = 0u32;
pub const SCARD_SCOPE_SYSTEM: SCARD_SCOPE = 2u32;
pub const SCARD_SCOPE_TERMINAL: u32 = 1u32;
pub const SCARD_SHARE_DIRECT: u32 = 3u32;
pub const SCARD_SHARE_EXCLUSIVE: u32 = 1u32;
pub const SCARD_SHARE_SHARED: u32 = 2u32;
pub const SCARD_SPECIFIC: u32 = 6u32;
pub type SCARD_STATE = u32;
pub const SCARD_STATE_UNAWARE: SCARD_STATE = 0u32;
pub const SCARD_STATE_IGNORE: SCARD_STATE = 1u32;
pub const SCARD_STATE_UNAVAILABLE: SCARD_STATE = 8u32;
pub const SCARD_STATE_EMPTY: SCARD_STATE = 16u32;
pub const SCARD_STATE_PRESENT: SCARD_STATE = 32u32;
pub const SCARD_STATE_ATRMATCH: SCARD_STATE = 64u32;
pub const SCARD_STATE_EXCLUSIVE: SCARD_STATE = 128u32;
pub const SCARD_STATE_INUSE: SCARD_STATE = 256u32;
pub const SCARD_STATE_MUTE: SCARD_STATE = 512u32;
pub const SCARD_STATE_CHANGED: SCARD_STATE = 2u32;
pub const SCARD_STATE_UNKNOWN: SCARD_STATE = 4u32;
pub const SCARD_STATE_UNPOWERED: u32 = 1024u32;
pub const SCARD_SWALLOWED: u32 = 3u32;
pub const SCARD_T0_CMD_LENGTH: u32 = 5u32;
#[repr(C)]
pub struct SCARD_T0_COMMAND {
    pub bCla: u8,
    pub bIns: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bP3: u8,
}
impl ::core::marker::Copy for SCARD_T0_COMMAND {}
impl ::core::clone::Clone for SCARD_T0_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCARD_T0_HEADER_LENGTH: u32 = 7u32;
#[repr(C)]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: u8,
    pub bSw2: u8,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
impl ::core::marker::Copy for SCARD_T0_REQUEST {}
impl ::core::clone::Clone for SCARD_T0_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [u8; 5],
}
impl ::core::marker::Copy for SCARD_T0_REQUEST_0 {}
impl ::core::clone::Clone for SCARD_T0_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2u32;
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1u32;
pub const SCARD_T1_MAX_IFS: u32 = 254u32;
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3u32;
#[repr(C)]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
impl ::core::marker::Copy for SCARD_T1_REQUEST {}
impl ::core::clone::Clone for SCARD_T1_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SCARD_UNKNOWN: u32 = 0u32;
pub const SCARD_UNPOWER_CARD: u32 = 2u32;
pub const SCARD_WARM_RESET: u32 = 2u32;
pub const SCERR_NOCARDNAME: u32 = 16384u32;
pub const SCERR_NOGUIDS: u32 = 32768u32;
pub const SC_DLG_FORCE_UI: u32 = 4u32;
pub const SC_DLG_MINIMAL_UI: u32 = 1u32;
pub const SC_DLG_NO_UI: u32 = 2u32;
pub const SECPKG_ALT_ATTR: u32 = 2147483648u32;
pub const SECPKG_ATTR_C_FULL_IDENT_TOKEN: u32 = 2147483781u32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCESS_DENIED: super::super::Foundation::NTSTATUS = -1073741790i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_DISABLED: super::super::Foundation::NTSTATUS = -1073741710i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_EXPIRED: super::super::Foundation::NTSTATUS = -1073741421i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_LOCKED_OUT: super::super::Foundation::NTSTATUS = -1073741260i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_ACCOUNT_RESTRICTION: super::super::Foundation::NTSTATUS = -1073741714i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::super::Foundation::NTSTATUS = -1073740781i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_DOWNGRADE_DETECTED: super::super::Foundation::NTSTATUS = -1073740920i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_FAILURE: super::super::Foundation::NTSTATUS = -1073741715i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::super::Foundation::NTSTATUS = -1073741477i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_LOGON_SESSION: super::super::Foundation::NTSTATUS = -1073741729i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_NO_SUCH_USER: super::super::Foundation::NTSTATUS = -1073741724i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_EXPIRED: super::super::Foundation::NTSTATUS = -1073741711i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_PASSWORD_MUST_CHANGE: super::super::Foundation::NTSTATUS = -1073741276i32;
#[cfg(feature = "Win32_Foundation")]
pub const STATUS_WRONG_PASSWORD: super::super::Foundation::NTSTATUS = -1073741718i32;
#[repr(C)]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
impl ::core::marker::Copy for SecHandle {}
impl ::core::clone::Clone for SecHandle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SecPkgContext_ClientCreds {
    pub AuthBufferLen: u32,
    pub AuthBuffer: *mut u8,
}
impl ::core::marker::Copy for SecPkgContext_ClientCreds {}
impl ::core::clone::Clone for SecPkgContext_ClientCreds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USERNAME_TARGET_CREDENTIAL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USERNAME_TARGET_CREDENTIAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
