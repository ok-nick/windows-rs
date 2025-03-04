#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerBeginSession(dwflags: u32, eaptype: EAP_METHOD_TYPE, pattributearray: *const EAP_ATTRIBUTES, htokenimpersonateuser: super::super::Foundation::HANDLE, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, dwmaxsendpacketsize: u32, pconnectionid: *const ::windows_sys::core::GUID, func: NotificationHandler, pcontextdata: *mut ::core::ffi::c_void, psessionid: *mut u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerClearConnection(pconnectionid: *mut ::windows_sys::core::GUID, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigBlob2Xml(dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconfigin: u32, pconfigin: *const u8, ppconfigdoc: *mut super::super::Data::Xml::MsXml::IXMLDOMDocument2, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigXml2Blob(dwflags: u32, pconfigdoc: super::super::Data::Xml::MsXml::IXMLDOMNode, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerCredentialsXml2Blob(dwflags: u32, pcredentialsdoc: super::super::Data::Xml::MsXml::IXMLDOMNode, dwsizeofconfigin: u32, pconfigin: *const u8, pdwsizeofcredentialsout: *mut u32, ppcredentialsout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerEndSession(sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerFreeEapError(peaperror: *mut EAP_ERROR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerFreeErrorMemory(peaperror: *mut EAP_ERROR);
    pub fn EapHostPeerFreeMemory(pdata: *mut u8);
    pub fn EapHostPeerFreeRuntimeMemory(pdata: *mut u8);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetAuthStatus(sessionhandle: u32, authparam: EapHostPeerAuthParams, pcbauthdata: *mut u32, ppauthdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetDataToUnplumbCredentials(pconnectionidthatlastsavedcreds: *mut ::windows_sys::core::GUID, phcredentialimpersonationtoken: *mut isize, sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR, fsavetocredman: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetEncryptedPassword(dwsizeofpassword: u32, szpassword: super::super::Foundation::PWSTR, ppszencpassword: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetIdentity(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, htokenimpersonateuser: super::super::Foundation::HANDLE, pfinvokeui: *mut super::super::Foundation::BOOL, pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut super::super::Foundation::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetMethodProperties(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, huserimpersonationtoken: super::super::Foundation::HANDLE, dweapconndatasize: u32, pbeapconndata: *const u8, dwuserdatasize: u32, pbuserdata: *const u8, pmethodpropertyarray: *mut EAP_METHOD_PROPERTY_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetMethods(peapmethodinfoarray: *mut EAP_METHOD_INFO_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetResponseAttributes(sessionhandle: u32, pattribs: *mut EAP_ATTRIBUTES, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetResult(sessionhandle: u32, reason: EapHostPeerMethodResultReason, ppresult: *mut EapHostPeerMethodResult, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetSendPacket(sessionhandle: u32, pcbsendpacket: *mut u32, ppsendpacket: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetUIContext(sessionhandle: u32, pdwsizeofuicontextdata: *mut u32, ppuicontextdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    pub fn EapHostPeerInitialize() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeConfigUI(hwndparent: super::super::Foundation::HWND, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconfigin: u32, pconfigin: *const u8, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeIdentityUI(dwversion: u32, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, hwndparent: super::super::Foundation::HWND, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut super::super::Foundation::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeInteractiveUI(hwndparent: super::super::Foundation::HWND, dwsizeofuicontextdata: u32, puicontextdata: *const u8, pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerProcessReceivedPacket(sessionhandle: u32, cbreceivepacket: u32, preceivepacket: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryCredentialInputFields(huserimpersonationtoken: super::super::Foundation::HANDLE, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, dweapconndatasize: u32, pbeapconndata: *const u8, peapconfiginputfieldarray: *mut EAP_CONFIG_INPUT_FIELD_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryInteractiveUIInputFields(dwversion: u32, dwflags: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapinteractiveuidata: *mut EAP_INTERACTIVE_UI_DATA, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields(dwversion: u32, dwflags: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapinteractiveuidata: *const EAP_INTERACTIVE_UI_DATA, pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryUserBlobFromCredentialInputFields(huserimpersonationtoken: super::super::Foundation::HANDLE, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, dweapconndatasize: u32, pbeapconndata: *const u8, peapconfiginputfieldarray: *const EAP_CONFIG_INPUT_FIELD_ARRAY, pdwuserblobsize: *mut u32, ppbuserblob: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerSetResponseAttributes(sessionhandle: u32, pattribs: *const EAP_ATTRIBUTES, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerSetUIContext(sessionhandle: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    pub fn EapHostPeerUninitialize();
}
pub const CERTIFICATE_HASH_LENGTH: u32 = 20u32;
pub const EAPCODE_Failure: u32 = 4u32;
pub const EAPCODE_Request: u32 = 1u32;
pub const EAPCODE_Response: u32 = 2u32;
pub const EAPCODE_Success: u32 = 3u32;
#[repr(C)]
pub struct EAPHOST_AUTH_INFO {
    pub status: EAPHOST_AUTH_STATUS,
    pub dwErrorCode: u32,
    pub dwReasonCode: u32,
}
impl ::core::marker::Copy for EAPHOST_AUTH_INFO {}
impl ::core::clone::Clone for EAPHOST_AUTH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EAPHOST_AUTH_STATUS = i32;
pub const EapHostInvalidSession: EAPHOST_AUTH_STATUS = 0i32;
pub const EapHostAuthNotStarted: EAPHOST_AUTH_STATUS = 1i32;
pub const EapHostAuthIdentityExchange: EAPHOST_AUTH_STATUS = 2i32;
pub const EapHostAuthNegotiatingType: EAPHOST_AUTH_STATUS = 3i32;
pub const EapHostAuthInProgress: EAPHOST_AUTH_STATUS = 4i32;
pub const EapHostAuthSucceeded: EAPHOST_AUTH_STATUS = 5i32;
pub const EapHostAuthFailed: EAPHOST_AUTH_STATUS = 6i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAPHOST_IDENTITY_UI_PARAMS {
    pub eapMethodType: EAP_METHOD_TYPE,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: super::super::Foundation::PWSTR,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAPHOST_IDENTITY_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAPHOST_IDENTITY_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAPHOST_INTERACTIVE_UI_PARAMS {
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAPHOST_INTERACTIVE_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EAPHOST_METHOD_API_VERSION: u32 = 1u32;
pub const EAPHOST_PEER_API_VERSION: u32 = 1u32;
#[repr(C)]
pub struct EAP_ATTRIBUTE {
    pub eaType: EAP_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub pValue: *mut u8,
}
impl ::core::marker::Copy for EAP_ATTRIBUTE {}
impl ::core::clone::Clone for EAP_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EAP_ATTRIBUTES {
    pub dwNumberOfAttributes: u32,
    pub pAttribs: *mut EAP_ATTRIBUTE,
}
impl ::core::marker::Copy for EAP_ATTRIBUTES {}
impl ::core::clone::Clone for EAP_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EAP_ATTRIBUTE_TYPE = i32;
pub const eatMinimum: EAP_ATTRIBUTE_TYPE = 0i32;
pub const eatUserName: EAP_ATTRIBUTE_TYPE = 1i32;
pub const eatUserPassword: EAP_ATTRIBUTE_TYPE = 2i32;
pub const eatMD5CHAPPassword: EAP_ATTRIBUTE_TYPE = 3i32;
pub const eatNASIPAddress: EAP_ATTRIBUTE_TYPE = 4i32;
pub const eatNASPort: EAP_ATTRIBUTE_TYPE = 5i32;
pub const eatServiceType: EAP_ATTRIBUTE_TYPE = 6i32;
pub const eatFramedProtocol: EAP_ATTRIBUTE_TYPE = 7i32;
pub const eatFramedIPAddress: EAP_ATTRIBUTE_TYPE = 8i32;
pub const eatFramedIPNetmask: EAP_ATTRIBUTE_TYPE = 9i32;
pub const eatFramedRouting: EAP_ATTRIBUTE_TYPE = 10i32;
pub const eatFilterId: EAP_ATTRIBUTE_TYPE = 11i32;
pub const eatFramedMTU: EAP_ATTRIBUTE_TYPE = 12i32;
pub const eatFramedCompression: EAP_ATTRIBUTE_TYPE = 13i32;
pub const eatLoginIPHost: EAP_ATTRIBUTE_TYPE = 14i32;
pub const eatLoginService: EAP_ATTRIBUTE_TYPE = 15i32;
pub const eatLoginTCPPort: EAP_ATTRIBUTE_TYPE = 16i32;
pub const eatUnassigned17: EAP_ATTRIBUTE_TYPE = 17i32;
pub const eatReplyMessage: EAP_ATTRIBUTE_TYPE = 18i32;
pub const eatCallbackNumber: EAP_ATTRIBUTE_TYPE = 19i32;
pub const eatCallbackId: EAP_ATTRIBUTE_TYPE = 20i32;
pub const eatUnassigned21: EAP_ATTRIBUTE_TYPE = 21i32;
pub const eatFramedRoute: EAP_ATTRIBUTE_TYPE = 22i32;
pub const eatFramedIPXNetwork: EAP_ATTRIBUTE_TYPE = 23i32;
pub const eatState: EAP_ATTRIBUTE_TYPE = 24i32;
pub const eatClass: EAP_ATTRIBUTE_TYPE = 25i32;
pub const eatVendorSpecific: EAP_ATTRIBUTE_TYPE = 26i32;
pub const eatSessionTimeout: EAP_ATTRIBUTE_TYPE = 27i32;
pub const eatIdleTimeout: EAP_ATTRIBUTE_TYPE = 28i32;
pub const eatTerminationAction: EAP_ATTRIBUTE_TYPE = 29i32;
pub const eatCalledStationId: EAP_ATTRIBUTE_TYPE = 30i32;
pub const eatCallingStationId: EAP_ATTRIBUTE_TYPE = 31i32;
pub const eatNASIdentifier: EAP_ATTRIBUTE_TYPE = 32i32;
pub const eatProxyState: EAP_ATTRIBUTE_TYPE = 33i32;
pub const eatLoginLATService: EAP_ATTRIBUTE_TYPE = 34i32;
pub const eatLoginLATNode: EAP_ATTRIBUTE_TYPE = 35i32;
pub const eatLoginLATGroup: EAP_ATTRIBUTE_TYPE = 36i32;
pub const eatFramedAppleTalkLink: EAP_ATTRIBUTE_TYPE = 37i32;
pub const eatFramedAppleTalkNetwork: EAP_ATTRIBUTE_TYPE = 38i32;
pub const eatFramedAppleTalkZone: EAP_ATTRIBUTE_TYPE = 39i32;
pub const eatAcctStatusType: EAP_ATTRIBUTE_TYPE = 40i32;
pub const eatAcctDelayTime: EAP_ATTRIBUTE_TYPE = 41i32;
pub const eatAcctInputOctets: EAP_ATTRIBUTE_TYPE = 42i32;
pub const eatAcctOutputOctets: EAP_ATTRIBUTE_TYPE = 43i32;
pub const eatAcctSessionId: EAP_ATTRIBUTE_TYPE = 44i32;
pub const eatAcctAuthentic: EAP_ATTRIBUTE_TYPE = 45i32;
pub const eatAcctSessionTime: EAP_ATTRIBUTE_TYPE = 46i32;
pub const eatAcctInputPackets: EAP_ATTRIBUTE_TYPE = 47i32;
pub const eatAcctOutputPackets: EAP_ATTRIBUTE_TYPE = 48i32;
pub const eatAcctTerminateCause: EAP_ATTRIBUTE_TYPE = 49i32;
pub const eatAcctMultiSessionId: EAP_ATTRIBUTE_TYPE = 50i32;
pub const eatAcctLinkCount: EAP_ATTRIBUTE_TYPE = 51i32;
pub const eatAcctEventTimeStamp: EAP_ATTRIBUTE_TYPE = 55i32;
pub const eatMD5CHAPChallenge: EAP_ATTRIBUTE_TYPE = 60i32;
pub const eatNASPortType: EAP_ATTRIBUTE_TYPE = 61i32;
pub const eatPortLimit: EAP_ATTRIBUTE_TYPE = 62i32;
pub const eatLoginLATPort: EAP_ATTRIBUTE_TYPE = 63i32;
pub const eatTunnelType: EAP_ATTRIBUTE_TYPE = 64i32;
pub const eatTunnelMediumType: EAP_ATTRIBUTE_TYPE = 65i32;
pub const eatTunnelClientEndpoint: EAP_ATTRIBUTE_TYPE = 66i32;
pub const eatTunnelServerEndpoint: EAP_ATTRIBUTE_TYPE = 67i32;
pub const eatARAPPassword: EAP_ATTRIBUTE_TYPE = 70i32;
pub const eatARAPFeatures: EAP_ATTRIBUTE_TYPE = 71i32;
pub const eatARAPZoneAccess: EAP_ATTRIBUTE_TYPE = 72i32;
pub const eatARAPSecurity: EAP_ATTRIBUTE_TYPE = 73i32;
pub const eatARAPSecurityData: EAP_ATTRIBUTE_TYPE = 74i32;
pub const eatPasswordRetry: EAP_ATTRIBUTE_TYPE = 75i32;
pub const eatPrompt: EAP_ATTRIBUTE_TYPE = 76i32;
pub const eatConnectInfo: EAP_ATTRIBUTE_TYPE = 77i32;
pub const eatConfigurationToken: EAP_ATTRIBUTE_TYPE = 78i32;
pub const eatEAPMessage: EAP_ATTRIBUTE_TYPE = 79i32;
pub const eatSignature: EAP_ATTRIBUTE_TYPE = 80i32;
pub const eatARAPChallengeResponse: EAP_ATTRIBUTE_TYPE = 84i32;
pub const eatAcctInterimInterval: EAP_ATTRIBUTE_TYPE = 85i32;
pub const eatNASIPv6Address: EAP_ATTRIBUTE_TYPE = 95i32;
pub const eatFramedInterfaceId: EAP_ATTRIBUTE_TYPE = 96i32;
pub const eatFramedIPv6Prefix: EAP_ATTRIBUTE_TYPE = 97i32;
pub const eatLoginIPv6Host: EAP_ATTRIBUTE_TYPE = 98i32;
pub const eatFramedIPv6Route: EAP_ATTRIBUTE_TYPE = 99i32;
pub const eatFramedIPv6Pool: EAP_ATTRIBUTE_TYPE = 100i32;
pub const eatARAPGuestLogon: EAP_ATTRIBUTE_TYPE = 8096i32;
pub const eatCertificateOID: EAP_ATTRIBUTE_TYPE = 8097i32;
pub const eatEAPConfiguration: EAP_ATTRIBUTE_TYPE = 8098i32;
pub const eatPEAPEmbeddedEAPTypeId: EAP_ATTRIBUTE_TYPE = 8099i32;
pub const eatPEAPFastRoamedSession: EAP_ATTRIBUTE_TYPE = 8100i32;
pub const eatFastRoamedSession: EAP_ATTRIBUTE_TYPE = 8100i32;
pub const eatEAPTLV: EAP_ATTRIBUTE_TYPE = 8102i32;
pub const eatCredentialsChanged: EAP_ATTRIBUTE_TYPE = 8103i32;
pub const eatInnerEapMethodType: EAP_ATTRIBUTE_TYPE = 8104i32;
pub const eatClearTextPassword: EAP_ATTRIBUTE_TYPE = 8107i32;
pub const eatQuarantineSoH: EAP_ATTRIBUTE_TYPE = 8150i32;
pub const eatCertificateThumbprint: EAP_ATTRIBUTE_TYPE = 8250i32;
pub const eatPeerId: EAP_ATTRIBUTE_TYPE = 9000i32;
pub const eatServerId: EAP_ATTRIBUTE_TYPE = 9001i32;
pub const eatMethodId: EAP_ATTRIBUTE_TYPE = 9002i32;
pub const eatEMSK: EAP_ATTRIBUTE_TYPE = 9003i32;
pub const eatSessionId: EAP_ATTRIBUTE_TYPE = 9004i32;
pub const eatReserved: EAP_ATTRIBUTE_TYPE = -1i32;
#[repr(C)]
pub struct EAP_AUTHENTICATOR_METHOD_ROUTINES {
    pub dwSizeInBytes: u32,
    pub pEapType: *mut EAP_METHOD_TYPE,
    pub EapMethodAuthenticatorInitialize: isize,
    pub EapMethodAuthenticatorBeginSession: isize,
    pub EapMethodAuthenticatorUpdateInnerMethodParams: isize,
    pub EapMethodAuthenticatorReceivePacket: isize,
    pub EapMethodAuthenticatorSendPacket: isize,
    pub EapMethodAuthenticatorGetAttributes: isize,
    pub EapMethodAuthenticatorSetAttributes: isize,
    pub EapMethodAuthenticatorGetResult: isize,
    pub EapMethodAuthenticatorEndSession: isize,
    pub EapMethodAuthenticatorShutdown: isize,
}
impl ::core::marker::Copy for EAP_AUTHENTICATOR_METHOD_ROUTINES {}
impl ::core::clone::Clone for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EAP_AUTHENTICATOR_SEND_TIMEOUT = i32;
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_NONE: EAP_AUTHENTICATOR_SEND_TIMEOUT = 0i32;
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_BASIC: EAP_AUTHENTICATOR_SEND_TIMEOUT = 1i32;
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_INTERACTIVE: EAP_AUTHENTICATOR_SEND_TIMEOUT = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CONFIG_INPUT_FIELD_ARRAY {
    pub dwVersion: u32,
    pub dwNumberOfFields: u32,
    pub pFields: *mut EAP_CONFIG_INPUT_FIELD_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_CONFIG_INPUT_FIELD_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CONFIG_INPUT_FIELD_DATA {
    pub dwSize: u32,
    pub Type: EAP_CONFIG_INPUT_FIELD_TYPE,
    pub dwFlagProps: u32,
    pub pwszLabel: super::super::Foundation::PWSTR,
    pub pwszData: super::super::Foundation::PWSTR,
    pub dwMinDataLength: u32,
    pub dwMaxDataLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_CONFIG_INPUT_FIELD_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_CONFIG_INPUT_FIELD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
pub type EAP_CONFIG_INPUT_FIELD_TYPE = i32;
pub const EapConfigInputUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 0i32;
pub const EapConfigInputPassword: EAP_CONFIG_INPUT_FIELD_TYPE = 1i32;
pub const EapConfigInputNetworkUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 2i32;
pub const EapConfigInputNetworkPassword: EAP_CONFIG_INPUT_FIELD_TYPE = 3i32;
pub const EapConfigInputPin: EAP_CONFIG_INPUT_FIELD_TYPE = 4i32;
pub const EapConfigInputPSK: EAP_CONFIG_INPUT_FIELD_TYPE = 5i32;
pub const EapConfigInputEdit: EAP_CONFIG_INPUT_FIELD_TYPE = 6i32;
pub const EapConfigSmartCardUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 7i32;
pub const EapConfigSmartCardError: EAP_CONFIG_INPUT_FIELD_TYPE = 8i32;
pub const EAP_CREDENTIAL_VERSION: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CRED_EXPIRY_REQ {
    pub curCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub newCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_CRED_EXPIRY_REQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_CRED_EXPIRY_REQ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_ERROR {
    pub dwWinError: u32,
    pub r#type: EAP_METHOD_TYPE,
    pub dwReasonCode: u32,
    pub rootCauseGuid: ::windows_sys::core::GUID,
    pub repairGuid: ::windows_sys::core::GUID,
    pub helpLinkGuid: ::windows_sys::core::GUID,
    pub pRootCauseString: super::super::Foundation::PWSTR,
    pub pRepairString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_ERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EAP_E_AUTHENTICATION_FAILED: u32 = 2151809045u32;
pub const EAP_E_CERT_STORE_INACCESSIBLE: u32 = 2151809040u32;
pub const EAP_E_EAPHOST_EAPQEC_INACCESSIBLE: u32 = 2151809043u32;
pub const EAP_E_EAPHOST_FIRST: i32 = -2143158272i32;
pub const EAP_E_EAPHOST_IDENTITY_UNKNOWN: u32 = 2151809044u32;
pub const EAP_E_EAPHOST_LAST: i32 = -2143158017i32;
pub const EAP_E_EAPHOST_METHOD_INVALID_PACKET: u32 = 2151809047u32;
pub const EAP_E_EAPHOST_METHOD_NOT_INSTALLED: u32 = 2151809041u32;
pub const EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED: u32 = 2151809056u32;
pub const EAP_E_EAPHOST_REMOTE_INVALID_PACKET: u32 = 2151809048u32;
pub const EAP_E_EAPHOST_THIRDPARTY_METHOD_HOST_RESET: u32 = 2151809042u32;
pub const EAP_E_EAPHOST_XML_MALFORMED: u32 = 2151809049u32;
pub const EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO: u32 = 2151809050u32;
pub const EAP_E_NO_SMART_CARD_READER: u32 = 2151809299u32;
pub const EAP_E_SERVER_CERT_EXPIRED: u32 = 2151809538u32;
pub const EAP_E_SERVER_CERT_INVALID: u32 = 2151809537u32;
pub const EAP_E_SERVER_CERT_NOT_FOUND: u32 = 2151809536u32;
pub const EAP_E_SERVER_CERT_OTHER_ERROR: u32 = 2151809540u32;
pub const EAP_E_SERVER_CERT_REVOKED: u32 = 2151809539u32;
pub const EAP_E_SERVER_FIRST: i32 = -2143157760i32;
pub const EAP_E_SERVER_LAST: i32 = -2143157505i32;
pub const EAP_E_SERVER_ROOT_CERT_FIRST: i32 = -2143157248i32;
pub const EAP_E_SERVER_ROOT_CERT_INVALID: u32 = 2151810049u32;
pub const EAP_E_SERVER_ROOT_CERT_LAST: i32 = -2143156993i32;
pub const EAP_E_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 2151810054u32;
pub const EAP_E_SERVER_ROOT_CERT_NOT_FOUND: u32 = 2151810048u32;
pub const EAP_E_SIM_NOT_VALID: u32 = 2151810304u32;
pub const EAP_E_USER_CERT_EXPIRED: u32 = 2151809282u32;
pub const EAP_E_USER_CERT_INVALID: u32 = 2151809281u32;
pub const EAP_E_USER_CERT_NOT_FOUND: u32 = 2151809280u32;
pub const EAP_E_USER_CERT_OTHER_ERROR: u32 = 2151809284u32;
pub const EAP_E_USER_CERT_REJECTED: u32 = 2151809285u32;
pub const EAP_E_USER_CERT_REVOKED: u32 = 2151809283u32;
pub const EAP_E_USER_CREDENTIALS_REJECTED: u32 = 2151809297u32;
pub const EAP_E_USER_FIRST: i32 = -2143158016i32;
pub const EAP_E_USER_LAST: i32 = -2143157761i32;
pub const EAP_E_USER_NAME_PASSWORD_REJECTED: u32 = 2151809298u32;
pub const EAP_E_USER_ROOT_CERT_EXPIRED: u32 = 2151809794u32;
pub const EAP_E_USER_ROOT_CERT_FIRST: i32 = -2143157504i32;
pub const EAP_E_USER_ROOT_CERT_INVALID: u32 = 2151809793u32;
pub const EAP_E_USER_ROOT_CERT_LAST: i32 = -2143157249i32;
pub const EAP_E_USER_ROOT_CERT_NOT_FOUND: u32 = 2151809792u32;
pub const EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
pub const EAP_FLAG_FULL_AUTH: u32 = 4096u32;
pub const EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
pub const EAP_FLAG_LOGON: u32 = 4u32;
pub const EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
pub const EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
pub const EAP_FLAG_ONLY_EAP_TLS: u32 = 16777216u32;
pub const EAP_FLAG_PREFER_ALT_CREDENTIALS: u32 = 8192u32;
pub const EAP_FLAG_PREVIEW: u32 = 8u32;
pub const EAP_FLAG_PRE_LOGON: u32 = 131072u32;
pub const EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
pub const EAP_FLAG_Reserved1: u32 = 1u32;
pub const EAP_FLAG_Reserved2: u32 = 16u32;
pub const EAP_FLAG_Reserved3: u32 = 128u32;
pub const EAP_FLAG_Reserved4: u32 = 256u32;
pub const EAP_FLAG_Reserved5: u32 = 1024u32;
pub const EAP_FLAG_Reserved6: u32 = 2048u32;
pub const EAP_FLAG_Reserved7: u32 = 16384u32;
pub const EAP_FLAG_Reserved8: u32 = 1048576u32;
pub const EAP_FLAG_Reserved9: u32 = 4194304u32;
pub const EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
pub const EAP_FLAG_SUPRESS_UI: u32 = 65536u32;
pub const EAP_FLAG_USER_AUTH: u32 = 262144u32;
pub const EAP_FLAG_VPN: u32 = 8388608u32;
pub const EAP_GROUP_MASK: i32 = 65280i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_INTERACTIVE_UI_DATA {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwDataType: EAP_INTERACTIVE_UI_DATA_TYPE,
    pub cbUiData: u32,
    pub pbUiData: EAP_UI_DATA_FORMAT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_INTERACTIVE_UI_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_INTERACTIVE_UI_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EAP_INTERACTIVE_UI_DATA_TYPE = i32;
pub const EapCredReq: EAP_INTERACTIVE_UI_DATA_TYPE = 0i32;
pub const EapCredResp: EAP_INTERACTIVE_UI_DATA_TYPE = 1i32;
pub const EapCredExpiryReq: EAP_INTERACTIVE_UI_DATA_TYPE = 2i32;
pub const EapCredExpiryResp: EAP_INTERACTIVE_UI_DATA_TYPE = 3i32;
pub const EapCredLogonReq: EAP_INTERACTIVE_UI_DATA_TYPE = 4i32;
pub const EapCredLogonResp: EAP_INTERACTIVE_UI_DATA_TYPE = 5i32;
pub const EAP_INTERACTIVE_UI_DATA_VERSION: u32 = 1u32;
pub const EAP_INVALID_PACKET: u32 = 2151809048u32;
pub const EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED: u32 = 1078067222u32;
pub const EAP_I_EAPHOST_FIRST: i32 = -2143158272i32;
pub const EAP_I_EAPHOST_LAST: i32 = -2143158017i32;
pub const EAP_I_USER_ACCOUNT_OTHER_ERROR: u32 = 1078067472u32;
pub const EAP_I_USER_FIRST: i32 = 1078067456i32;
pub const EAP_I_USER_LAST: i32 = 1078067711i32;
pub const EAP_METHOD_AUTHENTICATOR_CONFIG_IS_IDENTITY_PRIVACY: u32 = 1u32;
pub type EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = i32;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_DISCARD: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 0i32;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_SEND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 1i32;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESULT: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 2i32;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESPOND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 3i32;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_AUTHENTICATE: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 4i32;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_HANDLE_IDENTITY: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_AUTHENTICATOR_RESULT {
    pub fIsSuccess: super::super::Foundation::BOOL,
    pub dwFailureReason: u32,
    pub pAuthAttribs: *mut EAP_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_AUTHENTICATOR_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: super::super::Foundation::PWSTR,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfo: *mut EAP_METHOD_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_ARRAY {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_ARRAY_EX {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_INFO_ARRAY_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_INFO_ARRAY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_EX {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: super::super::Foundation::PWSTR,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfoArray: *mut EAP_METHOD_INFO_ARRAY_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EAP_METHOD_INVALID_PACKET: u32 = 2151809047u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY {
    pub eapMethodPropertyType: EAP_METHOD_PROPERTY_TYPE,
    pub eapMethodPropertyValueType: EAP_METHOD_PROPERTY_VALUE_TYPE,
    pub eapMethodPropertyValue: EAP_METHOD_PROPERTY_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_ARRAY {
    pub dwNumberOfProperties: u32,
    pub pMethodProperty: *mut EAP_METHOD_PROPERTY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EAP_METHOD_PROPERTY_TYPE = i32;
pub const emptPropCipherSuiteNegotiation: EAP_METHOD_PROPERTY_TYPE = 0i32;
pub const emptPropMutualAuth: EAP_METHOD_PROPERTY_TYPE = 1i32;
pub const emptPropIntegrity: EAP_METHOD_PROPERTY_TYPE = 2i32;
pub const emptPropReplayProtection: EAP_METHOD_PROPERTY_TYPE = 3i32;
pub const emptPropConfidentiality: EAP_METHOD_PROPERTY_TYPE = 4i32;
pub const emptPropKeyDerivation: EAP_METHOD_PROPERTY_TYPE = 5i32;
pub const emptPropKeyStrength64: EAP_METHOD_PROPERTY_TYPE = 6i32;
pub const emptPropKeyStrength128: EAP_METHOD_PROPERTY_TYPE = 7i32;
pub const emptPropKeyStrength256: EAP_METHOD_PROPERTY_TYPE = 8i32;
pub const emptPropKeyStrength512: EAP_METHOD_PROPERTY_TYPE = 9i32;
pub const emptPropKeyStrength1024: EAP_METHOD_PROPERTY_TYPE = 10i32;
pub const emptPropDictionaryAttackResistance: EAP_METHOD_PROPERTY_TYPE = 11i32;
pub const emptPropFastReconnect: EAP_METHOD_PROPERTY_TYPE = 12i32;
pub const emptPropCryptoBinding: EAP_METHOD_PROPERTY_TYPE = 13i32;
pub const emptPropSessionIndependence: EAP_METHOD_PROPERTY_TYPE = 14i32;
pub const emptPropFragmentation: EAP_METHOD_PROPERTY_TYPE = 15i32;
pub const emptPropChannelBinding: EAP_METHOD_PROPERTY_TYPE = 16i32;
pub const emptPropNap: EAP_METHOD_PROPERTY_TYPE = 17i32;
pub const emptPropStandalone: EAP_METHOD_PROPERTY_TYPE = 18i32;
pub const emptPropMppeEncryption: EAP_METHOD_PROPERTY_TYPE = 19i32;
pub const emptPropTunnelMethod: EAP_METHOD_PROPERTY_TYPE = 20i32;
pub const emptPropSupportsConfig: EAP_METHOD_PROPERTY_TYPE = 21i32;
pub const emptPropCertifiedMethod: EAP_METHOD_PROPERTY_TYPE = 22i32;
pub const emptPropHiddenMethod: EAP_METHOD_PROPERTY_TYPE = 23i32;
pub const emptPropMachineAuth: EAP_METHOD_PROPERTY_TYPE = 24i32;
pub const emptPropUserAuth: EAP_METHOD_PROPERTY_TYPE = 25i32;
pub const emptPropIdentityPrivacy: EAP_METHOD_PROPERTY_TYPE = 26i32;
pub const emptPropMethodChaining: EAP_METHOD_PROPERTY_TYPE = 27i32;
pub const emptPropSharedStateEquivalence: EAP_METHOD_PROPERTY_TYPE = 28i32;
pub const emptLegacyMethodPropertyFlag: EAP_METHOD_PROPERTY_TYPE = 31i32;
pub const emptPropVendorSpecific: EAP_METHOD_PROPERTY_TYPE = 255i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EAP_METHOD_PROPERTY_VALUE {
    pub empvBool: EAP_METHOD_PROPERTY_VALUE_BOOL,
    pub empvDword: EAP_METHOD_PROPERTY_VALUE_DWORD,
    pub empvString: EAP_METHOD_PROPERTY_VALUE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_VALUE_BOOL {
    pub length: u32,
    pub value: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE_BOOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EAP_METHOD_PROPERTY_VALUE_DWORD {
    pub length: u32,
    pub value: u32,
}
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE_DWORD {}
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EAP_METHOD_PROPERTY_VALUE_STRING {
    pub length: u32,
    pub value: *mut u8,
}
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE_STRING {}
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EAP_METHOD_PROPERTY_VALUE_TYPE = i32;
pub const empvtBool: EAP_METHOD_PROPERTY_VALUE_TYPE = 0i32;
pub const empvtDword: EAP_METHOD_PROPERTY_VALUE_TYPE = 1i32;
pub const empvtString: EAP_METHOD_PROPERTY_VALUE_TYPE = 2i32;
#[repr(C)]
pub struct EAP_METHOD_TYPE {
    pub eapType: EAP_TYPE,
    pub dwAuthorId: u32,
}
impl ::core::marker::Copy for EAP_METHOD_TYPE {}
impl ::core::clone::Clone for EAP_METHOD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EAP_PEER_FLAG_GUEST_ACCESS: u32 = 64u32;
pub const EAP_PEER_FLAG_HEALTH_STATE_CHANGE: u32 = 32768u32;
#[repr(C)]
pub struct EAP_PEER_METHOD_ROUTINES {
    pub dwVersion: u32,
    pub pEapType: *mut EAP_TYPE,
    pub EapPeerInitialize: isize,
    pub EapPeerGetIdentity: isize,
    pub EapPeerBeginSession: isize,
    pub EapPeerSetCredentials: isize,
    pub EapPeerProcessRequestPacket: isize,
    pub EapPeerGetResponsePacket: isize,
    pub EapPeerGetResult: isize,
    pub EapPeerGetUIContext: isize,
    pub EapPeerSetUIContext: isize,
    pub EapPeerGetResponseAttributes: isize,
    pub EapPeerSetResponseAttributes: isize,
    pub EapPeerEndSession: isize,
    pub EapPeerShutdown: isize,
}
impl ::core::marker::Copy for EAP_PEER_METHOD_ROUTINES {}
impl ::core::clone::Clone for EAP_PEER_METHOD_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct EAP_TYPE {
    pub r#type: u8,
    pub dwVendorId: u32,
    pub dwVendorType: u32,
}
impl ::core::marker::Copy for EAP_TYPE {}
impl ::core::clone::Clone for EAP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EAP_UI_DATA_FORMAT {
    pub credData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub credExpiryData: *mut EAP_CRED_EXPIRY_REQ,
    pub credLogonData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_UI_DATA_FORMAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_UI_DATA_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EAP_UI_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
pub const EAP_UI_INPUT_FIELD_PROPS_READ_ONLY: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapCertificateCredential {
    pub certHash: [u8; 20],
    pub password: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapCertificateCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapCertificateCredential {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EapCode = i32;
pub const EapCodeMinimum: EapCode = 1i32;
pub const EapCodeRequest: EapCode = 1i32;
pub const EapCodeResponse: EapCode = 2i32;
pub const EapCodeSuccess: EapCode = 3i32;
pub const EapCodeFailure: EapCode = 4i32;
pub const EapCodeMaximum: EapCode = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapCredential {
    pub credType: EapCredentialType,
    pub credData: EapCredentialTypeData,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapCredential {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EapCredentialType = i32;
pub const EAP_EMPTY_CREDENTIAL: EapCredentialType = 0i32;
pub const EAP_USERNAME_PASSWORD_CREDENTIAL: EapCredentialType = 1i32;
pub const EAP_WINLOGON_CREDENTIAL: EapCredentialType = 2i32;
pub const EAP_CERTIFICATE_CREDENTIAL: EapCredentialType = 3i32;
pub const EAP_SIM_CREDENTIAL: EapCredentialType = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EapCredentialTypeData {
    pub username_password: EapUsernamePasswordCredential,
    pub certificate: EapCertificateCredential,
    pub sim: EapSimCredential,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapCredentialTypeData {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapCredentialTypeData {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EapHostPeerAuthParams = i32;
pub const EapHostPeerAuthStatus: EapHostPeerAuthParams = 1i32;
pub const EapHostPeerIdentity: EapHostPeerAuthParams = 2i32;
pub const EapHostPeerIdentityExtendedInfo: EapHostPeerAuthParams = 3i32;
pub const EapHostNapInfo: EapHostPeerAuthParams = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapHostPeerMethodResult {
    pub fIsSuccess: super::super::Foundation::BOOL,
    pub dwFailureReasonCode: u32,
    pub fSaveConnectionData: super::super::Foundation::BOOL,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub fSaveUserData: super::super::Foundation::BOOL,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub pAttribArray: *mut EAP_ATTRIBUTES,
    pub isolationState: ISOLATION_STATE,
    pub pEapMethodInfo: *mut EAP_METHOD_INFO,
    pub pEapError: *mut EAP_ERROR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapHostPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapHostPeerMethodResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EapHostPeerMethodResultReason = i32;
pub const EapHostPeerMethodResultAltSuccessReceived: EapHostPeerMethodResultReason = 1i32;
pub const EapHostPeerMethodResultTimeout: EapHostPeerMethodResultReason = 2i32;
pub const EapHostPeerMethodResultFromMethod: EapHostPeerMethodResultReason = 3i32;
pub type EapHostPeerResponseAction = i32;
pub const EapHostPeerResponseDiscard: EapHostPeerResponseAction = 0i32;
pub const EapHostPeerResponseSend: EapHostPeerResponseAction = 1i32;
pub const EapHostPeerResponseResult: EapHostPeerResponseAction = 2i32;
pub const EapHostPeerResponseInvokeUi: EapHostPeerResponseAction = 3i32;
pub const EapHostPeerResponseRespond: EapHostPeerResponseAction = 4i32;
pub const EapHostPeerResponseStartAuthentication: EapHostPeerResponseAction = 5i32;
pub const EapHostPeerResponseNone: EapHostPeerResponseAction = 6i32;
#[repr(C)]
pub struct EapPacket {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EapPacket {}
impl ::core::clone::Clone for EapPacket {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapPeerMethodOutput {
    pub action: EapPeerMethodResponseAction,
    pub fAllowNotifications: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapPeerMethodOutput {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapPeerMethodOutput {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EapPeerMethodResponseAction = i32;
pub const EapPeerMethodResponseActionDiscard: EapPeerMethodResponseAction = 0i32;
pub const EapPeerMethodResponseActionSend: EapPeerMethodResponseAction = 1i32;
pub const EapPeerMethodResponseActionResult: EapPeerMethodResponseAction = 2i32;
pub const EapPeerMethodResponseActionInvokeUI: EapPeerMethodResponseAction = 3i32;
pub const EapPeerMethodResponseActionRespond: EapPeerMethodResponseAction = 4i32;
pub const EapPeerMethodResponseActionNone: EapPeerMethodResponseAction = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapPeerMethodResult {
    pub fIsSuccess: super::super::Foundation::BOOL,
    pub dwFailureReasonCode: u32,
    pub fSaveConnectionData: super::super::Foundation::BOOL,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub fSaveUserData: super::super::Foundation::BOOL,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub pAttribArray: *mut EAP_ATTRIBUTES,
    pub pEapError: *mut EAP_ERROR,
    pub pNgcKerbTicket: *mut NgcTicketContext,
    pub fSaveToCredMan: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapPeerMethodResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EapPeerMethodResultReason = i32;
pub const EapPeerMethodResultUnknown: EapPeerMethodResultReason = 1i32;
pub const EapPeerMethodResultSuccess: EapPeerMethodResultReason = 2i32;
pub const EapPeerMethodResultFailure: EapPeerMethodResultReason = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapSimCredential {
    pub iccID: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapSimCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapSimCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapUsernamePasswordCredential {
    pub username: super::super::Foundation::PWSTR,
    pub password: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapUsernamePasswordCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapUsernamePasswordCredential {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FACILITY_EAP_MESSAGE: u32 = 2114u32;
pub const GUID_EapHost_Cause_CertStoreInaccessible: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 4] };
pub const GUID_EapHost_Cause_EapNegotiationFailed: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 28] };
pub const GUID_EapHost_Cause_EapQecInaccessible: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 3, 18] };
pub const GUID_EapHost_Cause_Generic_AuthFailure: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 4] };
pub const GUID_EapHost_Cause_IdentityUnknown: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 2, 4] };
pub const GUID_EapHost_Cause_MethodDLLNotFound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 1] };
pub const GUID_EapHost_Cause_MethodDoesNotSupportOperation: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 30] };
pub const GUID_EapHost_Cause_Method_Config_Does_Not_Support_Sso: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3659054386, data2: 79, data3: 16890, data4: [174, 8, 11, 200, 94, 88, 69, 172] };
pub const GUID_EapHost_Cause_No_SmartCardReader_Found: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 43] };
pub const GUID_EapHost_Cause_Server_CertExpired: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 5] };
pub const GUID_EapHost_Cause_Server_CertInvalid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 6] };
pub const GUID_EapHost_Cause_Server_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 7] };
pub const GUID_EapHost_Cause_Server_CertOtherError: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 8] };
pub const GUID_EapHost_Cause_Server_CertRevoked: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 8] };
pub const GUID_EapHost_Cause_Server_Root_CertNameRequired: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 18] };
pub const GUID_EapHost_Cause_Server_Root_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 18] };
pub const GUID_EapHost_Cause_SimNotValid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 3, 4] };
pub const GUID_EapHost_Cause_ThirdPartyMethod_Host_Reset: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 2, 18] };
pub const GUID_EapHost_Cause_User_Account_OtherProblem: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 14] };
pub const GUID_EapHost_Cause_User_CertExpired: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 9] };
pub const GUID_EapHost_Cause_User_CertInvalid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 10] };
pub const GUID_EapHost_Cause_User_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 11] };
pub const GUID_EapHost_Cause_User_CertOtherError: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 12] };
pub const GUID_EapHost_Cause_User_CertRejected: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 13] };
pub const GUID_EapHost_Cause_User_CertRevoked: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 14] };
pub const GUID_EapHost_Cause_User_CredsRejected: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 2, 14] };
pub const GUID_EapHost_Cause_User_Root_CertExpired: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 15] };
pub const GUID_EapHost_Cause_User_Root_CertInvalid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 16] };
pub const GUID_EapHost_Cause_User_Root_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 17] };
pub const GUID_EapHost_Cause_XmlMalformed: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 29] };
pub const GUID_EapHost_Default: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const GUID_EapHost_Help_ObtainingCerts: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4113952419,
    data2: 7133,
    data3: 18122,
    data4: [162, 252, 166, 101, 89, 57, 183, 232],
};
pub const GUID_EapHost_Help_Troubleshooting: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 858815183, data2: 1688, data3: 16826, data4: [176, 20, 234, 10, 46, 184, 208, 168] };
pub const GUID_EapHost_Repair_ContactAdmin_AuthFailure: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 31] };
pub const GUID_EapHost_Repair_ContactAdmin_CertNameAbsent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 41] };
pub const GUID_EapHost_Repair_ContactAdmin_CertStoreInaccessible: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 36] };
pub const GUID_EapHost_Repair_ContactAdmin_IdentityUnknown: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 32] };
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserAccount: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 37] };
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserCert: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 44] };
pub const GUID_EapHost_Repair_ContactAdmin_MethodNotFound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 34] };
pub const GUID_EapHost_Repair_ContactAdmin_NegotiationFailed: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 33] };
pub const GUID_EapHost_Repair_ContactAdmin_NoSmartCardReader: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 42] };
pub const GUID_EapHost_Repair_ContactAdmin_RootCertInvalid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 38] };
pub const GUID_EapHost_Repair_ContactAdmin_RootCertNotFound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 39] };
pub const GUID_EapHost_Repair_ContactAdmin_RootExpired: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 40] };
pub const GUID_EapHost_Repair_ContactSysadmin: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 2] };
pub const GUID_EapHost_Repair_Method_Not_Support_Sso: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 45] };
pub const GUID_EapHost_Repair_No_ValidSim_Found: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 46] };
pub const GUID_EapHost_Repair_RestartNap: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 35] };
pub const GUID_EapHost_Repair_Retry_Authentication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 27] };
pub const GUID_EapHost_Repair_Server_ClientSelectServerCert: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 24] };
pub const GUID_EapHost_Repair_User_AuthFailure: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 25] };
pub const GUID_EapHost_Repair_User_GetNewCert: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 26] };
pub const GUID_EapHost_Repair_User_SelectValidCert: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 27] };
pub type IAccountingProviderConfig = *mut ::core::ffi::c_void;
pub type IAuthenticationProviderConfig = *mut ::core::ffi::c_void;
pub type IEAPProviderConfig = *mut ::core::ffi::c_void;
pub type IEAPProviderConfig2 = *mut ::core::ffi::c_void;
pub type IEAPProviderConfig3 = *mut ::core::ffi::c_void;
pub type IRouterProtocolConfig = *mut ::core::ffi::c_void;
pub type ISOLATION_STATE = i32;
pub const ISOLATION_STATE_UNKNOWN: ISOLATION_STATE = 0i32;
pub const ISOLATION_STATE_NOT_RESTRICTED: ISOLATION_STATE = 1i32;
pub const ISOLATION_STATE_IN_PROBATION: ISOLATION_STATE = 2i32;
pub const ISOLATION_STATE_RESTRICTED_ACCESS: ISOLATION_STATE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LEGACY_IDENTITY_UI_PARAMS {
    pub eapType: u32,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: super::super::Foundation::PWSTR,
    pub dwError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LEGACY_IDENTITY_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LEGACY_IDENTITY_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LEGACY_INTERACTIVE_UI_PARAMS {
    pub eapType: u32,
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
}
impl ::core::marker::Copy for LEGACY_INTERACTIVE_UI_PARAMS {}
impl ::core::clone::Clone for LEGACY_INTERACTIVE_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAXEAPCODE: u32 = 4u32;
pub const MAX_EAP_CONFIG_INPUT_FIELD_LENGTH: u32 = 256u32;
pub const MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH: u32 = 1024u32;
pub const NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH: u32 = 90u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NgcTicketContext {
    pub wszTicket: [u16; 45],
    pub hKey: usize,
    pub hImpersonateToken: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NgcTicketContext {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NgcTicketContext {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NotificationHandler = ::core::option::Option<unsafe extern "system" fn(connectionid: ::windows_sys::core::GUID, pcontextdata: *mut ::core::ffi::c_void)>;
pub type PPP_EAP_ACTION = i32;
pub const EAPACTION_NoAction: PPP_EAP_ACTION = 0i32;
pub const EAPACTION_Authenticate: PPP_EAP_ACTION = 1i32;
pub const EAPACTION_Done: PPP_EAP_ACTION = 2i32;
pub const EAPACTION_SendAndDone: PPP_EAP_ACTION = 3i32;
pub const EAPACTION_Send: PPP_EAP_ACTION = 4i32;
pub const EAPACTION_SendWithTimeout: PPP_EAP_ACTION = 5i32;
pub const EAPACTION_SendWithTimeoutInteractive: PPP_EAP_ACTION = 6i32;
pub const EAPACTION_IndicateTLV: PPP_EAP_ACTION = 7i32;
pub const EAPACTION_IndicateIdentity: PPP_EAP_ACTION = 8i32;
#[repr(C)]
pub struct PPP_EAP_INFO {
    pub dwSizeInBytes: u32,
    pub dwEapTypeId: u32,
    pub RasEapInitialize: isize,
    pub RasEapBegin: isize,
    pub RasEapEnd: isize,
    pub RasEapMakeMessage: isize,
}
impl ::core::marker::Copy for PPP_EAP_INFO {}
impl ::core::clone::Clone for PPP_EAP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PPP_EAP_INPUT {
    pub dwSizeInBytes: u32,
    pub fFlags: u32,
    pub fAuthenticator: super::super::Foundation::BOOL,
    pub pwszIdentity: super::super::Foundation::PWSTR,
    pub pwszPassword: super::super::Foundation::PWSTR,
    pub bInitialId: u8,
    pub pUserAttributes: *mut RAS_AUTH_ATTRIBUTE,
    pub fAuthenticationComplete: super::super::Foundation::BOOL,
    pub dwAuthResultCode: u32,
    pub hTokenImpersonateUser: super::super::Foundation::HANDLE,
    pub fSuccessPacketReceived: super::super::Foundation::BOOL,
    pub fDataReceivedFromInteractiveUI: super::super::Foundation::BOOL,
    pub pDataFromInteractiveUI: *mut u8,
    pub dwSizeOfDataFromInteractiveUI: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeOfConnectionData: u32,
    pub pUserData: *mut u8,
    pub dwSizeOfUserData: u32,
    pub hReserved: super::super::Foundation::HANDLE,
    pub guidConnectionId: ::windows_sys::core::GUID,
    pub isVpn: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PPP_EAP_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PPP_EAP_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PPP_EAP_OUTPUT {
    pub dwSizeInBytes: u32,
    pub Action: PPP_EAP_ACTION,
    pub dwAuthResultCode: u32,
    pub pUserAttributes: *mut RAS_AUTH_ATTRIBUTE,
    pub fInvokeInteractiveUI: super::super::Foundation::BOOL,
    pub pUIContextData: *mut u8,
    pub dwSizeOfUIContextData: u32,
    pub fSaveConnectionData: super::super::Foundation::BOOL,
    pub pConnectionData: *mut u8,
    pub dwSizeOfConnectionData: u32,
    pub fSaveUserData: super::super::Foundation::BOOL,
    pub pUserData: *mut u8,
    pub dwSizeOfUserData: u32,
    pub pNgcKerbTicket: *mut NgcTicketContext,
    pub fSaveToCredMan: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PPP_EAP_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PPP_EAP_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PPP_EAP_PACKET {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for PPP_EAP_PACKET {}
impl ::core::clone::Clone for PPP_EAP_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RAS_AUTH_ATTRIBUTE {
    pub raaType: RAS_AUTH_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub Value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RAS_AUTH_ATTRIBUTE {}
impl ::core::clone::Clone for RAS_AUTH_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RAS_AUTH_ATTRIBUTE_TYPE = i32;
pub const raatMinimum: RAS_AUTH_ATTRIBUTE_TYPE = 0i32;
pub const raatUserName: RAS_AUTH_ATTRIBUTE_TYPE = 1i32;
pub const raatUserPassword: RAS_AUTH_ATTRIBUTE_TYPE = 2i32;
pub const raatMD5CHAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = 3i32;
pub const raatNASIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = 4i32;
pub const raatNASPort: RAS_AUTH_ATTRIBUTE_TYPE = 5i32;
pub const raatServiceType: RAS_AUTH_ATTRIBUTE_TYPE = 6i32;
pub const raatFramedProtocol: RAS_AUTH_ATTRIBUTE_TYPE = 7i32;
pub const raatFramedIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = 8i32;
pub const raatFramedIPNetmask: RAS_AUTH_ATTRIBUTE_TYPE = 9i32;
pub const raatFramedRouting: RAS_AUTH_ATTRIBUTE_TYPE = 10i32;
pub const raatFilterId: RAS_AUTH_ATTRIBUTE_TYPE = 11i32;
pub const raatFramedMTU: RAS_AUTH_ATTRIBUTE_TYPE = 12i32;
pub const raatFramedCompression: RAS_AUTH_ATTRIBUTE_TYPE = 13i32;
pub const raatLoginIPHost: RAS_AUTH_ATTRIBUTE_TYPE = 14i32;
pub const raatLoginService: RAS_AUTH_ATTRIBUTE_TYPE = 15i32;
pub const raatLoginTCPPort: RAS_AUTH_ATTRIBUTE_TYPE = 16i32;
pub const raatUnassigned17: RAS_AUTH_ATTRIBUTE_TYPE = 17i32;
pub const raatReplyMessage: RAS_AUTH_ATTRIBUTE_TYPE = 18i32;
pub const raatCallbackNumber: RAS_AUTH_ATTRIBUTE_TYPE = 19i32;
pub const raatCallbackId: RAS_AUTH_ATTRIBUTE_TYPE = 20i32;
pub const raatUnassigned21: RAS_AUTH_ATTRIBUTE_TYPE = 21i32;
pub const raatFramedRoute: RAS_AUTH_ATTRIBUTE_TYPE = 22i32;
pub const raatFramedIPXNetwork: RAS_AUTH_ATTRIBUTE_TYPE = 23i32;
pub const raatState: RAS_AUTH_ATTRIBUTE_TYPE = 24i32;
pub const raatClass: RAS_AUTH_ATTRIBUTE_TYPE = 25i32;
pub const raatVendorSpecific: RAS_AUTH_ATTRIBUTE_TYPE = 26i32;
pub const raatSessionTimeout: RAS_AUTH_ATTRIBUTE_TYPE = 27i32;
pub const raatIdleTimeout: RAS_AUTH_ATTRIBUTE_TYPE = 28i32;
pub const raatTerminationAction: RAS_AUTH_ATTRIBUTE_TYPE = 29i32;
pub const raatCalledStationId: RAS_AUTH_ATTRIBUTE_TYPE = 30i32;
pub const raatCallingStationId: RAS_AUTH_ATTRIBUTE_TYPE = 31i32;
pub const raatNASIdentifier: RAS_AUTH_ATTRIBUTE_TYPE = 32i32;
pub const raatProxyState: RAS_AUTH_ATTRIBUTE_TYPE = 33i32;
pub const raatLoginLATService: RAS_AUTH_ATTRIBUTE_TYPE = 34i32;
pub const raatLoginLATNode: RAS_AUTH_ATTRIBUTE_TYPE = 35i32;
pub const raatLoginLATGroup: RAS_AUTH_ATTRIBUTE_TYPE = 36i32;
pub const raatFramedAppleTalkLink: RAS_AUTH_ATTRIBUTE_TYPE = 37i32;
pub const raatFramedAppleTalkNetwork: RAS_AUTH_ATTRIBUTE_TYPE = 38i32;
pub const raatFramedAppleTalkZone: RAS_AUTH_ATTRIBUTE_TYPE = 39i32;
pub const raatAcctStatusType: RAS_AUTH_ATTRIBUTE_TYPE = 40i32;
pub const raatAcctDelayTime: RAS_AUTH_ATTRIBUTE_TYPE = 41i32;
pub const raatAcctInputOctets: RAS_AUTH_ATTRIBUTE_TYPE = 42i32;
pub const raatAcctOutputOctets: RAS_AUTH_ATTRIBUTE_TYPE = 43i32;
pub const raatAcctSessionId: RAS_AUTH_ATTRIBUTE_TYPE = 44i32;
pub const raatAcctAuthentic: RAS_AUTH_ATTRIBUTE_TYPE = 45i32;
pub const raatAcctSessionTime: RAS_AUTH_ATTRIBUTE_TYPE = 46i32;
pub const raatAcctInputPackets: RAS_AUTH_ATTRIBUTE_TYPE = 47i32;
pub const raatAcctOutputPackets: RAS_AUTH_ATTRIBUTE_TYPE = 48i32;
pub const raatAcctTerminateCause: RAS_AUTH_ATTRIBUTE_TYPE = 49i32;
pub const raatAcctMultiSessionId: RAS_AUTH_ATTRIBUTE_TYPE = 50i32;
pub const raatAcctLinkCount: RAS_AUTH_ATTRIBUTE_TYPE = 51i32;
pub const raatAcctEventTimeStamp: RAS_AUTH_ATTRIBUTE_TYPE = 55i32;
pub const raatMD5CHAPChallenge: RAS_AUTH_ATTRIBUTE_TYPE = 60i32;
pub const raatNASPortType: RAS_AUTH_ATTRIBUTE_TYPE = 61i32;
pub const raatPortLimit: RAS_AUTH_ATTRIBUTE_TYPE = 62i32;
pub const raatLoginLATPort: RAS_AUTH_ATTRIBUTE_TYPE = 63i32;
pub const raatTunnelType: RAS_AUTH_ATTRIBUTE_TYPE = 64i32;
pub const raatTunnelMediumType: RAS_AUTH_ATTRIBUTE_TYPE = 65i32;
pub const raatTunnelClientEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = 66i32;
pub const raatTunnelServerEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = 67i32;
pub const raatARAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = 70i32;
pub const raatARAPFeatures: RAS_AUTH_ATTRIBUTE_TYPE = 71i32;
pub const raatARAPZoneAccess: RAS_AUTH_ATTRIBUTE_TYPE = 72i32;
pub const raatARAPSecurity: RAS_AUTH_ATTRIBUTE_TYPE = 73i32;
pub const raatARAPSecurityData: RAS_AUTH_ATTRIBUTE_TYPE = 74i32;
pub const raatPasswordRetry: RAS_AUTH_ATTRIBUTE_TYPE = 75i32;
pub const raatPrompt: RAS_AUTH_ATTRIBUTE_TYPE = 76i32;
pub const raatConnectInfo: RAS_AUTH_ATTRIBUTE_TYPE = 77i32;
pub const raatConfigurationToken: RAS_AUTH_ATTRIBUTE_TYPE = 78i32;
pub const raatEAPMessage: RAS_AUTH_ATTRIBUTE_TYPE = 79i32;
pub const raatSignature: RAS_AUTH_ATTRIBUTE_TYPE = 80i32;
pub const raatARAPChallengeResponse: RAS_AUTH_ATTRIBUTE_TYPE = 84i32;
pub const raatAcctInterimInterval: RAS_AUTH_ATTRIBUTE_TYPE = 85i32;
pub const raatNASIPv6Address: RAS_AUTH_ATTRIBUTE_TYPE = 95i32;
pub const raatFramedInterfaceId: RAS_AUTH_ATTRIBUTE_TYPE = 96i32;
pub const raatFramedIPv6Prefix: RAS_AUTH_ATTRIBUTE_TYPE = 97i32;
pub const raatLoginIPv6Host: RAS_AUTH_ATTRIBUTE_TYPE = 98i32;
pub const raatFramedIPv6Route: RAS_AUTH_ATTRIBUTE_TYPE = 99i32;
pub const raatFramedIPv6Pool: RAS_AUTH_ATTRIBUTE_TYPE = 100i32;
pub const raatARAPGuestLogon: RAS_AUTH_ATTRIBUTE_TYPE = 8096i32;
pub const raatCertificateOID: RAS_AUTH_ATTRIBUTE_TYPE = 8097i32;
pub const raatEAPConfiguration: RAS_AUTH_ATTRIBUTE_TYPE = 8098i32;
pub const raatPEAPEmbeddedEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = 8099i32;
pub const raatInnerEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = 8099i32;
pub const raatPEAPFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = 8100i32;
pub const raatFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = 8100i32;
pub const raatEAPTLV: RAS_AUTH_ATTRIBUTE_TYPE = 8102i32;
pub const raatCredentialsChanged: RAS_AUTH_ATTRIBUTE_TYPE = 8103i32;
pub const raatCertificateThumbprint: RAS_AUTH_ATTRIBUTE_TYPE = 8250i32;
pub const raatPeerId: RAS_AUTH_ATTRIBUTE_TYPE = 9000i32;
pub const raatServerId: RAS_AUTH_ATTRIBUTE_TYPE = 9001i32;
pub const raatMethodId: RAS_AUTH_ATTRIBUTE_TYPE = 9002i32;
pub const raatEMSK: RAS_AUTH_ATTRIBUTE_TYPE = 9003i32;
pub const raatSessionId: RAS_AUTH_ATTRIBUTE_TYPE = 9004i32;
pub const raatReserved: RAS_AUTH_ATTRIBUTE_TYPE = -1i32;
pub const RAS_EAP_FLAG_8021X_AUTH: u32 = 128u32;
pub const RAS_EAP_FLAG_ALTERNATIVE_USER_DB: u32 = 2048u32;
pub const RAS_EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
pub const RAS_EAP_FLAG_FIRST_LINK: u32 = 16u32;
pub const RAS_EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
pub const RAS_EAP_FLAG_HOSTED_IN_PEAP: u32 = 256u32;
pub const RAS_EAP_FLAG_LOGON: u32 = 4u32;
pub const RAS_EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
pub const RAS_EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
pub const RAS_EAP_FLAG_PEAP_FORCE_FULL_AUTH: u32 = 4096u32;
pub const RAS_EAP_FLAG_PEAP_UPFRONT: u32 = 1024u32;
pub const RAS_EAP_FLAG_PREVIEW: u32 = 8u32;
pub const RAS_EAP_FLAG_PRE_LOGON: u32 = 131072u32;
pub const RAS_EAP_FLAG_RESERVED: u32 = 1048576u32;
pub const RAS_EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
pub const RAS_EAP_FLAG_ROUTER: u32 = 1u32;
pub const RAS_EAP_FLAG_SAVE_CREDMAN: u32 = 2097152u32;
pub const RAS_EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
pub const RAS_EAP_ROLE_AUTHENTICATEE: u32 = 2u32;
pub const RAS_EAP_ROLE_AUTHENTICATOR: u32 = 1u32;
pub const RAS_EAP_ROLE_EXCLUDE_IN_EAP: u32 = 4u32;
pub const RAS_EAP_ROLE_EXCLUDE_IN_PEAP: u32 = 8u32;
pub const RAS_EAP_ROLE_EXCLUDE_IN_VPN: u32 = 16u32;
