#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Security_AppLocker")]
pub mod AppLocker;
#[cfg(feature = "Win32_Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Win32_Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Win32_Security_ConfigurationSnapin")]
pub mod ConfigurationSnapin;
#[cfg(feature = "Win32_Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Win32_Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Win32_Security_DiagnosticDataQuery")]
pub mod DiagnosticDataQuery;
#[cfg(feature = "Win32_Security_DirectoryServices")]
pub mod DirectoryServices;
#[cfg(feature = "Win32_Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub mod ExtensibleAuthenticationProtocol;
#[cfg(feature = "Win32_Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "Win32_Security_LicenseProtection")]
pub mod LicenseProtection;
#[cfg(feature = "Win32_Security_NetworkAccessProtection")]
pub mod NetworkAccessProtection;
#[cfg(feature = "Win32_Security_Tpm")]
pub mod Tpm;
#[cfg(feature = "Win32_Security_WinTrust")]
pub mod WinTrust;
#[cfg(feature = "Win32_Security_WinWlx")]
pub mod WinWlx;
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheck(psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckAndAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PSTR, objectname: super::Foundation::PSTR, securitydescriptor: *const SECURITY_DESCRIPTOR, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: super::Foundation::BOOL, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckAndAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PWSTR, objectname: super::Foundation::PWSTR, securitydescriptor: *const SECURITY_DESCRIPTOR, desiredaccess: u32, genericmapping: *const GENERIC_MAPPING, objectcreation: super::Foundation::BOOL, grantedaccess: *mut u32, accessstatus: *mut i32, pfgenerateonclose: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByType(psecuritydescriptor: *const SECURITY_DESCRIPTOR, principalselfsid: super::Foundation::PSID, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, objecttypelist: *mut OBJECT_TYPE_LIST, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccess: *mut u32, accessstatus: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeAndAuditAlarmA(
        subsystemname: super::Foundation::PSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PSTR,
        objectname: super::Foundation::PSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatus: *mut i32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeAndAuditAlarmW(
        subsystemname: super::Foundation::PWSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PWSTR,
        objectname: super::Foundation::PWSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatus: *mut i32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultList(psecuritydescriptor: *const SECURITY_DESCRIPTOR, principalselfsid: super::Foundation::PSID, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, objecttypelist: *mut OBJECT_TYPE_LIST, objecttypelistlength: u32, genericmapping: *const GENERIC_MAPPING, privilegeset: *mut PRIVILEGE_SET, privilegesetlength: *mut u32, grantedaccesslist: *mut u32, accessstatuslist: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmA(
        subsystemname: super::Foundation::PSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PSTR,
        objectname: super::Foundation::PSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(
        subsystemname: super::Foundation::PSTR,
        handleid: *const ::core::ffi::c_void,
        clienttoken: super::Foundation::HANDLE,
        objecttypename: super::Foundation::PSTR,
        objectname: super::Foundation::PSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccess: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmByHandleW(
        subsystemname: super::Foundation::PWSTR,
        handleid: *const ::core::ffi::c_void,
        clienttoken: super::Foundation::HANDLE,
        objecttypename: super::Foundation::PWSTR,
        objectname: super::Foundation::PWSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccesslist: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessCheckByTypeResultListAndAuditAlarmW(
        subsystemname: super::Foundation::PWSTR,
        handleid: *const ::core::ffi::c_void,
        objecttypename: super::Foundation::PWSTR,
        objectname: super::Foundation::PWSTR,
        securitydescriptor: *const SECURITY_DESCRIPTOR,
        principalselfsid: super::Foundation::PSID,
        desiredaccess: u32,
        audittype: AUDIT_EVENT_TYPE,
        flags: u32,
        objecttypelist: *mut OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        genericmapping: *const GENERIC_MAPPING,
        objectcreation: super::Foundation::BOOL,
        grantedaccesslist: *mut u32,
        accessstatuslist: *mut u32,
        pfgenerateonclose: *mut i32,
    ) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedAce(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessAllowedObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows_sys::core::GUID, inheritedobjecttypeguid: *const ::windows_sys::core::GUID, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedAce(pacl: *mut ACL, dwacerevision: u32, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAccessDeniedObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows_sys::core::GUID, inheritedobjecttypeguid: *const ::windows_sys::core::GUID, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAce(pacl: *mut ACL, dwacerevision: u32, dwstartingaceindex: u32, pacelist: *const ::core::ffi::c_void, nacelistlength: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessAce(pacl: *mut ACL, dwacerevision: u32, dwaccessmask: u32, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessAceEx(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, dwaccessmask: u32, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAuditAccessObjectAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, objecttypeguid: *const ::windows_sys::core::GUID, inheritedobjecttypeguid: *const ::windows_sys::core::GUID, psid: super::Foundation::PSID, bauditsuccess: super::Foundation::BOOL, bauditfailure: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConditionalAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, acetype: u8, accessmask: u32, psid: super::Foundation::PSID, conditionstr: super::Foundation::PWSTR, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddMandatoryAce(pacl: *mut ACL, dwacerevision: ACE_REVISION, aceflags: ACE_FLAGS, mandatorypolicy: u32, plabelsid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddResourceAttributeAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID, pattributeinfo: *const CLAIM_SECURITY_ATTRIBUTES_INFORMATION, preturnlength: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddScopedPolicyIDAce(pacl: *mut ACL, dwacerevision: u32, aceflags: ACE_FLAGS, accessmask: u32, psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustTokenGroups(tokenhandle: super::Foundation::HANDLE, resettodefault: super::Foundation::BOOL, newstate: *const TOKEN_GROUPS, bufferlength: u32, previousstate: *mut TOKEN_GROUPS, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustTokenPrivileges(tokenhandle: super::Foundation::HANDLE, disableallprivileges: super::Foundation::BOOL, newstate: *const TOKEN_PRIVILEGES, bufferlength: u32, previousstate: *mut TOKEN_PRIVILEGES, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateAndInitializeSid(pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8, nsubauthority0: u32, nsubauthority1: u32, nsubauthority2: u32, nsubauthority3: u32, nsubauthority4: u32, nsubauthority5: u32, nsubauthority6: u32, nsubauthority7: u32, psid: *mut super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateLocallyUniqueId(luid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreAllAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreAnyAccessesGranted(grantedaccess: u32, desiredaccess: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenCapability(tokenhandle: super::Foundation::HANDLE, capabilitysidtocheck: super::Foundation::PSID, hascapability: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenMembership(tokenhandle: super::Foundation::HANDLE, sidtocheck: super::Foundation::PSID, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckTokenMembershipEx(tokenhandle: super::Foundation::HANDLE, sidtocheck: super::Foundation::PSID, flags: u32, ismember: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertToAutoInheritPrivateObjectSecurity(parentdescriptor: *const SECURITY_DESCRIPTOR, currentsecuritydescriptor: *const SECURITY_DESCRIPTOR, newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttype: *const ::windows_sys::core::GUID, isdirectoryobject: super::Foundation::BOOLEAN, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopySid(ndestinationsidlength: u32, pdestinationsid: super::Foundation::PSID, psourcesid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurity(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, isdirectoryobject: super::Foundation::BOOL, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurityEx(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttype: *const ::windows_sys::core::GUID, iscontainerobject: super::Foundation::BOOL, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePrivateObjectSecurityWithMultipleInheritance(parentdescriptor: *const SECURITY_DESCRIPTOR, creatordescriptor: *const SECURITY_DESCRIPTOR, newdescriptor: *mut *mut SECURITY_DESCRIPTOR, objecttypes: *const *const ::windows_sys::core::GUID, guidcount: u32, iscontainerobject: super::Foundation::BOOL, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, token: super::Foundation::HANDLE, genericmapping: *const GENERIC_MAPPING) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRestrictedToken(existingtokenhandle: super::Foundation::HANDLE, flags: CREATE_RESTRICTED_TOKEN_FLAGS, disablesidcount: u32, sidstodisable: *const SID_AND_ATTRIBUTES, deleteprivilegecount: u32, privilegestodelete: *const LUID_AND_ATTRIBUTES, restrictedsidcount: u32, sidstorestrict: *const SID_AND_ATTRIBUTES, newtokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateWellKnownSid(wellknownsidtype: WELL_KNOWN_SID_TYPE, domainsid: super::Foundation::PSID, psid: super::Foundation::PSID, cbsid: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteAce(pacl: *mut ACL, dwaceindex: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveCapabilitySidsFromName(capname: super::Foundation::PWSTR, capabilitygroupsids: *mut *mut super::Foundation::PSID, capabilitygroupsidcount: *mut u32, capabilitysids: *mut *mut super::Foundation::PSID, capabilitysidcount: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPrivateObjectSecurity(objectdescriptor: *const *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DuplicateToken(existingtokenhandle: super::Foundation::HANDLE, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, duplicatetokenhandle: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DuplicateTokenEx(hexistingtoken: super::Foundation::HANDLE, dwdesiredaccess: TOKEN_ACCESS_MASK, lptokenattributes: *const SECURITY_ATTRIBUTES, impersonationlevel: SECURITY_IMPERSONATION_LEVEL, tokentype: TOKEN_TYPE, phnewtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualDomainSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID, pfequal: *mut super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualPrefixSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualSid(psid1: super::Foundation::PSID, psid2: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFirstFreeAce(pacl: *const ACL, pace: *mut *mut ::core::ffi::c_void) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeSid(psid: super::Foundation::PSID) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAce(pacl: *const ACL, dwaceindex: u32, pace: *mut *mut ::core::ffi::c_void) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAclInformation(pacl: *const ACL, paclinformation: *mut ::core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerAce(acl: *const ACL, startingaceindex: u32, appcontainerace: *mut *mut ::core::ffi::c_void, appcontaineraceindex: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCachedSigningLevel(file: super::Foundation::HANDLE, flags: *mut u32, signinglevel: *mut u32, thumbprint: *mut u8, thumbprintsize: *mut u32, thumbprintalgorithm: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSecurityA(lpfilename: super::Foundation::PSTR, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileSecurityW(lpfilename: super::Foundation::PWSTR, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKernelObjectSecurity(handle: super::Foundation::HANDLE, requestedinformation: u32, psecuritydescriptor: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLengthSid(psid: super::Foundation::PSID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateObjectSecurity(objectdescriptor: *const SECURITY_DESCRIPTOR, securityinformation: u32, resultantdescriptor: *mut SECURITY_DESCRIPTOR, descriptorlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pcontrol: *mut u16, lpdwrevision: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorDacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbdaclpresent: *mut i32, pdacl: *mut *mut ACL, lpbdacldefaulted: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorGroup(psecuritydescriptor: *const SECURITY_DESCRIPTOR, pgroup: *mut super::Foundation::PSID, lpbgroupdefaulted: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorLength(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorOwner(psecuritydescriptor: *const SECURITY_DESCRIPTOR, powner: *mut super::Foundation::PSID, lpbownerdefaulted: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorRMControl(securitydescriptor: *const SECURITY_DESCRIPTOR, rmcontrol: *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSecurityDescriptorSacl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, lpbsaclpresent: *mut i32, psacl: *mut *mut ACL, lpbsacldefaulted: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidIdentifierAuthority(psid: super::Foundation::PSID) -> *mut SID_IDENTIFIER_AUTHORITY;
    pub fn GetSidLengthRequired(nsubauthoritycount: u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidSubAuthority(psid: super::Foundation::PSID, nsubauthority: u32) -> *mut u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSidSubAuthorityCount(psid: super::Foundation::PSID) -> *mut u8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTokenInformation(tokenhandle: super::Foundation::HANDLE, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *mut ::core::ffi::c_void, tokeninformationlength: u32, returnlength: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectSecurity(hobj: super::Foundation::HANDLE, psirequested: *const u32, psid: *mut SECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsAccountDomainSid(psid: super::Foundation::PSID, pdomainsid: super::Foundation::PSID, cbdomainsid: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateAnonymousToken(threadhandle: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateLoggedOnUser(htoken: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateSelf(impersonationlevel: SECURITY_IMPERSONATION_LEVEL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeAcl(pacl: *mut ACL, nacllength: u32, dwaclrevision: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSecurityDescriptor(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, dwrevision: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSid(sid: super::Foundation::PSID, pidentifierauthority: *const SID_IDENTIFIER_AUTHORITY, nsubauthoritycount: u8) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTokenRestricted(tokenhandle: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidAcl(pacl: *const ACL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidSecurityDescriptor(psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidSid(psid: super::Foundation::PSID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWellKnownSid(psid: super::Foundation::PSID, wellknownsidtype: WELL_KNOWN_SID_TYPE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserA(lpszusername: super::Foundation::PSTR, lpszdomain: super::Foundation::PSTR, lpszpassword: super::Foundation::PSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserExA(lpszusername: super::Foundation::PSTR, lpszdomain: super::Foundation::PSTR, lpszpassword: super::Foundation::PSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE, pplogonsid: *mut super::Foundation::PSID, ppprofilebuffer: *mut *mut ::core::ffi::c_void, pdwprofilelength: *mut u32, pquotalimits: *mut QUOTA_LIMITS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserExW(lpszusername: super::Foundation::PWSTR, lpszdomain: super::Foundation::PWSTR, lpszpassword: super::Foundation::PWSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE, pplogonsid: *mut super::Foundation::PSID, ppprofilebuffer: *mut *mut ::core::ffi::c_void, pdwprofilelength: *mut u32, pquotalimits: *mut QUOTA_LIMITS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogonUserW(lpszusername: super::Foundation::PWSTR, lpszdomain: super::Foundation::PWSTR, lpszpassword: super::Foundation::PWSTR, dwlogontype: LOGON32_LOGON, dwlogonprovider: LOGON32_PROVIDER, phtoken: *mut super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountNameA(lpsystemname: super::Foundation::PSTR, lpaccountname: super::Foundation::PSTR, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountNameW(lpsystemname: super::Foundation::PWSTR, lpaccountname: super::Foundation::PWSTR, sid: super::Foundation::PSID, cbsid: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountSidA(lpsystemname: super::Foundation::PSTR, sid: super::Foundation::PSID, name: super::Foundation::PSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupAccountSidW(lpsystemname: super::Foundation::PWSTR, sid: super::Foundation::PSID, name: super::Foundation::PWSTR, cchname: *mut u32, referenceddomainname: super::Foundation::PWSTR, cchreferenceddomainname: *mut u32, peuse: *mut SID_NAME_USE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeDisplayNameA(lpsystemname: super::Foundation::PSTR, lpname: super::Foundation::PSTR, lpdisplayname: super::Foundation::PSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeDisplayNameW(lpsystemname: super::Foundation::PWSTR, lpname: super::Foundation::PWSTR, lpdisplayname: super::Foundation::PWSTR, cchdisplayname: *mut u32, lplanguageid: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeNameA(lpsystemname: super::Foundation::PSTR, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PSTR, cchname: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeNameW(lpsystemname: super::Foundation::PWSTR, lpluid: *const super::Foundation::LUID, lpname: super::Foundation::PWSTR, cchname: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeValueA(lpsystemname: super::Foundation::PSTR, lpname: super::Foundation::PSTR, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupPrivilegeValueW(lpsystemname: super::Foundation::PWSTR, lpname: super::Foundation::PWSTR, lpluid: *mut super::Foundation::LUID) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeAbsoluteSD(pselfrelativesecuritydescriptor: *const SECURITY_DESCRIPTOR, pabsolutesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwabsolutesecuritydescriptorsize: *mut u32, pdacl: *mut ACL, lpdwdaclsize: *mut u32, psacl: *mut ACL, lpdwsaclsize: *mut u32, powner: super::Foundation::PSID, lpdwownersize: *mut u32, pprimarygroup: super::Foundation::PSID, lpdwprimarygroupsize: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeSelfRelativeSD(pabsolutesecuritydescriptor: *const SECURITY_DESCRIPTOR, pselfrelativesecuritydescriptor: *mut SECURITY_DESCRIPTOR, lpdwbufferlength: *mut u32) -> super::Foundation::BOOL;
    pub fn MapGenericMask(accessmask: *mut u32, genericmapping: *const GENERIC_MAPPING);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectCloseAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectCloseAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectDeleteAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectDeleteAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, generateonclose: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectOpenAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PSTR, objectname: super::Foundation::PSTR, psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, grantedaccess: u32, privileges: *const PRIVILEGE_SET, objectcreation: super::Foundation::BOOL, accessgranted: super::Foundation::BOOL, generateonclose: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectOpenAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, objecttypename: super::Foundation::PWSTR, objectname: super::Foundation::PWSTR, psecuritydescriptor: *const SECURITY_DESCRIPTOR, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, grantedaccess: u32, privileges: *const PRIVILEGE_SET, objectcreation: super::Foundation::BOOL, accessgranted: super::Foundation::BOOL, generateonclose: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectPrivilegeAuditAlarmA(subsystemname: super::Foundation::PSTR, handleid: *const ::core::ffi::c_void, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectPrivilegeAuditAlarmW(subsystemname: super::Foundation::PWSTR, handleid: *const ::core::ffi::c_void, clienttoken: super::Foundation::HANDLE, desiredaccess: u32, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegeCheck(clienttoken: super::Foundation::HANDLE, requiredprivileges: *mut PRIVILEGE_SET, pfresult: *mut i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegedServiceAuditAlarmA(subsystemname: super::Foundation::PSTR, servicename: super::Foundation::PSTR, clienttoken: super::Foundation::HANDLE, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivilegedServiceAuditAlarmW(subsystemname: super::Foundation::PWSTR, servicename: super::Foundation::PWSTR, clienttoken: super::Foundation::HANDLE, privileges: *const PRIVILEGE_SET, accessgranted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    pub fn QuerySecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevertToSelf() -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlConvertSidToUnicodeString(unicodestring: *mut super::Foundation::UNICODE_STRING, sid: super::Foundation::PSID, allocatedestinationstring: super::Foundation::BOOLEAN) -> super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlNormalizeSecurityDescriptor(securitydescriptor: *mut *mut SECURITY_DESCRIPTOR, securitydescriptorlength: u32, newsecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, newsecuritydescriptorlength: *mut u32, checkonly: super::Foundation::BOOLEAN) -> super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAclInformation(pacl: *mut ACL, paclinformation: *const ::core::ffi::c_void, naclinformationlength: u32, dwaclinformationclass: ACL_INFORMATION_CLASS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCachedSigningLevel(sourcefiles: *const super::Foundation::HANDLE, sourcefilecount: u32, flags: u32, targetfile: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileSecurityA(lpfilename: super::Foundation::PSTR, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFileSecurityW(lpfilename: super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetKernelObjectSecurity(handle: super::Foundation::HANDLE, securityinformation: u32, securitydescriptor: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrivateObjectSecurity(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, genericmapping: *const GENERIC_MAPPING, token: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPrivateObjectSecurityEx(securityinformation: u32, modificationdescriptor: *const SECURITY_DESCRIPTOR, objectssecuritydescriptor: *mut *mut SECURITY_DESCRIPTOR, autoinheritflags: SECURITY_AUTO_INHERIT_FLAGS, genericmapping: *const GENERIC_MAPPING, token: super::Foundation::HANDLE) -> super::Foundation::BOOL;
    pub fn SetSecurityAccessMask(securityinformation: u32, desiredaccess: *mut u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorControl(psecuritydescriptor: *const SECURITY_DESCRIPTOR, controlbitsofinterest: u16, controlbitstoset: u16) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorDacl(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bdaclpresent: super::Foundation::BOOL, pdacl: *const ACL, bdacldefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorGroup(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, pgroup: super::Foundation::PSID, bgroupdefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorOwner(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, powner: super::Foundation::PSID, bownerdefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorRMControl(securitydescriptor: *mut SECURITY_DESCRIPTOR, rmcontrol: *const u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSecurityDescriptorSacl(psecuritydescriptor: *mut SECURITY_DESCRIPTOR, bsaclpresent: super::Foundation::BOOL, psacl: *const ACL, bsacldefaulted: super::Foundation::BOOL) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTokenInformation(tokenhandle: super::Foundation::HANDLE, tokeninformationclass: TOKEN_INFORMATION_CLASS, tokeninformation: *const ::core::ffi::c_void, tokeninformationlength: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectSecurity(hobj: super::Foundation::HANDLE, psirequested: *const OBJECT_SECURITY_INFORMATION, psid: *const SECURITY_DESCRIPTOR) -> super::Foundation::BOOL;
}
#[repr(C)]
pub struct ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_ALLOWED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_CALLBACK_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_ALLOWED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_ALLOWED_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_ALLOWED_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_DENIED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_DENIED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_CALLBACK_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_DENIED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for ACCESS_DENIED_OBJECT_ACE {}
impl ::core::clone::Clone for ACCESS_DENIED_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACCESS_REASONS {
    pub Data: [u32; 32],
}
impl ::core::marker::Copy for ACCESS_REASONS {}
impl ::core::clone::Clone for ACCESS_REASONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ACE_FLAGS = u32;
pub const CONTAINER_INHERIT_ACE: ACE_FLAGS = 2u32;
pub const FAILED_ACCESS_ACE_FLAG: ACE_FLAGS = 128u32;
pub const INHERIT_ONLY_ACE: ACE_FLAGS = 8u32;
pub const INHERITED_ACE: ACE_FLAGS = 16u32;
pub const NO_PROPAGATE_INHERIT_ACE: ACE_FLAGS = 4u32;
pub const OBJECT_INHERIT_ACE: ACE_FLAGS = 1u32;
pub const SUCCESSFUL_ACCESS_ACE_FLAG: ACE_FLAGS = 64u32;
pub const SUB_CONTAINERS_AND_OBJECTS_INHERIT: ACE_FLAGS = 3u32;
pub const SUB_CONTAINERS_ONLY_INHERIT: ACE_FLAGS = 2u32;
pub const SUB_OBJECTS_ONLY_INHERIT: ACE_FLAGS = 1u32;
pub const INHERIT_NO_PROPAGATE: ACE_FLAGS = 4u32;
pub const INHERIT_ONLY: ACE_FLAGS = 8u32;
pub const NO_INHERITANCE: ACE_FLAGS = 0u32;
pub const INHERIT_ONLY_ACE_: ACE_FLAGS = 8u32;
#[repr(C)]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
impl ::core::marker::Copy for ACE_HEADER {}
impl ::core::clone::Clone for ACE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ACE_REVISION = u32;
pub const ACL_REVISION: ACE_REVISION = 2u32;
pub const ACL_REVISION_DS: ACE_REVISION = 4u32;
#[repr(C)]
pub struct ACL {
    pub AclRevision: u8,
    pub Sbz1: u8,
    pub AclSize: u16,
    pub AceCount: u16,
    pub Sbz2: u16,
}
impl ::core::marker::Copy for ACL {}
impl ::core::clone::Clone for ACL {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ACL_INFORMATION_CLASS = i32;
pub const AclRevisionInformation: ACL_INFORMATION_CLASS = 1i32;
pub const AclSizeInformation: ACL_INFORMATION_CLASS = 2i32;
#[repr(C)]
pub struct ACL_REVISION_INFORMATION {
    pub AclRevision: u32,
}
impl ::core::marker::Copy for ACL_REVISION_INFORMATION {}
impl ::core::clone::Clone for ACL_REVISION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACL_SIZE_INFORMATION {
    pub AceCount: u32,
    pub AclBytesInUse: u32,
    pub AclBytesFree: u32,
}
impl ::core::marker::Copy for ACL_SIZE_INFORMATION {}
impl ::core::clone::Clone for ACL_SIZE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AUDIT_EVENT_TYPE = i32;
pub const AuditEventObjectAccess: AUDIT_EVENT_TYPE = 0i32;
pub const AuditEventDirectoryServiceAccess: AUDIT_EVENT_TYPE = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut CLAIM_SECURITY_ATTRIBUTE_V1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CLAIM_SECURITY_ATTRIBUTE_FLAGS = u32;
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = 1u32;
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: CLAIM_SECURITY_ATTRIBUTE_FLAGS = 2u32;
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = 4u32;
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: CLAIM_SECURITY_ATTRIBUTE_FLAGS = 8u32;
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: CLAIM_SECURITY_ATTRIBUTE_FLAGS = 16u32;
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: CLAIM_SECURITY_ATTRIBUTE_FLAGS = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut ::core::ffi::c_void,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: u32,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: CLAIM_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0,
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    pub pInt64: [u32; 1],
    pub pUint64: [u32; 1],
    pub ppString: [u32; 1],
    pub pFqbn: [u32; 1],
    pub pOctetString: [u32; 1],
}
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {}
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: super::Foundation::PWSTR,
    pub ValueType: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_V1_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut super::Foundation::PWSTR,
    pub pFqbn: *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLAIM_SECURITY_ATTRIBUTE_V1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = u16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = 1u16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = 2u16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = 3u16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = 16u16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = 4u16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = 5u16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: CLAIM_SECURITY_ATTRIBUTE_VALUE_TYPE = 6u16;
pub type CREATE_RESTRICTED_TOKEN_FLAGS = u32;
pub const DISABLE_MAX_PRIVILEGE: CREATE_RESTRICTED_TOKEN_FLAGS = 1u32;
pub const SANDBOX_INERT: CREATE_RESTRICTED_TOKEN_FLAGS = 2u32;
pub const LUA_TOKEN: CREATE_RESTRICTED_TOKEN_FLAGS = 4u32;
pub const WRITE_RESTRICTED: CREATE_RESTRICTED_TOKEN_FLAGS = 8u32;
pub const CVT_SECONDS: u32 = 1u32;
pub type ENUM_PERIOD = i32;
pub const ENUM_PERIOD_INVALID: ENUM_PERIOD = -1i32;
pub const ENUM_PERIOD_SECONDS: ENUM_PERIOD = 0i32;
pub const ENUM_PERIOD_MINUTES: ENUM_PERIOD = 1i32;
pub const ENUM_PERIOD_HOURS: ENUM_PERIOD = 2i32;
pub const ENUM_PERIOD_DAYS: ENUM_PERIOD = 3i32;
pub const ENUM_PERIOD_WEEKS: ENUM_PERIOD = 4i32;
pub const ENUM_PERIOD_MONTHS: ENUM_PERIOD = 5i32;
pub const ENUM_PERIOD_YEARS: ENUM_PERIOD = 6i32;
#[repr(C)]
pub struct GENERIC_MAPPING {
    pub GenericRead: u32,
    pub GenericWrite: u32,
    pub GenericExecute: u32,
    pub GenericAll: u32,
}
impl ::core::marker::Copy for GENERIC_MAPPING {}
impl ::core::clone::Clone for GENERIC_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HDIAGNOSTIC_DATA_QUERY_SESSION = isize;
pub type HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION = isize;
pub type HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION = isize;
pub type HDIAGNOSTIC_EVENT_TAG_DESCRIPTION = isize;
pub type HDIAGNOSTIC_RECORD = isize;
pub type HDIAGNOSTIC_REPORT = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LLFILETIME {
    pub Anonymous: LLFILETIME_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LLFILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LLFILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union LLFILETIME_0 {
    pub ll: i64,
    pub ft: super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LLFILETIME_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LLFILETIME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LOGON32_LOGON = u32;
pub const LOGON32_LOGON_BATCH: LOGON32_LOGON = 4u32;
pub const LOGON32_LOGON_INTERACTIVE: LOGON32_LOGON = 2u32;
pub const LOGON32_LOGON_NETWORK: LOGON32_LOGON = 3u32;
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: LOGON32_LOGON = 8u32;
pub const LOGON32_LOGON_NEW_CREDENTIALS: LOGON32_LOGON = 9u32;
pub const LOGON32_LOGON_SERVICE: LOGON32_LOGON = 5u32;
pub const LOGON32_LOGON_UNLOCK: LOGON32_LOGON = 7u32;
pub type LOGON32_PROVIDER = u32;
pub const LOGON32_PROVIDER_DEFAULT: LOGON32_PROVIDER = 0u32;
pub const LOGON32_PROVIDER_WINNT50: LOGON32_PROVIDER = 3u32;
pub const LOGON32_PROVIDER_WINNT40: LOGON32_PROVIDER = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: super::Foundation::LUID,
    pub Attributes: TOKEN_PRIVILEGES_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LUID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LUID_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MANDATORY_LEVEL = i32;
pub const MandatoryLevelUntrusted: MANDATORY_LEVEL = 0i32;
pub const MandatoryLevelLow: MANDATORY_LEVEL = 1i32;
pub const MandatoryLevelMedium: MANDATORY_LEVEL = 2i32;
pub const MandatoryLevelHigh: MANDATORY_LEVEL = 3i32;
pub const MandatoryLevelSystem: MANDATORY_LEVEL = 4i32;
pub const MandatoryLevelSecureProcess: MANDATORY_LEVEL = 5i32;
pub const MandatoryLevelCount: MANDATORY_LEVEL = 6i32;
pub type NCRYPT_DESCRIPTOR_HANDLE = isize;
pub type NCRYPT_STREAM_HANDLE = isize;
pub type OBJECT_SECURITY_INFORMATION = u32;
pub const ATTRIBUTE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 32u32;
pub const BACKUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 65536u32;
pub const DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 4u32;
pub const GROUP_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 2u32;
pub const LABEL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 16u32;
pub const OWNER_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 1u32;
pub const PROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 2147483648u32;
pub const PROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 1073741824u32;
pub const SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 8u32;
pub const SCOPE_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 64u32;
pub const UNPROTECTED_DACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 536870912u32;
pub const UNPROTECTED_SACL_SECURITY_INFORMATION: OBJECT_SECURITY_INFORMATION = 268435456u32;
#[repr(C)]
pub struct OBJECT_TYPE_LIST {
    pub Level: u16,
    pub Sbz: u16,
    pub ObjectType: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for OBJECT_TYPE_LIST {}
impl ::core::clone::Clone for OBJECT_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PLSA_AP_CALL_PACKAGE_UNTRUSTED = ::core::option::Option<unsafe extern "system" fn(clientrequest: *const *const ::core::ffi::c_void, protocolsubmitbuffer: *const ::core::ffi::c_void, clientbufferbase: *const ::core::ffi::c_void, submitbufferlength: u32, protocolreturnbuffer: *mut *mut ::core::ffi::c_void, returnbufferlength: *mut u32, protocolstatus: *mut i32) -> super::Foundation::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRIVILEGE_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRIVILEGE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct QUOTA_LIMITS {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
}
impl ::core::marker::Copy for QUOTA_LIMITS {}
impl ::core::clone::Clone for QUOTA_LIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SAFER_LEVEL_HANDLE = isize;
pub type SC_HANDLE = isize;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut ::core::ffi::c_void,
    pub bInheritHandle: super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SECURITY_AUTO_INHERIT_FLAGS = u32;
pub const SEF_AVOID_OWNER_CHECK: SECURITY_AUTO_INHERIT_FLAGS = 16u32;
pub const SEF_AVOID_OWNER_RESTRICTION: SECURITY_AUTO_INHERIT_FLAGS = 4096u32;
pub const SEF_AVOID_PRIVILEGE_CHECK: SECURITY_AUTO_INHERIT_FLAGS = 8u32;
pub const SEF_DACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = 1u32;
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: SECURITY_AUTO_INHERIT_FLAGS = 4u32;
pub const SEF_DEFAULT_GROUP_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = 64u32;
pub const SEF_DEFAULT_OWNER_FROM_PARENT: SECURITY_AUTO_INHERIT_FLAGS = 32u32;
pub const SEF_MACL_NO_EXECUTE_UP: SECURITY_AUTO_INHERIT_FLAGS = 1024u32;
pub const SEF_MACL_NO_READ_UP: SECURITY_AUTO_INHERIT_FLAGS = 512u32;
pub const SEF_MACL_NO_WRITE_UP: SECURITY_AUTO_INHERIT_FLAGS = 256u32;
pub const SEF_SACL_AUTO_INHERIT: SECURITY_AUTO_INHERIT_FLAGS = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_CAPABILITIES {
    pub AppContainerSid: super::Foundation::PSID,
    pub Capabilities: *mut SID_AND_ATTRIBUTES,
    pub CapabilityCount: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_DESCRIPTOR {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: u16,
    pub Owner: super::Foundation::PSID,
    pub Group: super::Foundation::PSID,
    pub Sacl: *mut ACL,
    pub Dacl: *mut ACL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SECURITY_IMPERSONATION_LEVEL = i32;
pub const SecurityAnonymous: SECURITY_IMPERSONATION_LEVEL = 0i32;
pub const SecurityIdentification: SECURITY_IMPERSONATION_LEVEL = 1i32;
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = 2i32;
pub const SecurityDelegation: SECURITY_IMPERSONATION_LEVEL = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_QUALITY_OF_SERVICE {
    pub Length: u32,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub ContextTrackingMode: u8,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_QUALITY_OF_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_QUALITY_OF_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SEC_THREAD_START = ::core::option::Option<unsafe extern "system" fn(lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_ACCESS_REPLY {
    pub Size: u32,
    pub ResultListCount: u32,
    pub GrantedAccess: *mut u32,
    pub AccessStatus: *mut u32,
    pub AccessReason: *mut ACCESS_REASONS,
    pub Privileges: *mut *mut PRIVILEGE_SET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_ACCESS_REPLY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_ACCESS_REPLY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_ACCESS_REQUEST {
    pub Size: u32,
    pub SeSecurityDescriptor: *mut SE_SECURITY_DESCRIPTOR,
    pub DesiredAccess: u32,
    pub PreviouslyGrantedAccess: u32,
    pub PrincipalSelfSid: super::Foundation::PSID,
    pub GenericMapping: *mut GENERIC_MAPPING,
    pub ObjectTypeListCount: u32,
    pub ObjectTypeList: *mut OBJECT_TYPE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_ACCESS_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_ACCESS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_IMPERSONATION_STATE {
    pub Token: *mut ::core::ffi::c_void,
    pub CopyOnOpen: super::Foundation::BOOLEAN,
    pub EffectiveOnly: super::Foundation::BOOLEAN,
    pub Level: SECURITY_IMPERSONATION_LEVEL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_IMPERSONATION_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_IMPERSONATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SE_SECURITY_DESCRIPTOR {
    pub Size: u32,
    pub Flags: u32,
    pub SecurityDescriptor: *mut SECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SE_SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SE_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union SE_SID {
    pub Sid: SID,
    pub Buffer: [u8; 68],
}
impl ::core::marker::Copy for SE_SID {}
impl ::core::clone::Clone for SE_SID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SID {
    pub Revision: u8,
    pub SubAuthorityCount: u8,
    pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
    pub SubAuthority: [u32; 1],
}
impl ::core::marker::Copy for SID {}
impl ::core::clone::Clone for SID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: super::Foundation::PSID,
    pub Attributes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_AND_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_AND_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: u32,
    pub SidAttr: *mut SID_AND_ATTRIBUTES,
    pub Hash: [usize; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_AND_ATTRIBUTES_HASH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_AND_ATTRIBUTES_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}
impl ::core::marker::Copy for SID_IDENTIFIER_AUTHORITY {}
impl ::core::clone::Clone for SID_IDENTIFIER_AUTHORITY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SID_NAME_USE = i32;
pub const SidTypeUser: SID_NAME_USE = 1i32;
pub const SidTypeGroup: SID_NAME_USE = 2i32;
pub const SidTypeDomain: SID_NAME_USE = 3i32;
pub const SidTypeAlias: SID_NAME_USE = 4i32;
pub const SidTypeWellKnownGroup: SID_NAME_USE = 5i32;
pub const SidTypeDeletedAccount: SID_NAME_USE = 6i32;
pub const SidTypeInvalid: SID_NAME_USE = 7i32;
pub const SidTypeUnknown: SID_NAME_USE = 8i32;
pub const SidTypeComputer: SID_NAME_USE = 9i32;
pub const SidTypeLabel: SID_NAME_USE = 10i32;
pub const SidTypeLogonSession: SID_NAME_USE = 11i32;
#[repr(C)]
pub struct SYSTEM_ACCESS_FILTER_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ACCESS_FILTER_ACE {}
impl ::core::clone::Clone for SYSTEM_ACCESS_FILTER_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_ALARM_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_ALARM_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_CALLBACK_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_ALARM_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: u32,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_ALARM_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_ALARM_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_AUDIT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_AUDIT_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_CALLBACK_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_CALLBACK_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_AUDIT_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub Flags: SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: ::windows_sys::core::GUID,
    pub InheritedObjectType: ::windows_sys::core::GUID,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_AUDIT_OBJECT_ACE {}
impl ::core::clone::Clone for SYSTEM_AUDIT_OBJECT_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SYSTEM_AUDIT_OBJECT_ACE_FLAGS = u32;
pub const ACE_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = 1u32;
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: SYSTEM_AUDIT_OBJECT_ACE_FLAGS = 2u32;
#[repr(C)]
pub struct SYSTEM_MANDATORY_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_MANDATORY_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_MANDATORY_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESS_TRUST_LABEL_ACE {}
impl ::core::clone::Clone for SYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_RESOURCE_ATTRIBUTE_ACE {}
impl ::core::clone::Clone for SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_SCOPED_POLICY_ID_ACE {
    pub Header: ACE_HEADER,
    pub Mask: u32,
    pub SidStart: u32,
}
impl ::core::marker::Copy for SYSTEM_SCOPED_POLICY_ID_ACE {}
impl ::core::clone::Clone for SYSTEM_SCOPED_POLICY_ID_ACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub Privileges: *mut TOKEN_PRIVILEGES,
    pub AuthenticationId: super::Foundation::LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: u32,
    pub AppContainerNumber: u32,
    pub PackageSid: super::Foundation::PSID,
    pub CapabilitiesHash: *mut SID_AND_ATTRIBUTES_HASH,
    pub TrustLevelSid: super::Foundation::PSID,
    pub SecurityAttributes: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_ACCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_ACCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOKEN_ACCESS_MASK = u32;
pub const TOKEN_DELETE: TOKEN_ACCESS_MASK = 65536u32;
pub const TOKEN_READ_CONTROL: TOKEN_ACCESS_MASK = 131072u32;
pub const TOKEN_WRITE_DAC: TOKEN_ACCESS_MASK = 262144u32;
pub const TOKEN_WRITE_OWNER: TOKEN_ACCESS_MASK = 524288u32;
pub const TOKEN_ACCESS_SYSTEM_SECURITY: TOKEN_ACCESS_MASK = 16777216u32;
pub const TOKEN_ASSIGN_PRIMARY: TOKEN_ACCESS_MASK = 1u32;
pub const TOKEN_DUPLICATE: TOKEN_ACCESS_MASK = 2u32;
pub const TOKEN_IMPERSONATE: TOKEN_ACCESS_MASK = 4u32;
pub const TOKEN_QUERY: TOKEN_ACCESS_MASK = 8u32;
pub const TOKEN_QUERY_SOURCE: TOKEN_ACCESS_MASK = 16u32;
pub const TOKEN_ADJUST_PRIVILEGES: TOKEN_ACCESS_MASK = 32u32;
pub const TOKEN_ADJUST_GROUPS: TOKEN_ACCESS_MASK = 64u32;
pub const TOKEN_ADJUST_DEFAULT: TOKEN_ACCESS_MASK = 128u32;
pub const TOKEN_ADJUST_SESSIONID: TOKEN_ACCESS_MASK = 256u32;
pub const TOKEN_ALL_ACCESS: TOKEN_ACCESS_MASK = 983295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_APPCONTAINER_INFORMATION {
    pub TokenAppContainer: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_APPCONTAINER_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_APPCONTAINER_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TOKEN_AUDIT_POLICY {
    pub PerUserPolicy: [u8; 30],
}
impl ::core::marker::Copy for TOKEN_AUDIT_POLICY {}
impl ::core::clone::Clone for TOKEN_AUDIT_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_CONTROL {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ModifiedId: super::Foundation::LUID,
    pub TokenSource: TOKEN_SOURCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_CONTROL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: *mut ACL,
}
impl ::core::marker::Copy for TOKEN_DEFAULT_DACL {}
impl ::core::clone::Clone for TOKEN_DEFAULT_DACL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TOKEN_DEVICE_CLAIMS {}
impl ::core::clone::Clone for TOKEN_DEVICE_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: u32,
}
impl ::core::marker::Copy for TOKEN_ELEVATION {}
impl ::core::clone::Clone for TOKEN_ELEVATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOKEN_ELEVATION_TYPE = i32;
pub const TokenElevationTypeDefault: TOKEN_ELEVATION_TYPE = 1i32;
pub const TokenElevationTypeFull: TOKEN_ELEVATION_TYPE = 2i32;
pub const TokenElevationTypeLimited: TOKEN_ELEVATION_TYPE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_GROUPS {
    pub GroupCount: u32,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_GROUPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_GROUPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: u32,
    pub SidLength: u32,
    pub Sids: *mut SID_AND_ATTRIBUTES,
    pub RestrictedSidCount: u32,
    pub RestrictedSidLength: u32,
    pub RestrictedSids: *mut SID_AND_ATTRIBUTES,
    pub PrivilegeCount: u32,
    pub PrivilegeLength: u32,
    pub Privileges: *mut LUID_AND_ATTRIBUTES,
    pub AuthenticationId: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_GROUPS_AND_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_GROUPS_AND_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOKEN_INFORMATION_CLASS = i32;
pub const TokenUser: TOKEN_INFORMATION_CLASS = 1i32;
pub const TokenGroups: TOKEN_INFORMATION_CLASS = 2i32;
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = 3i32;
pub const TokenOwner: TOKEN_INFORMATION_CLASS = 4i32;
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = 5i32;
pub const TokenDefaultDacl: TOKEN_INFORMATION_CLASS = 6i32;
pub const TokenSource: TOKEN_INFORMATION_CLASS = 7i32;
pub const TokenType: TOKEN_INFORMATION_CLASS = 8i32;
pub const TokenImpersonationLevel: TOKEN_INFORMATION_CLASS = 9i32;
pub const TokenStatistics: TOKEN_INFORMATION_CLASS = 10i32;
pub const TokenRestrictedSids: TOKEN_INFORMATION_CLASS = 11i32;
pub const TokenSessionId: TOKEN_INFORMATION_CLASS = 12i32;
pub const TokenGroupsAndPrivileges: TOKEN_INFORMATION_CLASS = 13i32;
pub const TokenSessionReference: TOKEN_INFORMATION_CLASS = 14i32;
pub const TokenSandBoxInert: TOKEN_INFORMATION_CLASS = 15i32;
pub const TokenAuditPolicy: TOKEN_INFORMATION_CLASS = 16i32;
pub const TokenOrigin: TOKEN_INFORMATION_CLASS = 17i32;
pub const TokenElevationType: TOKEN_INFORMATION_CLASS = 18i32;
pub const TokenLinkedToken: TOKEN_INFORMATION_CLASS = 19i32;
pub const TokenElevation: TOKEN_INFORMATION_CLASS = 20i32;
pub const TokenHasRestrictions: TOKEN_INFORMATION_CLASS = 21i32;
pub const TokenAccessInformation: TOKEN_INFORMATION_CLASS = 22i32;
pub const TokenVirtualizationAllowed: TOKEN_INFORMATION_CLASS = 23i32;
pub const TokenVirtualizationEnabled: TOKEN_INFORMATION_CLASS = 24i32;
pub const TokenIntegrityLevel: TOKEN_INFORMATION_CLASS = 25i32;
pub const TokenUIAccess: TOKEN_INFORMATION_CLASS = 26i32;
pub const TokenMandatoryPolicy: TOKEN_INFORMATION_CLASS = 27i32;
pub const TokenLogonSid: TOKEN_INFORMATION_CLASS = 28i32;
pub const TokenIsAppContainer: TOKEN_INFORMATION_CLASS = 29i32;
pub const TokenCapabilities: TOKEN_INFORMATION_CLASS = 30i32;
pub const TokenAppContainerSid: TOKEN_INFORMATION_CLASS = 31i32;
pub const TokenAppContainerNumber: TOKEN_INFORMATION_CLASS = 32i32;
pub const TokenUserClaimAttributes: TOKEN_INFORMATION_CLASS = 33i32;
pub const TokenDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = 34i32;
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS = 35i32;
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = 36i32;
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = 37i32;
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = 38i32;
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = 39i32;
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = 40i32;
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = 41i32;
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = 42i32;
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = 43i32;
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = 44i32;
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = 45i32;
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS = 46i32;
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = 47i32;
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = 48i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_LINKED_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_LINKED_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_MANDATORY_LABEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_MANDATORY_LABEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: TOKEN_MANDATORY_POLICY_ID,
}
impl ::core::marker::Copy for TOKEN_MANDATORY_POLICY {}
impl ::core::clone::Clone for TOKEN_MANDATORY_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOKEN_MANDATORY_POLICY_ID = u32;
pub const TOKEN_MANDATORY_POLICY_OFF: TOKEN_MANDATORY_POLICY_ID = 0u32;
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: TOKEN_MANDATORY_POLICY_ID = 1u32;
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: TOKEN_MANDATORY_POLICY_ID = 2u32;
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: TOKEN_MANDATORY_POLICY_ID = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_ORIGIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_OWNER {
    pub Owner: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_OWNER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_OWNER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_PRIMARY_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_PRIMARY_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_PRIVILEGES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_PRIVILEGES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOKEN_PRIVILEGES_ATTRIBUTES = u32;
pub const SE_PRIVILEGE_ENABLED: TOKEN_PRIVILEGES_ATTRIBUTES = 2u32;
pub const SE_PRIVILEGE_ENABLED_BY_DEFAULT: TOKEN_PRIVILEGES_ATTRIBUTES = 1u32;
pub const SE_PRIVILEGE_REMOVED: TOKEN_PRIVILEGES_ATTRIBUTES = 4u32;
pub const SE_PRIVILEGE_USED_FOR_ACCESS: TOKEN_PRIVILEGES_ATTRIBUTES = 2147483648u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_SOURCE {
    pub SourceName: [super::Foundation::CHAR; 8],
    pub SourceIdentifier: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_SOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_STATISTICS {
    pub TokenId: super::Foundation::LUID,
    pub AuthenticationId: super::Foundation::LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_STATISTICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TOKEN_TYPE = i32;
pub const TokenPrimary: TOKEN_TYPE = 1i32;
pub const TokenImpersonation: TOKEN_TYPE = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_USER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_USER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TOKEN_USER_CLAIMS {}
impl ::core::clone::Clone for TOKEN_USER_CLAIMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WELL_KNOWN_SID_TYPE = i32;
pub const WinNullSid: WELL_KNOWN_SID_TYPE = 0i32;
pub const WinWorldSid: WELL_KNOWN_SID_TYPE = 1i32;
pub const WinLocalSid: WELL_KNOWN_SID_TYPE = 2i32;
pub const WinCreatorOwnerSid: WELL_KNOWN_SID_TYPE = 3i32;
pub const WinCreatorGroupSid: WELL_KNOWN_SID_TYPE = 4i32;
pub const WinCreatorOwnerServerSid: WELL_KNOWN_SID_TYPE = 5i32;
pub const WinCreatorGroupServerSid: WELL_KNOWN_SID_TYPE = 6i32;
pub const WinNtAuthoritySid: WELL_KNOWN_SID_TYPE = 7i32;
pub const WinDialupSid: WELL_KNOWN_SID_TYPE = 8i32;
pub const WinNetworkSid: WELL_KNOWN_SID_TYPE = 9i32;
pub const WinBatchSid: WELL_KNOWN_SID_TYPE = 10i32;
pub const WinInteractiveSid: WELL_KNOWN_SID_TYPE = 11i32;
pub const WinServiceSid: WELL_KNOWN_SID_TYPE = 12i32;
pub const WinAnonymousSid: WELL_KNOWN_SID_TYPE = 13i32;
pub const WinProxySid: WELL_KNOWN_SID_TYPE = 14i32;
pub const WinEnterpriseControllersSid: WELL_KNOWN_SID_TYPE = 15i32;
pub const WinSelfSid: WELL_KNOWN_SID_TYPE = 16i32;
pub const WinAuthenticatedUserSid: WELL_KNOWN_SID_TYPE = 17i32;
pub const WinRestrictedCodeSid: WELL_KNOWN_SID_TYPE = 18i32;
pub const WinTerminalServerSid: WELL_KNOWN_SID_TYPE = 19i32;
pub const WinRemoteLogonIdSid: WELL_KNOWN_SID_TYPE = 20i32;
pub const WinLogonIdsSid: WELL_KNOWN_SID_TYPE = 21i32;
pub const WinLocalSystemSid: WELL_KNOWN_SID_TYPE = 22i32;
pub const WinLocalServiceSid: WELL_KNOWN_SID_TYPE = 23i32;
pub const WinNetworkServiceSid: WELL_KNOWN_SID_TYPE = 24i32;
pub const WinBuiltinDomainSid: WELL_KNOWN_SID_TYPE = 25i32;
pub const WinBuiltinAdministratorsSid: WELL_KNOWN_SID_TYPE = 26i32;
pub const WinBuiltinUsersSid: WELL_KNOWN_SID_TYPE = 27i32;
pub const WinBuiltinGuestsSid: WELL_KNOWN_SID_TYPE = 28i32;
pub const WinBuiltinPowerUsersSid: WELL_KNOWN_SID_TYPE = 29i32;
pub const WinBuiltinAccountOperatorsSid: WELL_KNOWN_SID_TYPE = 30i32;
pub const WinBuiltinSystemOperatorsSid: WELL_KNOWN_SID_TYPE = 31i32;
pub const WinBuiltinPrintOperatorsSid: WELL_KNOWN_SID_TYPE = 32i32;
pub const WinBuiltinBackupOperatorsSid: WELL_KNOWN_SID_TYPE = 33i32;
pub const WinBuiltinReplicatorSid: WELL_KNOWN_SID_TYPE = 34i32;
pub const WinBuiltinPreWindows2000CompatibleAccessSid: WELL_KNOWN_SID_TYPE = 35i32;
pub const WinBuiltinRemoteDesktopUsersSid: WELL_KNOWN_SID_TYPE = 36i32;
pub const WinBuiltinNetworkConfigurationOperatorsSid: WELL_KNOWN_SID_TYPE = 37i32;
pub const WinAccountAdministratorSid: WELL_KNOWN_SID_TYPE = 38i32;
pub const WinAccountGuestSid: WELL_KNOWN_SID_TYPE = 39i32;
pub const WinAccountKrbtgtSid: WELL_KNOWN_SID_TYPE = 40i32;
pub const WinAccountDomainAdminsSid: WELL_KNOWN_SID_TYPE = 41i32;
pub const WinAccountDomainUsersSid: WELL_KNOWN_SID_TYPE = 42i32;
pub const WinAccountDomainGuestsSid: WELL_KNOWN_SID_TYPE = 43i32;
pub const WinAccountComputersSid: WELL_KNOWN_SID_TYPE = 44i32;
pub const WinAccountControllersSid: WELL_KNOWN_SID_TYPE = 45i32;
pub const WinAccountCertAdminsSid: WELL_KNOWN_SID_TYPE = 46i32;
pub const WinAccountSchemaAdminsSid: WELL_KNOWN_SID_TYPE = 47i32;
pub const WinAccountEnterpriseAdminsSid: WELL_KNOWN_SID_TYPE = 48i32;
pub const WinAccountPolicyAdminsSid: WELL_KNOWN_SID_TYPE = 49i32;
pub const WinAccountRasAndIasServersSid: WELL_KNOWN_SID_TYPE = 50i32;
pub const WinNTLMAuthenticationSid: WELL_KNOWN_SID_TYPE = 51i32;
pub const WinDigestAuthenticationSid: WELL_KNOWN_SID_TYPE = 52i32;
pub const WinSChannelAuthenticationSid: WELL_KNOWN_SID_TYPE = 53i32;
pub const WinThisOrganizationSid: WELL_KNOWN_SID_TYPE = 54i32;
pub const WinOtherOrganizationSid: WELL_KNOWN_SID_TYPE = 55i32;
pub const WinBuiltinIncomingForestTrustBuildersSid: WELL_KNOWN_SID_TYPE = 56i32;
pub const WinBuiltinPerfMonitoringUsersSid: WELL_KNOWN_SID_TYPE = 57i32;
pub const WinBuiltinPerfLoggingUsersSid: WELL_KNOWN_SID_TYPE = 58i32;
pub const WinBuiltinAuthorizationAccessSid: WELL_KNOWN_SID_TYPE = 59i32;
pub const WinBuiltinTerminalServerLicenseServersSid: WELL_KNOWN_SID_TYPE = 60i32;
pub const WinBuiltinDCOMUsersSid: WELL_KNOWN_SID_TYPE = 61i32;
pub const WinBuiltinIUsersSid: WELL_KNOWN_SID_TYPE = 62i32;
pub const WinIUserSid: WELL_KNOWN_SID_TYPE = 63i32;
pub const WinBuiltinCryptoOperatorsSid: WELL_KNOWN_SID_TYPE = 64i32;
pub const WinUntrustedLabelSid: WELL_KNOWN_SID_TYPE = 65i32;
pub const WinLowLabelSid: WELL_KNOWN_SID_TYPE = 66i32;
pub const WinMediumLabelSid: WELL_KNOWN_SID_TYPE = 67i32;
pub const WinHighLabelSid: WELL_KNOWN_SID_TYPE = 68i32;
pub const WinSystemLabelSid: WELL_KNOWN_SID_TYPE = 69i32;
pub const WinWriteRestrictedCodeSid: WELL_KNOWN_SID_TYPE = 70i32;
pub const WinCreatorOwnerRightsSid: WELL_KNOWN_SID_TYPE = 71i32;
pub const WinCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = 72i32;
pub const WinNonCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = 73i32;
pub const WinEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = 74i32;
pub const WinAccountReadonlyControllersSid: WELL_KNOWN_SID_TYPE = 75i32;
pub const WinBuiltinEventLogReadersGroup: WELL_KNOWN_SID_TYPE = 76i32;
pub const WinNewEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = 77i32;
pub const WinBuiltinCertSvcDComAccessGroup: WELL_KNOWN_SID_TYPE = 78i32;
pub const WinMediumPlusLabelSid: WELL_KNOWN_SID_TYPE = 79i32;
pub const WinLocalLogonSid: WELL_KNOWN_SID_TYPE = 80i32;
pub const WinConsoleLogonSid: WELL_KNOWN_SID_TYPE = 81i32;
pub const WinThisOrganizationCertificateSid: WELL_KNOWN_SID_TYPE = 82i32;
pub const WinApplicationPackageAuthoritySid: WELL_KNOWN_SID_TYPE = 83i32;
pub const WinBuiltinAnyPackageSid: WELL_KNOWN_SID_TYPE = 84i32;
pub const WinCapabilityInternetClientSid: WELL_KNOWN_SID_TYPE = 85i32;
pub const WinCapabilityInternetClientServerSid: WELL_KNOWN_SID_TYPE = 86i32;
pub const WinCapabilityPrivateNetworkClientServerSid: WELL_KNOWN_SID_TYPE = 87i32;
pub const WinCapabilityPicturesLibrarySid: WELL_KNOWN_SID_TYPE = 88i32;
pub const WinCapabilityVideosLibrarySid: WELL_KNOWN_SID_TYPE = 89i32;
pub const WinCapabilityMusicLibrarySid: WELL_KNOWN_SID_TYPE = 90i32;
pub const WinCapabilityDocumentsLibrarySid: WELL_KNOWN_SID_TYPE = 91i32;
pub const WinCapabilitySharedUserCertificatesSid: WELL_KNOWN_SID_TYPE = 92i32;
pub const WinCapabilityEnterpriseAuthenticationSid: WELL_KNOWN_SID_TYPE = 93i32;
pub const WinCapabilityRemovableStorageSid: WELL_KNOWN_SID_TYPE = 94i32;
pub const WinBuiltinRDSRemoteAccessServersSid: WELL_KNOWN_SID_TYPE = 95i32;
pub const WinBuiltinRDSEndpointServersSid: WELL_KNOWN_SID_TYPE = 96i32;
pub const WinBuiltinRDSManagementServersSid: WELL_KNOWN_SID_TYPE = 97i32;
pub const WinUserModeDriversSid: WELL_KNOWN_SID_TYPE = 98i32;
pub const WinBuiltinHyperVAdminsSid: WELL_KNOWN_SID_TYPE = 99i32;
pub const WinAccountCloneableControllersSid: WELL_KNOWN_SID_TYPE = 100i32;
pub const WinBuiltinAccessControlAssistanceOperatorsSid: WELL_KNOWN_SID_TYPE = 101i32;
pub const WinBuiltinRemoteManagementUsersSid: WELL_KNOWN_SID_TYPE = 102i32;
pub const WinAuthenticationAuthorityAssertedSid: WELL_KNOWN_SID_TYPE = 103i32;
pub const WinAuthenticationServiceAssertedSid: WELL_KNOWN_SID_TYPE = 104i32;
pub const WinLocalAccountSid: WELL_KNOWN_SID_TYPE = 105i32;
pub const WinLocalAccountAndAdministratorSid: WELL_KNOWN_SID_TYPE = 106i32;
pub const WinAccountProtectedUsersSid: WELL_KNOWN_SID_TYPE = 107i32;
pub const WinCapabilityAppointmentsSid: WELL_KNOWN_SID_TYPE = 108i32;
pub const WinCapabilityContactsSid: WELL_KNOWN_SID_TYPE = 109i32;
pub const WinAccountDefaultSystemManagedSid: WELL_KNOWN_SID_TYPE = 110i32;
pub const WinBuiltinDefaultSystemManagedGroupSid: WELL_KNOWN_SID_TYPE = 111i32;
pub const WinBuiltinStorageReplicaAdminsSid: WELL_KNOWN_SID_TYPE = 112i32;
pub const WinAccountKeyAdminsSid: WELL_KNOWN_SID_TYPE = 113i32;
pub const WinAccountEnterpriseKeyAdminsSid: WELL_KNOWN_SID_TYPE = 114i32;
pub const WinAuthenticationKeyTrustSid: WELL_KNOWN_SID_TYPE = 115i32;
pub const WinAuthenticationKeyPropertyMFASid: WELL_KNOWN_SID_TYPE = 116i32;
pub const WinAuthenticationKeyPropertyAttestationSid: WELL_KNOWN_SID_TYPE = 117i32;
pub const WinAuthenticationFreshKeyAuthSid: WELL_KNOWN_SID_TYPE = 118i32;
pub const WinBuiltinDeviceOwnersSid: WELL_KNOWN_SID_TYPE = 119i32;
