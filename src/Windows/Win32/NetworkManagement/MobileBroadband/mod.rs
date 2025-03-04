#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDummyMBNUCMExt(pub ::windows::core::IUnknown);
impl IDummyMBNUCMExt {}
unsafe impl ::windows::core::Interface for IDummyMBNUCMExt {
    type Vtable = IDummyMBNUCMExt_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_ffff_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IDummyMBNUCMExt> for ::windows::core::IUnknown {
    fn from(value: IDummyMBNUCMExt) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDummyMBNUCMExt> for ::windows::core::IUnknown {
    fn from(value: &IDummyMBNUCMExt) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDummyMBNUCMExt> for super::super::System::Com::IDispatch {
    fn from(value: IDummyMBNUCMExt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDummyMBNUCMExt> for super::super::System::Com::IDispatch {
    fn from(value: &IDummyMBNUCMExt) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IDummyMBNUCMExt {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyMBNUCMExt_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnection(pub ::windows::core::IUnknown);
impl IMbnConnection {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectionID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, connectionmode: MBN_CONNECTION_MODE, strprofile: Param1) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(connectionmode), strprofile.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionState(&self, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(connectionstate), ::core::mem::transmute(profilename)).ok()
    }
    pub unsafe fn GetVoiceCallState(&self) -> ::windows::core::Result<MBN_VOICE_CALL_STATE> {
        let mut result__: <MBN_VOICE_CALL_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_VOICE_CALL_STATE>(result__)
    }
    pub unsafe fn GetActivationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnConnection {
    type Vtable = IMbnConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200d_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnection> for ::windows::core::IUnknown {
    fn from(value: IMbnConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnection> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaceid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionmode: MBN_CONNECTION_MODE, strprofile: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, networkerror: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionContext(pub ::windows::core::IUnknown);
impl IMbnConnectionContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProvisionedContexts(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProvisionedContext<'a, Param0: ::windows::core::IntoParam<'a, MBN_CONTEXT>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, provisionedcontexts: Param0, providerid: Param1) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), provisionedcontexts.into_param().abi(), providerid.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionContext {
    type Vtable = IMbnConnectionContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200b_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionContext> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionContext> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provisionedcontexts: ::core::mem::ManuallyDrop<MBN_CONTEXT>, providerid: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionContextEvents(pub ::windows::core::IUnknown);
impl IMbnConnectionContextEvents {
    pub unsafe fn OnProvisionedContextListChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionContext>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetProvisionedContextComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionContext>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionContextEvents {
    type Vtable = IMbnConnectionContextEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200c_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionContextEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionContextEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionContextEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionContextEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionContextEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionContextEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionContextEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionEvents(pub ::windows::core::IUnknown);
impl IMbnConnectionEvents {
    pub unsafe fn OnConnectComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newconnection.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnDisconnectComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newconnection.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnConnectStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
    pub unsafe fn OnVoiceCallStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionEvents {
    type Vtable = IMbnConnectionEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200e_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newconnection: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionManager(pub ::windows::core::IUnknown);
impl IMbnConnectionManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, connectionid: Param0) -> ::windows::core::Result<IMbnConnection> {
        let mut result__: <IMbnConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), connectionid.into_param().abi(), &mut result__).from_abi::<IMbnConnection>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnections(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionManager {
    type Vtable = IMbnConnectionManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201d_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionManager> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectionid: super::super::Foundation::PWSTR, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionManagerEvents(pub ::windows::core::IUnknown);
impl IMbnConnectionManagerEvents {
    pub unsafe fn OnConnectionArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, newconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newconnection.into_param().abi()).ok()
    }
    pub unsafe fn OnConnectionRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnection>>(&self, oldconnection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), oldconnection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionManagerEvents {
    type Vtable = IMbnConnectionManagerEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201e_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionManagerEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionManagerEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionManagerEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionProfile(pub ::windows::core::IUnknown);
impl IMbnConnectionProfile {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProfileXmlData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), strprofile.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfile {
    type Vtable = IMbnConnectionProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2010_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionProfile> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionProfile> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profiledata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, strprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionProfileEvents(pub ::windows::core::IUnknown);
impl IMbnConnectionProfileEvents {
    pub unsafe fn OnProfileUpdate<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, newprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newprofile.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfileEvents {
    type Vtable = IMbnConnectionProfileEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2011_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionProfileEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfileEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionProfileEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfileEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfileEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfileEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionProfileManager(pub ::windows::core::IUnknown);
impl IMbnConnectionProfileManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConnectionProfiles<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, mbninterface: Param0) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, mbninterface: Param0, profilename: Param1) -> ::windows::core::Result<IMbnConnectionProfile> {
        let mut result__: <IMbnConnectionProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), profilename.into_param().abi(), &mut result__).from_abi::<IMbnConnectionProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateConnectionProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, xmlprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), xmlprofile.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfileManager {
    type Vtable = IMbnConnectionProfileManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200f_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionProfileManager> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfileManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionProfileManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfileManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterface: ::windows::core::RawPtr, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterface: ::windows::core::RawPtr, profilename: super::super::Foundation::PWSTR, connectionprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xmlprofile: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnConnectionProfileManagerEvents(pub ::windows::core::IUnknown);
impl IMbnConnectionProfileManagerEvents {
    pub unsafe fn OnConnectionProfileArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, newconnectionprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newconnectionprofile.into_param().abi()).ok()
    }
    pub unsafe fn OnConnectionProfileRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnConnectionProfile>>(&self, oldconnectionprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), oldconnectionprofile.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnConnectionProfileManagerEvents {
    type Vtable = IMbnConnectionProfileManagerEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201f_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnConnectionProfileManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnConnectionProfileManagerEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnConnectionProfileManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnConnectionProfileManagerEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnConnectionProfileManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnConnectionProfileManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnConnectionProfileManagerEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldconnectionprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnDeviceService(pub ::windows::core::IUnknown);
impl IMbnDeviceService {
    pub unsafe fn QuerySupportedCommands(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn OpenCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn CloseCommandSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(deviceservicedata), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCommand(&self, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(deviceservicedata), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn OpenDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn CloseDataSession(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteData(&self, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(deviceservicedata), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceServiceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCommandSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDataSessionOpen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceService {
    type Vtable = IMbnDeviceService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3bb9a71_dc70_4be9_a4da_7886ae8b191b);
}
impl ::core::convert::From<IMbnDeviceService> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnDeviceService> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaceid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceserviceid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnDeviceServiceStateEvents(pub ::windows::core::IUnknown);
impl IMbnDeviceServiceStateEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSessionsStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfaceid: Param0, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(statechange)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServiceStateEvents {
    type Vtable = IMbnDeviceServiceStateEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d3ff196_89ee_49d8_8b60_33ffddffc58d);
}
impl ::core::convert::From<IMbnDeviceServiceStateEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServiceStateEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnDeviceServiceStateEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServiceStateEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServiceStateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServiceStateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServiceStateEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnDeviceServicesContext(pub ::windows::core::IUnknown);
impl IMbnDeviceServicesContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateDeviceServices(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, deviceserviceid: Param0) -> ::windows::core::Result<IMbnDeviceService> {
        let mut result__: <IMbnDeviceService as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), deviceserviceid.into_param().abi(), &mut result__).from_abi::<IMbnDeviceService>(result__)
    }
    pub unsafe fn MaxCommandSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn MaxDataSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServicesContext {
    type Vtable = IMbnDeviceServicesContext_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc5ac347_1592_4068_80bb_6a57580150d8);
}
impl ::core::convert::From<IMbnDeviceServicesContext> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServicesContext) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnDeviceServicesContext> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServicesContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServicesContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServicesContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesContext_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceserviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndeviceservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxcommandsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxdatasize: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnDeviceServicesEvents(pub ::windows::core::IUnknown);
impl IMbnDeviceServicesEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuerySupportedCommandsComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(commandidlist), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    pub unsafe fn OnOpenCommandSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    pub unsafe fn OnCloseCommandSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetCommandComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(responseid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQueryCommandComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(responseid), ::core::mem::transmute(deviceservicedata), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(eventid), ::core::mem::transmute(deviceservicedata)).ok()
    }
    pub unsafe fn OnOpenDataSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    pub unsafe fn OnCloseDataSessionComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    pub unsafe fn OnWriteDataComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(status), ::core::mem::transmute(requestid)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnReadData<'a, Param0: ::windows::core::IntoParam<'a, IMbnDeviceService>>(&self, deviceservice: Param0, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), deviceservice.into_param().abi(), ::core::mem::transmute(deviceservicedata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInterfaceStateChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, interfaceid: Param0, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), ::core::mem::transmute(statechange)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServicesEvents {
    type Vtable = IMbnDeviceServicesEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a900c19_6824_4e97_b76e_cf239d0ca642);
}
impl ::core::convert::From<IMbnDeviceServicesEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServicesEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnDeviceServicesEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServicesEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServicesEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServicesEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, status: ::windows::core::HRESULT, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceservice: ::windows::core::RawPtr, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnDeviceServicesManager(pub ::windows::core::IUnknown);
impl IMbnDeviceServicesManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceServicesContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, networkinterfaceid: Param0) -> ::windows::core::Result<IMbnDeviceServicesContext> {
        let mut result__: <IMbnDeviceServicesContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), networkinterfaceid.into_param().abi(), &mut result__).from_abi::<IMbnDeviceServicesContext>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnDeviceServicesManager {
    type Vtable = IMbnDeviceServicesManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20a26258_6811_4478_ac1d_13324e45e41c);
}
impl ::core::convert::From<IMbnDeviceServicesManager> for ::windows::core::IUnknown {
    fn from(value: IMbnDeviceServicesManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnDeviceServicesManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnDeviceServicesManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnDeviceServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnDeviceServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnDeviceServicesManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, networkinterfaceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mbndevicescontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnInterface(pub ::windows::core::IUnknown);
impl IMbnInterface {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInterfaceCapability(&self) -> ::windows::core::Result<MBN_INTERFACE_CAPS> {
        let mut result__: <MBN_INTERFACE_CAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_INTERFACE_CAPS>(result__)
    }
    pub unsafe fn GetSubscriberInformation(&self) -> ::windows::core::Result<IMbnSubscriberInformation> {
        let mut result__: <IMbnSubscriberInformation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMbnSubscriberInformation>(result__)
    }
    pub unsafe fn GetReadyState(&self) -> ::windows::core::Result<MBN_READY_STATE> {
        let mut result__: <MBN_READY_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_READY_STATE>(result__)
    }
    pub unsafe fn InEmergencyMode(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHomeProvider(&self) -> ::windows::core::Result<MBN_PROVIDER> {
        let mut result__: <MBN_PROVIDER as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_PROVIDER>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPreferredProviders(&self, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(preferredproviders), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(age), ::core::mem::transmute(visibleproviders)).ok()
    }
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConnection(&self) -> ::windows::core::Result<IMbnConnection> {
        let mut result__: <IMbnConnection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMbnConnection>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnInterface {
    type Vtable = IMbnInterface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2001_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnInterface> for ::windows::core::IUnknown {
    fn from(value: IMbnInterface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnInterface> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaceid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfacecaps: *mut ::core::mem::ManuallyDrop<MBN_INTERFACE_CAPS>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subscriberinformation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, readystate: *mut MBN_READY_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, emergencymode: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, homeprovider: *mut ::core::mem::ManuallyDrop<MBN_PROVIDER>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbnconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnInterfaceEvents(pub ::windows::core::IUnknown);
impl IMbnInterfaceEvents {
    pub unsafe fn OnInterfaceCapabilityAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSubscriberInformationChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnReadyStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnEmergencyModeChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnHomeProviderAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnPreferredProvidersChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetPreferredProvidersComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnScanNetworkComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnInterfaceEvents {
    type Vtable = IMbnInterfaceEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2002_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnInterfaceEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnInterfaceEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnInterfaceEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterfaceEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterfaceEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterfaceEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnInterfaceManager(pub ::windows::core::IUnknown);
impl IMbnInterfaceManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInterface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, interfaceid: Param0) -> ::windows::core::Result<IMbnInterface> {
        let mut result__: <IMbnInterface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), interfaceid.into_param().abi(), &mut result__).from_abi::<IMbnInterface>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnInterfaceManager {
    type Vtable = IMbnInterfaceManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201b_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnInterfaceManager> for ::windows::core::IUnknown {
    fn from(value: IMbnInterfaceManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnInterfaceManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterfaceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterfaceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterfaceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaceid: super::super::Foundation::PWSTR, mbninterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnInterfaceManagerEvents(pub ::windows::core::IUnknown);
impl IMbnInterfaceManagerEvents {
    pub unsafe fn OnInterfaceArrival<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnInterfaceRemoval<'a, Param0: ::windows::core::IntoParam<'a, IMbnInterface>>(&self, oldinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), oldinterface.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnInterfaceManagerEvents {
    type Vtable = IMbnInterfaceManagerEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201c_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnInterfaceManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnInterfaceManagerEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnInterfaceManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnInterfaceManagerEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnInterfaceManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnInterfaceManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnInterfaceManagerEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnMultiCarrier(pub ::windows::core::IUnknown);
impl IMbnMultiCarrier {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHomeProvider(&self, homeprovider: *const MBN_PROVIDER2) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(homeprovider), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPreferredProviders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVisibleProviders(&self, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(age), ::core::mem::transmute(visibleproviders)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSupportedCellularClasses(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn GetCurrentCellularClass(&self) -> ::windows::core::Result<MBN_CELLULAR_CLASS> {
        let mut result__: <MBN_CELLULAR_CLASS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_CELLULAR_CLASS>(result__)
    }
    pub unsafe fn ScanNetwork(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnMultiCarrier {
    type Vtable = IMbnMultiCarrier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2020_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnMultiCarrier> for ::windows::core::IUnknown {
    fn from(value: IMbnMultiCarrier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnMultiCarrier> for ::windows::core::IUnknown {
    fn from(value: &IMbnMultiCarrier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnMultiCarrier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnMultiCarrier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrier_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, homeprovider: *const ::core::mem::ManuallyDrop<MBN_PROVIDER2>, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnMultiCarrierEvents(pub ::windows::core::IUnknown);
impl IMbnMultiCarrierEvents {
    pub unsafe fn OnSetHomeProviderComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnCurrentCellularClassChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
    pub unsafe fn OnPreferredProvidersChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
    pub unsafe fn OnScanNetworkComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), mbninterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnInterfaceCapabilityChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnMultiCarrier>>(&self, mbninterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), mbninterface.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnMultiCarrierEvents {
    type Vtable = IMbnMultiCarrierEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_2021_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnMultiCarrierEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnMultiCarrierEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnMultiCarrierEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnMultiCarrierEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnMultiCarrierEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnMultiCarrierEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnMultiCarrierEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mbninterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnPin(pub ::windows::core::IUnknown);
impl IMbnPin {
    pub unsafe fn PinType(&self) -> ::windows::core::Result<MBN_PIN_TYPE> {
        let mut result__: <MBN_PIN_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_PIN_TYPE>(result__)
    }
    pub unsafe fn PinFormat(&self) -> ::windows::core::Result<MBN_PIN_FORMAT> {
        let mut result__: <MBN_PIN_FORMAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_PIN_FORMAT>(result__)
    }
    pub unsafe fn PinLengthMin(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn PinLengthMax(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn PinMode(&self) -> ::windows::core::Result<MBN_PIN_MODE> {
        let mut result__: <MBN_PIN_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_PIN_MODE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pin.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Disable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pin.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pin.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Change<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pin: Param0, newpin: Param1) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pin.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unblock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, puk: Param0, newpin: Param1) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), puk.into_param().abi(), newpin.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPinManager(&self) -> ::windows::core::Result<IMbnPinManager> {
        let mut result__: <IMbnPinManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMbnPinManager>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnPin {
    type Vtable = IMbnPin_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2007_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnPin> for ::windows::core::IUnknown {
    fn from(value: IMbnPin) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnPin> for ::windows::core::IUnknown {
    fn from(value: &IMbnPin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPin_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pintype: *mut MBN_PIN_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinformat: *mut MBN_PIN_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinlengthmin: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinlengthmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinmode: *mut MBN_PIN_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puk: super::super::Foundation::PWSTR, newpin: super::super::Foundation::PWSTR, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnPinEvents(pub ::windows::core::IUnknown);
impl IMbnPinEvents {
    pub unsafe fn OnEnableComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnDisableComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnEnterComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnChangeComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnUnblockComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPin>>(&self, pin: Param0, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pin.into_param().abi(), ::core::mem::transmute(pininfo), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnPinEvents {
    type Vtable = IMbnPinEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2008_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnPinEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnPinEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnPinEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnPinEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPinEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPinEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::windows::core::RawPtr, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnPinManager(pub ::windows::core::IUnknown);
impl IMbnPinManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPinList(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn GetPin(&self, pintype: MBN_PIN_TYPE) -> ::windows::core::Result<IMbnPin> {
        let mut result__: <IMbnPin as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pintype), &mut result__).from_abi::<IMbnPin>(result__)
    }
    pub unsafe fn GetPinState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnPinManager {
    type Vtable = IMbnPinManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2005_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnPinManager> for ::windows::core::IUnknown {
    fn from(value: IMbnPinManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnPinManager> for ::windows::core::IUnknown {
    fn from(value: &IMbnPinManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPinManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pintype: MBN_PIN_TYPE, pin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnPinManagerEvents(pub ::windows::core::IUnknown);
impl IMbnPinManagerEvents {
    pub unsafe fn OnPinListAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnPinManager>>(&self, pinmanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pinmanager.into_param().abi()).ok()
    }
    pub unsafe fn OnGetPinStateComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnPinManager>, Param1: ::windows::core::IntoParam<'a, MBN_PIN_INFO>>(&self, pinmanager: Param0, pininfo: Param1, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pinmanager.into_param().abi(), pininfo.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnPinManagerEvents {
    type Vtable = IMbnPinManagerEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2006_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnPinManagerEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnPinManagerEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnPinManagerEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnPinManagerEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnPinManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnPinManagerEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnPinManagerEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinmanager: ::windows::core::RawPtr, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnRadio(pub ::windows::core::IUnknown);
impl IMbnRadio {
    pub unsafe fn SoftwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__: <MBN_RADIO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_RADIO>(result__)
    }
    pub unsafe fn HardwareRadioState(&self) -> ::windows::core::Result<MBN_RADIO> {
        let mut result__: <MBN_RADIO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_RADIO>(result__)
    }
    pub unsafe fn SetSoftwareRadioState(&self, radiostate: MBN_RADIO) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiostate), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnRadio {
    type Vtable = IMbnRadio_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdccccab6_201f_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnRadio> for ::windows::core::IUnknown {
    fn from(value: IMbnRadio) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnRadio> for ::windows::core::IUnknown {
    fn from(value: &IMbnRadio) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRadio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRadio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadio_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, softwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hardwareradiostate: *mut MBN_RADIO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnRadioEvents(pub ::windows::core::IUnknown);
impl IMbnRadioEvents {
    pub unsafe fn OnRadioStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRadio>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetSoftwareRadioStateComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnRadio>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnRadioEvents {
    type Vtable = IMbnRadioEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdddab6_201f_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnRadioEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnRadioEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnRadioEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnRadioEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRadioEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRadioEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRadioEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnRegistration(pub ::windows::core::IUnknown);
impl IMbnRegistration {
    pub unsafe fn GetRegisterState(&self) -> ::windows::core::Result<MBN_REGISTER_STATE> {
        let mut result__: <MBN_REGISTER_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_REGISTER_STATE>(result__)
    }
    pub unsafe fn GetRegisterMode(&self) -> ::windows::core::Result<MBN_REGISTER_MODE> {
        let mut result__: <MBN_REGISTER_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_REGISTER_MODE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProviderID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRoamingText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAvailableDataClasses(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCurrentDataClass(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetRegistrationNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPacketAttachNetworkError(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRegisterMode<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, registermode: MBN_REGISTER_MODE, providerid: Param1, dataclass: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(registermode), providerid.into_param().abi(), ::core::mem::transmute(dataclass), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnRegistration {
    type Vtable = IMbnRegistration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2009_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnRegistration> for ::windows::core::IUnknown {
    fn from(value: IMbnRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnRegistration> for ::windows::core::IUnknown {
    fn from(value: &IMbnRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, registerstate: *mut MBN_REGISTER_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, registermode: *mut MBN_REGISTER_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providerid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, providername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, roamingtext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availabledataclasses: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, currentdataclass: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, registrationnetworkerror: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packetattachnetworkerror: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, registermode: MBN_REGISTER_MODE, providerid: super::super::Foundation::PWSTR, dataclass: u32, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnRegistrationEvents(pub ::windows::core::IUnknown);
impl IMbnRegistrationEvents {
    pub unsafe fn OnRegisterModeAvailable<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnRegisterStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnPacketServiceStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnSetRegisterModeComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnRegistration>>(&self, newinterface: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), newinterface.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnRegistrationEvents {
    type Vtable = IMbnRegistrationEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_200a_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnRegistrationEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnRegistrationEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnRegistrationEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnRegistrationEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnRegistrationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnRegistrationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnRegistrationEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnServiceActivation(pub ::windows::core::IUnknown);
impl IMbnServiceActivation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activate(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(vendorspecificdata), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnServiceActivation {
    type Vtable = IMbnServiceActivation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2017_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnServiceActivation> for ::windows::core::IUnknown {
    fn from(value: IMbnServiceActivation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnServiceActivation> for ::windows::core::IUnknown {
    fn from(value: &IMbnServiceActivation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnServiceActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnServiceActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnServiceActivationEvents(pub ::windows::core::IUnknown);
impl IMbnServiceActivationEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnActivationComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnServiceActivation>>(&self, serviceactivation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), serviceactivation.into_param().abi(), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(requestid), ::core::mem::transmute(status), ::core::mem::transmute(networkerror)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnServiceActivationEvents {
    type Vtable = IMbnServiceActivationEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2018_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnServiceActivationEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnServiceActivationEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnServiceActivationEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnServiceActivationEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnServiceActivationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnServiceActivationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnServiceActivationEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, serviceactivation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows::core::HRESULT, networkerror: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSignal(pub ::windows::core::IUnknown);
impl IMbnSignal {
    pub unsafe fn GetSignalStrength(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSignalError(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnSignal {
    type Vtable = IMbnSignal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2003_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnSignal> for ::windows::core::IUnknown {
    fn from(value: IMbnSignal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSignal> for ::windows::core::IUnknown {
    fn from(value: &IMbnSignal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignal_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalstrength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, signalerror: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSignalEvents(pub ::windows::core::IUnknown);
impl IMbnSignalEvents {
    pub unsafe fn OnSignalStateChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSignal>>(&self, newinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newinterface.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnSignalEvents {
    type Vtable = IMbnSignalEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2004_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnSignalEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnSignalEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSignalEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnSignalEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSignalEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSignalEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSignalEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSms(pub ::windows::core::IUnknown);
impl IMbnSms {
    pub unsafe fn GetSmsConfiguration(&self) -> ::windows::core::Result<IMbnSmsConfiguration> {
        let mut result__: <IMbnSmsConfiguration as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMbnSmsConfiguration>(result__)
    }
    pub unsafe fn SetSmsConfiguration<'a, Param0: ::windows::core::IntoParam<'a, IMbnSmsConfiguration>>(&self, smsconfiguration: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), smsconfiguration.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmsSendPdu<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pdudata: Param0, size: u8) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdudata.into_param().abi(), ::core::mem::transmute(size), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SmsSendCdma<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, address: Param0, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), address.into_param().abi(), ::core::mem::transmute(encoding), ::core::mem::transmute(language), ::core::mem::transmute(sizeincharacters), ::core::mem::transmute(message), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SmsSendCdmaPdu(&self, message: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(message), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SmsRead(&self, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsfilter), ::core::mem::transmute(smsformat), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SmsDelete(&self, smsfilter: *const MBN_SMS_FILTER) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsfilter), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSmsStatus(&self) -> ::windows::core::Result<MBN_SMS_STATUS_INFO> {
        let mut result__: <MBN_SMS_STATUS_INFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_SMS_STATUS_INFO>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnSms {
    type Vtable = IMbnSms_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2015_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnSms> for ::windows::core::IUnknown {
    fn from(value: IMbnSms) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSms> for ::windows::core::IUnknown {
    fn from(value: &IMbnSms) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSms {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSms {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSms_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, smsconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, smsconfiguration: ::windows::core::RawPtr, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdudata: super::super::Foundation::PWSTR, size: u8, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, address: super::super::Foundation::PWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSmsConfiguration(pub ::windows::core::IUnknown);
impl IMbnSmsConfiguration {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceCenterAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceCenterAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, scaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), scaddress.into_param().abi()).ok()
    }
    pub unsafe fn MaxMessageIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn CdmaShortMsgSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SmsFormat(&self) -> ::windows::core::Result<MBN_SMS_FORMAT> {
        let mut result__: <MBN_SMS_FORMAT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_SMS_FORMAT>(result__)
    }
    pub unsafe fn SetSmsFormat(&self, smsformat: MBN_SMS_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(smsformat)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsConfiguration {
    type Vtable = IMbnSmsConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2012_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnSmsConfiguration> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSmsConfiguration> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaddress: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shortmsgsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, smsformat: *mut MBN_SMS_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSmsEvents(pub ::windows::core::IUnknown);
impl IMbnSmsEvents {
    pub unsafe fn OnSmsConfigurationChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sms.into_param().abi()).ok()
    }
    pub unsafe fn OnSetSmsConfigurationComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnSmsSendComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsReadComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(smsformat), ::core::mem::transmute(readmsgs), ::core::mem::transmute(moremsgs), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSmsNewClass0Message<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(smsformat), ::core::mem::transmute(readmsgs)).ok()
    }
    pub unsafe fn OnSmsDeleteComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), sms.into_param().abi(), ::core::mem::transmute(requestid), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn OnSmsStatusChange<'a, Param0: ::windows::core::IntoParam<'a, IMbnSms>>(&self, sms: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), sms.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsEvents {
    type Vtable = IMbnSmsEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2016_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnSmsEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSmsEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: i16, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sms: ::windows::core::RawPtr, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sms: ::windows::core::RawPtr, requestid: u32, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sms: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSmsReadMsgPdu(pub ::windows::core::IUnknown);
impl IMbnSmsReadMsgPdu {
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__: <MBN_MSG_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_MSG_STATUS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PduData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsReadMsgPdu {
    type Vtable = IMbnSmsReadMsgPdu_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2013_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnSmsReadMsgPdu> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsReadMsgPdu) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSmsReadMsgPdu> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsReadMsgPdu) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsReadMsgPdu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsReadMsgPdu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgPdu_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdudata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSmsReadMsgTextCdma(pub ::windows::core::IUnknown);
impl IMbnSmsReadMsgTextCdma {
    pub unsafe fn Index(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<MBN_MSG_STATUS> {
        let mut result__: <MBN_MSG_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_MSG_STATUS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn EncodingID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_ENCODING> {
        let mut result__: <MBN_SMS_CDMA_ENCODING as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_SMS_CDMA_ENCODING>(result__)
    }
    pub unsafe fn LanguageID(&self) -> ::windows::core::Result<MBN_SMS_CDMA_LANG> {
        let mut result__: <MBN_SMS_CDMA_LANG as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<MBN_SMS_CDMA_LANG>(result__)
    }
    pub unsafe fn SizeInCharacters(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnSmsReadMsgTextCdma {
    type Vtable = IMbnSmsReadMsgTextCdma_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2014_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnSmsReadMsgTextCdma> for ::windows::core::IUnknown {
    fn from(value: IMbnSmsReadMsgTextCdma) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSmsReadMsgTextCdma> for ::windows::core::IUnknown {
    fn from(value: &IMbnSmsReadMsgTextCdma) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSmsReadMsgTextCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSmsReadMsgTextCdma {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSmsReadMsgTextCdma_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: *mut MBN_MSG_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, address: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timestamp: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sizeincharacters: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnSubscriberInformation(pub ::windows::core::IUnknown);
impl IMbnSubscriberInformation {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriberID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimIccID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TelephoneNumbers(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnSubscriberInformation {
    type Vtable = IMbnSubscriberInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x459ecc43_bcf5_11dc_a8a8_001321f1405f);
}
impl ::core::convert::From<IMbnSubscriberInformation> for ::windows::core::IUnknown {
    fn from(value: IMbnSubscriberInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnSubscriberInformation> for ::windows::core::IUnknown {
    fn from(value: &IMbnSubscriberInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnSubscriberInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnSubscriberInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnSubscriberInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subscriberid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, simiccid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnVendorSpecificEvents(pub ::windows::core::IUnknown);
impl IMbnVendorSpecificEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnEventNotification<'a, Param0: ::windows::core::IntoParam<'a, IMbnVendorSpecificOperation>>(&self, vendoroperation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), vendoroperation.into_param().abi(), ::core::mem::transmute(vendorspecificdata)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetVendorSpecificComplete<'a, Param0: ::windows::core::IntoParam<'a, IMbnVendorSpecificOperation>>(&self, vendoroperation: Param0, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), vendoroperation.into_param().abi(), ::core::mem::transmute(vendorspecificdata), ::core::mem::transmute(requestid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMbnVendorSpecificEvents {
    type Vtable = IMbnVendorSpecificEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_201a_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnVendorSpecificEvents> for ::windows::core::IUnknown {
    fn from(value: IMbnVendorSpecificEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnVendorSpecificEvents> for ::windows::core::IUnknown {
    fn from(value: &IMbnVendorSpecificEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnVendorSpecificEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnVendorSpecificEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vendoroperation: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMbnVendorSpecificOperation(pub ::windows::core::IUnknown);
impl IMbnVendorSpecificOperation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetVendorSpecific(&self, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(vendorspecificdata), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMbnVendorSpecificOperation {
    type Vtable = IMbnVendorSpecificOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcbbbab6_2019_4bbb_aaee_338e368af6fa);
}
impl ::core::convert::From<IMbnVendorSpecificOperation> for ::windows::core::IUnknown {
    fn from(value: IMbnVendorSpecificOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMbnVendorSpecificOperation> for ::windows::core::IUnknown {
    fn from(value: &IMbnVendorSpecificOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMbnVendorSpecificOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMbnVendorSpecificOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMbnVendorSpecificOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_ACTIVATION_STATE(pub i32);
pub const MBN_ACTIVATION_STATE_NONE: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(0i32);
pub const MBN_ACTIVATION_STATE_ACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(1i32);
pub const MBN_ACTIVATION_STATE_ACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(2i32);
pub const MBN_ACTIVATION_STATE_DEACTIVATED: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(3i32);
pub const MBN_ACTIVATION_STATE_DEACTIVATING: MBN_ACTIVATION_STATE = MBN_ACTIVATION_STATE(4i32);
impl ::core::convert::From<i32> for MBN_ACTIVATION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_ACTIVATION_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_AUTH_PROTOCOL(pub i32);
pub const MBN_AUTH_PROTOCOL_NONE: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(0i32);
pub const MBN_AUTH_PROTOCOL_PAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(1i32);
pub const MBN_AUTH_PROTOCOL_CHAP: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(2i32);
pub const MBN_AUTH_PROTOCOL_MSCHAPV2: MBN_AUTH_PROTOCOL = MBN_AUTH_PROTOCOL(3i32);
impl ::core::convert::From<i32> for MBN_AUTH_PROTOCOL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_AUTH_PROTOCOL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_BAND_CLASS(pub i32);
pub const MBN_BAND_CLASS_NONE: MBN_BAND_CLASS = MBN_BAND_CLASS(0i32);
pub const MBN_BAND_CLASS_0: MBN_BAND_CLASS = MBN_BAND_CLASS(1i32);
pub const MBN_BAND_CLASS_I: MBN_BAND_CLASS = MBN_BAND_CLASS(2i32);
pub const MBN_BAND_CLASS_II: MBN_BAND_CLASS = MBN_BAND_CLASS(4i32);
pub const MBN_BAND_CLASS_III: MBN_BAND_CLASS = MBN_BAND_CLASS(8i32);
pub const MBN_BAND_CLASS_IV: MBN_BAND_CLASS = MBN_BAND_CLASS(16i32);
pub const MBN_BAND_CLASS_V: MBN_BAND_CLASS = MBN_BAND_CLASS(32i32);
pub const MBN_BAND_CLASS_VI: MBN_BAND_CLASS = MBN_BAND_CLASS(64i32);
pub const MBN_BAND_CLASS_VII: MBN_BAND_CLASS = MBN_BAND_CLASS(128i32);
pub const MBN_BAND_CLASS_VIII: MBN_BAND_CLASS = MBN_BAND_CLASS(256i32);
pub const MBN_BAND_CLASS_IX: MBN_BAND_CLASS = MBN_BAND_CLASS(512i32);
pub const MBN_BAND_CLASS_X: MBN_BAND_CLASS = MBN_BAND_CLASS(1024i32);
pub const MBN_BAND_CLASS_XI: MBN_BAND_CLASS = MBN_BAND_CLASS(2048i32);
pub const MBN_BAND_CLASS_XII: MBN_BAND_CLASS = MBN_BAND_CLASS(4096i32);
pub const MBN_BAND_CLASS_XIII: MBN_BAND_CLASS = MBN_BAND_CLASS(8192i32);
pub const MBN_BAND_CLASS_XIV: MBN_BAND_CLASS = MBN_BAND_CLASS(16384i32);
pub const MBN_BAND_CLASS_XV: MBN_BAND_CLASS = MBN_BAND_CLASS(32768i32);
pub const MBN_BAND_CLASS_XVI: MBN_BAND_CLASS = MBN_BAND_CLASS(65536i32);
pub const MBN_BAND_CLASS_XVII: MBN_BAND_CLASS = MBN_BAND_CLASS(131072i32);
pub const MBN_BAND_CLASS_CUSTOM: MBN_BAND_CLASS = MBN_BAND_CLASS(-2147483648i32);
impl ::core::convert::From<i32> for MBN_BAND_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_BAND_CLASS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_CELLULAR_CLASS(pub i32);
pub const MBN_CELLULAR_CLASS_NONE: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(0i32);
pub const MBN_CELLULAR_CLASS_GSM: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(1i32);
pub const MBN_CELLULAR_CLASS_CDMA: MBN_CELLULAR_CLASS = MBN_CELLULAR_CLASS(2i32);
impl ::core::convert::From<i32> for MBN_CELLULAR_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_CELLULAR_CLASS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_COMPRESSION(pub i32);
pub const MBN_COMPRESSION_NONE: MBN_COMPRESSION = MBN_COMPRESSION(0i32);
pub const MBN_COMPRESSION_ENABLE: MBN_COMPRESSION = MBN_COMPRESSION(1i32);
impl ::core::convert::From<i32> for MBN_COMPRESSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_COMPRESSION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_CONNECTION_MODE(pub i32);
pub const MBN_CONNECTION_MODE_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(0i32);
pub const MBN_CONNECTION_MODE_TMP_PROFILE: MBN_CONNECTION_MODE = MBN_CONNECTION_MODE(1i32);
impl ::core::convert::From<i32> for MBN_CONNECTION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_CONNECTION_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_CONTEXT {
    pub contextID: u32,
    pub contextType: MBN_CONTEXT_TYPE,
    pub accessString: super::super::Foundation::BSTR,
    pub userName: super::super::Foundation::BSTR,
    pub password: super::super::Foundation::BSTR,
    pub compression: MBN_COMPRESSION,
    pub authType: MBN_AUTH_PROTOCOL,
}
#[cfg(feature = "Win32_Foundation")]
impl MBN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_CONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_CONTEXT").field("contextID", &self.contextID).field("contextType", &self.contextType).field("accessString", &self.accessString).field("userName", &self.userName).field("password", &self.password).field("compression", &self.compression).field("authType", &self.authType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.contextID == other.contextID && self.contextType == other.contextType && self.accessString == other.accessString && self.userName == other.userName && self.password == other.password && self.compression == other.compression && self.authType == other.authType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_CONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_CONTEXT_CONSTANTS(pub i32);
pub const MBN_ACCESSSTRING_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(100i32);
pub const MBN_USERNAME_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
pub const MBN_PASSWORD_LEN: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(255i32);
pub const MBN_CONTEXT_ID_APPEND: MBN_CONTEXT_CONSTANTS = MBN_CONTEXT_CONSTANTS(-1i32);
impl ::core::convert::From<i32> for MBN_CONTEXT_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_CONTEXT_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_CONTEXT_TYPE(pub i32);
pub const MBN_CONTEXT_TYPE_NONE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(0i32);
pub const MBN_CONTEXT_TYPE_INTERNET: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(1i32);
pub const MBN_CONTEXT_TYPE_VPN: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(2i32);
pub const MBN_CONTEXT_TYPE_VOICE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(3i32);
pub const MBN_CONTEXT_TYPE_VIDEO_SHARE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(4i32);
pub const MBN_CONTEXT_TYPE_CUSTOM: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(5i32);
pub const MBN_CONTEXT_TYPE_PURCHASE: MBN_CONTEXT_TYPE = MBN_CONTEXT_TYPE(6i32);
impl ::core::convert::From<i32> for MBN_CONTEXT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_CONTEXT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_CTRL_CAPS(pub i32);
pub const MBN_CTRL_CAPS_NONE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(0i32);
pub const MBN_CTRL_CAPS_REG_MANUAL: MBN_CTRL_CAPS = MBN_CTRL_CAPS(1i32);
pub const MBN_CTRL_CAPS_HW_RADIO_SWITCH: MBN_CTRL_CAPS = MBN_CTRL_CAPS(2i32);
pub const MBN_CTRL_CAPS_CDMA_MOBILE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(4i32);
pub const MBN_CTRL_CAPS_CDMA_SIMPLE_IP: MBN_CTRL_CAPS = MBN_CTRL_CAPS(8i32);
pub const MBN_CTRL_CAPS_PROTECT_UNIQUEID: MBN_CTRL_CAPS = MBN_CTRL_CAPS(16i32);
pub const MBN_CTRL_CAPS_MODEL_MULTI_CARRIER: MBN_CTRL_CAPS = MBN_CTRL_CAPS(32i32);
pub const MBN_CTRL_CAPS_USSD: MBN_CTRL_CAPS = MBN_CTRL_CAPS(64i32);
pub const MBN_CTRL_CAPS_MULTI_MODE: MBN_CTRL_CAPS = MBN_CTRL_CAPS(128i32);
impl ::core::convert::From<i32> for MBN_CTRL_CAPS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_CTRL_CAPS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_DATA_CLASS(pub i32);
pub const MBN_DATA_CLASS_NONE: MBN_DATA_CLASS = MBN_DATA_CLASS(0i32);
pub const MBN_DATA_CLASS_GPRS: MBN_DATA_CLASS = MBN_DATA_CLASS(1i32);
pub const MBN_DATA_CLASS_EDGE: MBN_DATA_CLASS = MBN_DATA_CLASS(2i32);
pub const MBN_DATA_CLASS_UMTS: MBN_DATA_CLASS = MBN_DATA_CLASS(4i32);
pub const MBN_DATA_CLASS_HSDPA: MBN_DATA_CLASS = MBN_DATA_CLASS(8i32);
pub const MBN_DATA_CLASS_HSUPA: MBN_DATA_CLASS = MBN_DATA_CLASS(16i32);
pub const MBN_DATA_CLASS_LTE: MBN_DATA_CLASS = MBN_DATA_CLASS(32i32);
pub const MBN_DATA_CLASS_5G_NSA: MBN_DATA_CLASS = MBN_DATA_CLASS(64i32);
pub const MBN_DATA_CLASS_5G_SA: MBN_DATA_CLASS = MBN_DATA_CLASS(128i32);
pub const MBN_DATA_CLASS_1XRTT: MBN_DATA_CLASS = MBN_DATA_CLASS(65536i32);
pub const MBN_DATA_CLASS_1XEVDO: MBN_DATA_CLASS = MBN_DATA_CLASS(131072i32);
pub const MBN_DATA_CLASS_1XEVDO_REVA: MBN_DATA_CLASS = MBN_DATA_CLASS(262144i32);
pub const MBN_DATA_CLASS_1XEVDV: MBN_DATA_CLASS = MBN_DATA_CLASS(524288i32);
pub const MBN_DATA_CLASS_3XRTT: MBN_DATA_CLASS = MBN_DATA_CLASS(1048576i32);
pub const MBN_DATA_CLASS_1XEVDO_REVB: MBN_DATA_CLASS = MBN_DATA_CLASS(2097152i32);
pub const MBN_DATA_CLASS_UMB: MBN_DATA_CLASS = MBN_DATA_CLASS(4194304i32);
pub const MBN_DATA_CLASS_CUSTOM: MBN_DATA_CLASS = MBN_DATA_CLASS(-2147483648i32);
impl ::core::convert::From<i32> for MBN_DATA_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_DATA_CLASS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_DEVICE_SERVICE {
    pub deviceServiceID: super::super::Foundation::BSTR,
    pub dataWriteSupported: i16,
    pub dataReadSupported: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl MBN_DEVICE_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_DEVICE_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_DEVICE_SERVICE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_DEVICE_SERVICE").field("deviceServiceID", &self.deviceServiceID).field("dataWriteSupported", &self.dataWriteSupported).field("dataReadSupported", &self.dataReadSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_DEVICE_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.deviceServiceID == other.deviceServiceID && self.dataWriteSupported == other.dataWriteSupported && self.dataReadSupported == other.dataReadSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_DEVICE_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_DEVICE_SERVICE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_DEVICE_SERVICES_INTERFACE_STATE(pub i32);
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(0i32);
pub const MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL: MBN_DEVICE_SERVICES_INTERFACE_STATE = MBN_DEVICE_SERVICES_INTERFACE_STATE(1i32);
impl ::core::convert::From<i32> for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_DEVICE_SERVICES_INTERFACE_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_DEVICE_SERVICE_SESSIONS_STATE(pub i32);
pub const MBN_DEVICE_SERVICE_SESSIONS_RESTORED: MBN_DEVICE_SERVICE_SESSIONS_STATE = MBN_DEVICE_SERVICE_SESSIONS_STATE(0i32);
impl ::core::convert::From<i32> for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_DEVICE_SERVICE_SESSIONS_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_INTERFACE_CAPS {
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub voiceClass: MBN_VOICE_CLASS,
    pub dataClass: u32,
    pub customDataClass: super::super::Foundation::BSTR,
    pub gsmBandClass: u32,
    pub cdmaBandClass: u32,
    pub customBandClass: super::super::Foundation::BSTR,
    pub smsCaps: u32,
    pub controlCaps: u32,
    pub deviceID: super::super::Foundation::BSTR,
    pub manufacturer: super::super::Foundation::BSTR,
    pub model: super::super::Foundation::BSTR,
    pub firmwareInfo: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MBN_INTERFACE_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_INTERFACE_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_INTERFACE_CAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_INTERFACE_CAPS")
            .field("cellularClass", &self.cellularClass)
            .field("voiceClass", &self.voiceClass)
            .field("dataClass", &self.dataClass)
            .field("customDataClass", &self.customDataClass)
            .field("gsmBandClass", &self.gsmBandClass)
            .field("cdmaBandClass", &self.cdmaBandClass)
            .field("customBandClass", &self.customBandClass)
            .field("smsCaps", &self.smsCaps)
            .field("controlCaps", &self.controlCaps)
            .field("deviceID", &self.deviceID)
            .field("manufacturer", &self.manufacturer)
            .field("model", &self.model)
            .field("firmwareInfo", &self.firmwareInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_INTERFACE_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.cellularClass == other.cellularClass
            && self.voiceClass == other.voiceClass
            && self.dataClass == other.dataClass
            && self.customDataClass == other.customDataClass
            && self.gsmBandClass == other.gsmBandClass
            && self.cdmaBandClass == other.cdmaBandClass
            && self.customBandClass == other.customBandClass
            && self.smsCaps == other.smsCaps
            && self.controlCaps == other.controlCaps
            && self.deviceID == other.deviceID
            && self.manufacturer == other.manufacturer
            && self.model == other.model
            && self.firmwareInfo == other.firmwareInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_INTERFACE_CAPS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_INTERFACE_CAPS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_INTERFACE_CAPS_CONSTANTS(pub i32);
pub const MBN_DEVICEID_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(18i32);
pub const MBN_MANUFACTURER_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
pub const MBN_MODEL_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
pub const MBN_FIRMWARE_LEN: MBN_INTERFACE_CAPS_CONSTANTS = MBN_INTERFACE_CAPS_CONSTANTS(32i32);
impl ::core::convert::From<i32> for MBN_INTERFACE_CAPS_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_INTERFACE_CAPS_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_MSG_STATUS(pub i32);
pub const MBN_MSG_STATUS_NEW: MBN_MSG_STATUS = MBN_MSG_STATUS(0i32);
pub const MBN_MSG_STATUS_OLD: MBN_MSG_STATUS = MBN_MSG_STATUS(1i32);
pub const MBN_MSG_STATUS_DRAFT: MBN_MSG_STATUS = MBN_MSG_STATUS(2i32);
pub const MBN_MSG_STATUS_SENT: MBN_MSG_STATUS = MBN_MSG_STATUS(3i32);
impl ::core::convert::From<i32> for MBN_MSG_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_MSG_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_PIN_CONSTANTS(pub i32);
pub const MBN_ATTEMPTS_REMAINING_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
pub const MBN_PIN_LENGTH_UNKNOWN: MBN_PIN_CONSTANTS = MBN_PIN_CONSTANTS(-1i32);
impl ::core::convert::From<i32> for MBN_PIN_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_PIN_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_PIN_FORMAT(pub i32);
pub const MBN_PIN_FORMAT_NONE: MBN_PIN_FORMAT = MBN_PIN_FORMAT(0i32);
pub const MBN_PIN_FORMAT_NUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(1i32);
pub const MBN_PIN_FORMAT_ALPHANUMERIC: MBN_PIN_FORMAT = MBN_PIN_FORMAT(2i32);
impl ::core::convert::From<i32> for MBN_PIN_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_PIN_FORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MBN_PIN_INFO {
    pub pinState: MBN_PIN_STATE,
    pub pinType: MBN_PIN_TYPE,
    pub attemptsRemaining: u32,
}
impl MBN_PIN_INFO {}
impl ::core::default::Default for MBN_PIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MBN_PIN_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_PIN_INFO").field("pinState", &self.pinState).field("pinType", &self.pinType).field("attemptsRemaining", &self.attemptsRemaining).finish()
    }
}
impl ::core::cmp::PartialEq for MBN_PIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pinState == other.pinState && self.pinType == other.pinType && self.attemptsRemaining == other.attemptsRemaining
    }
}
impl ::core::cmp::Eq for MBN_PIN_INFO {}
unsafe impl ::windows::core::Abi for MBN_PIN_INFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_PIN_MODE(pub i32);
pub const MBN_PIN_MODE_ENABLED: MBN_PIN_MODE = MBN_PIN_MODE(1i32);
pub const MBN_PIN_MODE_DISABLED: MBN_PIN_MODE = MBN_PIN_MODE(2i32);
impl ::core::convert::From<i32> for MBN_PIN_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_PIN_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_PIN_STATE(pub i32);
pub const MBN_PIN_STATE_NONE: MBN_PIN_STATE = MBN_PIN_STATE(0i32);
pub const MBN_PIN_STATE_ENTER: MBN_PIN_STATE = MBN_PIN_STATE(1i32);
pub const MBN_PIN_STATE_UNBLOCK: MBN_PIN_STATE = MBN_PIN_STATE(2i32);
impl ::core::convert::From<i32> for MBN_PIN_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_PIN_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_PIN_TYPE(pub i32);
pub const MBN_PIN_TYPE_NONE: MBN_PIN_TYPE = MBN_PIN_TYPE(0i32);
pub const MBN_PIN_TYPE_CUSTOM: MBN_PIN_TYPE = MBN_PIN_TYPE(1i32);
pub const MBN_PIN_TYPE_PIN1: MBN_PIN_TYPE = MBN_PIN_TYPE(2i32);
pub const MBN_PIN_TYPE_PIN2: MBN_PIN_TYPE = MBN_PIN_TYPE(3i32);
pub const MBN_PIN_TYPE_DEVICE_SIM_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(4i32);
pub const MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(5i32);
pub const MBN_PIN_TYPE_NETWORK_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(6i32);
pub const MBN_PIN_TYPE_NETWORK_SUBSET_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(7i32);
pub const MBN_PIN_TYPE_SVC_PROVIDER_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(8i32);
pub const MBN_PIN_TYPE_CORPORATE_PIN: MBN_PIN_TYPE = MBN_PIN_TYPE(9i32);
pub const MBN_PIN_TYPE_SUBSIDY_LOCK: MBN_PIN_TYPE = MBN_PIN_TYPE(10i32);
impl ::core::convert::From<i32> for MBN_PIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_PIN_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER {
    pub providerID: super::super::Foundation::BSTR,
    pub providerState: u32,
    pub providerName: super::super::Foundation::BSTR,
    pub dataClass: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MBN_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_PROVIDER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_PROVIDER").field("providerID", &self.providerID).field("providerState", &self.providerState).field("providerName", &self.providerName).field("dataClass", &self.dataClass).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.providerID == other.providerID && self.providerState == other.providerState && self.providerName == other.providerName && self.dataClass == other.dataClass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_PROVIDER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_PROVIDER {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MBN_PROVIDER2 {
    pub provider: MBN_PROVIDER,
    pub cellularClass: MBN_CELLULAR_CLASS,
    pub signalStrength: u32,
    pub signalError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MBN_PROVIDER2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MBN_PROVIDER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MBN_PROVIDER2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_PROVIDER2").field("provider", &self.provider).field("cellularClass", &self.cellularClass).field("signalStrength", &self.signalStrength).field("signalError", &self.signalError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MBN_PROVIDER2 {
    fn eq(&self, other: &Self) -> bool {
        self.provider == other.provider && self.cellularClass == other.cellularClass && self.signalStrength == other.signalStrength && self.signalError == other.signalError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MBN_PROVIDER2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MBN_PROVIDER2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_PROVIDER_CONSTANTS(pub i32);
pub const MBN_PROVIDERNAME_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(20i32);
pub const MBN_PROVIDERID_LEN: MBN_PROVIDER_CONSTANTS = MBN_PROVIDER_CONSTANTS(6i32);
impl ::core::convert::From<i32> for MBN_PROVIDER_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_PROVIDER_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_PROVIDER_STATE(pub i32);
pub const MBN_PROVIDER_STATE_NONE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(0i32);
pub const MBN_PROVIDER_STATE_HOME: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(1i32);
pub const MBN_PROVIDER_STATE_FORBIDDEN: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(2i32);
pub const MBN_PROVIDER_STATE_PREFERRED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(4i32);
pub const MBN_PROVIDER_STATE_VISIBLE: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(8i32);
pub const MBN_PROVIDER_STATE_REGISTERED: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(16i32);
pub const MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER: MBN_PROVIDER_STATE = MBN_PROVIDER_STATE(32i32);
impl ::core::convert::From<i32> for MBN_PROVIDER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_PROVIDER_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_RADIO(pub i32);
pub const MBN_RADIO_OFF: MBN_RADIO = MBN_RADIO(0i32);
pub const MBN_RADIO_ON: MBN_RADIO = MBN_RADIO(1i32);
impl ::core::convert::From<i32> for MBN_RADIO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_RADIO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_READY_STATE(pub i32);
pub const MBN_READY_STATE_OFF: MBN_READY_STATE = MBN_READY_STATE(0i32);
pub const MBN_READY_STATE_INITIALIZED: MBN_READY_STATE = MBN_READY_STATE(1i32);
pub const MBN_READY_STATE_SIM_NOT_INSERTED: MBN_READY_STATE = MBN_READY_STATE(2i32);
pub const MBN_READY_STATE_BAD_SIM: MBN_READY_STATE = MBN_READY_STATE(3i32);
pub const MBN_READY_STATE_FAILURE: MBN_READY_STATE = MBN_READY_STATE(4i32);
pub const MBN_READY_STATE_NOT_ACTIVATED: MBN_READY_STATE = MBN_READY_STATE(5i32);
pub const MBN_READY_STATE_DEVICE_LOCKED: MBN_READY_STATE = MBN_READY_STATE(6i32);
pub const MBN_READY_STATE_DEVICE_BLOCKED: MBN_READY_STATE = MBN_READY_STATE(7i32);
pub const MBN_READY_STATE_NO_ESIM_PROFILE: MBN_READY_STATE = MBN_READY_STATE(8i32);
impl ::core::convert::From<i32> for MBN_READY_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_READY_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_REGISTER_MODE(pub i32);
pub const MBN_REGISTER_MODE_NONE: MBN_REGISTER_MODE = MBN_REGISTER_MODE(0i32);
pub const MBN_REGISTER_MODE_AUTOMATIC: MBN_REGISTER_MODE = MBN_REGISTER_MODE(1i32);
pub const MBN_REGISTER_MODE_MANUAL: MBN_REGISTER_MODE = MBN_REGISTER_MODE(2i32);
impl ::core::convert::From<i32> for MBN_REGISTER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_REGISTER_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_REGISTER_STATE(pub i32);
pub const MBN_REGISTER_STATE_NONE: MBN_REGISTER_STATE = MBN_REGISTER_STATE(0i32);
pub const MBN_REGISTER_STATE_DEREGISTERED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(1i32);
pub const MBN_REGISTER_STATE_SEARCHING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(2i32);
pub const MBN_REGISTER_STATE_HOME: MBN_REGISTER_STATE = MBN_REGISTER_STATE(3i32);
pub const MBN_REGISTER_STATE_ROAMING: MBN_REGISTER_STATE = MBN_REGISTER_STATE(4i32);
pub const MBN_REGISTER_STATE_PARTNER: MBN_REGISTER_STATE = MBN_REGISTER_STATE(5i32);
pub const MBN_REGISTER_STATE_DENIED: MBN_REGISTER_STATE = MBN_REGISTER_STATE(6i32);
impl ::core::convert::From<i32> for MBN_REGISTER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_REGISTER_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_REGISTRATION_CONSTANTS(pub i32);
pub const MBN_ROAMTEXT_LEN: MBN_REGISTRATION_CONSTANTS = MBN_REGISTRATION_CONSTANTS(64i32);
pub const MBN_CDMA_DEFAULT_PROVIDER_ID: MBN_REGISTRATION_CONSTANTS = MBN_REGISTRATION_CONSTANTS(0i32);
impl ::core::convert::From<i32> for MBN_REGISTRATION_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_REGISTRATION_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_SIGNAL_CONSTANTS(pub i32);
pub const MBN_RSSI_DEFAULT: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(-1i32);
pub const MBN_RSSI_DISABLE: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(0i32);
pub const MBN_RSSI_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
pub const MBN_ERROR_RATE_UNKNOWN: MBN_SIGNAL_CONSTANTS = MBN_SIGNAL_CONSTANTS(99i32);
impl ::core::convert::From<i32> for MBN_SIGNAL_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_SIGNAL_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_SMS_CAPS(pub i32);
pub const MBN_SMS_CAPS_NONE: MBN_SMS_CAPS = MBN_SMS_CAPS(0i32);
pub const MBN_SMS_CAPS_PDU_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(1i32);
pub const MBN_SMS_CAPS_PDU_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(2i32);
pub const MBN_SMS_CAPS_TEXT_RECEIVE: MBN_SMS_CAPS = MBN_SMS_CAPS(4i32);
pub const MBN_SMS_CAPS_TEXT_SEND: MBN_SMS_CAPS = MBN_SMS_CAPS(8i32);
impl ::core::convert::From<i32> for MBN_SMS_CAPS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_SMS_CAPS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_SMS_CDMA_ENCODING(pub i32);
pub const MBN_SMS_CDMA_ENCODING_OCTET: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(0i32);
pub const MBN_SMS_CDMA_ENCODING_EPM: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(1i32);
pub const MBN_SMS_CDMA_ENCODING_7BIT_ASCII: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(2i32);
pub const MBN_SMS_CDMA_ENCODING_IA5: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(3i32);
pub const MBN_SMS_CDMA_ENCODING_UNICODE: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(4i32);
pub const MBN_SMS_CDMA_ENCODING_SHIFT_JIS: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(5i32);
pub const MBN_SMS_CDMA_ENCODING_KOREAN: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(6i32);
pub const MBN_SMS_CDMA_ENCODING_LATIN_HEBREW: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(7i32);
pub const MBN_SMS_CDMA_ENCODING_LATIN: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(8i32);
pub const MBN_SMS_CDMA_ENCODING_GSM_7BIT: MBN_SMS_CDMA_ENCODING = MBN_SMS_CDMA_ENCODING(9i32);
impl ::core::convert::From<i32> for MBN_SMS_CDMA_ENCODING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_SMS_CDMA_ENCODING {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_SMS_CDMA_LANG(pub i32);
pub const MBN_SMS_CDMA_LANG_NONE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(0i32);
pub const MBN_SMS_CDMA_LANG_ENGLISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(1i32);
pub const MBN_SMS_CDMA_LANG_FRENCH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(2i32);
pub const MBN_SMS_CDMA_LANG_SPANISH: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(3i32);
pub const MBN_SMS_CDMA_LANG_JAPANESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(4i32);
pub const MBN_SMS_CDMA_LANG_KOREAN: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(5i32);
pub const MBN_SMS_CDMA_LANG_CHINESE: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(6i32);
pub const MBN_SMS_CDMA_LANG_HEBREW: MBN_SMS_CDMA_LANG = MBN_SMS_CDMA_LANG(7i32);
impl ::core::convert::From<i32> for MBN_SMS_CDMA_LANG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_SMS_CDMA_LANG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MBN_SMS_FILTER {
    pub flag: MBN_SMS_FLAG,
    pub messageIndex: u32,
}
impl MBN_SMS_FILTER {}
impl ::core::default::Default for MBN_SMS_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MBN_SMS_FILTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_SMS_FILTER").field("flag", &self.flag).field("messageIndex", &self.messageIndex).finish()
    }
}
impl ::core::cmp::PartialEq for MBN_SMS_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag && self.messageIndex == other.messageIndex
    }
}
impl ::core::cmp::Eq for MBN_SMS_FILTER {}
unsafe impl ::windows::core::Abi for MBN_SMS_FILTER {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_SMS_FLAG(pub i32);
pub const MBN_SMS_FLAG_ALL: MBN_SMS_FLAG = MBN_SMS_FLAG(0i32);
pub const MBN_SMS_FLAG_INDEX: MBN_SMS_FLAG = MBN_SMS_FLAG(1i32);
pub const MBN_SMS_FLAG_NEW: MBN_SMS_FLAG = MBN_SMS_FLAG(2i32);
pub const MBN_SMS_FLAG_OLD: MBN_SMS_FLAG = MBN_SMS_FLAG(3i32);
pub const MBN_SMS_FLAG_SENT: MBN_SMS_FLAG = MBN_SMS_FLAG(4i32);
pub const MBN_SMS_FLAG_DRAFT: MBN_SMS_FLAG = MBN_SMS_FLAG(5i32);
impl ::core::convert::From<i32> for MBN_SMS_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_SMS_FLAG {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_SMS_FORMAT(pub i32);
pub const MBN_SMS_FORMAT_NONE: MBN_SMS_FORMAT = MBN_SMS_FORMAT(0i32);
pub const MBN_SMS_FORMAT_PDU: MBN_SMS_FORMAT = MBN_SMS_FORMAT(1i32);
pub const MBN_SMS_FORMAT_TEXT: MBN_SMS_FORMAT = MBN_SMS_FORMAT(2i32);
impl ::core::convert::From<i32> for MBN_SMS_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_SMS_FORMAT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_SMS_STATUS_FLAG(pub i32);
pub const MBN_SMS_FLAG_NONE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(0i32);
pub const MBN_SMS_FLAG_MESSAGE_STORE_FULL: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(1i32);
pub const MBN_SMS_FLAG_NEW_MESSAGE: MBN_SMS_STATUS_FLAG = MBN_SMS_STATUS_FLAG(2i32);
impl ::core::convert::From<i32> for MBN_SMS_STATUS_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_SMS_STATUS_FLAG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MBN_SMS_STATUS_INFO {
    pub flag: u32,
    pub messageIndex: u32,
}
impl MBN_SMS_STATUS_INFO {}
impl ::core::default::Default for MBN_SMS_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MBN_SMS_STATUS_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MBN_SMS_STATUS_INFO").field("flag", &self.flag).field("messageIndex", &self.messageIndex).finish()
    }
}
impl ::core::cmp::PartialEq for MBN_SMS_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.flag == other.flag && self.messageIndex == other.messageIndex
    }
}
impl ::core::cmp::Eq for MBN_SMS_STATUS_INFO {}
unsafe impl ::windows::core::Abi for MBN_SMS_STATUS_INFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_VOICE_CALL_STATE(pub i32);
pub const MBN_VOICE_CALL_STATE_NONE: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(0i32);
pub const MBN_VOICE_CALL_STATE_IN_PROGRESS: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(1i32);
pub const MBN_VOICE_CALL_STATE_HANGUP: MBN_VOICE_CALL_STATE = MBN_VOICE_CALL_STATE(2i32);
impl ::core::convert::From<i32> for MBN_VOICE_CALL_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_VOICE_CALL_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MBN_VOICE_CLASS(pub i32);
pub const MBN_VOICE_CLASS_NONE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(0i32);
pub const MBN_VOICE_CLASS_NO_VOICE: MBN_VOICE_CLASS = MBN_VOICE_CLASS(1i32);
pub const MBN_VOICE_CLASS_SEPARATE_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(2i32);
pub const MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA: MBN_VOICE_CLASS = MBN_VOICE_CLASS(3i32);
impl ::core::convert::From<i32> for MBN_VOICE_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MBN_VOICE_CLASS {
    type Abi = Self;
}
pub const MbnConnectionManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05c_4418_11dd_90ed_001c257ccff1);
pub const MbnConnectionProfileManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05a_4418_11dd_90ed_001c257ccff1);
pub const MbnDeviceServicesManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2269daa3_2a9f_4165_a501_ce00a6f7a75b);
pub const MbnInterfaceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdfee05b_4418_11dd_90ed_001c257ccff1);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WWAEXT_SMS_CONSTANTS(pub i32);
pub const MBN_MESSAGE_INDEX_NONE: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(0i32);
pub const MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(0i32);
pub const MBN_CDMA_SHORT_MSG_SIZE_MAX: WWAEXT_SMS_CONSTANTS = WWAEXT_SMS_CONSTANTS(160i32);
impl ::core::convert::From<i32> for WWAEXT_SMS_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WWAEXT_SMS_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct __DummyPinType__ {
    pub pinType: u32,
}
impl __DummyPinType__ {}
impl ::core::default::Default for __DummyPinType__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for __DummyPinType__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("__DummyPinType__").field("pinType", &self.pinType).finish()
    }
}
impl ::core::cmp::PartialEq for __DummyPinType__ {
    fn eq(&self, other: &Self) -> bool {
        self.pinType == other.pinType
    }
}
impl ::core::cmp::Eq for __DummyPinType__ {}
unsafe impl ::windows::core::Abi for __DummyPinType__ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct __mbnapi_ReferenceRemainingTypes__ {
    pub bandClass: MBN_BAND_CLASS,
    pub contextConstants: MBN_CONTEXT_CONSTANTS,
    pub ctrlCaps: MBN_CTRL_CAPS,
    pub dataClass: MBN_DATA_CLASS,
    pub interfaceCapsConstants: MBN_INTERFACE_CAPS_CONSTANTS,
    pub pinConstants: MBN_PIN_CONSTANTS,
    pub providerConstants: MBN_PROVIDER_CONSTANTS,
    pub providerState: MBN_PROVIDER_STATE,
    pub registrationConstants: MBN_REGISTRATION_CONSTANTS,
    pub signalConstants: MBN_SIGNAL_CONSTANTS,
    pub smsCaps: MBN_SMS_CAPS,
    pub smsConstants: WWAEXT_SMS_CONSTANTS,
    pub wwaextSmsConstants: WWAEXT_SMS_CONSTANTS,
    pub smsStatusFlag: MBN_SMS_STATUS_FLAG,
}
impl __mbnapi_ReferenceRemainingTypes__ {}
impl ::core::default::Default for __mbnapi_ReferenceRemainingTypes__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for __mbnapi_ReferenceRemainingTypes__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("__mbnapi_ReferenceRemainingTypes__")
            .field("bandClass", &self.bandClass)
            .field("contextConstants", &self.contextConstants)
            .field("ctrlCaps", &self.ctrlCaps)
            .field("dataClass", &self.dataClass)
            .field("interfaceCapsConstants", &self.interfaceCapsConstants)
            .field("pinConstants", &self.pinConstants)
            .field("providerConstants", &self.providerConstants)
            .field("providerState", &self.providerState)
            .field("registrationConstants", &self.registrationConstants)
            .field("signalConstants", &self.signalConstants)
            .field("smsCaps", &self.smsCaps)
            .field("smsConstants", &self.smsConstants)
            .field("wwaextSmsConstants", &self.wwaextSmsConstants)
            .field("smsStatusFlag", &self.smsStatusFlag)
            .finish()
    }
}
impl ::core::cmp::PartialEq for __mbnapi_ReferenceRemainingTypes__ {
    fn eq(&self, other: &Self) -> bool {
        self.bandClass == other.bandClass
            && self.contextConstants == other.contextConstants
            && self.ctrlCaps == other.ctrlCaps
            && self.dataClass == other.dataClass
            && self.interfaceCapsConstants == other.interfaceCapsConstants
            && self.pinConstants == other.pinConstants
            && self.providerConstants == other.providerConstants
            && self.providerState == other.providerState
            && self.registrationConstants == other.registrationConstants
            && self.signalConstants == other.signalConstants
            && self.smsCaps == other.smsCaps
            && self.smsConstants == other.smsConstants
            && self.wwaextSmsConstants == other.wwaextSmsConstants
            && self.smsStatusFlag == other.smsStatusFlag
    }
}
impl ::core::cmp::Eq for __mbnapi_ReferenceRemainingTypes__ {}
unsafe impl ::windows::core::Abi for __mbnapi_ReferenceRemainingTypes__ {
    type Abi = Self;
}
