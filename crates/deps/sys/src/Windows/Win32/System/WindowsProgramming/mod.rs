#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDelBackupEntryA(lpcszfilelist: super::super::Foundation::PSTR, lpcszbackupdir: super::super::Foundation::PSTR, lpcszbasename: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDelBackupEntryW(lpcszfilelist: super::super::Foundation::PWSTR, lpcszbackupdir: super::super::Foundation::PWSTR, lpcszbasename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdvInstallFileA(hwnd: super::super::Foundation::HWND, lpszsourcedir: super::super::Foundation::PSTR, lpszsourcefile: super::super::Foundation::PSTR, lpszdestdir: super::super::Foundation::PSTR, lpszdestfile: super::super::Foundation::PSTR, dwflags: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdvInstallFileW(hwnd: super::super::Foundation::HWND, lpszsourcedir: super::super::Foundation::PWSTR, lpszsourcefile: super::super::Foundation::PWSTR, lpszdestdir: super::super::Foundation::PWSTR, lpszdestfile: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApphelpCheckShellObject(objectclsid: *const ::windows_sys::core::GUID, bshimifnecessary: super::super::Foundation::BOOL, pullflags: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelDeviceWakeupRequest(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn CloseINFEngine(hinf: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerExA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    pub fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICloseProvider(hdc: super::super::Graphics::Gdi::HDC);
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreateOffscreen(hdc: super::super::Graphics::Gdi::HDC, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreateOverlay(hdc: super::super::Graphics::Gdi::HDC, lpoffscreensurf: *mut ::core::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreatePrimary(hdc: super::super::Graphics::Gdi::HDC, lplpsurface: *mut *mut DCISURFACEINFO) -> i32;
    pub fn DCIDestroy(pdci: *mut DCISURFACEINFO);
    pub fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32;
    pub fn DCIEndAccess(pdci: *mut DCISURFACEINFO);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCIEnum(hdc: super::super::Graphics::Gdi::HDC, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeA(pszfileordirname: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeRunDLL32W(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeW(pszfileordirname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameA(hostname: super::super::Foundation::PSTR, computername: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameW(hostname: super::super::Foundation::PWSTR, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableProcessOptionalXStateFeatures(features: u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteCabA(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOA, preserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteCabW(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOW, preserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractFilesA(pszcabname: super::super::Foundation::PSTR, pszexpanddir: super::super::Foundation::PSTR, dwflags: u32, pszfilelist: super::super::Foundation::PSTR, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractFilesW(pszcabname: super::super::Foundation::PWSTR, pszexpanddir: super::super::Foundation::PWSTR, dwflags: u32, pszfilelist: super::super::Foundation::PWSTR, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveMarkNotExistA(lpfilelist: super::super::Foundation::PSTR, lpdir: super::super::Foundation::PSTR, lpbasename: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveMarkNotExistW(lpfilelist: super::super::Foundation::PWSTR, lpdir: super::super::Foundation::PWSTR, lpbasename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PSTR, pszinf: super::super::Foundation::PSTR, pszsection: super::super::Foundation::PSTR, pszbackupdir: super::super::Foundation::PSTR, pszbasebackupfile: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinf: super::super::Foundation::PWSTR, pszsection: super::super::Foundation::PWSTR, pszbackupdir: super::super::Foundation::PWSTR, pszbasebackupfile: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreW(hdlg: super::super::Foundation::HWND, lpfilelist: super::super::Foundation::PWSTR, lpdir: super::super::Foundation::PWSTR, lpbasename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> super::super::Foundation::BOOL;
    pub fn GdiEntry13() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameA(lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameW(lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetDCRegionData(hdc: super::super::Graphics::Gdi::HDC, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
    pub fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableExA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableExW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileIntA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, ndefault: i32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileIntW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, ndefault: i32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionA(lpappname: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionNamesA(lpszreturnbuffer: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionNamesW(lpszreturnbuffer: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpdefault: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpdefault: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStructA(lpszsection: super::super::Foundation::PSTR, lpszkey: super::super::Foundation::PSTR, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStructW(lpszsection: super::super::Foundation::PWSTR, lpszkey: super::super::Foundation::PWSTR, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileIntA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, ndefault: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileIntW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, ndefault: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileSectionA(lpappname: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpdefault: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpdefault: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemRegistryQuota(pdwquotaallowed: *mut u32, pdwquotaused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    pub fn GetThreadEnabledXStateFeatures() -> u64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameA(lpbuffer: super::super::Foundation::PSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameW(lpbuffer: super::super::Foundation::PWSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileA(lpszfilename: super::super::Foundation::PSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileExA(lpszfilename: super::super::Foundation::PSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileExW(lpszfilename: super::super::Foundation::PWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileW(lpszfilename: super::super::Foundation::PWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetWindowRegionData(hwnd: super::super::Foundation::HWND, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
    pub fn GlobalCompact(dwminfree: u32) -> usize;
    pub fn GlobalFix(hmem: isize);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalUnWire(hmem: isize) -> super::super::Foundation::BOOL;
    pub fn GlobalUnfix(hmem: isize);
    pub fn GlobalWire(hmem: isize) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPGetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPGetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPQueryIMEA(param0: *mut IMEPROA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPQueryIMEW(param0: *mut IMEPROW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPSetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPSetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsApiSetImplemented(contract: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadHugeReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadHugeWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNativeVhdBoot(nativevhdboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTokenUntrusted(tokenhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LaunchINFSectionExW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LaunchINFSectionW(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparams: super::super::Foundation::PWSTR, nshow: i32) -> i32;
    pub fn LocalCompact(uminfree: u32) -> usize;
    pub fn LocalShrink(hmem: isize, cbnewsize: u32) -> usize;
    pub fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NeedReboot(dwrebootcheck: u32) -> super::super::Foundation::BOOL;
    pub fn NeedRebootInit() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtClose(handle: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtDeviceIoControlFile(filehandle: super::super::Foundation::HANDLE, event: super::super::Foundation::HANDLE, apcroutine: PIO_APC_ROUTINE, apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: *mut ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtNotifyChangeMultipleKeys(
        masterkeyhandle: super::super::Foundation::HANDLE,
        count: u32,
        subordinateobjects: *const OBJECT_ATTRIBUTES,
        event: super::super::Foundation::HANDLE,
        apcroutine: PIO_APC_ROUTINE,
        apccontext: *const ::core::ffi::c_void,
        iostatusblock: *mut IO_STATUS_BLOCK,
        completionfilter: u32,
        watchtree: super::super::Foundation::BOOLEAN,
        buffer: *mut ::core::ffi::c_void,
        buffersize: u32,
        asynchronous: super::super::Foundation::BOOLEAN,
    ) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtOpenFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryMultipleValueKey(keyhandle: super::super::Foundation::HANDLE, valueentries: *mut KEY_VALUE_ENTRY, entrycount: u32, valuebuffer: *mut ::core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryObject(handle: super::super::Foundation::HANDLE, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: *mut ::core::ffi::c_void, objectinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut ::core::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQuerySystemTime(systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtRenameKey(keyhandle: super::super::Foundation::HANDLE, newname: *const super::super::Foundation::UNICODE_STRING) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtSetInformationKey(keyhandle: super::super::Foundation::HANDLE, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::core::ffi::c_void, keysetinformationlength: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtWaitForSingleObject(handle: super::super::Foundation::HANDLE, alertable: super::super::Foundation::BOOLEAN, timeout: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenINFEngineA(pszinffilename: super::super::Foundation::PSTR, pszinstallsection: super::super::Foundation::PSTR, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenINFEngineW(pszinffilename: super::super::Foundation::PWSTR, pszinstallsection: super::super::Foundation::PWSTR, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenMutexA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenSemaphoreA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWaitableTimerA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    pub fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
    pub fn QueryInterruptTime(lpinterrupttime: *mut u64);
    pub fn QueryInterruptTimePrecise(lpinterrupttimeprecise: *mut u64);
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProcessCycleTime(processhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadCycleTime(threadhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> super::super::Foundation::BOOL;
    pub fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise: *mut u64);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RebootCheckOnInstallA(hwnd: super::super::Foundation::HWND, pszinf: super::super::Foundation::PSTR, pszsec: super::super::Foundation::PSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RebootCheckOnInstallW(hwnd: super::super::Foundation::HWND, pszinf: super::super::Foundation::PWSTR, pszsec: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecordFeatureUsage(featureid: u32, kind: u32, addend: u32, originname: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegInstallA(hmod: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PSTR, psttable: *const STRTABLEA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegInstallW(hmod: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PWSTR, psttable: *const STRTABLEW) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegRestoreAllA(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PSTR, hkbckupkey: super::Registry::HKEY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegRestoreAllW(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PWSTR, hkbckupkey: super::Registry::HKEY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreA(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: super::super::Foundation::PSTR, pcszsubkey: super::super::Foundation::PSTR, pcszvaluename: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PSTR, pszinf: super::super::Foundation::PSTR, pszsection: super::super::Foundation::PSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinf: super::super::Foundation::PWSTR, pszsection: super::super::Foundation::PWSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreW(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PWSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: super::super::Foundation::PWSTR, pcszsubkey: super::super::Foundation::PWSTR, pcszvaluename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplacePartitionUnit(targetpartition: super::super::Foundation::PWSTR, sparepartition: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RequestDeviceWakeup(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlAnsiStringToUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING);
    pub fn RtlGetReturnAddressHijackTarget() -> usize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInitUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: super::super::Foundation::PWSTR);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlRaiseCustomSystemEventTrigger(triggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnicodeStringToAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnicodeStringToOemString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlUnicodeToMultiByteSize(bytesinmultibytestring: *mut u32, unicodestring: super::super::Foundation::PWSTR, bytesinunicodestring: u32) -> super::super::Foundation::NTSTATUS;
    pub fn RtlUniform(seed: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunSetupCommandA(hwnd: super::super::Foundation::HWND, szcmdname: super::super::Foundation::PSTR, szinfsection: super::super::Foundation::PSTR, szdir: super::super::Foundation::PSTR, lpsztitle: super::super::Foundation::PSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunSetupCommandW(hwnd: super::super::Foundation::HWND, szcmdname: super::super::Foundation::PWSTR, szinfsection: super::super::Foundation::PWSTR, szdir: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendIMEMessageExA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendIMEMessageExW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentStringsA(newenvironment: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableExA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableExW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
    pub fn SetHandleCount(unumber: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMessageWaitingIndicator(hmsgindicator: super::super::Foundation::HANDLE, ulmsgcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SignalObjectAndWait(hobjecttosignal: super::super::Foundation::HANDLE, hobjecttowaiton: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    pub fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: PFEATURE_STATE_CHANGE_CALLBACK, context: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringA(pszinffilename: super::super::Foundation::PSTR, pszinstallsection: super::super::Foundation::PSTR, psztranslatesection: super::super::Foundation::PSTR, psztranslatekey: super::super::Foundation::PSTR, pszbuffer: super::super::Foundation::PSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringExA(hinf: *mut ::core::ffi::c_void, pszinffilename: super::super::Foundation::PSTR, psztranslatesection: super::super::Foundation::PSTR, psztranslatekey: super::super::Foundation::PSTR, pszbuffer: super::super::Foundation::PSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringExW(hinf: *mut ::core::ffi::c_void, pszinffilename: super::super::Foundation::PWSTR, psztranslatesection: super::super::Foundation::PWSTR, psztranslatekey: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringW(pszinffilename: super::super::Foundation::PWSTR, pszinstallsection: super::super::Foundation::PWSTR, psztranslatesection: super::super::Foundation::PWSTR, psztranslatekey: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn UnsubscribeFeatureStateChangeNotification(subscription: FEATURE_STATE_CHANGE_SUBSCRIPTION);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserUnInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserUnInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSEnableIME(param0: super::super::Foundation::HWND, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSGetEnableStatus(param0: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSGetIMEHotkey(param0: super::super::Foundation::HWND) -> u32;
    pub fn WinWatchClose(hww: HWINWATCH);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchDidStatusChange(hww: HWINWATCH) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn WinWatchGetClipList(hww: HWINWATCH, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchNotify(hww: HWINWATCH, notifycallback: WINWATCHNOTIFYPROC, notifyparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchOpen(hwnd: super::super::Foundation::HWND) -> HWINWATCH;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpGetLockdownPolicy(hostinformation: *const WLDP_HOST_INFORMATION, lockdownstate: *mut u32, lockdownflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpIsClassInApprovedList(classid: *const ::windows_sys::core::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut super::super::Foundation::BOOL, optionalflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpIsDynamicCodePolicyEnabled(isenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpQueryDeviceSecurityInformation(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpQueryDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::core::ffi::c_void, imagesize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpSetDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileSectionA(lpappname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStructA(lpszsection: super::super::Foundation::PSTR, lpszkey: super::super::Foundation::PSTR, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStructW(lpszsection: super::super::Foundation::PWSTR, lpszkey: super::super::Foundation::PWSTR, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileSectionA(lpappname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    pub fn _hread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, lbytes: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn _hwrite(hfile: i32, lpbuffer: super::super::Foundation::PSTR, lbytes: i32) -> i32;
    pub fn _lclose(hfile: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lcreat(lppathname: super::super::Foundation::PSTR, iattribute: i32) -> i32;
    pub fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lopen(lppathname: super::super::Foundation::PSTR, ireadwrite: i32) -> i32;
    pub fn _lread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, ubytes: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lwrite(hfile: i32, lpbuffer: super::super::Foundation::PSTR, ubytes: u32) -> u32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrlenW(string: *const u16) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcslen(string: *const u16) -> usize;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16;
}
pub const AADBE_ADD_ENTRY: u32 = 1u32;
pub const AADBE_DEL_ENTRY: u32 = 2u32;
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32u32;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4u32;
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128u32;
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2u32;
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1u32;
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8u32;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16u32;
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: *mut ::core::ffi::c_void,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
}
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1u32;
pub const AC_LINE_BACKUP_POWER: u32 = 2u32;
pub const AC_LINE_OFFLINE: u32 = 0u32;
pub const AC_LINE_ONLINE: u32 = 1u32;
pub const AC_LINE_UNKNOWN: u32 = 255u32;
pub const ADN_DEL_IF_EMPTY: u32 = 1u32;
pub const ADN_DEL_UNC_PATHS: u32 = 8u32;
pub const ADN_DONT_DEL_DIR: u32 = 4u32;
pub const ADN_DONT_DEL_SUBDIRS: u32 = 2u32;
pub const AFSR_BACKNEW: u32 = 2u32;
pub const AFSR_EXTRAINCREFCNT: u32 = 2048u32;
pub const AFSR_NODELETENEW: u32 = 4u32;
pub const AFSR_NOMESSAGES: u32 = 8u32;
pub const AFSR_NOPROGRESS: u32 = 16u32;
pub const AFSR_RESTORE: u32 = 1u32;
pub const AFSR_UPDREFCNT: u32 = 512u32;
pub const AFSR_USEREFCNT: u32 = 1024u32;
pub const AIF_FORCE_FILE_IN_USE: u32 = 8u32;
pub const AIF_NOLANGUAGECHECK: u32 = 268435456u32;
pub const AIF_NOOVERWRITE: u32 = 16u32;
pub const AIF_NOSKIP: u32 = 2u32;
pub const AIF_NOVERSIONCHECK: u32 = 4u32;
pub const AIF_NO_VERSION_DIALOG: u32 = 32u32;
pub const AIF_QUIET: u32 = 536870912u32;
pub const AIF_REPLACEONLY: u32 = 1024u32;
pub const AIF_WARNIFSKIP: u32 = 1u32;
pub const ALINF_BKINSTALL: u32 = 32u32;
pub const ALINF_CHECKBKDATA: u32 = 128u32;
pub const ALINF_DELAYREGISTEROCX: u32 = 512u32;
pub const ALINF_NGCONV: u32 = 8u32;
pub const ALINF_QUIET: u32 = 4u32;
pub const ALINF_ROLLBACK: u32 = 64u32;
pub const ALINF_ROLLBKDOALL: u32 = 256u32;
pub const ALINF_UPDHLPDLLS: u32 = 16u32;
pub type APPLICATION_RECOVERY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pvparameter: *mut ::core::ffi::c_void) -> u32>;
pub const ARSR_NOMESSAGES: u32 = 8u32;
pub const ARSR_REGSECTION: u32 = 128u32;
pub const ARSR_REMOVREGBKDATA: u32 = 4096u32;
pub const ARSR_RESTORE: u32 = 1u32;
pub const ATOM_FLAG_GLOBAL: u32 = 2u32;
pub const AT_ARP: u32 = 640u32;
pub const AT_NULL: u32 = 642u32;
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11u32;
pub const BACKUP_INVALID: u32 = 0u32;
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536u32;
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1u32;
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768u32;
pub const BATTERY_FLAG_CHARGING: u32 = 8u32;
pub const BATTERY_FLAG_CRITICAL: u32 = 4u32;
pub const BATTERY_FLAG_HIGH: u32 = 1u32;
pub const BATTERY_FLAG_LOW: u32 = 2u32;
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128u32;
pub const BATTERY_FLAG_UNKNOWN: u32 = 255u32;
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295u32;
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINFOA {
    pub pszCab: super::super::Foundation::PSTR,
    pub pszInf: super::super::Foundation::PSTR,
    pub pszSection: super::super::Foundation::PSTR,
    pub szSrcPath: [super::super::Foundation::CHAR; 260],
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINFOW {
    pub pszCab: super::super::Foundation::PWSTR,
    pub pszInf: super::super::Foundation::PWSTR,
    pub pszSection: super::super::Foundation::PWSTR,
    pub szSrcPath: [u16; 260],
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CABINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CABINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CATID_DeleteBrowsingHistory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 835385060,
    data2: 54954,
    data3: 16528,
    data4: [160, 80, 165, 172, 137, 114, 233, 239],
};
pub const CBR_110: u32 = 110u32;
pub const CBR_115200: u32 = 115200u32;
pub const CBR_1200: u32 = 1200u32;
pub const CBR_128000: u32 = 128000u32;
pub const CBR_14400: u32 = 14400u32;
pub const CBR_19200: u32 = 19200u32;
pub const CBR_2400: u32 = 2400u32;
pub const CBR_256000: u32 = 256000u32;
pub const CBR_300: u32 = 300u32;
pub const CBR_38400: u32 = 38400u32;
pub const CBR_4800: u32 = 4800u32;
pub const CBR_56000: u32 = 56000u32;
pub const CBR_57600: u32 = 57600u32;
pub const CBR_600: u32 = 600u32;
pub const CBR_9600: u32 = 9600u32;
pub const CE_DNS: u32 = 2048u32;
pub const CE_IOE: u32 = 1024u32;
pub const CE_MODE: u32 = 32768u32;
pub const CE_OOP: u32 = 4096u32;
pub const CE_PTO: u32 = 512u32;
pub const CE_TXFULL: u32 = 256u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIENT_ID {
    pub UniqueProcess: super::super::Foundation::HANDLE,
    pub UniqueThread: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CL_NL_IP: u32 = 771u32;
pub const CL_NL_IPX: u32 = 769u32;
pub const CL_TL_NBF: u32 = 1025u32;
pub const CL_TL_UDP: u32 = 1027u32;
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128u32;
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1u32;
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512u32;
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256u32;
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096u32;
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64u32;
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2u32;
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32u32;
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8u32;
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4u32;
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16u32;
pub const CONTEXT_SIZE: u32 = 16u32;
pub const COPYFILE2_IO_CYCLE_SIZE_MAX: u32 = 1073741824u32;
pub const COPYFILE2_IO_CYCLE_SIZE_MIN: u32 = 4096u32;
pub const COPYFILE2_IO_RATE_MIN: u32 = 512u32;
pub const COPYFILE2_MESSAGE_COPY_OFFLOAD: i32 = 1i32;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: u32 = 8u32;
pub const COPY_FILE_COPY_SYMLINK: u32 = 2048u32;
pub const COPY_FILE_DIRECTORY: u32 = 128u32;
pub const COPY_FILE_DISABLE_PRE_ALLOCATION: u32 = 67108864u32;
pub const COPY_FILE_DONT_REQUEST_DEST_WRITE_DAC: u32 = 33554432u32;
pub const COPY_FILE_ENABLE_LOW_FREE_SPACE_MODE: u32 = 134217728u32;
pub const COPY_FILE_FAIL_IF_EXISTS: u32 = 1u32;
pub const COPY_FILE_IGNORE_EDP_BLOCK: u32 = 4194304u32;
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: u32 = 8388608u32;
pub const COPY_FILE_NO_BUFFERING: u32 = 4096u32;
pub const COPY_FILE_NO_OFFLOAD: u32 = 262144u32;
pub const COPY_FILE_OPEN_AND_COPY_REPARSE_POINT: u32 = 2097152u32;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: u32 = 4u32;
pub const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: u32 = 268435456u32;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: u32 = 8192u32;
pub const COPY_FILE_RESTARTABLE: u32 = 2u32;
pub const COPY_FILE_RESUME_FROM_PAUSE: u32 = 16384u32;
pub const COPY_FILE_SKIP_ALTERNATE_STREAMS: u32 = 32768u32;
pub const CO_TL_NBF: u32 = 1024u32;
pub const CO_TL_SPP: u32 = 1030u32;
pub const CO_TL_SPX: u32 = 1026u32;
pub const CO_TL_TCP: u32 = 1028u32;
pub const CP_DIRECT: u32 = 2u32;
pub const CP_HWND: u32 = 0u32;
pub const CP_LEVEL: u32 = 3u32;
pub const CP_OPEN: u32 = 1u32;
pub const CREATE_FOR_DIR: u32 = 2u32;
pub const CREATE_FOR_IMPORT: u32 = 1u32;
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub Size: u32,
    pub TriggerId: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CameraUIControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 383099582,
    data2: 45509,
    data3: 18355,
    data4: [142, 174, 204, 188, 244, 82, 199, 232],
};
#[repr(transparent)]
pub struct CameraUIControlCaptureMode(pub i32);
impl CameraUIControlCaptureMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlCaptureMode {}
impl ::core::clone::Clone for CameraUIControlCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraUIControlLinearSelectionMode(pub i32);
impl CameraUIControlLinearSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlLinearSelectionMode {}
impl ::core::clone::Clone for CameraUIControlLinearSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraUIControlMode(pub i32);
impl CameraUIControlMode {
    pub const Browse: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlMode {}
impl ::core::clone::Clone for CameraUIControlMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraUIControlPhotoFormat(pub i32);
impl CameraUIControlPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraUIControlPhotoFormat {}
impl ::core::clone::Clone for CameraUIControlPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraUIControlVideoFormat(pub i32);
impl CameraUIControlVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlVideoFormat {}
impl ::core::clone::Clone for CameraUIControlVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraUIControlViewType(pub i32);
impl CameraUIControlViewType {
    pub const SingleItem: Self = Self(0i32);
    pub const ItemList: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraUIControlViewType {}
impl ::core::clone::Clone for CameraUIControlViewType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DATETIME {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub min: u16,
    pub sec: u16,
}
impl ::core::marker::Copy for DATETIME {}
impl ::core::clone::Clone for DATETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DCICMD {
    pub dwCommand: u32,
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DCICMD {}
impl ::core::clone::Clone for DCICMD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DCICREATEINPUT {
    pub cmd: DCICMD,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwDCICaps: u32,
    pub dwBitCount: u32,
    pub lpSurface: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DCICREATEINPUT {}
impl ::core::clone::Clone for DCICREATEINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DCICREATEOFFSCREENSURFACE: u32 = 2u32;
pub const DCICREATEOVERLAYSURFACE: u32 = 3u32;
pub const DCICREATEPRIMARYSURFACE: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DCIENUMINPUT {
    pub cmd: DCICMD,
    pub rSrc: super::super::Foundation::RECT,
    pub rDst: super::super::Foundation::RECT,
    pub EnumCallback: isize,
    pub lpContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DCIENUMINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DCIENUMSURFACE: u32 = 4u32;
pub const DCIESCAPE: u32 = 5u32;
#[repr(C)]
pub struct DCIOFFSCREEN {
    pub dciInfo: DCISURFACEINFO,
    pub Draw: isize,
    pub SetClipList: isize,
    pub SetDestination: isize,
}
impl ::core::marker::Copy for DCIOFFSCREEN {}
impl ::core::clone::Clone for DCIOFFSCREEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DCIOVERLAY {
    pub dciInfo: DCISURFACEINFO,
    pub dwChromakeyValue: u32,
    pub dwChromakeyMask: u32,
}
impl ::core::marker::Copy for DCIOVERLAY {}
impl ::core::clone::Clone for DCIOVERLAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DCISURFACEINFO {
    pub dwSize: u32,
    pub dwDCICaps: u32,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lStride: i32,
    pub dwBitCount: u32,
    pub dwOffSurface: usize,
    pub wSelSurface: u16,
    pub wReserved: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub BeginAccess: isize,
    pub EndAccess: isize,
    pub DestroySurface: isize,
}
impl ::core::marker::Copy for DCISURFACEINFO {}
impl ::core::clone::Clone for DCISURFACEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DCI_1632_ACCESS: u32 = 64u32;
pub const DCI_ASYNC: u32 = 1024u32;
pub const DCI_CANOVERLAY: u32 = 65536u32;
pub const DCI_CAN_STRETCHX: u32 = 4096u32;
pub const DCI_CAN_STRETCHXN: u32 = 16384u32;
pub const DCI_CAN_STRETCHY: u32 = 8192u32;
pub const DCI_CAN_STRETCHYN: u32 = 32768u32;
pub const DCI_CHROMAKEY: u32 = 32u32;
pub const DCI_DWORDALIGN: u32 = 256u32;
pub const DCI_DWORDSIZE: u32 = 128u32;
pub const DCI_ERR_CURRENTLYNOTAVAIL: i32 = -5i32;
pub const DCI_ERR_HEIGHTALIGN: i32 = -21i32;
pub const DCI_ERR_INVALIDCLIPLIST: i32 = -15i32;
pub const DCI_ERR_INVALIDPOSITION: i32 = -13i32;
pub const DCI_ERR_INVALIDRECT: i32 = -6i32;
pub const DCI_ERR_INVALIDSTRETCH: i32 = -14i32;
pub const DCI_ERR_OUTOFMEMORY: i32 = -12i32;
pub const DCI_ERR_SURFACEISOBSCURED: i32 = -16i32;
pub const DCI_ERR_TOOBIGHEIGHT: i32 = -9i32;
pub const DCI_ERR_TOOBIGSIZE: i32 = -11i32;
pub const DCI_ERR_TOOBIGWIDTH: i32 = -10i32;
pub const DCI_ERR_UNSUPPORTEDFORMAT: i32 = -7i32;
pub const DCI_ERR_UNSUPPORTEDMASK: i32 = -8i32;
pub const DCI_ERR_WIDTHALIGN: i32 = -20i32;
pub const DCI_ERR_XALIGN: i32 = -17i32;
pub const DCI_ERR_XYALIGN: i32 = -19i32;
pub const DCI_ERR_YALIGN: i32 = -18i32;
pub const DCI_FAIL_GENERIC: i32 = -1i32;
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3i32;
pub const DCI_FAIL_UNSUPPORTED: i32 = -4i32;
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2i32;
pub const DCI_OFFSCREEN: u32 = 1u32;
pub const DCI_OK: u32 = 0u32;
pub const DCI_OVERLAY: u32 = 2u32;
pub const DCI_PRIMARY: u32 = 0u32;
pub const DCI_STATUS_CHROMAKEYCHANGED: u32 = 16u32;
pub const DCI_STATUS_FORMATCHANGED: u32 = 4u32;
pub const DCI_STATUS_POINTERCHANGED: u32 = 1u32;
pub const DCI_STATUS_STRIDECHANGED: u32 = 2u32;
pub const DCI_STATUS_SURFACEINFOCHANGED: u32 = 8u32;
pub const DCI_STATUS_WASSTILLDRAWING: u32 = 32u32;
pub const DCI_SURFACE_TYPE: u32 = 15u32;
pub const DCI_VERSION: u32 = 256u32;
pub const DCI_VISIBLE: u32 = 16u32;
pub const DCI_WRITEONLY: u32 = 512u32;
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1u32;
pub type DECISION_LOCATION = i32;
pub const DECISION_LOCATION_REFRESH_GLOBAL_DATA: DECISION_LOCATION = 0i32;
pub const DECISION_LOCATION_PARAMETER_VALIDATION: DECISION_LOCATION = 1i32;
pub const DECISION_LOCATION_AUDIT: DECISION_LOCATION = 2i32;
pub const DECISION_LOCATION_FAILED_CONVERT_GUID: DECISION_LOCATION = 3i32;
pub const DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID: DECISION_LOCATION = 4i32;
pub const DECISION_LOCATION_GLOBAL_BUILT_IN_LIST: DECISION_LOCATION = 5i32;
pub const DECISION_LOCATION_PROVIDER_BUILT_IN_LIST: DECISION_LOCATION = 6i32;
pub const DECISION_LOCATION_ENFORCE_STATE_LIST: DECISION_LOCATION = 7i32;
pub const DECISION_LOCATION_NOT_FOUND: DECISION_LOCATION = 8i32;
pub const DECISION_LOCATION_UNKNOWN: DECISION_LOCATION = 9i32;
pub const DELAYLOAD_GPA_FAILURE: u32 = 4u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA64,
    pub TargetDllName: super::super::Foundation::PSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut ::core::ffi::c_void,
    pub Unused: *mut ::core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELAYLOAD_INFO {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut IMAGE_THUNK_DATA32,
    pub TargetDllName: super::super::Foundation::PSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut ::core::ffi::c_void,
    pub Unused: *mut ::core::ffi::c_void,
    pub LastError: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELAYLOAD_INFO {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELAYLOAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: super::super::Foundation::PSTR,
    pub Ordinal: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELAYLOAD_PROC_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DELETE_BROWSING_HISTORY_COOKIES: u32 = 2u32;
pub const DELETE_BROWSING_HISTORY_DOWNLOADHISTORY: u32 = 64u32;
pub const DELETE_BROWSING_HISTORY_FORMDATA: u32 = 8u32;
pub const DELETE_BROWSING_HISTORY_HISTORY: u32 = 1u32;
pub const DELETE_BROWSING_HISTORY_PASSWORDS: u32 = 16u32;
pub const DELETE_BROWSING_HISTORY_PRESERVEFAVORITES: u32 = 32u32;
pub const DELETE_BROWSING_HISTORY_TIF: u32 = 4u32;
pub const DOCKINFO_DOCKED: u32 = 2u32;
pub const DOCKINFO_UNDOCKED: u32 = 1u32;
pub const DOCKINFO_USER_SUPPLIED: u32 = 4u32;
pub const DRIVE_CDROM: u32 = 5u32;
pub const DRIVE_FIXED: u32 = 3u32;
pub const DRIVE_NO_ROOT_DIR: u32 = 1u32;
pub const DRIVE_RAMDISK: u32 = 6u32;
pub const DRIVE_REMOTE: u32 = 4u32;
pub const DRIVE_REMOVABLE: u32 = 2u32;
pub const DRIVE_UNKNOWN: u32 = 0u32;
pub const DTR_CONTROL_DISABLE: u32 = 0u32;
pub const DTR_CONTROL_ENABLE: u32 = 1u32;
pub const DTR_CONTROL_HANDSHAKE: u32 = 2u32;
pub const DefaultBrowserSyncSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 986199075, data2: 12562, data3: 19110, data4: [155, 91, 31, 235, 35, 208, 197, 249] };
pub const EFSRPC_SECURE_ONLY: u32 = 8u32;
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16u32;
pub const EFS_USE_RECOVERY_KEYS: u32 = 1u32;
pub const ENTITY_LIST_ID: u32 = 0u32;
pub const ENTITY_TYPE_ID: u32 = 1u32;
pub type ENUM_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lpsurfaceinfo: *mut DCISURFACEINFO, lpcontext: *mut ::core::ffi::c_void)>;
pub const ER_ICMP: u32 = 896u32;
pub const EVENPARITY: u32 = 2u32;
pub const EVENTLOG_FULL_INFO: u32 = 0u32;
pub const EditionUpgradeBroker: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3290892327,
    data2: 20281,
    data3: 17887,
    data4: [146, 136, 18, 255, 107, 133, 169, 33],
};
pub const EditionUpgradeHelper: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 24604147, data2: 47535, data3: 20048, data4: [155, 28, 86, 233, 49, 22, 215, 4] };
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1u32;
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2u32;
pub type FEATURE_CHANGE_TIME = i32;
pub const FEATURE_CHANGE_TIME_READ: FEATURE_CHANGE_TIME = 0i32;
pub const FEATURE_CHANGE_TIME_MODULE_RELOAD: FEATURE_CHANGE_TIME = 1i32;
pub const FEATURE_CHANGE_TIME_SESSION: FEATURE_CHANGE_TIME = 2i32;
pub const FEATURE_CHANGE_TIME_REBOOT: FEATURE_CHANGE_TIME = 3i32;
pub type FEATURE_ENABLED_STATE = i32;
pub const FEATURE_ENABLED_STATE_DEFAULT: FEATURE_ENABLED_STATE = 0i32;
pub const FEATURE_ENABLED_STATE_DISABLED: FEATURE_ENABLED_STATE = 1i32;
pub const FEATURE_ENABLED_STATE_ENABLED: FEATURE_ENABLED_STATE = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FEATURE_ERROR {
    pub hr: ::windows_sys::core::HRESULT,
    pub lineNumber: u16,
    pub file: super::super::Foundation::PSTR,
    pub process: super::super::Foundation::PSTR,
    pub module: super::super::Foundation::PSTR,
    pub callerReturnAddressOffset: u32,
    pub callerModule: super::super::Foundation::PSTR,
    pub message: super::super::Foundation::PSTR,
    pub originLineNumber: u16,
    pub originFile: super::super::Foundation::PSTR,
    pub originModule: super::super::Foundation::PSTR,
    pub originCallerReturnAddressOffset: u32,
    pub originCallerModule: super::super::Foundation::PSTR,
    pub originName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FEATURE_ERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FEATURE_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FEATURE_STATE_CHANGE_SUBSCRIPTION = isize;
pub type FH_SERVICE_PIPE_HANDLE = isize;
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1u32;
#[repr(C)]
pub struct FILE_CASE_SENSITIVE_INFO {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_CASE_SENSITIVE_INFO {}
impl ::core::clone::Clone for FILE_CASE_SENSITIVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FILE_COMPLETE_IF_OPLOCKED: u32 = 256u32;
pub const FILE_CREATED: u32 = 2u32;
pub const FILE_CREATE_TREE_CONNECTION: u32 = 128u32;
pub const FILE_DELETE_ON_CLOSE: u32 = 4096u32;
pub const FILE_DIRECTORY_FILE: u32 = 1u32;
pub const FILE_DIR_DISALLOWED: u32 = 9u32;
pub const FILE_DISPOSITION_FLAG_DELETE: u32 = 1u32;
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: u32 = 0u32;
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: u32 = 4u32;
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: u32 = 16u32;
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: u32 = 8u32;
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: u32 = 2u32;
#[repr(C)]
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: u32,
}
impl ::core::marker::Copy for FILE_DISPOSITION_INFO_EX {}
impl ::core::clone::Clone for FILE_DISPOSITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FILE_DOES_NOT_EXIST: u32 = 5u32;
pub const FILE_ENCRYPTABLE: u32 = 0u32;
pub const FILE_EXISTS: u32 = 4u32;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
pub type FILE_INFORMATION_CLASS = i32;
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = 1i32;
pub const FILE_IS_ENCRYPTED: u32 = 1u32;
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5u32;
pub const FILE_NON_DIRECTORY_FILE: u32 = 64u32;
pub const FILE_NO_COMPRESSION: u32 = 32768u32;
pub const FILE_NO_EA_KNOWLEDGE: u32 = 512u32;
pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 8u32;
pub const FILE_OPENED: u32 = 1u32;
pub const FILE_OPEN_BY_FILE_ID: u32 = 8192u32;
pub const FILE_OPEN_FOR_BACKUP_INTENT: u32 = 16384u32;
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 8388608u32;
pub const FILE_OPEN_NO_RECALL: u32 = 4194304u32;
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024u32;
pub const FILE_OPEN_REPARSE_POINT: u32 = 2097152u32;
pub const FILE_OPEN_REQUIRING_OPLOCK: u32 = 65536u32;
pub const FILE_OVERWRITTEN: u32 = 3u32;
pub const FILE_RANDOM_ACCESS: u32 = 2048u32;
pub const FILE_READ_ONLY: u32 = 8u32;
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2u32;
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
pub const FILE_RESERVE_OPFILTER: u32 = 1048576u32;
pub const FILE_ROOT_DIR: u32 = 3u32;
pub const FILE_SEQUENTIAL_ONLY: u32 = 4u32;
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1u32;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2u32;
pub const FILE_SUPERSEDED: u32 = 0u32;
pub const FILE_SYNCHRONOUS_IO_ALERT: u32 = 16u32;
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 32u32;
pub const FILE_SYSTEM_ATTR: u32 = 2u32;
pub const FILE_SYSTEM_DIR: u32 = 4u32;
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6u32;
pub const FILE_TYPE_CHAR: u32 = 2u32;
pub const FILE_TYPE_DISK: u32 = 1u32;
pub const FILE_TYPE_PIPE: u32 = 3u32;
pub const FILE_TYPE_REMOTE: u32 = 32768u32;
pub const FILE_TYPE_UNKNOWN: u32 = 0u32;
pub const FILE_UNKNOWN: u32 = 5u32;
pub const FILE_USER_DISALLOWED: u32 = 7u32;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215u32;
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_SET_FLAGS: u32 = 54u32;
pub const FILE_WRITE_THROUGH: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1u32;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255u32;
pub const FS_CASE_IS_PRESERVED: u32 = 2u32;
pub const FS_CASE_SENSITIVE: u32 = 1u32;
pub const FS_FILE_COMPRESSION: u32 = 16u32;
pub const FS_FILE_ENCRYPTION: u32 = 131072u32;
pub const FS_PERSISTENT_ACLS: u32 = 8u32;
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4u32;
pub const FS_VOL_IS_COMPRESSED: u32 = 32768u32;
pub const GMEM_DDESHARE: u32 = 8192u32;
pub const GMEM_DISCARDABLE: u32 = 256u32;
pub const GMEM_DISCARDED: u32 = 16384u32;
pub const GMEM_INVALID_HANDLE: u32 = 32768u32;
pub const GMEM_LOCKCOUNT: u32 = 255u32;
pub const GMEM_LOWER: u32 = 4096u32;
pub const GMEM_MODIFY: u32 = 128u32;
pub const GMEM_NOCOMPACT: u32 = 16u32;
pub const GMEM_NODISCARD: u32 = 32u32;
pub const GMEM_NOTIFY: u32 = 16384u32;
pub const GMEM_NOT_BANKED: u32 = 4096u32;
pub const GMEM_SHARE: u32 = 8192u32;
pub const GMEM_VALID_FLAGS: u32 = 32626u32;
pub const HANJA_WINDOW: u32 = 2u32;
pub const HINSTANCE_ERROR: u32 = 32u32;
pub type HWINWATCH = isize;
pub const HW_PROFILE_GUIDLEN: u32 = 39u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [super::super::Foundation::CHAR; 39],
    pub szHwProfileName: [super::super::Foundation::CHAR; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HW_PROFILE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HW_PROFILE_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl ::core::marker::Copy for HW_PROFILE_INFOW {}
impl ::core::clone::Clone for HW_PROFILE_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ICameraUIControl = *mut ::core::ffi::c_void;
pub type ICameraUIControlEventCallback = *mut ::core::ffi::c_void;
pub type IClipServiceNotificationHelper = *mut ::core::ffi::c_void;
pub type IContainerActivationHelper = *mut ::core::ffi::c_void;
pub type IDefaultBrowserSyncSettings = *mut ::core::ffi::c_void;
pub type IDeleteBrowsingHistory = *mut ::core::ffi::c_void;
pub const IE4_BACKNEW: u32 = 2u32;
pub const IE4_EXTRAINCREFCNT: u32 = 2048u32;
pub const IE4_FRDOALL: u32 = 256u32;
pub const IE4_NODELETENEW: u32 = 4u32;
pub const IE4_NOENUMKEY: u32 = 32u32;
pub const IE4_NOMESSAGES: u32 = 8u32;
pub const IE4_NOPROGRESS: u32 = 16u32;
pub const IE4_NO_CRC_MAPPING: u32 = 64u32;
pub const IE4_REGSECTION: u32 = 128u32;
pub const IE4_REMOVREGBKDATA: u32 = 4096u32;
pub const IE4_RESTORE: u32 = 1u32;
pub const IE4_UPDREFCNT: u32 = 512u32;
pub const IE4_USEREFCNT: u32 = 1024u32;
pub const IE_BADID: i32 = -1i32;
pub const IE_BAUDRATE: i32 = -12i32;
pub const IE_BYTESIZE: i32 = -11i32;
pub const IE_DEFAULT: i32 = -5i32;
pub const IE_HARDWARE: i32 = -10i32;
pub const IE_MEMORY: i32 = -4i32;
pub const IE_NOPEN: i32 = -3i32;
pub const IE_OPEN: i32 = -2i32;
pub type IEditionUpgradeBroker = *mut ::core::ffi::c_void;
pub type IEditionUpgradeHelper = *mut ::core::ffi::c_void;
pub const IF_GENERIC: u32 = 512u32;
pub const IF_MIB: u32 = 514u32;
pub const IGNORE: u32 = 0u32;
#[repr(C)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA32_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl ::core::marker::Copy for IMAGE_THUNK_DATA64_0 {}
impl ::core::clone::Clone for IMAGE_THUNK_DATA64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IMEA_INIT: u32 = 1u32;
pub const IMEA_NEXT: u32 = 2u32;
pub const IMEA_PREV: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROA {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u8; 50],
    pub szName: [u8; 80],
    pub szOptions: [u8; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEPROA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROW {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u16; 50],
    pub szName: [u16; 80],
    pub szOptions: [u16; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMEPROW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRUCT {
    pub fnc: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub wCount: u32,
    pub dchSource: u32,
    pub dchDest: u32,
    pub lParam1: super::super::Foundation::LPARAM,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lParam3: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IME_BANJAtoJUNJA: u32 = 19u32;
pub const IME_ENABLE_CONVERT: u32 = 2u32;
pub const IME_ENTERWORDREGISTERMODE: u32 = 24u32;
pub const IME_GETCONVERSIONMODE: u32 = 17u32;
pub const IME_GETIMECAPS: u32 = 3u32;
pub const IME_GETOPEN: u32 = 5u32;
pub const IME_GETVERSION: u32 = 7u32;
pub const IME_JOHABtoKS: u32 = 21u32;
pub const IME_JUNJAtoBANJA: u32 = 20u32;
pub const IME_KStoJOHAB: u32 = 22u32;
pub const IME_MAXPROCESS: u32 = 32u32;
pub const IME_MODE_ALPHANUMERIC: u32 = 1u32;
pub const IME_MODE_CODEINPUT: u32 = 128u32;
pub const IME_MODE_DBCSCHAR: u32 = 16u32;
pub const IME_MODE_HANJACONVERT: u32 = 4u32;
pub const IME_MODE_HIRAGANA: u32 = 4u32;
pub const IME_MODE_KATAKANA: u32 = 2u32;
pub const IME_MODE_NOCODEINPUT: u32 = 256u32;
pub const IME_MODE_NOROMAN: u32 = 64u32;
pub const IME_MODE_ROMAN: u32 = 32u32;
pub const IME_MODE_SBCSCHAR: u32 = 2u32;
pub const IME_MOVEIMEWINDOW: u32 = 8u32;
pub const IME_REQUEST_CONVERT: u32 = 1u32;
pub const IME_RS_DISKERROR: u32 = 14u32;
pub const IME_RS_ERROR: u32 = 1u32;
pub const IME_RS_ILLEGAL: u32 = 6u32;
pub const IME_RS_INVALID: u32 = 17u32;
pub const IME_RS_NEST: u32 = 18u32;
pub const IME_RS_NOIME: u32 = 2u32;
pub const IME_RS_NOROOM: u32 = 10u32;
pub const IME_RS_NOTFOUND: u32 = 7u32;
pub const IME_RS_SYSTEMMODAL: u32 = 19u32;
pub const IME_RS_TOOLONG: u32 = 5u32;
pub const IME_SENDVKEY: u32 = 19u32;
pub const IME_SETCONVERSIONFONTEX: u32 = 25u32;
pub const IME_SETCONVERSIONMODE: u32 = 16u32;
pub const IME_SETCONVERSIONWINDOW: u32 = 8u32;
pub const IME_SETOPEN: u32 = 4u32;
pub const IME_SET_MODE: u32 = 18u32;
pub const INFINITE: u32 = 4294967295u32;
pub const INFO_CLASS_GENERIC: u32 = 256u32;
pub const INFO_CLASS_IMPLEMENTATION: u32 = 768u32;
pub const INFO_CLASS_PROTOCOL: u32 = 512u32;
pub const INFO_TYPE_ADDRESS_OBJECT: u32 = 512u32;
pub const INFO_TYPE_CONNECTION: u32 = 768u32;
pub const INFO_TYPE_PROVIDER: u32 = 256u32;
pub const INTERIM_WINDOW: u32 = 0u32;
pub const INVALID_ENTITY_INSTANCE: i32 = -1i32;
pub const IOCTL_TDI_TL_IO_CONTROL_ENDPOINT: u32 = 2162744u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_STATUS_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_STATUS_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IO_STATUS_BLOCK_0 {
    pub Status: super::super::Foundation::NTSTATUS,
    pub Pointer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_STATUS_BLOCK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_STATUS_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IR_CHANGECONVERT: u32 = 289u32;
pub const IR_CLOSECONVERT: u32 = 290u32;
pub const IR_DBCSCHAR: u32 = 352u32;
pub const IR_FULLCONVERT: u32 = 291u32;
pub const IR_IMESELECT: u32 = 304u32;
pub const IR_MODEINFO: u32 = 400u32;
pub const IR_OPENCONVERT: u32 = 288u32;
pub const IR_STRING: u32 = 320u32;
pub const IR_STRINGEND: u32 = 257u32;
pub const IR_STRINGEX: u32 = 384u32;
pub const IR_STRINGSTART: u32 = 256u32;
pub const IR_UNDETERMINE: u32 = 368u32;
pub type IWindowsLockModeHelper = *mut ::core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JAVA_TRUST {
    pub cbSize: u32,
    pub flag: u32,
    pub fAllActiveXPermissions: super::super::Foundation::BOOL,
    pub fAllPermissions: super::super::Foundation::BOOL,
    pub dwEncodingType: u32,
    pub pbJavaPermissions: *mut u8,
    pub cbJavaPermissions: u32,
    pub pbSigner: *mut u8,
    pub cbSigner: u32,
    pub pwszZone: super::super::Foundation::PWSTR,
    pub guidZone: ::windows_sys::core::GUID,
    pub hVerify: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JAVA_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
impl ::core::marker::Copy for JIT_DEBUG_INFO {}
impl ::core::clone::Clone for JIT_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type KEY_SET_INFORMATION_CLASS = i32;
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = 0i32;
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = 1i32;
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = 2i32;
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = 3i32;
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = 4i32;
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = 5i32;
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = 6i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_VALUE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [*mut ::core::ffi::c_void; 2],
    pub InMemoryOrderLinks: super::Kernel::LIST_ENTRY,
    pub Reserved2: [*mut ::core::ffi::c_void; 2],
    pub DllBase: *mut ::core::ffi::c_void,
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub FullDllName: super::super::Foundation::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut ::core::ffi::c_void; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for LDR_DATA_TABLE_ENTRY_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for LDR_DATA_TABLE_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LIS_NOGRPCONV: u32 = 2u32;
pub const LIS_QUIET: u32 = 1u32;
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4u32;
pub const LOGON32_PROVIDER_WINNT35: u32 = 1u32;
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648u32;
pub const LPTx: u32 = 128u32;
pub const MARKPARITY: u32 = 3u32;
pub const MAXINTATOM: u32 = 49152u32;
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
pub const MAX_TDI_ENTITIES: u32 = 4096u32;
pub const MCW_DEFAULT: u32 = 0u32;
pub const MCW_HIDDEN: u32 = 16u32;
pub const MCW_RECT: u32 = 1u32;
pub const MCW_SCREEN: u32 = 4u32;
pub const MCW_VERTICAL: u32 = 8u32;
pub const MCW_WINDOW: u32 = 2u32;
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MODE_WINDOW: u32 = 1u32;
pub const NOPARITY: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub ObjectName: *mut super::super::Foundation::UNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
    pub SecurityQualityOfService: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type OBJECT_INFORMATION_CLASS = i32;
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = 0i32;
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = 2i32;
pub const ODDPARITY: u32 = 1u32;
pub const OFS_MAXPATHNAME: u32 = 128u32;
pub const ONE5STOPBITS: u32 = 1u32;
pub const ONESTOPBIT: u32 = 0u32;
pub const OPERATION_API_VERSION: u32 = 1u32;
pub const OVERWRITE_HIDDEN: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationreason: u32, delayloadinfo: *const DELAYLOAD_INFO) -> *mut ::core::ffi::c_void>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONA {
    pub szGUID: [super::super::Foundation::CHAR; 59],
    pub szDispName: [super::super::Foundation::CHAR; 128],
    pub szLocale: [super::super::Foundation::CHAR; 10],
    pub szStub: [super::super::Foundation::CHAR; 1040],
    pub szVersion: [super::super::Foundation::CHAR; 32],
    pub szCompID: [super::super::Foundation::CHAR; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERUSERSECTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONW {
    pub szGUID: [u16; 59],
    pub szDispName: [u16; 128],
    pub szLocale: [u16; 10],
    pub szStub: [u16; 1040],
    pub szVersion: [u16; 32],
    pub szCompID: [u16; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERUSERSECTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PFEATURE_STATE_CHANGE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
pub type PFIBER_CALLOUT_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpparameter: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[cfg(feature = "Win32_Foundation")]
pub type PIO_APC_ROUTINE = ::core::option::Option<unsafe extern "system" fn(apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32)>;
#[cfg(feature = "Win32_Foundation")]
pub type PQUERYACTCTXW_FUNC = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL>;
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4u32;
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144u32;
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072u32;
pub type PROC_THREAD_ATTRIBUTE_NUM = i32;
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = 0i32;
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = 2i32;
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = 3i32;
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = 4i32;
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = 5i32;
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = 6i32;
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = 7i32;
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = 9i32;
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = 11i32;
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = 13i32;
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = 14i32;
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = 15i32;
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = 16i32;
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = 17i32;
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = 18i32;
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = 22i32;
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = 24i32;
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = 25i32;
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = 26i32;
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = 27i32;
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535u32;
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536u32;
pub const PROGRESS_CANCEL: u32 = 1u32;
pub const PROGRESS_CONTINUE: u32 = 0u32;
pub const PROGRESS_QUIET: u32 = 3u32;
pub const PROGRESS_STOP: u32 = 2u32;
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295u32;
#[repr(C)]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl ::core::marker::Copy for PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::core::clone::Clone for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PWINSTATIONQUERYINFORMATIONW = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: WINSTATIONINFOCLASS, param3: *mut ::core::ffi::c_void, param4: u32, param5: *mut u32) -> super::super::Foundation::BOOLEAN>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISAPPAPPROVEDBYPOLICY_API = ::core::option::Option<unsafe extern "system" fn(packagefamilyname: super::super::Foundation::PWSTR, packageversion: u64) -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISDYNAMICCODEPOLICYENABLED_API = ::core::option::Option<unsafe extern "system" fn(pbenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYDEVICESECURITYINFORMATION_API = ::core::option::Option<unsafe extern "system" fn(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYDYNAMICODETRUST_API = ::core::option::Option<unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::core::ffi::c_void, imagesize: u32) -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED2_API = ::core::option::Option<unsafe extern "system" fn(setting: super::super::Foundation::PWSTR, enabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED_API = ::core::option::Option<unsafe extern "system" fn(setting: WLDP_POLICY_SETTING, enabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNMODE_API = ::core::option::Option<unsafe extern "system" fn(lockdownmode: *mut WLDP_WINDOWS_LOCKDOWN_MODE) -> ::windows_sys::core::HRESULT>;
pub type PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<unsafe extern "system" fn(lockdownrestriction: *mut WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows_sys::core::HRESULT>;
pub type PWLDP_RESETPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn() -> ::windows_sys::core::HRESULT>;
pub type PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API = ::core::option::Option<unsafe extern "system" fn() -> ::windows_sys::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_SETDYNAMICCODETRUST_API = ::core::option::Option<unsafe extern "system" fn(hfilehandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT>;
pub type PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API = ::core::option::Option<unsafe extern "system" fn(lockdownrestriction: WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows_sys::core::HRESULT>;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16u32;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8u32;
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648u32;
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4u32;
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000u32;
#[cfg(feature = "Win32_Foundation")]
pub type REGINSTALLA = ::core::option::Option<unsafe extern "system" fn(hm: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PSTR, psttable: *mut STRTABLEA) -> ::windows_sys::core::HRESULT>;
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
pub const RESETDEV: u32 = 7u32;
pub const RESTART_MAX_CMD_LINE: u32 = 1024u32;
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8u32;
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8u32;
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
pub const RSC_FLAG_DELAYREGISTEROCX: u32 = 512u32;
pub const RSC_FLAG_INF: u32 = 1u32;
pub const RSC_FLAG_NGCONV: u32 = 8u32;
pub const RSC_FLAG_QUIET: u32 = 4u32;
pub const RSC_FLAG_SETUPAPI: u32 = 1024u32;
pub const RSC_FLAG_SKIPDISKSPACECHECK: u32 = 2u32;
pub const RSC_FLAG_UPDHLPDLLS: u32 = 16u32;
pub const RTS_CONTROL_DISABLE: u32 = 0u32;
pub const RTS_CONTROL_ENABLE: u32 = 1u32;
pub const RTS_CONTROL_HANDSHAKE: u32 = 2u32;
pub const RTS_CONTROL_TOGGLE: u32 = 3u32;
pub const RUNCMDS_DELAYPOSTCMD: u32 = 4u32;
pub const RUNCMDS_NOWAIT: u32 = 2u32;
pub const RUNCMDS_QUIET: u32 = 1u32;
pub const SCS_32BIT_BINARY: u32 = 0u32;
pub const SCS_64BIT_BINARY: u32 = 6u32;
pub const SCS_DOS_BINARY: u32 = 1u32;
pub const SCS_OS216_BINARY: u32 = 5u32;
pub const SCS_PIF_BINARY: u32 = 3u32;
pub const SCS_POSIX_BINARY: u32 = 4u32;
pub const SCS_WOW_BINARY: u32 = 2u32;
pub const SHUTDOWN_NORETRY: u32 = 1u32;
pub const SPACEPARITY: u32 = 4u32;
pub const STARTF_HOLOGRAPHIC: u32 = 262144u32;
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16u32;
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4u32;
pub const STREAM_CONTAINS_SECURITY: u32 = 2u32;
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1u32;
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0u32;
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRENTRYA {
    pub pszName: super::super::Foundation::PSTR,
    pub pszValue: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRENTRYW {
    pub pszName: super::super::Foundation::PWSTR,
    pub pszValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STRINGEXSTRUCT {
    pub dwSize: u32,
    pub uDeterminePos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for STRINGEXSTRUCT {}
impl ::core::clone::Clone for STRINGEXSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRTABLEA {
    pub cEntries: u32,
    pub pse: *mut STRENTRYA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRTABLEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRTABLEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRTABLEW {
    pub cEntries: u32,
    pub pse: *mut STRENTRYW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRTABLEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRTABLEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [*mut ::core::ffi::c_void; 4],
    pub NumberOfProcessors: i8,
}
impl ::core::marker::Copy for SYSTEM_BASIC_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
impl ::core::marker::Copy for SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl ::core::marker::Copy for SYSTEM_EXCEPTION_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type SYSTEM_INFORMATION_CLASS = i32;
pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = 0i32;
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = 2i32;
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = 3i32;
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = 5i32;
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = 8i32;
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = 23i32;
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = 33i32;
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS = 37i32;
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = 45i32;
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS = 103i32;
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = 134i32;
#[repr(C)]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl ::core::marker::Copy for SYSTEM_INTERRUPT_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_INTERRUPT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl ::core::marker::Copy for SYSTEM_LOOKASIDE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_LOOKASIDE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl ::core::marker::Copy for SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut ::core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl ::core::marker::Copy for SYSTEM_POLICY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_POLICY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl ::core::marker::Copy for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::super::Foundation::UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: super::super::Foundation::HANDLE,
    pub Reserved2: *mut ::core::ffi::c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut ::core::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut ::core::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYSTEM_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: *mut ::core::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl ::core::marker::Copy for SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_TIMEOFDAY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const S_ALLTHRESHOLD: u32 = 2u32;
pub const S_LEGATO: u32 = 1u32;
pub const S_NORMAL: u32 = 0u32;
pub const S_PERIOD1024: u32 = 1u32;
pub const S_PERIOD2048: u32 = 2u32;
pub const S_PERIOD512: u32 = 0u32;
pub const S_PERIODVOICE: u32 = 3u32;
pub const S_QUEUEEMPTY: u32 = 0u32;
pub const S_SERBDNT: i32 = -5i32;
pub const S_SERDCC: i32 = -7i32;
pub const S_SERDDR: i32 = -14i32;
pub const S_SERDFQ: i32 = -13i32;
pub const S_SERDLN: i32 = -6i32;
pub const S_SERDMD: i32 = -10i32;
pub const S_SERDPT: i32 = -12i32;
pub const S_SERDSH: i32 = -11i32;
pub const S_SERDSR: i32 = -15i32;
pub const S_SERDST: i32 = -16i32;
pub const S_SERDTP: i32 = -8i32;
pub const S_SERDVL: i32 = -9i32;
pub const S_SERDVNA: i32 = -1i32;
pub const S_SERMACT: i32 = -3i32;
pub const S_SEROFM: i32 = -2i32;
pub const S_SERQFUL: i32 = -4i32;
pub const S_STACCATO: u32 = 2u32;
pub const S_THRESHOLD: u32 = 1u32;
pub const S_WHITE1024: u32 = 5u32;
pub const S_WHITE2048: u32 = 6u32;
pub const S_WHITE512: u32 = 4u32;
pub const S_WHITEVOICE: u32 = 7u32;
pub const TC_GP_TRAP: u32 = 2u32;
pub const TC_HARDERR: u32 = 1u32;
pub const TC_NORMAL: u32 = 0u32;
pub const TC_SIGNAL: u32 = 3u32;
pub type TDIENTITY_ENTITY_TYPE = u32;
pub const GENERIC_ENTITY: TDIENTITY_ENTITY_TYPE = 0u32;
pub const AT_ENTITY: TDIENTITY_ENTITY_TYPE = 640u32;
pub const CL_NL_ENTITY: TDIENTITY_ENTITY_TYPE = 769u32;
pub const CO_NL_ENTITY: TDIENTITY_ENTITY_TYPE = 768u32;
pub const CL_TL_ENTITY: TDIENTITY_ENTITY_TYPE = 1025u32;
pub const CO_TL_ENTITY: TDIENTITY_ENTITY_TYPE = 1024u32;
pub const ER_ENTITY: TDIENTITY_ENTITY_TYPE = 896u32;
pub const IF_ENTITY: TDIENTITY_ENTITY_TYPE = 512u32;
#[repr(C)]
pub struct TDIEntityID {
    pub tei_entity: TDIENTITY_ENTITY_TYPE,
    pub tei_instance: u32,
}
impl ::core::marker::Copy for TDIEntityID {}
impl ::core::clone::Clone for TDIEntityID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TDIObjectID {
    pub toi_entity: TDIEntityID,
    pub toi_class: u32,
    pub toi_type: u32,
    pub toi_id: u32,
}
impl ::core::marker::Copy for TDIObjectID {}
impl ::core::clone::Clone for TDIObjectID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TDI_TL_IO_CONTROL_ENDPOINT {
    pub Type: TDI_TL_IO_CONTROL_TYPE,
    pub Level: u32,
    pub Anonymous: TDI_TL_IO_CONTROL_ENDPOINT_0,
    pub InputBuffer: *mut ::core::ffi::c_void,
    pub InputBufferLength: u32,
    pub OutputBuffer: *mut ::core::ffi::c_void,
    pub OutputBufferLength: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union TDI_TL_IO_CONTROL_ENDPOINT_0 {
    pub IoControlCode: u32,
    pub OptionName: u32,
}
impl ::core::marker::Copy for TDI_TL_IO_CONTROL_ENDPOINT_0 {}
impl ::core::clone::Clone for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TDI_TL_IO_CONTROL_TYPE = i32;
pub const EndpointIoControlType: TDI_TL_IO_CONTROL_TYPE = 0i32;
pub const SetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = 1i32;
pub const GetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = 2i32;
pub const SocketIoControlType: TDI_TL_IO_CONTROL_TYPE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for THREAD_NAME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647u32;
pub const TWOSTOPBITS: u32 = 2u32;
pub const UMS_VERSION: u32 = 256u32;
#[repr(C)]
pub struct UNDETERMINESTRUCT {
    pub dwSize: u32,
    pub uDefIMESize: u32,
    pub uDefIMEPos: u32,
    pub uUndetTextLen: u32,
    pub uUndetTextPos: u32,
    pub uUndetAttrPos: u32,
    pub uCursorPos: u32,
    pub uDeltaStart: u32,
    pub uDetermineTextLen: u32,
    pub uDetermineTextPos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiTextLen: u32,
    pub uYomiTextPos: u32,
    pub uYomiDelimPos: u32,
}
impl ::core::marker::Copy for UNDETERMINESTRUCT {}
impl ::core::clone::Clone for UNDETERMINESTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VALUENAME = i32;
pub const VALUENAME_UNKNOWN: VALUENAME = 0i32;
pub const VALUENAME_ENTERPRISE_DEFINED_CLASS_ID: VALUENAME = 1i32;
pub const VALUENAME_BUILT_IN_LIST: VALUENAME = 2i32;
pub const VOLUME_NAME_DOS: u32 = 0u32;
pub const VOLUME_NAME_GUID: u32 = 1u32;
pub const VOLUME_NAME_NONE: u32 = 4u32;
pub const VOLUME_NAME_NT: u32 = 2u32;
pub type WINSTATIONINFOCLASS = i32;
pub const WinStationInformation: WINSTATIONINFOCLASS = 8i32;
#[repr(C)]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl ::core::marker::Copy for WINSTATIONINFORMATIONW {}
impl ::core::clone::Clone for WINSTATIONINFORMATIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type WINWATCHNOTIFYPROC = ::core::option::Option<unsafe extern "system" fn(hww: HWINWATCH, hwnd: super::super::Foundation::HWND, code: u32, lparam: super::super::Foundation::LPARAM)>;
pub const WINWATCHNOTIFY_CHANGED: u32 = 4u32;
pub const WINWATCHNOTIFY_CHANGING: u32 = 3u32;
pub const WINWATCHNOTIFY_DESTROY: u32 = 2u32;
pub const WINWATCHNOTIFY_START: u32 = 0u32;
pub const WINWATCHNOTIFY_STOP: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLDP_DEVICE_SECURITY_INFORMATION {
    pub UnlockIdSize: u32,
    pub UnlockId: *mut u8,
    pub ManufacturerIDLength: u32,
    pub ManufacturerID: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLDP_DEVICE_SECURITY_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLDP_DEVICE_SECURITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WLDP_FLAGS_SKIPSIGNATUREVALIDATION: u32 = 256u32;
pub type WLDP_HOST = i32;
pub const WLDP_HOST_RUNDLL32: WLDP_HOST = 0i32;
pub const WLDP_HOST_SVCHOST: WLDP_HOST = 1i32;
pub const WLDP_HOST_MAX: WLDP_HOST = 2i32;
pub type WLDP_HOST_ID = i32;
pub const WLDP_HOST_ID_UNKNOWN: WLDP_HOST_ID = 0i32;
pub const WLDP_HOST_ID_GLOBAL: WLDP_HOST_ID = 1i32;
pub const WLDP_HOST_ID_VBA: WLDP_HOST_ID = 2i32;
pub const WLDP_HOST_ID_WSH: WLDP_HOST_ID = 3i32;
pub const WLDP_HOST_ID_POWERSHELL: WLDP_HOST_ID = 4i32;
pub const WLDP_HOST_ID_IE: WLDP_HOST_ID = 5i32;
pub const WLDP_HOST_ID_MSI: WLDP_HOST_ID = 6i32;
pub const WLDP_HOST_ID_ALL: WLDP_HOST_ID = 7i32;
pub const WLDP_HOST_ID_MAX: WLDP_HOST_ID = 8i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: u32,
    pub dwHostId: WLDP_HOST_ID,
    pub szSource: super::super::Foundation::PWSTR,
    pub hSource: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLDP_HOST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WLDP_HOST_INFORMATION_REVISION: u32 = 1u32;
pub type WLDP_KEY = i32;
pub const KEY_UNKNOWN: WLDP_KEY = 0i32;
pub const KEY_OVERRIDE: WLDP_KEY = 1i32;
pub const KEY_ALL_KEYS: WLDP_KEY = 2i32;
pub const WLDP_LOCKDOWN_AUDIT_FLAG: u32 = 8u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG: u32 = 2u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_FLAG: u32 = 1u32;
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_EXCLUSION_FLAG: u32 = 16u32;
pub const WLDP_LOCKDOWN_OFF: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_UMCIENFORCE_FLAG: u32 = 4u32;
pub const WLDP_LOCKDOWN_UNDEFINED: u32 = 0u32;
pub type WLDP_POLICY_SETTING = i32;
pub const WLDP_POLICY_SETTING_AV_PERF_MODE: WLDP_POLICY_SETTING = 1000i32;
pub type WLDP_WINDOWS_LOCKDOWN_MODE = i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = 0i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: WLDP_WINDOWS_LOCKDOWN_MODE = 1i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = 2i32;
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: WLDP_WINDOWS_LOCKDOWN_MODE = 3i32;
pub type WLDP_WINDOWS_LOCKDOWN_RESTRICTION = i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 0i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 1i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 2i32;
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = 3i32;
pub const WM_CONVERTREQUEST: u32 = 266u32;
pub const WM_CONVERTRESULT: u32 = 267u32;
pub const WM_IMEKEYDOWN: u32 = 656u32;
pub const WM_IMEKEYUP: u32 = 657u32;
pub const WM_IME_REPORT: u32 = 640u32;
pub const WM_INTERIM: u32 = 268u32;
pub const WM_WNT_CONVERTREQUESTEX: u32 = 265u32;
#[repr(C)]
pub struct _D3DHAL_CALLBACKS(pub u8);
#[repr(C)]
pub struct _D3DHAL_GLOBALDRIVERDATA(pub u8);
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct tcp_request_query_information_ex32_xp {
    pub ID: TDIObjectID,
    pub Context: [u32; 4],
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for tcp_request_query_information_ex32_xp {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for tcp_request_query_information_ex32_xp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_request_query_information_ex_w2k {
    pub ID: TDIObjectID,
    pub Context: [u8; 16],
}
impl ::core::marker::Copy for tcp_request_query_information_ex_w2k {}
impl ::core::clone::Clone for tcp_request_query_information_ex_w2k {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_request_query_information_ex_xp {
    pub ID: TDIObjectID,
    pub Context: [usize; 2],
}
impl ::core::marker::Copy for tcp_request_query_information_ex_xp {}
impl ::core::clone::Clone for tcp_request_query_information_ex_xp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tcp_request_set_information_ex {
    pub ID: TDIObjectID,
    pub BufferSize: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for tcp_request_set_information_ex {}
impl ::core::clone::Clone for tcp_request_set_information_ex {
    fn clone(&self) -> Self {
        *self
    }
}
