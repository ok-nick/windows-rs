#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Media_Audio")]
pub mod Audio;
#[cfg(feature = "Win32_Media_DeviceManager")]
pub mod DeviceManager;
#[cfg(feature = "Win32_Media_DirectShow")]
pub mod DirectShow;
#[cfg(feature = "Win32_Media_DxMediaObjects")]
pub mod DxMediaObjects;
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub mod KernelStreaming;
#[cfg(feature = "Win32_Media_LibrarySharingServices")]
pub mod LibrarySharingServices;
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub mod MediaFoundation;
#[cfg(feature = "Win32_Media_MediaPlayer")]
pub mod MediaPlayer;
#[cfg(feature = "Win32_Media_Multimedia")]
pub mod Multimedia;
#[cfg(feature = "Win32_Media_PictureAcquisition")]
pub mod PictureAcquisition;
#[cfg(feature = "Win32_Media_Speech")]
pub mod Speech;
#[cfg(feature = "Win32_Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Win32_Media_WindowsMediaFormat")]
pub mod WindowsMediaFormat;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HTASK(pub isize);
impl ::core::default::Default for HTASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HTASK {}
unsafe impl ::windows::core::Abi for HTASK {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceClock(pub ::windows::core::IUnknown);
impl IReferenceClock {
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseTime<'a, Param2: ::windows::core::IntoParam<'a, super::Foundation::HANDLE>>(&self, basetime: i64, streamtime: i64, hevent: Param2) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(basetime), ::core::mem::transmute(streamtime), hevent.into_param().abi(), &mut result__).from_abi::<usize>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdvisePeriodic<'a, Param2: ::windows::core::IntoParam<'a, super::Foundation::HANDLE>>(&self, starttime: i64, periodtime: i64, hsemaphore: Param2) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(periodtime), hsemaphore.into_param().abi(), &mut result__).from_abi::<usize>(result__)
    }
    pub unsafe fn Unadvise(&self, dwadvisecookie: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwadvisecookie)).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceClock {
    type Vtable = IReferenceClock_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56a86897_0ad4_11ce_b03a_0020af0ba770);
}
impl ::core::convert::From<IReferenceClock> for ::windows::core::IUnknown {
    fn from(value: IReferenceClock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceClock> for ::windows::core::IUnknown {
    fn from(value: &IReferenceClock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptime: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwadvisecookie: usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceClock2(pub ::windows::core::IUnknown);
impl IReferenceClock2 {
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseTime<'a, Param2: ::windows::core::IntoParam<'a, super::Foundation::HANDLE>>(&self, basetime: i64, streamtime: i64, hevent: Param2) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(basetime), ::core::mem::transmute(streamtime), hevent.into_param().abi(), &mut result__).from_abi::<usize>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdvisePeriodic<'a, Param2: ::windows::core::IntoParam<'a, super::Foundation::HANDLE>>(&self, starttime: i64, periodtime: i64, hsemaphore: Param2) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(starttime), ::core::mem::transmute(periodtime), hsemaphore.into_param().abi(), &mut result__).from_abi::<usize>(result__)
    }
    pub unsafe fn Unadvise(&self, dwadvisecookie: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwadvisecookie)).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceClock2 {
    type Vtable = IReferenceClock2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36b73885_c2c8_11cf_8b46_00805f6cef60);
}
impl ::core::convert::From<IReferenceClock2> for ::windows::core::IUnknown {
    fn from(value: IReferenceClock2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceClock2> for ::windows::core::IUnknown {
    fn from(value: &IReferenceClock2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IReferenceClock2> for IReferenceClock {
    fn from(value: IReferenceClock2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceClock2> for IReferenceClock {
    fn from(value: &IReferenceClock2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReferenceClock> for IReferenceClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, IReferenceClock> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReferenceClock> for &IReferenceClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, IReferenceClock> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptime: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwadvisecookie: usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReferenceClockTimerControl(pub ::windows::core::IUnknown);
impl IReferenceClockTimerControl {
    pub unsafe fn SetDefaultTimerResolution(&self, timerresolution: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(timerresolution)).ok()
    }
    pub unsafe fn GetDefaultTimerResolution(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IReferenceClockTimerControl {
    type Vtable = IReferenceClockTimerControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebec459c_2eca_4d42_a8af_30df557614b8);
}
impl ::core::convert::From<IReferenceClockTimerControl> for ::windows::core::IUnknown {
    fn from(value: IReferenceClockTimerControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceClockTimerControl> for ::windows::core::IUnknown {
    fn from(value: &IReferenceClockTimerControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceClockTimerControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceClockTimerControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClockTimerControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timerresolution: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptimerresolution: *mut i64) -> ::windows::core::HRESULT,
);
pub const JOYERR_BASE: u32 = 160u32;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPDRVCALLBACK = unsafe extern "system" fn(hdrvr: Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
pub type LPTIMECALLBACK = unsafe extern "system" fn(utimerid: u32, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
pub const MAXERRORLENGTH: u32 = 256u32;
pub const MAXPNAMELEN: u32 = 32u32;
pub const MCIERR_BASE: u32 = 256u32;
pub const MCI_CD_OFFSET: u32 = 1088u32;
pub const MCI_SEQ_OFFSET: u32 = 1216u32;
pub const MCI_STRING_OFFSET: u32 = 512u32;
pub const MCI_VD_OFFSET: u32 = 1024u32;
pub const MCI_WAVE_OFFSET: u32 = 1152u32;
pub const MIDIERR_BASE: u32 = 64u32;
pub const MIXERR_BASE: u32 = 1024u32;
pub const MMSYSERR_ALLOCATED: u32 = 4u32;
pub const MMSYSERR_BADDB: u32 = 14u32;
pub const MMSYSERR_BADDEVICEID: u32 = 2u32;
pub const MMSYSERR_BADERRNUM: u32 = 9u32;
pub const MMSYSERR_BASE: u32 = 0u32;
pub const MMSYSERR_DELETEERROR: u32 = 18u32;
pub const MMSYSERR_ERROR: u32 = 1u32;
pub const MMSYSERR_HANDLEBUSY: u32 = 12u32;
pub const MMSYSERR_INVALFLAG: u32 = 10u32;
pub const MMSYSERR_INVALHANDLE: u32 = 5u32;
pub const MMSYSERR_INVALIDALIAS: u32 = 13u32;
pub const MMSYSERR_INVALPARAM: u32 = 11u32;
pub const MMSYSERR_KEYNOTFOUND: u32 = 15u32;
pub const MMSYSERR_LASTERROR: u32 = 21u32;
pub const MMSYSERR_MOREDATA: u32 = 21u32;
pub const MMSYSERR_NODRIVER: u32 = 6u32;
pub const MMSYSERR_NODRIVERCB: u32 = 20u32;
pub const MMSYSERR_NOERROR: u32 = 0u32;
pub const MMSYSERR_NOMEM: u32 = 7u32;
pub const MMSYSERR_NOTENABLED: u32 = 3u32;
pub const MMSYSERR_NOTSUPPORTED: u32 = 8u32;
pub const MMSYSERR_READERROR: u32 = 16u32;
pub const MMSYSERR_VALNOTFOUND: u32 = 19u32;
pub const MMSYSERR_WRITEERROR: u32 = 17u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MMTIME {
    pub wType: u32,
    pub u: MMTIME_0,
}
impl MMTIME {}
impl ::core::default::Default for MMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MMTIME {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MMTIME {}
unsafe impl ::windows::core::Abi for MMTIME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MMTIME_0 {
    pub ms: u32,
    pub sample: u32,
    pub cb: u32,
    pub ticks: u32,
    pub smpte: MMTIME_0_1,
    pub midi: MMTIME_0_0,
}
impl MMTIME_0 {}
impl ::core::default::Default for MMTIME_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MMTIME_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MMTIME_0 {}
unsafe impl ::windows::core::Abi for MMTIME_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MMTIME_0_0 {
    pub songptrpos: u32,
}
impl MMTIME_0_0 {}
impl ::core::default::Default for MMTIME_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MMTIME_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MMTIME_0_0 {}
unsafe impl ::windows::core::Abi for MMTIME_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MMTIME_0_1 {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub frame: u8,
    pub fps: u8,
    pub dummy: u8,
    pub pad: [u8; 2],
}
impl MMTIME_0_1 {}
impl ::core::default::Default for MMTIME_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MMTIME_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_smpte_e__Struct").field("hour", &self.hour).field("min", &self.min).field("sec", &self.sec).field("frame", &self.frame).field("fps", &self.fps).field("dummy", &self.dummy).field("pad", &self.pad).finish()
    }
}
impl ::core::cmp::PartialEq for MMTIME_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.min == other.min && self.sec == other.sec && self.frame == other.frame && self.fps == other.fps && self.dummy == other.dummy && self.pad == other.pad
    }
}
impl ::core::cmp::Eq for MMTIME_0_1 {}
unsafe impl ::windows::core::Abi for MMTIME_0_1 {
    type Abi = Self;
}
pub const MM_ADLIB: u32 = 9u32;
pub const MM_DRVM_CLOSE: u32 = 977u32;
pub const MM_DRVM_DATA: u32 = 978u32;
pub const MM_DRVM_ERROR: u32 = 979u32;
pub const MM_DRVM_OPEN: u32 = 976u32;
pub const MM_JOY1BUTTONDOWN: u32 = 949u32;
pub const MM_JOY1BUTTONUP: u32 = 951u32;
pub const MM_JOY1MOVE: u32 = 928u32;
pub const MM_JOY1ZMOVE: u32 = 930u32;
pub const MM_JOY2BUTTONDOWN: u32 = 950u32;
pub const MM_JOY2BUTTONUP: u32 = 952u32;
pub const MM_JOY2MOVE: u32 = 929u32;
pub const MM_JOY2ZMOVE: u32 = 931u32;
pub const MM_MCINOTIFY: u32 = 953u32;
pub const MM_MCISIGNAL: u32 = 971u32;
pub const MM_MICROSOFT: u32 = 1u32;
pub const MM_MIDI_MAPPER: u32 = 1u32;
pub const MM_MIM_CLOSE: u32 = 962u32;
pub const MM_MIM_DATA: u32 = 963u32;
pub const MM_MIM_ERROR: u32 = 965u32;
pub const MM_MIM_LONGDATA: u32 = 964u32;
pub const MM_MIM_LONGERROR: u32 = 966u32;
pub const MM_MIM_MOREDATA: u32 = 972u32;
pub const MM_MIM_OPEN: u32 = 961u32;
pub const MM_MIXM_CONTROL_CHANGE: u32 = 977u32;
pub const MM_MIXM_LINE_CHANGE: u32 = 976u32;
pub const MM_MOM_CLOSE: u32 = 968u32;
pub const MM_MOM_DONE: u32 = 969u32;
pub const MM_MOM_OPEN: u32 = 967u32;
pub const MM_MOM_POSITIONCB: u32 = 970u32;
pub const MM_MPU401_MIDIIN: u32 = 11u32;
pub const MM_MPU401_MIDIOUT: u32 = 10u32;
pub const MM_PC_JOYSTICK: u32 = 12u32;
pub const MM_SNDBLST_MIDIIN: u32 = 4u32;
pub const MM_SNDBLST_MIDIOUT: u32 = 3u32;
pub const MM_SNDBLST_SYNTH: u32 = 5u32;
pub const MM_SNDBLST_WAVEIN: u32 = 7u32;
pub const MM_SNDBLST_WAVEOUT: u32 = 6u32;
pub const MM_STREAM_CLOSE: u32 = 981u32;
pub const MM_STREAM_DONE: u32 = 982u32;
pub const MM_STREAM_ERROR: u32 = 983u32;
pub const MM_STREAM_OPEN: u32 = 980u32;
pub const MM_WAVE_MAPPER: u32 = 2u32;
pub const MM_WIM_CLOSE: u32 = 959u32;
pub const MM_WIM_DATA: u32 = 960u32;
pub const MM_WIM_OPEN: u32 = 958u32;
pub const MM_WOM_CLOSE: u32 = 956u32;
pub const MM_WOM_DONE: u32 = 957u32;
pub const MM_WOM_OPEN: u32 = 955u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TIMECAPS {
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
}
impl TIMECAPS {}
impl ::core::default::Default for TIMECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TIMECAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TIMECAPS").field("wPeriodMin", &self.wPeriodMin).field("wPeriodMax", &self.wPeriodMax).finish()
    }
}
impl ::core::cmp::PartialEq for TIMECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.wPeriodMin == other.wPeriodMin && self.wPeriodMax == other.wPeriodMax
    }
}
impl ::core::cmp::Eq for TIMECAPS {}
unsafe impl ::windows::core::Abi for TIMECAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union TIMECODE {
    pub Anonymous: TIMECODE_0,
    pub qw: u64,
}
impl TIMECODE {}
impl ::core::default::Default for TIMECODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIMECODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TIMECODE {}
unsafe impl ::windows::core::Abi for TIMECODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TIMECODE_0 {
    pub wFrameRate: u16,
    pub wFrameFract: u16,
    pub dwFrames: u32,
}
impl TIMECODE_0 {}
impl ::core::default::Default for TIMECODE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TIMECODE_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("wFrameRate", &self.wFrameRate).field("wFrameFract", &self.wFrameFract).field("dwFrames", &self.dwFrames).finish()
    }
}
impl ::core::cmp::PartialEq for TIMECODE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wFrameRate == other.wFrameRate && self.wFrameFract == other.wFrameFract && self.dwFrames == other.dwFrames
    }
}
impl ::core::cmp::Eq for TIMECODE_0 {}
unsafe impl ::windows::core::Abi for TIMECODE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TIMECODE_SAMPLE {
    pub qwTick: i64,
    pub timecode: TIMECODE,
    pub dwUser: u32,
    pub dwFlags: TIMECODE_SAMPLE_FLAGS,
}
impl TIMECODE_SAMPLE {}
impl ::core::default::Default for TIMECODE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIMECODE_SAMPLE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for TIMECODE_SAMPLE {}
unsafe impl ::windows::core::Abi for TIMECODE_SAMPLE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TIMECODE_SAMPLE_FLAGS(pub u32);
pub const ED_DEVCAP_TIMECODE_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(4121u32);
pub const ED_DEVCAP_ATN_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5047u32);
pub const ED_DEVCAP_RTC_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5050u32);
impl ::core::convert::From<u32> for TIMECODE_SAMPLE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TIMECODE_SAMPLE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TIMERR_BASE: u32 = 96u32;
pub const TIMERR_NOCANDO: u32 = 97u32;
pub const TIMERR_NOERROR: u32 = 0u32;
pub const TIMERR_STRUCT: u32 = 129u32;
pub const TIME_BYTES: u32 = 4u32;
pub const TIME_CALLBACK_EVENT_PULSE: u32 = 32u32;
pub const TIME_CALLBACK_EVENT_SET: u32 = 16u32;
pub const TIME_CALLBACK_FUNCTION: u32 = 0u32;
pub const TIME_KILL_SYNCHRONOUS: u32 = 256u32;
pub const TIME_MIDI: u32 = 16u32;
pub const TIME_MS: u32 = 1u32;
pub const TIME_ONESHOT: u32 = 0u32;
pub const TIME_PERIODIC: u32 = 1u32;
pub const TIME_SAMPLES: u32 = 2u32;
pub const TIME_SMPTE: u32 = 8u32;
pub const TIME_TICKS: u32 = 32u32;
pub const WAVERR_BASE: u32 = 32u32;
#[inline]
pub unsafe fn timeBeginPeriod(uperiod: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeBeginPeriod(uperiod: u32) -> u32;
        }
        ::core::mem::transmute(timeBeginPeriod(::core::mem::transmute(uperiod)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeEndPeriod(uperiod: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeEndPeriod(uperiod: u32) -> u32;
        }
        ::core::mem::transmute(timeEndPeriod(::core::mem::transmute(uperiod)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeGetDevCaps(ptc: *mut TIMECAPS, cbtc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeGetDevCaps(ptc: *mut TIMECAPS, cbtc: u32) -> u32;
        }
        ::core::mem::transmute(timeGetDevCaps(::core::mem::transmute(ptc), ::core::mem::transmute(cbtc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeGetSystemTime(pmmt: *mut MMTIME, cbmmt: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeGetSystemTime(pmmt: *mut MMTIME, cbmmt: u32) -> u32;
        }
        ::core::mem::transmute(timeGetSystemTime(::core::mem::transmute(pmmt), ::core::mem::transmute(cbmmt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeGetTime() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeGetTime() -> u32;
        }
        ::core::mem::transmute(timeGetTime())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeKillEvent(utimerid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeKillEvent(utimerid: u32) -> u32;
        }
        ::core::mem::transmute(timeKillEvent(::core::mem::transmute(utimerid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeSetEvent(udelay: u32, uresolution: u32, fptc: ::core::option::Option<LPTIMECALLBACK>, dwuser: usize, fuevent: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeSetEvent(udelay: u32, uresolution: u32, fptc: ::windows::core::RawPtr, dwuser: usize, fuevent: u32) -> u32;
        }
        ::core::mem::transmute(timeSetEvent(::core::mem::transmute(udelay), ::core::mem::transmute(uresolution), ::core::mem::transmute(fptc), ::core::mem::transmute(dwuser), ::core::mem::transmute(fuevent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
