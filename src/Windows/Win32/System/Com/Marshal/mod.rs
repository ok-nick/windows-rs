#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>);
        }
        ::core::mem::transmute(BSTR_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree64(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>);
        }
        ::core::mem::transmute(BSTR_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> u32;
        }
        ::core::mem::transmute(BSTR_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> u32;
        }
        ::core::mem::transmute(BSTR_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16);
        }
        ::core::mem::transmute(CLIPFORMAT_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16);
        }
        ::core::mem::transmute(CLIPFORMAT_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::core::mem::transmute(CLIPFORMAT_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::core::mem::transmute(CLIPFORMAT_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetMarshalSizeMax<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pulsize: *mut u32, riid: *const ::windows::core::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMarshalSizeMax(pulsize: *mut u32, riid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT;
        }
        CoGetMarshalSizeMax(::core::mem::transmute(pulsize), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetStandardMarshal<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(riid: *const ::windows::core::GUID, punk: Param1, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<IMarshal> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStandardMarshal(riid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, ppmarshal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMarshal as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetStandardMarshal(::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), &mut result__).from_abi::<IMarshal>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetStdMarshalEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0, smexflags: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStdMarshalEx(punkouter: ::windows::core::RawPtr, smexflags: u32, ppunkinner: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetStdMarshalEx(punkouter.into_param().abi(), ::core::mem::transmute(smexflags), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalHresult<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalHresult(pstm: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT;
        }
        CoMarshalHresult(pstm.into_param().abi(), ::core::mem::transmute(hresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalInterThreadInterfaceInStream<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<super::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterThreadInterfaceInStream(riid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoMarshalInterThreadInterfaceInStream(::core::mem::transmute(riid), punk.into_param().abi(), &mut result__).from_abi::<super::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pstm: Param0, riid: *const ::windows::core::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterface(pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT;
        }
        CoMarshalInterface(pstm.into_param().abi(), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoReleaseMarshalData<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoReleaseMarshalData(pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoReleaseMarshalData(pstm.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoUnmarshalHresult<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::core::Result<::windows::core::HRESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalHresult(pstm: ::windows::core::RawPtr, phresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::HRESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoUnmarshalHresult(pstm.into_param().abi(), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoUnmarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>, T: ::windows::core::Interface>(pstm: Param0) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalInterface(pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoUnmarshalInterface(pstm.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
        }
        ::core::mem::transmute(HACCEL_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
        }
        ::core::mem::transmute(HACCEL_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::core::mem::transmute(HACCEL_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::core::mem::transmute(HACCEL_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
        }
        ::core::mem::transmute(HBITMAP_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
        }
        ::core::mem::transmute(HBITMAP_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
        }
        ::core::mem::transmute(HBITMAP_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
        }
        ::core::mem::transmute(HBITMAP_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
        }
        ::core::mem::transmute(HDC_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
        }
        ::core::mem::transmute(HDC_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
        }
        ::core::mem::transmute(HDC_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
        }
        ::core::mem::transmute(HDC_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize);
        }
        ::core::mem::transmute(HGLOBAL_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize);
        }
        ::core::mem::transmute(HGLOBAL_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::core::mem::transmute(HGLOBAL_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::core::mem::transmute(HGLOBAL_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
        }
        ::core::mem::transmute(HICON_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
        }
        ::core::mem::transmute(HICON_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
        }
        ::core::mem::transmute(HICON_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
        }
        ::core::mem::transmute(HICON_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
        }
        ::core::mem::transmute(HMENU_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
        }
        ::core::mem::transmute(HMENU_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
        }
        ::core::mem::transmute(HMENU_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
        }
        ::core::mem::transmute(HMENU_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
        }
        ::core::mem::transmute(HPALETTE_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
        }
        ::core::mem::transmute(HPALETTE_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
        }
        ::core::mem::transmute(HPALETTE_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
        }
        ::core::mem::transmute(HPALETTE_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
        }
        ::core::mem::transmute(HWND_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
        }
        ::core::mem::transmute(HWND_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(HWND_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(HWND_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMarshal(pub ::windows::core::IUnknown);
impl IMarshal {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(pcid)).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(psize)).ok()
    }
    pub unsafe fn MarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMarshal {
    type Vtable = IMarshal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000003_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IMarshal> for ::windows::core::IUnknown {
    fn from(value: IMarshal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMarshal> for ::windows::core::IUnknown {
    fn from(value: &IMarshal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMarshal2(pub ::windows::core::IUnknown);
impl IMarshal2 {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(pcid)).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(psize)).ok()
    }
    pub unsafe fn MarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMarshal2 {
    type Vtable = IMarshal2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001cf_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IMarshal2> for ::windows::core::IUnknown {
    fn from(value: IMarshal2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMarshal2> for ::windows::core::IUnknown {
    fn from(value: &IMarshal2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMarshal2> for IMarshal {
    fn from(value: IMarshal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshal2> for IMarshal {
    fn from(value: &IMarshal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMarshal> for IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMarshal> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMarshal> for &IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMarshal> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMarshalingStream(pub ::windows::core::IUnknown);
impl IMarshalingStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    pub unsafe fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::IStream> {
        let mut result__: <super::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IStream>(result__)
    }
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(attribute), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMarshalingStream {
    type Vtable = IMarshalingStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f2f5e6_6102_4863_9f26_389a4676efde);
}
impl ::core::convert::From<IMarshalingStream> for ::windows::core::IUnknown {
    fn from(value: IMarshalingStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMarshalingStream> for ::windows::core::IUnknown {
    fn from(value: &IMarshalingStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMarshalingStream> for super::IStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for super::IStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStream> for IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStream> for &IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMarshalingStream> for super::ISequentialStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for super::ISequentialStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ISequentialStream> for IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::ISequentialStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ISequentialStream> for &IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshalingStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dlibmove: i64, dworigin: super::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, libnewsize: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfcommitflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::core::HRESULT,
);
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        ::core::mem::transmute(LPSAFEARRAY_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        ::core::mem::transmute(LPSAFEARRAY_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16);
        }
        ::core::mem::transmute(SNB_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16);
        }
        ::core::mem::transmute(SNB_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::core::mem::transmute(SNB_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::core::mem::transmute(SNB_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct STDMSHLFLAGS(pub i32);
pub const SMEXF_SERVER: STDMSHLFLAGS = STDMSHLFLAGS(1i32);
pub const SMEXF_HANDLER: STDMSHLFLAGS = STDMSHLFLAGS(2i32);
impl ::core::convert::From<i32> for STDMSHLFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for STDMSHLFLAGS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>);
        }
        ::core::mem::transmute(STGMEDIUM_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>);
        }
        ::core::mem::transmute(STGMEDIUM_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> u32;
        }
        ::core::mem::transmute(STGMEDIUM_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> u32;
        }
        ::core::mem::transmute(STGMEDIUM_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<super::VARIANT>);
        }
        ::core::mem::transmute(VARIANT_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree64(param0: *const u32, param1: *const ::core::mem::ManuallyDrop<super::VARIANT>);
        }
        ::core::mem::transmute(VARIANT_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::core::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<super::VARIANT>) -> u32;
        }
        ::core::mem::transmute(VARIANT_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const ::core::mem::ManuallyDrop<super::VARIANT>) -> u32;
        }
        ::core::mem::transmute(VARIANT_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::core::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
