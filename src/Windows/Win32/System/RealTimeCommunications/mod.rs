#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetworkTransportSettings(pub ::windows::core::IUnknown);
impl INetworkTransportSettings {
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(settingid), ::core::mem::transmute(lengthin), ::core::mem::transmute(valuein), ::core::mem::transmute(lengthout), ::core::mem::transmute(valueout)).ok()
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(settingid), ::core::mem::transmute(lengthin), ::core::mem::transmute(valuein), ::core::mem::transmute(lengthout), ::core::mem::transmute(valueout)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetworkTransportSettings {
    type Vtable = INetworkTransportSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e7abb2c_f2c1_4a61_bd35_deb7a08ab0f1);
}
impl ::core::convert::From<INetworkTransportSettings> for ::windows::core::IUnknown {
    fn from(value: INetworkTransportSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetworkTransportSettings> for ::windows::core::IUnknown {
    fn from(value: &INetworkTransportSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetworkTransportSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetworkTransportSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkTransportSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Networking_WinSock")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))] usize,
    #[cfg(feature = "Win32_Networking_WinSock")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INotificationTransportSync(pub ::windows::core::IUnknown);
impl INotificationTransportSync {
    pub unsafe fn CompleteDelivery(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for INotificationTransportSync {
    type Vtable = INotificationTransportSync_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eb1402_0ab8_49c0_9e14_a1ae4ba93058);
}
impl ::core::convert::From<INotificationTransportSync> for ::windows::core::IUnknown {
    fn from(value: INotificationTransportSync) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INotificationTransportSync> for ::windows::core::IUnknown {
    fn from(value: &INotificationTransportSync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INotificationTransportSync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INotificationTransportSync {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationTransportSync_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCBuddy(pub ::windows::core::IUnknown);
impl IRTCBuddy {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS> {
        let mut result__: <RTC_PRESENCE_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PRESENCE_STATUS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddy {
    type Vtable = IRTCBuddy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcb136c8_7b90_4e0c_befe_56edf0ba6f1c);
}
impl ::core::convert::From<IRTCBuddy> for ::windows::core::IUnknown {
    fn from(value: IRTCBuddy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCBuddy> for ::windows::core::IUnknown {
    fn from(value: &IRTCBuddy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCBuddy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCBuddy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCBuddy> for IRTCPresenceContact {
    fn from(value: IRTCBuddy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy> for IRTCPresenceContact {
    fn from(value: &IRTCBuddy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for IRTCBuddy {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for &IRTCBuddy {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpresentityuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfpersistent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fpersistent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrnotes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCBuddy2(pub ::windows::core::IUnknown);
impl IRTCBuddy2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS> {
        let mut result__: <RTC_PRESENCE_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PRESENCE_STATUS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__: <IRTCProfile2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> ::windows::core::Result<IRTCEnumGroups> {
        let mut result__: <IRTCEnumGroups as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumGroups>(result__)
    }
    pub unsafe fn Groups(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(enproperty), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn EnumeratePresenceDevices(&self) -> ::windows::core::Result<IRTCEnumPresenceDevices> {
        let mut result__: <IRTCEnumPresenceDevices as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumPresenceDevices>(result__)
    }
    pub unsafe fn PresenceDevices(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn SubscriptionType(&self) -> ::windows::core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE> {
        let mut result__: <RTC_BUDDY_SUBSCRIPTION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_BUDDY_SUBSCRIPTION_TYPE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddy2 {
    type Vtable = IRTCBuddy2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x102f9588_23e7_40e3_954d_cd7a1d5c0361);
}
impl ::core::convert::From<IRTCBuddy2> for ::windows::core::IUnknown {
    fn from(value: IRTCBuddy2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCBuddy2> for ::windows::core::IUnknown {
    fn from(value: &IRTCBuddy2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCBuddy2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCBuddy2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCBuddy2> for IRTCBuddy {
    fn from(value: IRTCBuddy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy2> for IRTCBuddy {
    fn from(value: &IRTCBuddy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCBuddy> for IRTCBuddy2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCBuddy> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCBuddy> for &IRTCBuddy2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCBuddy> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCBuddy2> for IRTCPresenceContact {
    fn from(value: IRTCBuddy2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddy2> for IRTCPresenceContact {
    fn from(value: &IRTCBuddy2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for IRTCBuddy2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for &IRTCBuddy2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddy2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpresentityuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfpersistent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fpersistent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrnotes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenumdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdevicescollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCBuddyEvent(pub ::windows::core::IUnknown);
impl IRTCBuddyEvent {
    pub unsafe fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__: <IRTCBuddy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCBuddy>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddyEvent {
    type Vtable = IRTCBuddyEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf36d755d_17e6_404e_954f_0fc07574c78d);
}
impl ::core::convert::From<IRTCBuddyEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCBuddyEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCBuddyEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCBuddyEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCBuddyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCBuddyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCBuddyEvent> for super::Com::IDispatch {
    fn from(value: IRTCBuddyEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCBuddyEvent> for super::Com::IDispatch {
    fn from(value: &IRTCBuddyEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCBuddyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCBuddyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCBuddyEvent2(pub ::windows::core::IUnknown);
impl IRTCBuddyEvent2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__: <IRTCBuddy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_BUDDY_EVENT_TYPE> {
        let mut result__: <RTC_BUDDY_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_BUDDY_EVENT_TYPE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddyEvent2 {
    type Vtable = IRTCBuddyEvent2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x484a7f1e_73f0_4990_bfc2_60bc3978a720);
}
impl ::core::convert::From<IRTCBuddyEvent2> for ::windows::core::IUnknown {
    fn from(value: IRTCBuddyEvent2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCBuddyEvent2> for ::windows::core::IUnknown {
    fn from(value: &IRTCBuddyEvent2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCBuddyEvent2> for IRTCBuddyEvent {
    fn from(value: IRTCBuddyEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCBuddyEvent2> for IRTCBuddyEvent {
    fn from(value: &IRTCBuddyEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCBuddyEvent> for IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCBuddyEvent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCBuddyEvent> for &IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCBuddyEvent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCBuddyEvent2> for super::Com::IDispatch {
    fn from(value: IRTCBuddyEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCBuddyEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCBuddyEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCBuddyEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyEvent2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCBuddyGroup(pub ::windows::core::IUnknown);
impl IRTCBuddyGroup {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrgroupname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrgroupname.into_param().abi()).ok()
    }
    pub unsafe fn AddBuddy<'a, Param0: ::windows::core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn RemoveBuddy<'a, Param0: ::windows::core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__: <IRTCEnumBuddies as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumBuddies>(result__)
    }
    pub unsafe fn Buddies(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__: <IRTCProfile2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile2>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddyGroup {
    type Vtable = IRTCBuddyGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60361e68_9164_4389_a4c6_d0b3925bda5e);
}
impl ::core::convert::From<IRTCBuddyGroup> for ::windows::core::IUnknown {
    fn from(value: IRTCBuddyGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCBuddyGroup> for ::windows::core::IUnknown {
    fn from(value: &IRTCBuddyGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCBuddyGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCBuddyGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrgroupname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCBuddyGroupEvent(pub ::windows::core::IUnknown);
impl IRTCBuddyGroupEvent {
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_GROUP_EVENT_TYPE> {
        let mut result__: <RTC_GROUP_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_GROUP_EVENT_TYPE>(result__)
    }
    pub unsafe fn Group(&self) -> ::windows::core::Result<IRTCBuddyGroup> {
        let mut result__: <IRTCBuddyGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCBuddyGroup>(result__)
    }
    pub unsafe fn Buddy(&self) -> ::windows::core::Result<IRTCBuddy2> {
        let mut result__: <IRTCBuddy2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCBuddy2>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCBuddyGroupEvent {
    type Vtable = IRTCBuddyGroupEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a79e1d1_b736_4414_96f8_bbc7f08863e4);
}
impl ::core::convert::From<IRTCBuddyGroupEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCBuddyGroupEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCBuddyGroupEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCBuddyGroupEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCBuddyGroupEvent> for super::Com::IDispatch {
    fn from(value: IRTCBuddyGroupEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCBuddyGroupEvent> for super::Com::IDispatch {
    fn from(value: &IRTCBuddyGroupEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCBuddyGroupEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCBuddyGroupEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClient(pub ::windows::core::IUnknown);
impl IRTCClient {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfilter)).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatypes), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSession<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(enlisten)).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> ::windows::core::Result<RTC_LISTEN_MODE> {
        let mut result__: <RTC_LISTEN_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_LISTEN_MODE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NetworkAddresses(&self, ftcp: i16, fexternal: i16) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ftcp), ::core::mem::transmute(fexternal), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAudioMuted(&self, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), ::core::mem::transmute(fmuted)).ok()
    }
    pub unsafe fn AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow> {
        let mut result__: <super::super::Media::DirectShow::IVideoWindow as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<super::super::Media::DirectShow::IVideoWindow>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredAudioDevice<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), bstrdevicename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetPreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredAEC(&self, benable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(benable)).ok()
    }
    pub unsafe fn PreferredAEC(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredVideoDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdevicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), bstrdevicename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreferredVideoDevice(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ActiveMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxbitrate)).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(lvalue)).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn NetworkQuality(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enapplet)).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(enapplet), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruseruri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), bstruseruri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    pub unsafe fn PlayRing(&self, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), ::core::mem::transmute(bplay)).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(endtmf)).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent)).ok()
    }
    pub unsafe fn IsTuned(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCClient {
    type Vtable = IRTCClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07829e45_9a34_408e_a011_bddf13487cd1);
}
impl ::core::convert::From<IRTCClient> for ::windows::core::IUnknown {
    fn from(value: IRTCClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClient> for ::windows::core::IUnknown {
    fn from(value: &IRTCClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lfilter: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plfilter: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatypes: i32, fpersistent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ftcp: i16, fexternal: i16, pvaddresses: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_DirectShow")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, benable: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdevicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxbitrate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lvalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plnetworkquality: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endtmf: RTC_DTMF) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwndparent: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pftuned: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClient2(pub ::windows::core::IUnknown);
impl IRTCClient2 {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PrepareForShutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfilter: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lfilter)).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredMediaTypes(&self, lmediatypes: i32, fpersistent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatypes), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn PreferredMediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MediaCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSession<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn SetListenForIncomingSessions(&self, enlisten: RTC_LISTEN_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(enlisten)).ok()
    }
    pub unsafe fn ListenForIncomingSessions(&self) -> ::windows::core::Result<RTC_LISTEN_MODE> {
        let mut result__: <RTC_LISTEN_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_LISTEN_MODE>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NetworkAddresses(&self, ftcp: i16, fexternal: i16) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ftcp), ::core::mem::transmute(fexternal), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn Volume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAudioMuted(&self, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), ::core::mem::transmute(fmuted)).ok()
    }
    pub unsafe fn AudioMuted(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn IVideoWindow(&self, endevice: RTC_VIDEO_DEVICE) -> ::windows::core::Result<super::super::Media::DirectShow::IVideoWindow> {
        let mut result__: <super::super::Media::DirectShow::IVideoWindow as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<super::super::Media::DirectShow::IVideoWindow>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredAudioDevice<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, endevice: RTC_AUDIO_DEVICE, bstrdevicename: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), bstrdevicename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreferredAudioDevice(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetPreferredVolume(&self, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn PreferredVolume(&self, endevice: RTC_AUDIO_DEVICE) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(endevice), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPreferredAEC(&self, benable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(benable)).ok()
    }
    pub unsafe fn PreferredAEC(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPreferredVideoDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdevicename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), bstrdevicename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreferredVideoDevice(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ActiveMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxBitrate(&self, lmaxbitrate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxbitrate)).ok()
    }
    pub unsafe fn MaxBitrate(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTemporalSpatialTradeOff(&self, lvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(lvalue)).ok()
    }
    pub unsafe fn TemporalSpatialTradeOff(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn NetworkQuality(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn StartT120Applet(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(enapplet)).ok()
    }
    pub unsafe fn StopT120Applets(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsT120AppletRunning(&self, enapplet: RTC_T120_APPLET) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(enapplet), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruseruri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), bstruseruri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalUserName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrusername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), bstrusername.into_param().abi()).ok()
    }
    pub unsafe fn PlayRing(&self, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), ::core::mem::transmute(bplay)).ok()
    }
    pub unsafe fn SendDTMF(&self, endtmf: RTC_DTMF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(endtmf)).ok()
    }
    pub unsafe fn InvokeTuningWizard(&self, hwndparent: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent)).ok()
    }
    pub unsafe fn IsTuned(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAnswerMode(&self, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), ::core::mem::transmute(enmode)).ok()
    }
    pub unsafe fn AnswerMode(&self, entype: RTC_SESSION_TYPE) -> ::windows::core::Result<RTC_ANSWER_MODE> {
        let mut result__: <RTC_ANSWER_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), &mut result__).from_abi::<RTC_ANSWER_MODE>(result__)
    }
    pub unsafe fn InvokeTuningWizardEx(&self, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(fallowaudio), ::core::mem::transmute(fallowvideo)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrclientname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), bstrclientname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCurVer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrclientcurver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), bstrclientcurver.into_param().abi()).ok()
    }
    pub unsafe fn InitializeEx(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSessionWithDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1, pprofile: Param2, lflags: i32) -> ::windows::core::Result<IRTCSession2> {
        let mut result__: <IRTCSession2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn SetSessionDescriptionManager<'a, Param0: ::windows::core::IntoParam<'a, IRTCSessionDescriptionManager>>(&self, psessiondescriptionmanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), psessiondescriptionmanager.into_param().abi()).ok()
    }
    pub unsafe fn SetPreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(ensecuritylevel)).ok()
    }
    pub unsafe fn PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__: <RTC_SECURITY_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(ensecuritytype), &mut result__).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn SetAllowedPorts(&self, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltransport), ::core::mem::transmute(enlistenmode)).ok()
    }
    pub unsafe fn AllowedPorts(&self, ltransport: i32) -> ::windows::core::Result<RTC_LISTEN_MODE> {
        let mut result__: <RTC_LISTEN_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltransport), &mut result__).from_abi::<RTC_LISTEN_MODE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCClient2 {
    type Vtable = IRTCClient2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c91d71d_1064_42da_bfa5_572beb8eea84);
}
impl ::core::convert::From<IRTCClient2> for ::windows::core::IUnknown {
    fn from(value: IRTCClient2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClient2> for ::windows::core::IUnknown {
    fn from(value: &IRTCClient2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCClient2> for IRTCClient {
    fn from(value: IRTCClient2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClient2> for IRTCClient {
    fn from(value: &IRTCClient2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCClient> for IRTCClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCClient> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCClient> for &IRTCClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCClient> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClient2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lfilter: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plfilter: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatypes: i32, fpersistent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enlisten: RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penlisten: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ftcp: i16, fexternal: i16, pvaddresses: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, fmuted: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_DirectShow")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, benable: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdevicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdevicename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxbitrate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxbitrate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lvalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plvalue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plnetworkquality: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enapplet: RTC_T120_APPLET) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enapplet: RTC_T120_APPLET, pfrunning: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_RING_TYPE, bplay: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endtmf: RTC_DTMF) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwndparent: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pftuned: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwndparent: isize, fallowaudio: i16, fallowvideo: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrclientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrclientcurver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psessiondescriptionmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClientEvent(pub ::windows::core::IUnknown);
impl IRTCClientEvent {
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_CLIENT_EVENT_TYPE> {
        let mut result__: <RTC_CLIENT_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_CLIENT_EVENT_TYPE>(result__)
    }
    pub unsafe fn Client(&self) -> ::windows::core::Result<IRTCClient> {
        let mut result__: <IRTCClient as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCClient>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCClientEvent {
    type Vtable = IRTCClientEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b493b7a_3cba_4170_9c8b_76a9dacdd644);
}
impl ::core::convert::From<IRTCClientEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCClientEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClientEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCClientEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClientEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClientEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCClientEvent> for super::Com::IDispatch {
    fn from(value: IRTCClientEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCClientEvent> for super::Com::IDispatch {
    fn from(value: &IRTCClientEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCClientEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCClientEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClientPortManagement(pub ::windows::core::IUnknown);
impl IRTCClientPortManagement {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartListenAddressAndPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternallocaladdress: Param0, linternallocalport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StopListenAddressAndPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternallocaladdress: Param0, linternallocalport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport)).ok()
    }
    pub unsafe fn GetPortRange(&self, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(enporttype), ::core::mem::transmute(plminvalue), ::core::mem::transmute(plmaxvalue)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientPortManagement {
    type Vtable = IRTCClientPortManagement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5df3f03_4bde_4417_aefe_71177bdaea66);
}
impl ::core::convert::From<IRTCClientPortManagement> for ::windows::core::IUnknown {
    fn from(value: IRTCClientPortManagement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClientPortManagement> for ::windows::core::IUnknown {
    fn from(value: &IRTCClientPortManagement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClientPortManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClientPortManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPortManagement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClientPresence(pub ::windows::core::IUnknown);
impl IRTCClientPresence {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnablePresence<'a, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, fusestorage: i16, varstorage: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(fusestorage), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Export<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0, freplaceall: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), varstorage.into_param().abi(), ::core::mem::transmute(freplaceall)).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__: <IRTCEnumBuddies as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumBuddies>(result__)
    }
    pub unsafe fn Buddies(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buddy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__: <IRTCBuddy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi::<IRTCBuddy>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddBuddy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fpersistent: i16, pprofile: Param4, lflags: i32) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__: <IRTCBuddy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fpersistent), pprofile.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn RemoveBuddy<'a, Param0: ::windows::core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> ::windows::core::Result<IRTCEnumWatchers> {
        let mut result__: <IRTCEnumWatchers as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumWatchers>(result__)
    }
    pub unsafe fn Watchers(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Watcher<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__: <IRTCWatcher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi::<IRTCWatcher>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddWatcher<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fblocked: i16, fpersistent: i16) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__: <IRTCWatcher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fblocked), ::core::mem::transmute(fpersistent), &mut result__).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn RemoveWatcher<'a, Param0: ::windows::core::IntoParam<'a, IRTCWatcher>>(&self, pwatcher: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pwatcher.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPresenceInfo<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(enstatus), bstrnotes.into_param().abi()).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__: <RTC_OFFER_WATCHER_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_OFFER_WATCHER_MODE>(result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(enmode)).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> ::windows::core::Result<RTC_PRIVACY_MODE> {
        let mut result__: <RTC_PRIVACY_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PRIVACY_MODE>(result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(enmode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientPresence {
    type Vtable = IRTCClientPresence_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c3cbcc_0744_42d1_968a_51aa1bb274c6);
}
impl ::core::convert::From<IRTCClientPresence> for ::windows::core::IUnknown {
    fn from(value: IRTCClientPresence) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClientPresence> for ::windows::core::IUnknown {
    fn from(value: &IRTCClientPresence) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClientPresence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClientPresence {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwatcher: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClientPresence2(pub ::windows::core::IUnknown);
impl IRTCClientPresence2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnablePresence<'a, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, fusestorage: i16, varstorage: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(fusestorage), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Export<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), varstorage.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Import<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varstorage: Param0, freplaceall: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), varstorage.into_param().abi(), ::core::mem::transmute(freplaceall)).ok()
    }
    pub unsafe fn EnumerateBuddies(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__: <IRTCEnumBuddies as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumBuddies>(result__)
    }
    pub unsafe fn Buddies(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Buddy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__: <IRTCBuddy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi::<IRTCBuddy>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddBuddy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fpersistent: i16, pprofile: Param4, lflags: i32) -> ::windows::core::Result<IRTCBuddy> {
        let mut result__: <IRTCBuddy as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fpersistent), pprofile.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IRTCBuddy>(result__)
    }
    pub unsafe fn RemoveBuddy<'a, Param0: ::windows::core::IntoParam<'a, IRTCBuddy>>(&self, pbuddy: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pbuddy.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateWatchers(&self) -> ::windows::core::Result<IRTCEnumWatchers> {
        let mut result__: <IRTCEnumWatchers as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumWatchers>(result__)
    }
    pub unsafe fn Watchers(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Watcher<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__: <IRTCWatcher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), &mut result__).from_abi::<IRTCWatcher>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddWatcher<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0, bstrusername: Param1, bstrdata: Param2, fblocked: i16, fpersistent: i16) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__: <IRTCWatcher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fblocked), ::core::mem::transmute(fpersistent), &mut result__).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn RemoveWatcher<'a, Param0: ::windows::core::IntoParam<'a, IRTCWatcher>>(&self, pwatcher: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pwatcher.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalPresenceInfo<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, enstatus: RTC_PRESENCE_STATUS, bstrnotes: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(enstatus), bstrnotes.into_param().abi()).ok()
    }
    pub unsafe fn OfferWatcherMode(&self) -> ::windows::core::Result<RTC_OFFER_WATCHER_MODE> {
        let mut result__: <RTC_OFFER_WATCHER_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_OFFER_WATCHER_MODE>(result__)
    }
    pub unsafe fn SetOfferWatcherMode(&self, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(enmode)).ok()
    }
    pub unsafe fn PrivacyMode(&self) -> ::windows::core::Result<RTC_PRIVACY_MODE> {
        let mut result__: <RTC_PRIVACY_MODE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PRIVACY_MODE>(result__)
    }
    pub unsafe fn SetPrivacyMode(&self, enmode: RTC_PRIVACY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(enmode)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnablePresenceEx<'a, Param0: ::windows::core::IntoParam<'a, IRTCProfile>, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, pprofile: Param0, varstorage: Param1, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pprofile.into_param().abi(), varstorage.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    pub unsafe fn DisablePresence(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, bstrgroupname: Param0, bstrdata: Param1, pprofile: Param2, lflags: i32) -> ::windows::core::Result<IRTCBuddyGroup> {
        let mut result__: <IRTCBuddyGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), bstrgroupname.into_param().abi(), bstrdata.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IRTCBuddyGroup>(result__)
    }
    pub unsafe fn RemoveGroup<'a, Param0: ::windows::core::IntoParam<'a, IRTCBuddyGroup>>(&self, pgroup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pgroup.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateGroups(&self) -> ::windows::core::Result<IRTCEnumGroups> {
        let mut result__: <IRTCEnumGroups as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumGroups>(result__)
    }
    pub unsafe fn Groups(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Group<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrgroupname: Param0) -> ::windows::core::Result<IRTCBuddyGroup> {
        let mut result__: <IRTCBuddyGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), bstrgroupname.into_param().abi(), &mut result__).from_abi::<IRTCBuddyGroup>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddWatcherEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, IRTCProfile>>(
        &self,
        bstrpresentityuri: Param0,
        bstrusername: Param1,
        bstrdata: Param2,
        enstate: RTC_WATCHER_STATE,
        fpersistent: i16,
        enscope: RTC_ACE_SCOPE,
        pprofile: Param6,
        lflags: i32,
    ) -> ::windows::core::Result<IRTCWatcher2> {
        let mut result__: <IRTCWatcher2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(
            ::core::mem::transmute_copy(self),
            bstrpresentityuri.into_param().abi(),
            bstrusername.into_param().abi(),
            bstrdata.into_param().abi(),
            ::core::mem::transmute(enstate),
            ::core::mem::transmute(fpersistent),
            ::core::mem::transmute(enscope),
            pprofile.into_param().abi(),
            ::core::mem::transmute(lflags),
            &mut result__,
        )
        .from_abi::<IRTCWatcher2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WatcherEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: Param1) -> ::windows::core::Result<IRTCWatcher2> {
        let mut result__: <IRTCWatcher2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(enmode), bstrpresentityuri.into_param().abi(), &mut result__).from_abi::<IRTCWatcher2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPresenceProperty<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(enproperty), bstrproperty.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(enproperty), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPresenceData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnamespace: Param0, bstrdata: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), bstrnamespace.into_param().abi(), bstrdata.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(penstatus), ::core::mem::transmute(pbstrnotes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddBuddyEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, IRTCProfile>>(
        &self,
        bstrpresentityuri: Param0,
        bstrusername: Param1,
        bstrdata: Param2,
        fpersistent: i16,
        ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE,
        pprofile: Param5,
        lflags: i32,
    ) -> ::windows::core::Result<IRTCBuddy2> {
        let mut result__: <IRTCBuddy2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi(), bstrusername.into_param().abi(), bstrdata.into_param().abi(), ::core::mem::transmute(fpersistent), ::core::mem::transmute(ensubscriptiontype), pprofile.into_param().abi(), ::core::mem::transmute(lflags), &mut result__).from_abi::<IRTCBuddy2>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCClientPresence2 {
    type Vtable = IRTCClientPresence2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad1809e8_62f7_4783_909a_29c9d2cb1d34);
}
impl ::core::convert::From<IRTCClientPresence2> for ::windows::core::IUnknown {
    fn from(value: IRTCClientPresence2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClientPresence2> for ::windows::core::IUnknown {
    fn from(value: &IRTCClientPresence2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClientPresence2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClientPresence2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCClientPresence2> for IRTCClientPresence {
    fn from(value: IRTCClientPresence2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientPresence2> for IRTCClientPresence {
    fn from(value: &IRTCClientPresence2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCClientPresence> for IRTCClientPresence2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCClientPresence> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCClientPresence> for &IRTCClientPresence2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCClientPresence> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientPresence2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fusestorage: i16, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, freplaceall: i16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuddy: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fblocked: i16, fpersistent: i16, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwatcher: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penmode: *mut RTC_PRIVACY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enmode: RTC_PRIVACY_MODE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, varstorage: ::core::mem::ManuallyDrop<super::Com::VARIANT>, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: i16, enscope: RTC_ACE_SCOPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrnamespace: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersistent: i16, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::windows::core::RawPtr, lflags: i32, ppbuddy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClientProvisioning(pub ::windows::core::IUnknown);
impl IRTCClientProvisioning {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprofilexml: Param0) -> ::windows::core::Result<IRTCProfile> {
        let mut result__: <IRTCProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrprofilexml.into_param().abi(), &mut result__).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn EnableProfile<'a, Param0: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0, lregisterflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprofile.into_param().abi(), ::core::mem::transmute(lregisterflags)).ok()
    }
    pub unsafe fn DisableProfile<'a, Param0: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> ::windows::core::Result<IRTCEnumProfiles> {
        let mut result__: <IRTCEnumProfiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumProfiles>(result__)
    }
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruseraccount: Param0, bstruserpassword: Param1, bstruseruri: Param2, bstrserver: Param3, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstruseraccount.into_param().abi(), bstruserpassword.into_param().abi(), bstruseruri.into_param().abi(), bstrserver.into_param().abi(), ::core::mem::transmute(ltransport), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCClientProvisioning {
    type Vtable = IRTCClientProvisioning_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f5cf06_65b9_4a80_a0e6_73cae3ef3822);
}
impl ::core::convert::From<IRTCClientProvisioning> for ::windows::core::IUnknown {
    fn from(value: IRTCClientProvisioning) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClientProvisioning> for ::windows::core::IUnknown {
    fn from(value: &IRTCClientProvisioning) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClientProvisioning {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClientProvisioning {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrprofilexml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lregisterflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCClientProvisioning2(pub ::windows::core::IUnknown);
impl IRTCClientProvisioning2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprofilexml: Param0) -> ::windows::core::Result<IRTCProfile> {
        let mut result__: <IRTCProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrprofilexml.into_param().abi(), &mut result__).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn EnableProfile<'a, Param0: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0, lregisterflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprofile.into_param().abi(), ::core::mem::transmute(lregisterflags)).ok()
    }
    pub unsafe fn DisableProfile<'a, Param0: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pprofile.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateProfiles(&self) -> ::windows::core::Result<IRTCEnumProfiles> {
        let mut result__: <IRTCEnumProfiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumProfiles>(result__)
    }
    pub unsafe fn Profiles(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruseraccount: Param0, bstruserpassword: Param1, bstruseruri: Param2, bstrserver: Param3, ltransport: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstruseraccount.into_param().abi(), bstruserpassword.into_param().abi(), bstruseruri.into_param().abi(), bstrserver.into_param().abi(), ::core::mem::transmute(ltransport), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn EnableProfileEx<'a, Param0: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, pprofile: Param0, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pprofile.into_param().abi(), ::core::mem::transmute(lregisterflags), ::core::mem::transmute(lroamingflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCClientProvisioning2 {
    type Vtable = IRTCClientProvisioning2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa70909b5_f40e_4587_bb75_e6bc0845023e);
}
impl ::core::convert::From<IRTCClientProvisioning2> for ::windows::core::IUnknown {
    fn from(value: IRTCClientProvisioning2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCClientProvisioning2> for ::windows::core::IUnknown {
    fn from(value: &IRTCClientProvisioning2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCClientProvisioning2> for IRTCClientProvisioning {
    fn from(value: IRTCClientProvisioning2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCClientProvisioning2> for IRTCClientProvisioning {
    fn from(value: &IRTCClientProvisioning2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCClientProvisioning> for IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCClientProvisioning> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCClientProvisioning> for &IRTCClientProvisioning2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCClientProvisioning> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCClientProvisioning2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrprofilexml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lregisterflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruserpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ltransport: i32, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lregisterflags: i32, lroamingflags: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCCollection(pub ::windows::core::IUnknown);
impl IRTCCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCCollection {
    type Vtable = IRTCCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec7c8096_b918_4044_94f1_e4fba0361d5c);
}
impl ::core::convert::From<IRTCCollection> for ::windows::core::IUnknown {
    fn from(value: IRTCCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCCollection> for ::windows::core::IUnknown {
    fn from(value: &IRTCCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCCollection> for super::Com::IDispatch {
    fn from(value: IRTCCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCCollection> for super::Com::IDispatch {
    fn from(value: &IRTCCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, pvariant: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppnewenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCDispatchEventNotification(pub ::windows::core::IUnknown);
impl IRTCDispatchEventNotification {}
unsafe impl ::windows::core::Interface for IRTCDispatchEventNotification {
    type Vtable = IRTCDispatchEventNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x176ddfbe_fec0_4d55_bc87_84cff1ef7f91);
}
impl ::core::convert::From<IRTCDispatchEventNotification> for ::windows::core::IUnknown {
    fn from(value: IRTCDispatchEventNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCDispatchEventNotification> for ::windows::core::IUnknown {
    fn from(value: &IRTCDispatchEventNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCDispatchEventNotification> for super::Com::IDispatch {
    fn from(value: IRTCDispatchEventNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCDispatchEventNotification> for super::Com::IDispatch {
    fn from(value: &IRTCDispatchEventNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCDispatchEventNotification {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCDispatchEventNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEnumBuddies(pub ::windows::core::IUnknown);
impl IRTCEnumBuddies {
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddy>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumBuddies> {
        let mut result__: <IRTCEnumBuddies as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumBuddies>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumBuddies {
    type Vtable = IRTCEnumBuddies_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7296917_5569_4b3b_b3af_98d1144b2b87);
}
impl ::core::convert::From<IRTCEnumBuddies> for ::windows::core::IUnknown {
    fn from(value: IRTCEnumBuddies) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEnumBuddies> for ::windows::core::IUnknown {
    fn from(value: &IRTCEnumBuddies) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEnumBuddies {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEnumBuddies {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumBuddies_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEnumGroups(pub ::windows::core::IUnknown);
impl IRTCEnumGroups {
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddyGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumGroups> {
        let mut result__: <IRTCEnumGroups as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumGroups>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumGroups {
    type Vtable = IRTCEnumGroups_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x742378d6_a141_4415_8f27_35d99076cf5d);
}
impl ::core::convert::From<IRTCEnumGroups> for ::windows::core::IUnknown {
    fn from(value: IRTCEnumGroups) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEnumGroups> for ::windows::core::IUnknown {
    fn from(value: &IRTCEnumGroups) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEnumGroups {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEnumGroups {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumGroups_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEnumParticipants(pub ::windows::core::IUnknown);
impl IRTCEnumParticipants {
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCParticipant>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumParticipants> {
        let mut result__: <IRTCEnumParticipants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumParticipants>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumParticipants {
    type Vtable = IRTCEnumParticipants_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd56f29_4a4f_41b2_ba5c_f5bccc060bf6);
}
impl ::core::convert::From<IRTCEnumParticipants> for ::windows::core::IUnknown {
    fn from(value: IRTCEnumParticipants) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEnumParticipants> for ::windows::core::IUnknown {
    fn from(value: &IRTCEnumParticipants) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEnumParticipants {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEnumParticipants {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumParticipants_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEnumPresenceDevices(pub ::windows::core::IUnknown);
impl IRTCEnumPresenceDevices {
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCPresenceDevice>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumPresenceDevices> {
        let mut result__: <IRTCEnumPresenceDevices as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumPresenceDevices>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumPresenceDevices {
    type Vtable = IRTCEnumPresenceDevices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x708c2ab7_8bf8_42f8_8c7d_635197ad5539);
}
impl ::core::convert::From<IRTCEnumPresenceDevices> for ::windows::core::IUnknown {
    fn from(value: IRTCEnumPresenceDevices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEnumPresenceDevices> for ::windows::core::IUnknown {
    fn from(value: &IRTCEnumPresenceDevices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEnumPresenceDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEnumPresenceDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumPresenceDevices_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEnumProfiles(pub ::windows::core::IUnknown);
impl IRTCEnumProfiles {
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCProfile>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumProfiles> {
        let mut result__: <IRTCEnumProfiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumProfiles>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumProfiles {
    type Vtable = IRTCEnumProfiles_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29b7c41c_ed82_4bca_84ad_39d5101b58e3);
}
impl ::core::convert::From<IRTCEnumProfiles> for ::windows::core::IUnknown {
    fn from(value: IRTCEnumProfiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEnumProfiles> for ::windows::core::IUnknown {
    fn from(value: &IRTCEnumProfiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEnumProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEnumProfiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumProfiles_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEnumUserSearchResults(pub ::windows::core::IUnknown);
impl IRTCEnumUserSearchResults {
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCUserSearchResult>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumUserSearchResults> {
        let mut result__: <IRTCEnumUserSearchResults as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumUserSearchResults>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumUserSearchResults {
    type Vtable = IRTCEnumUserSearchResults_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d4d877_aa5d_4a5b_8d0e_002a8067e0e8);
}
impl ::core::convert::From<IRTCEnumUserSearchResults> for ::windows::core::IUnknown {
    fn from(value: IRTCEnumUserSearchResults) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEnumUserSearchResults> for ::windows::core::IUnknown {
    fn from(value: &IRTCEnumUserSearchResults) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEnumUserSearchResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEnumUserSearchResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumUserSearchResults_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEnumWatchers(pub ::windows::core::IUnknown);
impl IRTCEnumWatchers {
    pub unsafe fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<IRTCWatcher>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(ppelements), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IRTCEnumWatchers> {
        let mut result__: <IRTCEnumWatchers as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumWatchers>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCEnumWatchers {
    type Vtable = IRTCEnumWatchers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87d55d7_db74_4ed1_9ca4_77a0e41b413e);
}
impl ::core::convert::From<IRTCEnumWatchers> for ::windows::core::IUnknown {
    fn from(value: IRTCEnumWatchers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEnumWatchers> for ::windows::core::IUnknown {
    fn from(value: &IRTCEnumWatchers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEnumWatchers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEnumWatchers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEnumWatchers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCEventNotification(pub ::windows::core::IUnknown);
impl IRTCEventNotification {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Event<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, rtcevent: RTC_EVENT, pevent: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtcevent), pevent.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCEventNotification {
    type Vtable = IRTCEventNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13fa24c7_5748_4b21_91f5_7397609ce747);
}
impl ::core::convert::From<IRTCEventNotification> for ::windows::core::IUnknown {
    fn from(value: IRTCEventNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCEventNotification> for ::windows::core::IUnknown {
    fn from(value: &IRTCEventNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCEventNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCEventNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCEventNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rtcevent: RTC_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCInfoEvent(pub ::windows::core::IUnknown);
impl IRTCInfoEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__: <IRTCSession2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__: <IRTCParticipant as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCParticipant>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Info(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InfoHeader(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCInfoEvent {
    type Vtable = IRTCInfoEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e1d68ae_1912_4f49_b2c3_594fadfd425f);
}
impl ::core::convert::From<IRTCInfoEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCInfoEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCInfoEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCInfoEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCInfoEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCInfoEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCInfoEvent> for super::Com::IDispatch {
    fn from(value: IRTCInfoEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCInfoEvent> for super::Com::IDispatch {
    fn from(value: &IRTCInfoEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCInfoEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCInfoEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCInfoEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrinfo: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrinfoheader: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCIntensityEvent(pub ::windows::core::IUnknown);
impl IRTCIntensityEvent {
    pub unsafe fn Level(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Min(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Max(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Direction(&self) -> ::windows::core::Result<RTC_AUDIO_DEVICE> {
        let mut result__: <RTC_AUDIO_DEVICE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_AUDIO_DEVICE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCIntensityEvent {
    type Vtable = IRTCIntensityEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c23bf51_390c_4992_a41d_41eec05b2a4b);
}
impl ::core::convert::From<IRTCIntensityEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCIntensityEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCIntensityEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCIntensityEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCIntensityEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCIntensityEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCIntensityEvent> for super::Com::IDispatch {
    fn from(value: IRTCIntensityEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCIntensityEvent> for super::Com::IDispatch {
    fn from(value: &IRTCIntensityEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCIntensityEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCIntensityEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCIntensityEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pllevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmin: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmax: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCMediaEvent(pub ::windows::core::IUnknown);
impl IRTCMediaEvent {
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_MEDIA_EVENT_TYPE> {
        let mut result__: <RTC_MEDIA_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_MEDIA_EVENT_TYPE>(result__)
    }
    pub unsafe fn EventReason(&self) -> ::windows::core::Result<RTC_MEDIA_EVENT_REASON> {
        let mut result__: <RTC_MEDIA_EVENT_REASON as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_MEDIA_EVENT_REASON>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCMediaEvent {
    type Vtable = IRTCMediaEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x099944fb_bcda_453e_8c41_e13da2adf7f3);
}
impl ::core::convert::From<IRTCMediaEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCMediaEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCMediaEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCMediaEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCMediaEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCMediaEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCMediaEvent> for super::Com::IDispatch {
    fn from(value: IRTCMediaEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCMediaEvent> for super::Com::IDispatch {
    fn from(value: &IRTCMediaEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCMediaEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCMediaEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMediaEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmediatype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCMediaRequestEvent(pub ::windows::core::IUnknown);
impl IRTCMediaRequestEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__: <IRTCSession2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn ProposedMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Accept(&self, lmediatypes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatypes)).ok()
    }
    pub unsafe fn RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__: <RTC_SECURITY_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ensecuritytype), &mut result__).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn Reject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REINVITE_STATE> {
        let mut result__: <RTC_REINVITE_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_REINVITE_STATE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCMediaRequestEvent {
    type Vtable = IRTCMediaRequestEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52572d15_148c_4d97_a36c_2da55c289d63);
}
impl ::core::convert::From<IRTCMediaRequestEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCMediaRequestEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCMediaRequestEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCMediaRequestEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCMediaRequestEvent> for super::Com::IDispatch {
    fn from(value: IRTCMediaRequestEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCMediaRequestEvent> for super::Com::IDispatch {
    fn from(value: &IRTCMediaRequestEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCMediaRequestEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMediaRequestEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatypes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCMessagingEvent(pub ::windows::core::IUnknown);
impl IRTCMessagingEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__: <IRTCParticipant as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_MESSAGING_EVENT_TYPE> {
        let mut result__: <RTC_MESSAGING_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_MESSAGING_EVENT_TYPE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageHeader(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn UserStatus(&self) -> ::windows::core::Result<RTC_MESSAGING_USER_STATUS> {
        let mut result__: <RTC_MESSAGING_USER_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_MESSAGING_USER_STATUS>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCMessagingEvent {
    type Vtable = IRTCMessagingEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3609541_1b29_4de5_a4ad_5aebaf319512);
}
impl ::core::convert::From<IRTCMessagingEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCMessagingEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCMessagingEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCMessagingEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCMessagingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCMessagingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCMessagingEvent> for super::Com::IDispatch {
    fn from(value: IRTCMessagingEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCMessagingEvent> for super::Com::IDispatch {
    fn from(value: &IRTCMessagingEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCMessagingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCMessagingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCMessagingEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmessage: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmessageheader: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCParticipant(pub ::windows::core::IUnknown);
impl IRTCParticipant {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Removable(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__: <RTC_PARTICIPANT_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PARTICIPANT_STATE>(result__)
    }
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCParticipant {
    type Vtable = IRTCParticipant_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae86add5_26b1_4414_af1d_b94cd938d739);
}
impl ::core::convert::From<IRTCParticipant> for ::windows::core::IUnknown {
    fn from(value: IRTCParticipant) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCParticipant> for ::windows::core::IUnknown {
    fn from(value: &IRTCParticipant) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCParticipant {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipant_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfremovable: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCParticipantStateChangeEvent(pub ::windows::core::IUnknown);
impl IRTCParticipantStateChangeEvent {
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__: <IRTCParticipant as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_PARTICIPANT_STATE> {
        let mut result__: <RTC_PARTICIPANT_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PARTICIPANT_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCParticipantStateChangeEvent {
    type Vtable = IRTCParticipantStateChangeEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09bcb597_f0fa_48f9_b420_468cea7fde04);
}
impl ::core::convert::From<IRTCParticipantStateChangeEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCParticipantStateChangeEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCParticipantStateChangeEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCParticipantStateChangeEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCParticipantStateChangeEvent> for super::Com::IDispatch {
    fn from(value: IRTCParticipantStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCParticipantStateChangeEvent> for super::Com::IDispatch {
    fn from(value: &IRTCParticipantStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCParticipantStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCParticipantStateChangeEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCPortManager(pub ::windows::core::IUnknown);
impl IRTCPortManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMapping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremoteaddress: Param0, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut super::super::Foundation::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut super::super::Foundation::BSTR, plexternallocalport: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrremoteaddress.into_param().abi(), ::core::mem::transmute(enporttype), ::core::mem::transmute(pbstrinternallocaladdress), ::core::mem::transmute(plinternallocalport), ::core::mem::transmute(pbstrexternallocaladdress), ::core::mem::transmute(plexternallocalport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateRemoteAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrremoteaddress: Param0, bstrinternallocaladdress: Param1, linternallocalport: i32, bstrexternallocaladdress: Param3, lexternallocalport: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrremoteaddress.into_param().abi(), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport), bstrexternallocaladdress.into_param().abi(), ::core::mem::transmute(lexternallocalport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseMapping<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinternallocaladdress: Param0, linternallocalport: i32, bstrexternallocaladdress: Param2, lexternallocaladdress: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrinternallocaladdress.into_param().abi(), ::core::mem::transmute(linternallocalport), bstrexternallocaladdress.into_param().abi(), ::core::mem::transmute(lexternallocaladdress)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCPortManager {
    type Vtable = IRTCPortManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda77c14b_6208_43ca_8ddf_5b60a0a69fac);
}
impl ::core::convert::From<IRTCPortManager> for ::windows::core::IUnknown {
    fn from(value: IRTCPortManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCPortManager> for ::windows::core::IUnknown {
    fn from(value: &IRTCPortManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCPortManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCPortManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPortManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plexternallocalport: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrremoteaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocalport: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrinternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lexternallocaladdress: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCPresenceContact(pub ::windows::core::IUnknown);
impl IRTCPresenceContact {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fpersistent)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCPresenceContact {
    type Vtable = IRTCPresenceContact_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b22f92c_cd90_42db_a733_212205c3e3df);
}
impl ::core::convert::From<IRTCPresenceContact> for ::windows::core::IUnknown {
    fn from(value: IRTCPresenceContact) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCPresenceContact> for ::windows::core::IUnknown {
    fn from(value: &IRTCPresenceContact) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCPresenceContact {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCPresenceContact {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceContact_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpresentityuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfpersistent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fpersistent: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCPresenceDataEvent(pub ::windows::core::IUnknown);
impl IRTCPresenceDataEvent {
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCPresenceDataEvent {
    type Vtable = IRTCPresenceDataEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38f0e78c_8b87_4c04_a82d_aedd83c909bb);
}
impl ::core::convert::From<IRTCPresenceDataEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCPresenceDataEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCPresenceDataEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCPresenceDataEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCPresenceDataEvent> for super::Com::IDispatch {
    fn from(value: IRTCPresenceDataEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCPresenceDataEvent> for super::Com::IDispatch {
    fn from(value: &IRTCPresenceDataEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCPresenceDataEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDataEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrnamespace: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCPresenceDevice(pub ::windows::core::IUnknown);
impl IRTCPresenceDevice {
    pub unsafe fn Status(&self) -> ::windows::core::Result<RTC_PRESENCE_STATUS> {
        let mut result__: <RTC_PRESENCE_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PRESENCE_STATUS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresenceProperty(&self, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(enproperty), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresenceData(&self, pbstrnamespace: *mut super::super::Foundation::BSTR, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrnamespace), ::core::mem::transmute(pbstrdata)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCPresenceDevice {
    type Vtable = IRTCPresenceDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc6a90dd_ad9a_48da_9b0c_2515e38521ad);
}
impl ::core::convert::From<IRTCPresenceDevice> for ::windows::core::IUnknown {
    fn from(value: IRTCPresenceDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCPresenceDevice> for ::windows::core::IUnknown {
    fn from(value: &IRTCPresenceDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCPresenceDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCPresenceDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrnotes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrnamespace: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCPresencePropertyEvent(pub ::windows::core::IUnknown);
impl IRTCPresencePropertyEvent {
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn PresenceProperty(&self) -> ::windows::core::Result<RTC_PRESENCE_PROPERTY> {
        let mut result__: <RTC_PRESENCE_PROPERTY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PRESENCE_PROPERTY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCPresencePropertyEvent {
    type Vtable = IRTCPresencePropertyEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf777f570_a820_49d5_86bd_e099493f1518);
}
impl ::core::convert::From<IRTCPresencePropertyEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCPresencePropertyEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCPresencePropertyEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCPresencePropertyEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCPresencePropertyEvent> for super::Com::IDispatch {
    fn from(value: IRTCPresencePropertyEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCPresencePropertyEvent> for super::Com::IDispatch {
    fn from(value: &IRTCPresencePropertyEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCPresencePropertyEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresencePropertyEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCPresenceStatusEvent(pub ::windows::core::IUnknown);
impl IRTCPresenceStatusEvent {
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalPresenceInfo(&self, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(penstatus), ::core::mem::transmute(pbstrnotes)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCPresenceStatusEvent {
    type Vtable = IRTCPresenceStatusEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78673f32_4a0f_462c_89aa_ee7706707678);
}
impl ::core::convert::From<IRTCPresenceStatusEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCPresenceStatusEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCPresenceStatusEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCPresenceStatusEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCPresenceStatusEvent> for super::Com::IDispatch {
    fn from(value: IRTCPresenceStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCPresenceStatusEvent> for super::Com::IDispatch {
    fn from(value: &IRTCPresenceStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCPresenceStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCPresenceStatusEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCProfile(pub ::windows::core::IUnknown);
impl IRTCProfile {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Key(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XML(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(enuri), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ClientBanner(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientMinVer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientCurVer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientUpdateURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruseruri: Param0, bstruseraccount: Param1, bstrpassword: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstruseruri.into_param().abi(), bstruseraccount.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE> {
        let mut result__: <RTC_REGISTRATION_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_REGISTRATION_STATE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCProfile {
    type Vtable = IRTCProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd07eca9e_4062_4dd4_9e7d_722a49ba7303);
}
impl ::core::convert::From<IRTCProfile> for ::windows::core::IUnknown {
    fn from(value: IRTCProfile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCProfile> for ::windows::core::IUnknown {
    fn from(value: &IRTCProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrkey: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enuri: RTC_PROVIDER_URI, pbstruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfbanner: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrminver: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrcurver: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrupdateuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseraccount: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCProfile2(pub ::windows::core::IUnknown);
impl IRTCProfile2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Key(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XML(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderURI(&self, enuri: RTC_PROVIDER_URI) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(enuri), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProviderData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ClientBanner(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientMinVer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientCurVer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientUpdateURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstruseruri: Param0, bstruseraccount: Param1, bstrpassword: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstruseruri.into_param().abi(), bstruseraccount.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    pub unsafe fn SessionCapabilities(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE> {
        let mut result__: <RTC_REGISTRATION_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_REGISTRATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Realm(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRealm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrealm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi()).ok()
    }
    pub unsafe fn AllowedAuth(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAllowedAuth(&self, lallowedauth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lallowedauth)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCProfile2 {
    type Vtable = IRTCProfile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b81f84e_bdc7_4184_9154_3cb2dd7917fb);
}
impl ::core::convert::From<IRTCProfile2> for ::windows::core::IUnknown {
    fn from(value: IRTCProfile2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCProfile2> for ::windows::core::IUnknown {
    fn from(value: &IRTCProfile2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCProfile2> for IRTCProfile {
    fn from(value: IRTCProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCProfile2> for IRTCProfile {
    fn from(value: &IRTCProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCProfile> for IRTCProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCProfile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCProfile> for &IRTCProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCProfile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrkey: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrxml: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enuri: RTC_PROVIDER_URI, pbstruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfbanner: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrminver: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrcurver: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrupdateuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseraccount: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstruseruri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstruseraccount: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsupportedsessions: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrealm: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plallowedauth: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lallowedauth: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCProfileEvent(pub ::windows::core::IUnknown);
impl IRTCProfileEvent {
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__: <IRTCProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCProfileEvent {
    type Vtable = IRTCProfileEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6d5ab3b_770e_43e8_800a_79b062395fca);
}
impl ::core::convert::From<IRTCProfileEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCProfileEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCProfileEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCProfileEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCProfileEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCProfileEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCProfileEvent> for super::Com::IDispatch {
    fn from(value: IRTCProfileEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCProfileEvent> for super::Com::IDispatch {
    fn from(value: &IRTCProfileEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCProfileEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCProfileEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCProfileEvent2(pub ::windows::core::IUnknown);
impl IRTCProfileEvent2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__: <IRTCProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_PROFILE_EVENT_TYPE> {
        let mut result__: <RTC_PROFILE_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_PROFILE_EVENT_TYPE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCProfileEvent2 {
    type Vtable = IRTCProfileEvent2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62e56edc_03fa_4121_94fb_23493fd0ae64);
}
impl ::core::convert::From<IRTCProfileEvent2> for ::windows::core::IUnknown {
    fn from(value: IRTCProfileEvent2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCProfileEvent2> for ::windows::core::IUnknown {
    fn from(value: &IRTCProfileEvent2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCProfileEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCProfileEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCProfileEvent2> for IRTCProfileEvent {
    fn from(value: IRTCProfileEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCProfileEvent2> for IRTCProfileEvent {
    fn from(value: &IRTCProfileEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCProfileEvent> for IRTCProfileEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCProfileEvent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCProfileEvent> for &IRTCProfileEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCProfileEvent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCProfileEvent2> for super::Com::IDispatch {
    fn from(value: IRTCProfileEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCProfileEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCProfileEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCProfileEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCProfileEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCProfileEvent2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCReInviteEvent(pub ::windows::core::IUnknown);
impl IRTCReInviteEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__: <IRTCSession2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Accept<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi()).ok()
    }
    pub unsafe fn Reject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REINVITE_STATE> {
        let mut result__: <RTC_REINVITE_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_REINVITE_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCReInviteEvent {
    type Vtable = IRTCReInviteEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11558d84_204c_43e7_99b0_2034e9417f7d);
}
impl ::core::convert::From<IRTCReInviteEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCReInviteEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCReInviteEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCReInviteEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCReInviteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCReInviteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCReInviteEvent> for super::Com::IDispatch {
    fn from(value: IRTCReInviteEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCReInviteEvent> for super::Com::IDispatch {
    fn from(value: &IRTCReInviteEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCReInviteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCReInviteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCReInviteEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstate: *mut RTC_REINVITE_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrcontenttype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrsessiondescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCRegistrationStateChangeEvent(pub ::windows::core::IUnknown);
impl IRTCRegistrationStateChangeEvent {
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__: <IRTCProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_REGISTRATION_STATE> {
        let mut result__: <RTC_REGISTRATION_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_REGISTRATION_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCRegistrationStateChangeEvent {
    type Vtable = IRTCRegistrationStateChangeEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62d0991b_50ab_4f02_b948_ca94f26f8f95);
}
impl ::core::convert::From<IRTCRegistrationStateChangeEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCRegistrationStateChangeEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCRegistrationStateChangeEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCRegistrationStateChangeEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCRegistrationStateChangeEvent> for super::Com::IDispatch {
    fn from(value: IRTCRegistrationStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCRegistrationStateChangeEvent> for super::Com::IDispatch {
    fn from(value: &IRTCRegistrationStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCRegistrationStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRegistrationStateChangeEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCRoamingEvent(pub ::windows::core::IUnknown);
impl IRTCRoamingEvent {
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_ROAMING_EVENT_TYPE> {
        let mut result__: <RTC_ROAMING_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_ROAMING_EVENT_TYPE>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__: <IRTCProfile2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCRoamingEvent {
    type Vtable = IRTCRoamingEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79960a6b_0cb1_4dc8_a805_7318e99902e8);
}
impl ::core::convert::From<IRTCRoamingEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCRoamingEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCRoamingEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCRoamingEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCRoamingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCRoamingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCRoamingEvent> for super::Com::IDispatch {
    fn from(value: IRTCRoamingEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCRoamingEvent> for super::Com::IDispatch {
    fn from(value: &IRTCRoamingEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCRoamingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCRoamingEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCRoamingEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSession(pub ::windows::core::IUnknown);
impl IRTCSession {
    pub unsafe fn Client(&self) -> ::windows::core::Result<IRTCClient> {
        let mut result__: <IRTCClient as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCClient>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__: <RTC_SESSION_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<RTC_SESSION_TYPE> {
        let mut result__: <RTC_SESSION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_SESSION_TYPE>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__: <IRTCProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn Participants(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn Answer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(enreason)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Redirect<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddParticipant<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraddress: Param0, bstrname: Param1) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__: <IRTCParticipant as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstraddress.into_param().abi(), bstrname.into_param().abi(), &mut result__).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn RemoveParticipant<'a, Param0: ::windows::core::IntoParam<'a, IRTCParticipant>>(&self, pparticipant: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pparticipant.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> ::windows::core::Result<IRTCEnumParticipants> {
        let mut result__: <IRTCEnumParticipants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumParticipants>(result__)
    }
    pub unsafe fn CanAddParticipants(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RedirectedUserURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RedirectedUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn NextRedirectedUser(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageheader: Param0, bstrmessage: Param1, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstrmessageheader.into_param().abi(), bstrmessage.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(enuserstatus), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEncryptionKey<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lmediatype: i32, encryptionkey: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatype), encryptionkey.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCSession {
    type Vtable = IRTCSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x387c8086_99be_42fb_9973_7c0fc0ca9fa8);
}
impl ::core::convert::From<IRTCSession> for ::windows::core::IUnknown {
    fn from(value: IRTCSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSession> for ::windows::core::IUnknown {
    fn from(value: &IRTCSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pparticipant: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfcanadd: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrmessageheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSession2(pub ::windows::core::IUnknown);
impl IRTCSession2 {
    pub unsafe fn Client(&self) -> ::windows::core::Result<IRTCClient> {
        let mut result__: <IRTCClient as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCClient>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__: <RTC_SESSION_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<RTC_SESSION_TYPE> {
        let mut result__: <RTC_SESSION_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_SESSION_TYPE>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile> {
        let mut result__: <IRTCProfile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile>(result__)
    }
    pub unsafe fn Participants(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn Answer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Terminate(&self, enreason: RTC_TERMINATE_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(enreason)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Redirect<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: Param1, pprofile: Param2, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(entype), bstrlocalphoneuri.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddParticipant<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstraddress: Param0, bstrname: Param1) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__: <IRTCParticipant as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstraddress.into_param().abi(), bstrname.into_param().abi(), &mut result__).from_abi::<IRTCParticipant>(result__)
    }
    pub unsafe fn RemoveParticipant<'a, Param0: ::windows::core::IntoParam<'a, IRTCParticipant>>(&self, pparticipant: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pparticipant.into_param().abi()).ok()
    }
    pub unsafe fn EnumerateParticipants(&self) -> ::windows::core::Result<IRTCEnumParticipants> {
        let mut result__: <IRTCEnumParticipants as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumParticipants>(result__)
    }
    pub unsafe fn CanAddParticipants(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RedirectedUserURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RedirectedUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn NextRedirectedUser(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmessageheader: Param0, bstrmessage: Param1, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstrmessageheader.into_param().abi(), bstrmessage.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SendMessageStatus(&self, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(enuserstatus), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn AddStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn RemoveStream(&self, lmediatype: i32, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatype), ::core::mem::transmute(lcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEncryptionKey<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lmediatype: i32, encryptionkey: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmediatype), encryptionkey.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrinfoheader: Param0, bstrinfo: Param1, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), bstrinfoheader.into_param().abi(), bstrinfo.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn SetPreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(ensecuritytype), ::core::mem::transmute(ensecuritylevel)).ok()
    }
    pub unsafe fn PreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__: <RTC_SECURITY_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(ensecuritytype), &mut result__).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn IsSecurityEnabled(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(ensecuritytype), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AnswerWithSessionDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReInviteWithSessionDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCSession2 {
    type Vtable = IRTCSession2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17d7cdfc_b007_484c_99d2_86a8a820991d);
}
impl ::core::convert::From<IRTCSession2> for ::windows::core::IUnknown {
    fn from(value: IRTCSession2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSession2> for ::windows::core::IUnknown {
    fn from(value: &IRTCSession2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCSession2> for IRTCSession {
    fn from(value: IRTCSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSession2> for IRTCSession {
    fn from(value: &IRTCSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCSession> for IRTCSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCSession> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCSession> for &IRTCSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCSession> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppclient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pentype: *mut RTC_SESSION_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enreason: RTC_TERMINATE_REASON) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprofile: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pparticipant: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfcanadd: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruseruri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrmessageheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatype: i32, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmediatype: i32, encryptionkey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrinfoheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrinfo: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionCallControl(pub ::windows::core::IUnknown);
impl IRTCSessionCallControl {
    pub unsafe fn Hold(&self, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcookie)).ok()
    }
    pub unsafe fn UnHold(&self, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcookie)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Forward<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrforwardtouri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrforwardtouri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Refer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrefertouri: Param0, bstrrefercookie: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrrefertouri.into_param().abi(), bstrrefercookie.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReferredByURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrreferredbyuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrreferredbyuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReferredByURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReferCookie<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrefercookie: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrrefercookie.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReferCookie(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn IsReferred(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionCallControl {
    type Vtable = IRTCSessionCallControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a50d94_190b_4f82_9530_3b8ebf60758a);
}
impl ::core::convert::From<IRTCSessionCallControl> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionCallControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionCallControl> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionCallControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionCallControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionCallControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionCallControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcookie: isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcookie: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrforwardtouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrrefertouri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrreferredbyuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrreferredbyuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrrefercookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrefercookie: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfisreferred: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionDescriptionManager(pub ::windows::core::IUnknown);
impl IRTCSessionDescriptionManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EvaluateSessionDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcontenttype: Param0, bstrsessiondescription: Param1, pfapplicationsession: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrcontenttype.into_param().abi(), bstrsessiondescription.into_param().abi(), ::core::mem::transmute(pfapplicationsession)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionDescriptionManager {
    type Vtable = IRTCSessionDescriptionManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba7f518e_d336_4070_93a6_865395c843f9);
}
impl ::core::convert::From<IRTCSessionDescriptionManager> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionDescriptionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionDescriptionManager> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionDescriptionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionDescriptionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionDescriptionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionDescriptionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrcontenttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrsessiondescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfapplicationsession: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionOperationCompleteEvent(pub ::windows::core::IUnknown);
impl IRTCSessionOperationCompleteEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionOperationCompleteEvent {
    type Vtable = IRTCSessionOperationCompleteEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6bff4c0_f7c8_4d3c_9a41_3550f78a95b0);
}
impl ::core::convert::From<IRTCSessionOperationCompleteEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionOperationCompleteEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionOperationCompleteEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCSessionOperationCompleteEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionOperationCompleteEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionOperationCompleteEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCSessionOperationCompleteEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionOperationCompleteEvent2(pub ::windows::core::IUnknown);
impl IRTCSessionOperationCompleteEvent2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Participant(&self) -> ::windows::core::Result<IRTCParticipant> {
        let mut result__: <IRTCParticipant as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCParticipant>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionOperationCompleteEvent2 {
    type Vtable = IRTCSessionOperationCompleteEvent2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6fc2a9b_d5bc_4241_b436_1b8460c13832);
}
impl ::core::convert::From<IRTCSessionOperationCompleteEvent2> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionOperationCompleteEvent2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent2> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionOperationCompleteEvent2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCSessionOperationCompleteEvent2> for IRTCSessionOperationCompleteEvent {
    fn from(value: IRTCSessionOperationCompleteEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent2> for IRTCSessionOperationCompleteEvent {
    fn from(value: &IRTCSessionOperationCompleteEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCSessionOperationCompleteEvent> for IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCSessionOperationCompleteEvent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCSessionOperationCompleteEvent> for &IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCSessionOperationCompleteEvent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCSessionOperationCompleteEvent2> for super::Com::IDispatch {
    fn from(value: IRTCSessionOperationCompleteEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCSessionOperationCompleteEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCSessionOperationCompleteEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCSessionOperationCompleteEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionOperationCompleteEvent2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppparticipant: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrcontenttype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrsessiondescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionPortManagement(pub ::windows::core::IUnknown);
impl IRTCSessionPortManagement {
    pub unsafe fn SetPortManager<'a, Param0: ::windows::core::IntoParam<'a, IRTCPortManager>>(&self, pportmanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pportmanager.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionPortManagement {
    type Vtable = IRTCSessionPortManagement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa072f1d6_0286_4e1f_85f2_17a2948456ec);
}
impl ::core::convert::From<IRTCSessionPortManagement> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionPortManagement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionPortManagement> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionPortManagement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionPortManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionPortManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionPortManagement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pportmanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionReferStatusEvent(pub ::windows::core::IUnknown);
impl IRTCSessionReferStatusEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__: <IRTCSession2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession2>(result__)
    }
    pub unsafe fn ReferStatus(&self) -> ::windows::core::Result<RTC_SESSION_REFER_STATUS> {
        let mut result__: <RTC_SESSION_REFER_STATUS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_SESSION_REFER_STATUS>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionReferStatusEvent {
    type Vtable = IRTCSessionReferStatusEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d8fc2cd_5d76_44ab_bb68_2a80353b34a2);
}
impl ::core::convert::From<IRTCSessionReferStatusEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionReferStatusEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionReferStatusEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionReferStatusEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCSessionReferStatusEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionReferStatusEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCSessionReferStatusEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionReferStatusEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCSessionReferStatusEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferStatusEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionReferredEvent(pub ::windows::core::IUnknown);
impl IRTCSessionReferredEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession2> {
        let mut result__: <IRTCSession2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReferredByURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReferToURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReferCookie(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Accept(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetReferredSessionState(&self, enstate: RTC_SESSION_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(enstate)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionReferredEvent {
    type Vtable = IRTCSessionReferredEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x176a6828_4fcc_4f28_a862_04597a6cf1c4);
}
impl ::core::convert::From<IRTCSessionReferredEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionReferredEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionReferredEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionReferredEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCSessionReferredEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionReferredEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCSessionReferredEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionReferredEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCSessionReferredEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionReferredEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrreferredbyuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrreferouri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrefercookie: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enstate: RTC_SESSION_STATE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionStateChangeEvent(pub ::windows::core::IUnknown);
impl IRTCSessionStateChangeEvent {
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__: <RTC_SESSION_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionStateChangeEvent {
    type Vtable = IRTCSessionStateChangeEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bad703_5952_48b3_9321_7f4500521506);
}
impl ::core::convert::From<IRTCSessionStateChangeEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionStateChangeEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionStateChangeEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionStateChangeEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCSessionStateChangeEvent> for super::Com::IDispatch {
    fn from(value: IRTCSessionStateChangeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCSessionStateChangeEvent> for super::Com::IDispatch {
    fn from(value: &IRTCSessionStateChangeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCSessionStateChangeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCSessionStateChangeEvent2(pub ::windows::core::IUnknown);
impl IRTCSessionStateChangeEvent2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Session(&self) -> ::windows::core::Result<IRTCSession> {
        let mut result__: <IRTCSession as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCSession>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_SESSION_STATE> {
        let mut result__: <RTC_SESSION_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_SESSION_STATE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MediaTypes(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn RemotePreferredSecurityLevel(&self, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows::core::Result<RTC_SECURITY_LEVEL> {
        let mut result__: <RTC_SECURITY_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ensecuritytype), &mut result__).from_abi::<RTC_SECURITY_LEVEL>(result__)
    }
    pub unsafe fn IsForked(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteSessionDescription(&self, pbstrcontenttype: *mut super::super::Foundation::BSTR, pbstrsessiondescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbstrcontenttype), ::core::mem::transmute(pbstrsessiondescription)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCSessionStateChangeEvent2 {
    type Vtable = IRTCSessionStateChangeEvent2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f933171_6f95_4880_80d9_2ec8d495d261);
}
impl ::core::convert::From<IRTCSessionStateChangeEvent2> for ::windows::core::IUnknown {
    fn from(value: IRTCSessionStateChangeEvent2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCSessionStateChangeEvent2> for ::windows::core::IUnknown {
    fn from(value: &IRTCSessionStateChangeEvent2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCSessionStateChangeEvent2> for IRTCSessionStateChangeEvent {
    fn from(value: IRTCSessionStateChangeEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCSessionStateChangeEvent2> for IRTCSessionStateChangeEvent {
    fn from(value: &IRTCSessionStateChangeEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCSessionStateChangeEvent> for IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCSessionStateChangeEvent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCSessionStateChangeEvent> for &IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCSessionStateChangeEvent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCSessionStateChangeEvent2> for super::Com::IDispatch {
    fn from(value: IRTCSessionStateChangeEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCSessionStateChangeEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCSessionStateChangeEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCSessionStateChangeEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCSessionStateChangeEvent2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_SESSION_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrstatustext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmediatypes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfisforked: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrcontenttype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrsessiondescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCUserSearch(pub ::windows::core::IUnknown);
impl IRTCUserSearch {
    pub unsafe fn CreateQuery(&self) -> ::windows::core::Result<IRTCUserSearchQuery> {
        let mut result__: <IRTCUserSearchQuery as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCUserSearchQuery>(result__)
    }
    pub unsafe fn ExecuteSearch<'a, Param0: ::windows::core::IntoParam<'a, IRTCUserSearchQuery>, Param1: ::windows::core::IntoParam<'a, IRTCProfile>>(&self, pquery: Param0, pprofile: Param1, lcookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pquery.into_param().abi(), pprofile.into_param().abi(), ::core::mem::transmute(lcookie)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCUserSearch {
    type Vtable = IRTCUserSearch_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb619882b_860c_4db4_be1b_693b6505bbe5);
}
impl ::core::convert::From<IRTCUserSearch> for ::windows::core::IUnknown {
    fn from(value: IRTCUserSearch) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCUserSearch> for ::windows::core::IUnknown {
    fn from(value: &IRTCUserSearch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCUserSearch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCUserSearch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearch_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pquery: ::windows::core::RawPtr, pprofile: ::windows::core::RawPtr, lcookie: isize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCUserSearchQuery(pub ::windows::core::IUnknown);
impl IRTCUserSearchQuery {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSearchTerm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstrvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchTerm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchTerms(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetSearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(enpreference), ::core::mem::transmute(lvalue)).ok()
    }
    pub unsafe fn SearchPreference(&self, enpreference: RTC_USER_SEARCH_PREFERENCE) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(enpreference), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSearchDomain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdomain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrdomain.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCUserSearchQuery {
    type Vtable = IRTCUserSearchQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x288300f5_d23a_4365_9a73_9985c98c2881);
}
impl ::core::convert::From<IRTCUserSearchQuery> for ::windows::core::IUnknown {
    fn from(value: IRTCUserSearchQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCUserSearchQuery> for ::windows::core::IUnknown {
    fn from(value: &IRTCUserSearchQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCUserSearchQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCUserSearchQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrnames: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdomain: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdomain: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCUserSearchResult(pub ::windows::core::IUnknown);
impl IRTCUserSearchResult {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self, encolumn: RTC_USER_SEARCH_COLUMN) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(encolumn), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCUserSearchResult {
    type Vtable = IRTCUserSearchResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851278b2_9592_480f_8db5_2de86b26b54d);
}
impl ::core::convert::From<IRTCUserSearchResult> for ::windows::core::IUnknown {
    fn from(value: IRTCUserSearchResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCUserSearchResult> for ::windows::core::IUnknown {
    fn from(value: &IRTCUserSearchResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCUserSearchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCUserSearchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCUserSearchResultsEvent(pub ::windows::core::IUnknown);
impl IRTCUserSearchResultsEvent {
    pub unsafe fn EnumerateResults(&self) -> ::windows::core::Result<IRTCEnumUserSearchResults> {
        let mut result__: <IRTCEnumUserSearchResults as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCEnumUserSearchResults>(result__)
    }
    pub unsafe fn Results(&self) -> ::windows::core::Result<IRTCCollection> {
        let mut result__: <IRTCCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCCollection>(result__)
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__: <IRTCProfile2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn Query(&self) -> ::windows::core::Result<IRTCUserSearchQuery> {
        let mut result__: <IRTCUserSearchQuery as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCUserSearchQuery>(result__)
    }
    pub unsafe fn Cookie(&self) -> ::windows::core::Result<isize> {
        let mut result__: <isize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn MoreAvailable(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCUserSearchResultsEvent {
    type Vtable = IRTCUserSearchResultsEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c8c3cd_7fac_4088_81c5_c24cbc0938e3);
}
impl ::core::convert::From<IRTCUserSearchResultsEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCUserSearchResultsEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCUserSearchResultsEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCUserSearchResultsEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCUserSearchResultsEvent> for super::Com::IDispatch {
    fn from(value: IRTCUserSearchResultsEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCUserSearchResultsEvent> for super::Com::IDispatch {
    fn from(value: &IRTCUserSearchResultsEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCUserSearchResultsEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCUserSearchResultsEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plcookie: *mut isize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfmoreavailable: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCWatcher(pub ::windows::core::IUnknown);
impl IRTCWatcher {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_WATCHER_STATE> {
        let mut result__: <RTC_WATCHER_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_WATCHER_STATE>(result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(enstate)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRTCWatcher {
    type Vtable = IRTCWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7cedad8_346b_4d1b_ac02_a2088df9be4f);
}
impl ::core::convert::From<IRTCWatcher> for ::windows::core::IUnknown {
    fn from(value: IRTCWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCWatcher> for ::windows::core::IUnknown {
    fn from(value: &IRTCWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCWatcher> for IRTCPresenceContact {
    fn from(value: IRTCWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher> for IRTCPresenceContact {
    fn from(value: &IRTCWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for IRTCWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for &IRTCWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpresentityuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfpersistent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fpersistent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCWatcher2(pub ::windows::core::IUnknown);
impl IRTCWatcher2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PresentityURI(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPresentityURI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpresentityuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrpresentityuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrdata.into_param().abi()).ok()
    }
    pub unsafe fn Persistent(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPersistent(&self, fpersistent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(fpersistent)).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<RTC_WATCHER_STATE> {
        let mut result__: <RTC_WATCHER_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_WATCHER_STATE>(result__)
    }
    pub unsafe fn SetState(&self, enstate: RTC_WATCHER_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(enstate)).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows::core::Result<IRTCProfile2> {
        let mut result__: <IRTCProfile2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCProfile2>(result__)
    }
    pub unsafe fn Scope(&self) -> ::windows::core::Result<RTC_ACE_SCOPE> {
        let mut result__: <RTC_ACE_SCOPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_ACE_SCOPE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCWatcher2 {
    type Vtable = IRTCWatcher2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4d9967f_d011_4b1d_91e3_aba78f96393d);
}
impl ::core::convert::From<IRTCWatcher2> for ::windows::core::IUnknown {
    fn from(value: IRTCWatcher2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCWatcher2> for ::windows::core::IUnknown {
    fn from(value: &IRTCWatcher2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCWatcher2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCWatcher2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCWatcher2> for IRTCWatcher {
    fn from(value: IRTCWatcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher2> for IRTCWatcher {
    fn from(value: &IRTCWatcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCWatcher> for IRTCWatcher2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCWatcher> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCWatcher> for &IRTCWatcher2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCWatcher> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRTCWatcher2> for IRTCPresenceContact {
    fn from(value: IRTCWatcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcher2> for IRTCPresenceContact {
    fn from(value: &IRTCWatcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for IRTCWatcher2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCPresenceContact> for &IRTCWatcher2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCPresenceContact> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcher2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpresentityuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpresentityuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdata: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfpersistent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fpersistent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penstate: *mut RTC_WATCHER_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enstate: RTC_WATCHER_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penscope: *mut RTC_ACE_SCOPE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCWatcherEvent(pub ::windows::core::IUnknown);
impl IRTCWatcherEvent {
    pub unsafe fn Watcher(&self) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__: <IRTCWatcher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCWatcher>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCWatcherEvent {
    type Vtable = IRTCWatcherEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf30d7261_587a_424f_822c_312788f43548);
}
impl ::core::convert::From<IRTCWatcherEvent> for ::windows::core::IUnknown {
    fn from(value: IRTCWatcherEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCWatcherEvent> for ::windows::core::IUnknown {
    fn from(value: &IRTCWatcherEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCWatcherEvent> for super::Com::IDispatch {
    fn from(value: IRTCWatcherEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCWatcherEvent> for super::Com::IDispatch {
    fn from(value: &IRTCWatcherEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCWatcherEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRTCWatcherEvent2(pub ::windows::core::IUnknown);
impl IRTCWatcherEvent2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Watcher(&self) -> ::windows::core::Result<IRTCWatcher> {
        let mut result__: <IRTCWatcher as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRTCWatcher>(result__)
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<RTC_WATCHER_EVENT_TYPE> {
        let mut result__: <RTC_WATCHER_EVENT_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<RTC_WATCHER_EVENT_TYPE>(result__)
    }
    pub unsafe fn StatusCode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRTCWatcherEvent2 {
    type Vtable = IRTCWatcherEvent2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe52891e8_188c_49af_b005_98ed13f83f9c);
}
impl ::core::convert::From<IRTCWatcherEvent2> for ::windows::core::IUnknown {
    fn from(value: IRTCWatcherEvent2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRTCWatcherEvent2> for ::windows::core::IUnknown {
    fn from(value: &IRTCWatcherEvent2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRTCWatcherEvent2> for IRTCWatcherEvent {
    fn from(value: IRTCWatcherEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRTCWatcherEvent2> for IRTCWatcherEvent {
    fn from(value: &IRTCWatcherEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCWatcherEvent> for IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCWatcherEvent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRTCWatcherEvent> for &IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRTCWatcherEvent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRTCWatcherEvent2> for super::Com::IDispatch {
    fn from(value: IRTCWatcherEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRTCWatcherEvent2> for super::Com::IDispatch {
    fn from(value: &IRTCWatcherEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRTCWatcherEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRTCWatcherEvent2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwatcher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstatuscode: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITransportSettingsInternal(pub ::windows::core::IUnknown);
impl ITransportSettingsInternal {
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn ApplySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(setting)).ok()
    }
    #[cfg(feature = "Win32_Networking_WinSock")]
    pub unsafe fn QuerySetting(&self, setting: *mut TRANSPORT_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(setting)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITransportSettingsInternal {
    type Vtable = ITransportSettingsInternal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5123e076_29e3_4bfd_84fe_0192d411e3e8);
}
impl ::core::convert::From<ITransportSettingsInternal> for ::windows::core::IUnknown {
    fn from(value: ITransportSettingsInternal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITransportSettingsInternal> for ::windows::core::IUnknown {
    fn from(value: &ITransportSettingsInternal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransportSettingsInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransportSettingsInternal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransportSettingsInternal_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Networking_WinSock")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))] usize,
    #[cfg(feature = "Win32_Networking_WinSock")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, setting: *mut TRANSPORT_SETTING) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Networking_WinSock"))] usize,
);
pub const RTCAU_BASIC: u32 = 1u32;
pub const RTCAU_DIGEST: u32 = 2u32;
pub const RTCAU_KERBEROS: u32 = 8u32;
pub const RTCAU_NTLM: u32 = 4u32;
pub const RTCAU_USE_LOGON_CRED: u32 = 65536u32;
pub const RTCCS_FAIL_ON_REDIRECT: u32 = 2u32;
pub const RTCCS_FORCE_PROFILE: u32 = 1u32;
pub const RTCClient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a42ea29_a2b7_40c4_b091_f6f024aa89be);
pub const RTCEF_ALL: u32 = 33554431u32;
pub const RTCEF_BUDDY: u32 = 256u32;
pub const RTCEF_BUDDY2: u32 = 262144u32;
pub const RTCEF_CLIENT: u32 = 1u32;
pub const RTCEF_GROUP: u32 = 8192u32;
pub const RTCEF_INFO: u32 = 4096u32;
pub const RTCEF_INTENSITY: u32 = 64u32;
pub const RTCEF_MEDIA: u32 = 32u32;
pub const RTCEF_MEDIA_REQUEST: u32 = 16384u32;
pub const RTCEF_MESSAGING: u32 = 128u32;
pub const RTCEF_PARTICIPANT_STATE_CHANGE: u32 = 16u32;
pub const RTCEF_PRESENCE_DATA: u32 = 8388608u32;
pub const RTCEF_PRESENCE_PROPERTY: u32 = 131072u32;
pub const RTCEF_PRESENCE_STATUS: u32 = 16777216u32;
pub const RTCEF_PROFILE: u32 = 1024u32;
pub const RTCEF_REGISTRATION_STATE_CHANGE: u32 = 2u32;
pub const RTCEF_REINVITE: u32 = 4194304u32;
pub const RTCEF_ROAMING: u32 = 65536u32;
pub const RTCEF_SESSION_OPERATION_COMPLETE: u32 = 8u32;
pub const RTCEF_SESSION_REFERRED: u32 = 2097152u32;
pub const RTCEF_SESSION_REFER_STATUS: u32 = 1048576u32;
pub const RTCEF_SESSION_STATE_CHANGE: u32 = 4u32;
pub const RTCEF_USERSEARCH: u32 = 2048u32;
pub const RTCEF_WATCHER: u32 = 512u32;
pub const RTCEF_WATCHER2: u32 = 524288u32;
pub const RTCIF_DISABLE_MEDIA: u32 = 1u32;
pub const RTCIF_DISABLE_STRICT_DNS: u32 = 8u32;
pub const RTCIF_DISABLE_UPNP: u32 = 2u32;
pub const RTCIF_ENABLE_SERVER_CLASS: u32 = 4u32;
pub const RTCMT_AUDIO_RECEIVE: u32 = 2u32;
pub const RTCMT_AUDIO_SEND: u32 = 1u32;
pub const RTCMT_T120_SENDRECV: u32 = 16u32;
pub const RTCMT_VIDEO_RECEIVE: u32 = 8u32;
pub const RTCMT_VIDEO_SEND: u32 = 4u32;
pub const RTCRF_REGISTER_ALL: u32 = 15u32;
pub const RTCRF_REGISTER_INVITE_SESSIONS: u32 = 1u32;
pub const RTCRF_REGISTER_MESSAGE_SESSIONS: u32 = 2u32;
pub const RTCRF_REGISTER_NOTIFY: u32 = 8u32;
pub const RTCRF_REGISTER_PRESENCE: u32 = 4u32;
pub const RTCRMF_ALL_ROAMING: u32 = 15u32;
pub const RTCRMF_BUDDY_ROAMING: u32 = 1u32;
pub const RTCRMF_PRESENCE_ROAMING: u32 = 4u32;
pub const RTCRMF_PROFILE_ROAMING: u32 = 8u32;
pub const RTCRMF_WATCHER_ROAMING: u32 = 2u32;
pub const RTCSI_APPLICATION: u32 = 32u32;
pub const RTCSI_IM: u32 = 8u32;
pub const RTCSI_MULTIPARTY_IM: u32 = 16u32;
pub const RTCSI_PC_TO_PC: u32 = 1u32;
pub const RTCSI_PC_TO_PHONE: u32 = 2u32;
pub const RTCSI_PHONE_TO_PHONE: u32 = 4u32;
pub const RTCTR_TCP: u32 = 2u32;
pub const RTCTR_TLS: u32 = 4u32;
pub const RTCTR_UDP: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_ACE_SCOPE(pub i32);
pub const RTCAS_SCOPE_USER: RTC_ACE_SCOPE = RTC_ACE_SCOPE(0i32);
pub const RTCAS_SCOPE_DOMAIN: RTC_ACE_SCOPE = RTC_ACE_SCOPE(1i32);
pub const RTCAS_SCOPE_ALL: RTC_ACE_SCOPE = RTC_ACE_SCOPE(2i32);
impl ::core::convert::From<i32> for RTC_ACE_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_ACE_SCOPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_ANSWER_MODE(pub i32);
pub const RTCAM_OFFER_SESSION_EVENT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(0i32);
pub const RTCAM_AUTOMATICALLY_ACCEPT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(1i32);
pub const RTCAM_AUTOMATICALLY_REJECT: RTC_ANSWER_MODE = RTC_ANSWER_MODE(2i32);
pub const RTCAM_NOT_SUPPORTED: RTC_ANSWER_MODE = RTC_ANSWER_MODE(3i32);
impl ::core::convert::From<i32> for RTC_ANSWER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_ANSWER_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_AUDIO_DEVICE(pub i32);
pub const RTCAD_SPEAKER: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(0i32);
pub const RTCAD_MICROPHONE: RTC_AUDIO_DEVICE = RTC_AUDIO_DEVICE(1i32);
impl ::core::convert::From<i32> for RTC_AUDIO_DEVICE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_AUDIO_DEVICE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_BUDDY_EVENT_TYPE(pub i32);
pub const RTCBET_BUDDY_ADD: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(0i32);
pub const RTCBET_BUDDY_REMOVE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(1i32);
pub const RTCBET_BUDDY_UPDATE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(2i32);
pub const RTCBET_BUDDY_STATE_CHANGE: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(3i32);
pub const RTCBET_BUDDY_ROAMED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(4i32);
pub const RTCBET_BUDDY_SUBSCRIBED: RTC_BUDDY_EVENT_TYPE = RTC_BUDDY_EVENT_TYPE(5i32);
impl ::core::convert::From<i32> for RTC_BUDDY_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_BUDDY_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_BUDDY_SUBSCRIPTION_TYPE(pub i32);
pub const RTCBT_SUBSCRIBED: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(0i32);
pub const RTCBT_ALWAYS_OFFLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(1i32);
pub const RTCBT_ALWAYS_ONLINE: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(2i32);
pub const RTCBT_POLL: RTC_BUDDY_SUBSCRIPTION_TYPE = RTC_BUDDY_SUBSCRIPTION_TYPE(3i32);
impl ::core::convert::From<i32> for RTC_BUDDY_SUBSCRIPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_BUDDY_SUBSCRIPTION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_CLIENT_EVENT_TYPE(pub i32);
pub const RTCCET_VOLUME_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(0i32);
pub const RTCCET_DEVICE_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(1i32);
pub const RTCCET_NETWORK_QUALITY_CHANGE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(2i32);
pub const RTCCET_ASYNC_CLEANUP_DONE: RTC_CLIENT_EVENT_TYPE = RTC_CLIENT_EVENT_TYPE(3i32);
impl ::core::convert::From<i32> for RTC_CLIENT_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_CLIENT_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_DTMF(pub i32);
pub const RTC_DTMF_0: RTC_DTMF = RTC_DTMF(0i32);
pub const RTC_DTMF_1: RTC_DTMF = RTC_DTMF(1i32);
pub const RTC_DTMF_2: RTC_DTMF = RTC_DTMF(2i32);
pub const RTC_DTMF_3: RTC_DTMF = RTC_DTMF(3i32);
pub const RTC_DTMF_4: RTC_DTMF = RTC_DTMF(4i32);
pub const RTC_DTMF_5: RTC_DTMF = RTC_DTMF(5i32);
pub const RTC_DTMF_6: RTC_DTMF = RTC_DTMF(6i32);
pub const RTC_DTMF_7: RTC_DTMF = RTC_DTMF(7i32);
pub const RTC_DTMF_8: RTC_DTMF = RTC_DTMF(8i32);
pub const RTC_DTMF_9: RTC_DTMF = RTC_DTMF(9i32);
pub const RTC_DTMF_STAR: RTC_DTMF = RTC_DTMF(10i32);
pub const RTC_DTMF_POUND: RTC_DTMF = RTC_DTMF(11i32);
pub const RTC_DTMF_A: RTC_DTMF = RTC_DTMF(12i32);
pub const RTC_DTMF_B: RTC_DTMF = RTC_DTMF(13i32);
pub const RTC_DTMF_C: RTC_DTMF = RTC_DTMF(14i32);
pub const RTC_DTMF_D: RTC_DTMF = RTC_DTMF(15i32);
pub const RTC_DTMF_FLASH: RTC_DTMF = RTC_DTMF(16i32);
impl ::core::convert::From<i32> for RTC_DTMF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_DTMF {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_EVENT(pub i32);
pub const RTCE_CLIENT: RTC_EVENT = RTC_EVENT(0i32);
pub const RTCE_REGISTRATION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(1i32);
pub const RTCE_SESSION_STATE_CHANGE: RTC_EVENT = RTC_EVENT(2i32);
pub const RTCE_SESSION_OPERATION_COMPLETE: RTC_EVENT = RTC_EVENT(3i32);
pub const RTCE_PARTICIPANT_STATE_CHANGE: RTC_EVENT = RTC_EVENT(4i32);
pub const RTCE_MEDIA: RTC_EVENT = RTC_EVENT(5i32);
pub const RTCE_INTENSITY: RTC_EVENT = RTC_EVENT(6i32);
pub const RTCE_MESSAGING: RTC_EVENT = RTC_EVENT(7i32);
pub const RTCE_BUDDY: RTC_EVENT = RTC_EVENT(8i32);
pub const RTCE_WATCHER: RTC_EVENT = RTC_EVENT(9i32);
pub const RTCE_PROFILE: RTC_EVENT = RTC_EVENT(10i32);
pub const RTCE_USERSEARCH: RTC_EVENT = RTC_EVENT(11i32);
pub const RTCE_INFO: RTC_EVENT = RTC_EVENT(12i32);
pub const RTCE_GROUP: RTC_EVENT = RTC_EVENT(13i32);
pub const RTCE_MEDIA_REQUEST: RTC_EVENT = RTC_EVENT(14i32);
pub const RTCE_ROAMING: RTC_EVENT = RTC_EVENT(15i32);
pub const RTCE_PRESENCE_PROPERTY: RTC_EVENT = RTC_EVENT(16i32);
pub const RTCE_PRESENCE_DATA: RTC_EVENT = RTC_EVENT(17i32);
pub const RTCE_PRESENCE_STATUS: RTC_EVENT = RTC_EVENT(18i32);
pub const RTCE_SESSION_REFER_STATUS: RTC_EVENT = RTC_EVENT(19i32);
pub const RTCE_SESSION_REFERRED: RTC_EVENT = RTC_EVENT(20i32);
pub const RTCE_REINVITE: RTC_EVENT = RTC_EVENT(21i32);
impl ::core::convert::From<i32> for RTC_EVENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_EVENT {
    type Abi = Self;
}
pub const RTC_E_ANOTHER_MEDIA_SESSION_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885961i32 as _);
pub const RTC_E_BASIC_AUTH_SET_TLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886017i32 as _);
pub const RTC_E_CLIENT_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886042i32 as _);
pub const RTC_E_CLIENT_ALREADY_SHUT_DOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886041i32 as _);
pub const RTC_E_CLIENT_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886043i32 as _);
pub const RTC_E_DESTINATION_ADDRESS_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886061i32 as _);
pub const RTC_E_DESTINATION_ADDRESS_MULTICAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886059i32 as _);
pub const RTC_E_DUPLICATE_BUDDY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886006i32 as _);
pub const RTC_E_DUPLICATE_GROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885998i32 as _);
pub const RTC_E_DUPLICATE_REALM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886013i32 as _);
pub const RTC_E_DUPLICATE_WATCHER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886005i32 as _);
pub const RTC_E_INVALID_ACL_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886000i32 as _);
pub const RTC_E_INVALID_ADDRESS_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886060i32 as _);
pub const RTC_E_INVALID_BUDDY_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886001i32 as _);
pub const RTC_E_INVALID_LISTEN_SOCKET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885957i32 as _);
pub const RTC_E_INVALID_OBJECT_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885983i32 as _);
pub const RTC_E_INVALID_PORTRANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885988i32 as _);
pub const RTC_E_INVALID_PREFERENCE_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885991i32 as _);
pub const RTC_E_INVALID_PROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886034i32 as _);
pub const RTC_E_INVALID_PROXY_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886058i32 as _);
pub const RTC_E_INVALID_REGISTRATION_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885971i32 as _);
pub const RTC_E_INVALID_SESSION_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886038i32 as _);
pub const RTC_E_INVALID_SESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886039i32 as _);
pub const RTC_E_INVALID_SIP_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886062i32 as _);
pub const RTC_E_LISTENING_SOCKET_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885958i32 as _);
pub const RTC_E_LOCAL_PHONE_NEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886036i32 as _);
pub const RTC_E_MALFORMED_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886004i32 as _);
pub const RTC_E_MAX_PENDING_OPERATIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885990i32 as _);
pub const RTC_E_MAX_REDIRECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885960i32 as _);
pub const RTC_E_MEDIA_AEC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886044i32 as _);
pub const RTC_E_MEDIA_AUDIO_DEVICE_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886047i32 as _);
pub const RTC_E_MEDIA_CONTROLLER_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886049i32 as _);
pub const RTC_E_MEDIA_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885970i32 as _);
pub const RTC_E_MEDIA_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885969i32 as _);
pub const RTC_E_MEDIA_NEED_TERMINAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886048i32 as _);
pub const RTC_E_MEDIA_SESSION_IN_HOLD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885962i32 as _);
pub const RTC_E_MEDIA_SESSION_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885963i32 as _);
pub const RTC_E_MEDIA_VIDEO_DEVICE_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886046i32 as _);
pub const RTC_E_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885950i32 as _);
pub const RTC_E_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885992i32 as _);
pub const RTC_E_NOT_PRESENCE_PROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885974i32 as _);
pub const RTC_E_NO_BUDDY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885996i32 as _);
pub const RTC_E_NO_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886035i32 as _);
pub const RTC_E_NO_GROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885999i32 as _);
pub const RTC_E_NO_PROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886037i32 as _);
pub const RTC_E_NO_REALM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885994i32 as _);
pub const RTC_E_NO_TRANSPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885993i32 as _);
pub const RTC_E_NO_WATCHER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885995i32 as _);
pub const RTC_E_OPERATION_WITH_TOO_MANY_PARTICIPANTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886018i32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_ALL_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755001i32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_BADNUMBER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131754997i32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755003i32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131754998i32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_NO_ANSWER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755002i32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_PL_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131755000i32 as _);
pub const RTC_E_PINT_STATUS_REJECTED_SW_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131754999i32 as _);
pub const RTC_E_PLATFORM_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885952i32 as _);
pub const RTC_E_POLICY_NOT_ALLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886012i32 as _);
pub const RTC_E_PORT_MANAGER_ALREADY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885956i32 as _);
pub const RTC_E_PORT_MAPPING_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886010i32 as _);
pub const RTC_E_PORT_MAPPING_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886011i32 as _);
pub const RTC_E_PRESENCE_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885982i32 as _);
pub const RTC_E_PRESENCE_NOT_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886040i32 as _);
pub const RTC_E_PROFILE_INVALID_SERVER_AUTHMETHOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886024i32 as _);
pub const RTC_E_PROFILE_INVALID_SERVER_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886025i32 as _);
pub const RTC_E_PROFILE_INVALID_SERVER_ROLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886023i32 as _);
pub const RTC_E_PROFILE_INVALID_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886021i32 as _);
pub const RTC_E_PROFILE_INVALID_SESSION_PARTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886020i32 as _);
pub const RTC_E_PROFILE_INVALID_SESSION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886019i32 as _);
pub const RTC_E_PROFILE_MULTIPLE_REGISTRARS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886022i32 as _);
pub const RTC_E_PROFILE_NO_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886032i32 as _);
pub const RTC_E_PROFILE_NO_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886031i32 as _);
pub const RTC_E_PROFILE_NO_PROVISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886033i32 as _);
pub const RTC_E_PROFILE_NO_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886028i32 as _);
pub const RTC_E_PROFILE_NO_SERVER_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886027i32 as _);
pub const RTC_E_PROFILE_NO_SERVER_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886026i32 as _);
pub const RTC_E_PROFILE_NO_USER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886030i32 as _);
pub const RTC_E_PROFILE_NO_USER_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886029i32 as _);
pub const RTC_E_PROFILE_SERVER_UNAUTHORIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886014i32 as _);
pub const RTC_E_REDIRECT_PROCESSING_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885959i32 as _);
pub const RTC_E_REFER_NOT_ACCEPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885968i32 as _);
pub const RTC_E_REFER_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885967i32 as _);
pub const RTC_E_REFER_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885966i32 as _);
pub const RTC_E_REGISTRATION_DEACTIVATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885949i32 as _);
pub const RTC_E_REGISTRATION_REJECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885948i32 as _);
pub const RTC_E_REGISTRATION_UNREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885947i32 as _);
pub const RTC_E_ROAMING_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885981i32 as _);
pub const RTC_E_ROAMING_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886002i32 as _);
pub const RTC_E_ROAMING_OPERATION_INTERRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886003i32 as _);
pub const RTC_E_SDP_CONNECTION_ADDR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886070i32 as _);
pub const RTC_E_SDP_FAILED_TO_BUILD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886067i32 as _);
pub const RTC_E_SDP_MULTICAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886071i32 as _);
pub const RTC_E_SDP_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886074i32 as _);
pub const RTC_E_SDP_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886069i32 as _);
pub const RTC_E_SDP_PARSE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886073i32 as _);
pub const RTC_E_SDP_UPDATE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886072i32 as _);
pub const RTC_E_SECURITY_LEVEL_ALREADY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885955i32 as _);
pub const RTC_E_SECURITY_LEVEL_NOT_COMPATIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886009i32 as _);
pub const RTC_E_SECURITY_LEVEL_NOT_DEFINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886008i32 as _);
pub const RTC_E_SECURITY_LEVEL_NOT_SUPPORTED_BY_PARTICIPANT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886007i32 as _);
pub const RTC_E_SIP_ADDITIONAL_PARTY_IN_TWO_PARTY_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885986i32 as _);
pub const RTC_E_SIP_AUTH_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886063i32 as _);
pub const RTC_E_SIP_AUTH_HEADER_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886065i32 as _);
pub const RTC_E_SIP_AUTH_TIME_SKEW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885972i32 as _);
pub const RTC_E_SIP_AUTH_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886064i32 as _);
pub const RTC_E_SIP_CALL_CONNECTION_NOT_ESTABLISHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885987i32 as _);
pub const RTC_E_SIP_CALL_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886055i32 as _);
pub const RTC_E_SIP_CODECS_DO_NOT_MATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886080i32 as _);
pub const RTC_E_SIP_DNS_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885978i32 as _);
pub const RTC_E_SIP_HEADER_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886075i32 as _);
pub const RTC_E_SIP_HIGH_SECURITY_SET_TLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886016i32 as _);
pub const RTC_E_SIP_HOLD_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885965i32 as _);
pub const RTC_E_SIP_INVALID_CERTIFICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885979i32 as _);
pub const RTC_E_SIP_INVITEE_PARTY_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885973i32 as _);
pub const RTC_E_SIP_INVITE_TRANSACTION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886066i32 as _);
pub const RTC_E_SIP_NEED_MORE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886056i32 as _);
pub const RTC_E_SIP_NO_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886077i32 as _);
pub const RTC_E_SIP_OTHER_PARTY_JOIN_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885984i32 as _);
pub const RTC_E_SIP_PARSE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886076i32 as _);
pub const RTC_E_SIP_PARTY_ALREADY_IN_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885985i32 as _);
pub const RTC_E_SIP_PEER_PARTICIPANT_IN_MULTIPARTY_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885951i32 as _);
pub const RTC_E_SIP_REFER_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885953i32 as _);
pub const RTC_E_SIP_REQUEST_DESTINATION_ADDR_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886054i32 as _);
pub const RTC_E_SIP_SSL_NEGOTIATION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886051i32 as _);
pub const RTC_E_SIP_SSL_TUNNEL_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886052i32 as _);
pub const RTC_E_SIP_STACK_SHUTDOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886050i32 as _);
pub const RTC_E_SIP_STREAM_NOT_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886078i32 as _);
pub const RTC_E_SIP_STREAM_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886079i32 as _);
pub const RTC_E_SIP_TCP_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885977i32 as _);
pub const RTC_E_SIP_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886068i32 as _);
pub const RTC_E_SIP_TLS_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885975i32 as _);
pub const RTC_E_SIP_TLS_INCOMPATIBLE_ENCRYPTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885980i32 as _);
pub const RTC_E_SIP_TRANSPORT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886057i32 as _);
pub const RTC_E_SIP_UDP_SIZE_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886053i32 as _);
pub const RTC_E_SIP_UNHOLD_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885964i32 as _);
pub const RTC_E_START_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131886045i32 as _);
pub const RTC_E_STATUS_CLIENT_ADDRESS_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820060i32 as _);
pub const RTC_E_STATUS_CLIENT_AMBIGUOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820059i32 as _);
pub const RTC_E_STATUS_CLIENT_BAD_EXTENSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820124i32 as _);
pub const RTC_E_STATUS_CLIENT_BAD_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820144i32 as _);
pub const RTC_E_STATUS_CLIENT_BUSY_HERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820058i32 as _);
pub const RTC_E_STATUS_CLIENT_CONFLICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820135i32 as _);
pub const RTC_E_STATUS_CLIENT_FORBIDDEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820141i32 as _);
pub const RTC_E_STATUS_CLIENT_GONE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820134i32 as _);
pub const RTC_E_STATUS_CLIENT_LENGTH_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820133i32 as _);
pub const RTC_E_STATUS_CLIENT_LOOP_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820062i32 as _);
pub const RTC_E_STATUS_CLIENT_METHOD_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820139i32 as _);
pub const RTC_E_STATUS_CLIENT_NOT_ACCEPTABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820138i32 as _);
pub const RTC_E_STATUS_CLIENT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820140i32 as _);
pub const RTC_E_STATUS_CLIENT_PAYMENT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820142i32 as _);
pub const RTC_E_STATUS_CLIENT_PROXY_AUTHENTICATION_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820137i32 as _);
pub const RTC_E_STATUS_CLIENT_REQUEST_ENTITY_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820131i32 as _);
pub const RTC_E_STATUS_CLIENT_REQUEST_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820136i32 as _);
pub const RTC_E_STATUS_CLIENT_REQUEST_URI_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820130i32 as _);
pub const RTC_E_STATUS_CLIENT_TEMPORARILY_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820064i32 as _);
pub const RTC_E_STATUS_CLIENT_TOO_MANY_HOPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820061i32 as _);
pub const RTC_E_STATUS_CLIENT_TRANSACTION_DOES_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820063i32 as _);
pub const RTC_E_STATUS_CLIENT_UNAUTHORIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820143i32 as _);
pub const RTC_E_STATUS_CLIENT_UNSUPPORTED_MEDIA_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820129i32 as _);
pub const RTC_E_STATUS_GLOBAL_BUSY_EVERYWHERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819944i32 as _);
pub const RTC_E_STATUS_GLOBAL_DECLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819941i32 as _);
pub const RTC_E_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819940i32 as _);
pub const RTC_E_STATUS_GLOBAL_NOT_ACCEPTABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131819938i32 as _);
pub const RTC_E_STATUS_INFO_CALL_FORWARDING: ::windows::core::HRESULT = ::windows::core::HRESULT(15663285i32 as _);
pub const RTC_E_STATUS_INFO_QUEUED: ::windows::core::HRESULT = ::windows::core::HRESULT(15663286i32 as _);
pub const RTC_E_STATUS_INFO_RINGING: ::windows::core::HRESULT = ::windows::core::HRESULT(15663284i32 as _);
pub const RTC_E_STATUS_INFO_TRYING: ::windows::core::HRESULT = ::windows::core::HRESULT(15663204i32 as _);
pub const RTC_E_STATUS_NOT_ACCEPTABLE_HERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820056i32 as _);
pub const RTC_E_STATUS_REDIRECT_ALTERNATIVE_SERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820164i32 as _);
pub const RTC_E_STATUS_REDIRECT_MOVED_PERMANENTLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820243i32 as _);
pub const RTC_E_STATUS_REDIRECT_MOVED_TEMPORARILY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820242i32 as _);
pub const RTC_E_STATUS_REDIRECT_MULTIPLE_CHOICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820244i32 as _);
pub const RTC_E_STATUS_REDIRECT_SEE_OTHER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820241i32 as _);
pub const RTC_E_STATUS_REDIRECT_USE_PROXY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820239i32 as _);
pub const RTC_E_STATUS_REQUEST_TERMINATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820057i32 as _);
pub const RTC_E_STATUS_SERVER_BAD_GATEWAY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820042i32 as _);
pub const RTC_E_STATUS_SERVER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820044i32 as _);
pub const RTC_E_STATUS_SERVER_NOT_IMPLEMENTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820043i32 as _);
pub const RTC_E_STATUS_SERVER_SERVER_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820040i32 as _);
pub const RTC_E_STATUS_SERVER_SERVICE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820041i32 as _);
pub const RTC_E_STATUS_SERVER_VERSION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131820039i32 as _);
pub const RTC_E_STATUS_SESSION_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(15663287i32 as _);
pub const RTC_E_STATUS_SUCCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(15663304i32 as _);
pub const RTC_E_TOO_MANY_GROUPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885997i32 as _);
pub const RTC_E_TOO_MANY_RETRIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885989i32 as _);
pub const RTC_E_TOO_SMALL_EXPIRES_VALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885976i32 as _);
pub const RTC_E_UDP_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2131885954i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_GROUP_EVENT_TYPE(pub i32);
pub const RTCGET_GROUP_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(0i32);
pub const RTCGET_GROUP_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(1i32);
pub const RTCGET_GROUP_UPDATE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(2i32);
pub const RTCGET_GROUP_BUDDY_ADD: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(3i32);
pub const RTCGET_GROUP_BUDDY_REMOVE: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(4i32);
pub const RTCGET_GROUP_ROAMED: RTC_GROUP_EVENT_TYPE = RTC_GROUP_EVENT_TYPE(5i32);
impl ::core::convert::From<i32> for RTC_GROUP_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_GROUP_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_LISTEN_MODE(pub i32);
pub const RTCLM_NONE: RTC_LISTEN_MODE = RTC_LISTEN_MODE(0i32);
pub const RTCLM_DYNAMIC: RTC_LISTEN_MODE = RTC_LISTEN_MODE(1i32);
pub const RTCLM_BOTH: RTC_LISTEN_MODE = RTC_LISTEN_MODE(2i32);
impl ::core::convert::From<i32> for RTC_LISTEN_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_LISTEN_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_MEDIA_EVENT_REASON(pub i32);
pub const RTCMER_NORMAL: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(0i32);
pub const RTCMER_HOLD: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(1i32);
pub const RTCMER_TIMEOUT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(2i32);
pub const RTCMER_BAD_DEVICE: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(3i32);
pub const RTCMER_NO_PORT: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(4i32);
pub const RTCMER_PORT_MAPPING_FAILED: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(5i32);
pub const RTCMER_REMOTE_REQUEST: RTC_MEDIA_EVENT_REASON = RTC_MEDIA_EVENT_REASON(6i32);
impl ::core::convert::From<i32> for RTC_MEDIA_EVENT_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_MEDIA_EVENT_REASON {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_MEDIA_EVENT_TYPE(pub i32);
pub const RTCMET_STOPPED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(0i32);
pub const RTCMET_STARTED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(1i32);
pub const RTCMET_FAILED: RTC_MEDIA_EVENT_TYPE = RTC_MEDIA_EVENT_TYPE(2i32);
impl ::core::convert::From<i32> for RTC_MEDIA_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_MEDIA_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_MESSAGING_EVENT_TYPE(pub i32);
pub const RTCMSET_MESSAGE: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(0i32);
pub const RTCMSET_STATUS: RTC_MESSAGING_EVENT_TYPE = RTC_MESSAGING_EVENT_TYPE(1i32);
impl ::core::convert::From<i32> for RTC_MESSAGING_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_MESSAGING_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_MESSAGING_USER_STATUS(pub i32);
pub const RTCMUS_IDLE: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(0i32);
pub const RTCMUS_TYPING: RTC_MESSAGING_USER_STATUS = RTC_MESSAGING_USER_STATUS(1i32);
impl ::core::convert::From<i32> for RTC_MESSAGING_USER_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_MESSAGING_USER_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_OFFER_WATCHER_MODE(pub i32);
pub const RTCOWM_OFFER_WATCHER_EVENT: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(0i32);
pub const RTCOWM_AUTOMATICALLY_ADD_WATCHER: RTC_OFFER_WATCHER_MODE = RTC_OFFER_WATCHER_MODE(1i32);
impl ::core::convert::From<i32> for RTC_OFFER_WATCHER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_OFFER_WATCHER_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_PARTICIPANT_STATE(pub i32);
pub const RTCPS_IDLE: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(0i32);
pub const RTCPS_PENDING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(1i32);
pub const RTCPS_INCOMING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(2i32);
pub const RTCPS_ANSWERING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(3i32);
pub const RTCPS_INPROGRESS: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(4i32);
pub const RTCPS_ALERTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(5i32);
pub const RTCPS_CONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(6i32);
pub const RTCPS_DISCONNECTING: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(7i32);
pub const RTCPS_DISCONNECTED: RTC_PARTICIPANT_STATE = RTC_PARTICIPANT_STATE(8i32);
impl ::core::convert::From<i32> for RTC_PARTICIPANT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_PARTICIPANT_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_PORT_TYPE(pub i32);
pub const RTCPT_AUDIO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(0i32);
pub const RTCPT_AUDIO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(1i32);
pub const RTCPT_VIDEO_RTP: RTC_PORT_TYPE = RTC_PORT_TYPE(2i32);
pub const RTCPT_VIDEO_RTCP: RTC_PORT_TYPE = RTC_PORT_TYPE(3i32);
pub const RTCPT_SIP: RTC_PORT_TYPE = RTC_PORT_TYPE(4i32);
impl ::core::convert::From<i32> for RTC_PORT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_PORT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_PRESENCE_PROPERTY(pub i32);
pub const RTCPP_PHONENUMBER: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(0i32);
pub const RTCPP_DISPLAYNAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(1i32);
pub const RTCPP_EMAIL: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(2i32);
pub const RTCPP_DEVICE_NAME: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(3i32);
pub const RTCPP_MULTIPLE: RTC_PRESENCE_PROPERTY = RTC_PRESENCE_PROPERTY(4i32);
impl ::core::convert::From<i32> for RTC_PRESENCE_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_PRESENCE_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_PRESENCE_STATUS(pub i32);
pub const RTCXS_PRESENCE_OFFLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(0i32);
pub const RTCXS_PRESENCE_ONLINE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(1i32);
pub const RTCXS_PRESENCE_AWAY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(2i32);
pub const RTCXS_PRESENCE_IDLE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(3i32);
pub const RTCXS_PRESENCE_BUSY: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(4i32);
pub const RTCXS_PRESENCE_BE_RIGHT_BACK: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(5i32);
pub const RTCXS_PRESENCE_ON_THE_PHONE: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(6i32);
pub const RTCXS_PRESENCE_OUT_TO_LUNCH: RTC_PRESENCE_STATUS = RTC_PRESENCE_STATUS(7i32);
impl ::core::convert::From<i32> for RTC_PRESENCE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_PRESENCE_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_PRIVACY_MODE(pub i32);
pub const RTCPM_BLOCK_LIST_EXCLUDED: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(0i32);
pub const RTCPM_ALLOW_LIST_ONLY: RTC_PRIVACY_MODE = RTC_PRIVACY_MODE(1i32);
impl ::core::convert::From<i32> for RTC_PRIVACY_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_PRIVACY_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_PROFILE_EVENT_TYPE(pub i32);
pub const RTCPFET_PROFILE_GET: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(0i32);
pub const RTCPFET_PROFILE_UPDATE: RTC_PROFILE_EVENT_TYPE = RTC_PROFILE_EVENT_TYPE(1i32);
impl ::core::convert::From<i32> for RTC_PROFILE_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_PROFILE_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_PROVIDER_URI(pub i32);
pub const RTCPU_URIHOMEPAGE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(0i32);
pub const RTCPU_URIHELPDESK: RTC_PROVIDER_URI = RTC_PROVIDER_URI(1i32);
pub const RTCPU_URIPERSONALACCOUNT: RTC_PROVIDER_URI = RTC_PROVIDER_URI(2i32);
pub const RTCPU_URIDISPLAYDURINGCALL: RTC_PROVIDER_URI = RTC_PROVIDER_URI(3i32);
pub const RTCPU_URIDISPLAYDURINGIDLE: RTC_PROVIDER_URI = RTC_PROVIDER_URI(4i32);
impl ::core::convert::From<i32> for RTC_PROVIDER_URI {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_PROVIDER_URI {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_REGISTRATION_STATE(pub i32);
pub const RTCRS_NOT_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(0i32);
pub const RTCRS_REGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(1i32);
pub const RTCRS_REGISTERED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(2i32);
pub const RTCRS_REJECTED: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(3i32);
pub const RTCRS_UNREGISTERING: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(4i32);
pub const RTCRS_ERROR: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(5i32);
pub const RTCRS_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(6i32);
pub const RTCRS_LOCAL_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(7i32);
pub const RTCRS_REMOTE_PA_LOGGED_OFF: RTC_REGISTRATION_STATE = RTC_REGISTRATION_STATE(8i32);
impl ::core::convert::From<i32> for RTC_REGISTRATION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_REGISTRATION_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_REINVITE_STATE(pub i32);
pub const RTCRIN_INCOMING: RTC_REINVITE_STATE = RTC_REINVITE_STATE(0i32);
pub const RTCRIN_SUCCEEDED: RTC_REINVITE_STATE = RTC_REINVITE_STATE(1i32);
pub const RTCRIN_FAIL: RTC_REINVITE_STATE = RTC_REINVITE_STATE(2i32);
impl ::core::convert::From<i32> for RTC_REINVITE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_REINVITE_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_RING_TYPE(pub i32);
pub const RTCRT_PHONE: RTC_RING_TYPE = RTC_RING_TYPE(0i32);
pub const RTCRT_MESSAGE: RTC_RING_TYPE = RTC_RING_TYPE(1i32);
pub const RTCRT_RINGBACK: RTC_RING_TYPE = RTC_RING_TYPE(2i32);
impl ::core::convert::From<i32> for RTC_RING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_RING_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_ROAMING_EVENT_TYPE(pub i32);
pub const RTCRET_BUDDY_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(0i32);
pub const RTCRET_WATCHER_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(1i32);
pub const RTCRET_PRESENCE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(2i32);
pub const RTCRET_PROFILE_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(3i32);
pub const RTCRET_WPENDING_ROAMING: RTC_ROAMING_EVENT_TYPE = RTC_ROAMING_EVENT_TYPE(4i32);
impl ::core::convert::From<i32> for RTC_ROAMING_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_ROAMING_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_SECURITY_LEVEL(pub i32);
pub const RTCSECL_UNSUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(1i32);
pub const RTCSECL_SUPPORTED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(2i32);
pub const RTCSECL_REQUIRED: RTC_SECURITY_LEVEL = RTC_SECURITY_LEVEL(3i32);
impl ::core::convert::From<i32> for RTC_SECURITY_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_SECURITY_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_SECURITY_TYPE(pub i32);
pub const RTCSECT_AUDIO_VIDEO_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(0i32);
pub const RTCSECT_T120_MEDIA_ENCRYPTION: RTC_SECURITY_TYPE = RTC_SECURITY_TYPE(1i32);
impl ::core::convert::From<i32> for RTC_SECURITY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_SECURITY_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_SESSION_REFER_STATUS(pub i32);
pub const RTCSRS_REFERRING: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(0i32);
pub const RTCSRS_ACCEPTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(1i32);
pub const RTCSRS_ERROR: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(2i32);
pub const RTCSRS_REJECTED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(3i32);
pub const RTCSRS_DROPPED: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(4i32);
pub const RTCSRS_DONE: RTC_SESSION_REFER_STATUS = RTC_SESSION_REFER_STATUS(5i32);
impl ::core::convert::From<i32> for RTC_SESSION_REFER_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_SESSION_REFER_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_SESSION_STATE(pub i32);
pub const RTCSS_IDLE: RTC_SESSION_STATE = RTC_SESSION_STATE(0i32);
pub const RTCSS_INCOMING: RTC_SESSION_STATE = RTC_SESSION_STATE(1i32);
pub const RTCSS_ANSWERING: RTC_SESSION_STATE = RTC_SESSION_STATE(2i32);
pub const RTCSS_INPROGRESS: RTC_SESSION_STATE = RTC_SESSION_STATE(3i32);
pub const RTCSS_CONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(4i32);
pub const RTCSS_DISCONNECTED: RTC_SESSION_STATE = RTC_SESSION_STATE(5i32);
pub const RTCSS_HOLD: RTC_SESSION_STATE = RTC_SESSION_STATE(6i32);
pub const RTCSS_REFER: RTC_SESSION_STATE = RTC_SESSION_STATE(7i32);
impl ::core::convert::From<i32> for RTC_SESSION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_SESSION_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_SESSION_TYPE(pub i32);
pub const RTCST_PC_TO_PC: RTC_SESSION_TYPE = RTC_SESSION_TYPE(0i32);
pub const RTCST_PC_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(1i32);
pub const RTCST_PHONE_TO_PHONE: RTC_SESSION_TYPE = RTC_SESSION_TYPE(2i32);
pub const RTCST_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(3i32);
pub const RTCST_MULTIPARTY_IM: RTC_SESSION_TYPE = RTC_SESSION_TYPE(4i32);
pub const RTCST_APPLICATION: RTC_SESSION_TYPE = RTC_SESSION_TYPE(5i32);
impl ::core::convert::From<i32> for RTC_SESSION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_SESSION_TYPE {
    type Abi = Self;
}
pub const RTC_S_ROAMING_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(15597633i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_T120_APPLET(pub i32);
pub const RTCTA_WHITEBOARD: RTC_T120_APPLET = RTC_T120_APPLET(0i32);
pub const RTCTA_APPSHARING: RTC_T120_APPLET = RTC_T120_APPLET(1i32);
impl ::core::convert::From<i32> for RTC_T120_APPLET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_T120_APPLET {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_TERMINATE_REASON(pub i32);
pub const RTCTR_NORMAL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(0i32);
pub const RTCTR_DND: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(1i32);
pub const RTCTR_BUSY: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(2i32);
pub const RTCTR_REJECT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(3i32);
pub const RTCTR_TIMEOUT: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(4i32);
pub const RTCTR_SHUTDOWN: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(5i32);
pub const RTCTR_INSUFFICIENT_SECURITY_LEVEL: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(6i32);
pub const RTCTR_NOT_SUPPORTED: RTC_TERMINATE_REASON = RTC_TERMINATE_REASON(7i32);
impl ::core::convert::From<i32> for RTC_TERMINATE_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_TERMINATE_REASON {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_USER_SEARCH_COLUMN(pub i32);
pub const RTCUSC_URI: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(0i32);
pub const RTCUSC_DISPLAYNAME: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(1i32);
pub const RTCUSC_TITLE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(2i32);
pub const RTCUSC_OFFICE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(3i32);
pub const RTCUSC_PHONE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(4i32);
pub const RTCUSC_COMPANY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(5i32);
pub const RTCUSC_CITY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(6i32);
pub const RTCUSC_STATE: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(7i32);
pub const RTCUSC_COUNTRY: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(8i32);
pub const RTCUSC_EMAIL: RTC_USER_SEARCH_COLUMN = RTC_USER_SEARCH_COLUMN(9i32);
impl ::core::convert::From<i32> for RTC_USER_SEARCH_COLUMN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_USER_SEARCH_COLUMN {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_USER_SEARCH_PREFERENCE(pub i32);
pub const RTCUSP_MAX_MATCHES: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(0i32);
pub const RTCUSP_TIME_LIMIT: RTC_USER_SEARCH_PREFERENCE = RTC_USER_SEARCH_PREFERENCE(1i32);
impl ::core::convert::From<i32> for RTC_USER_SEARCH_PREFERENCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_USER_SEARCH_PREFERENCE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_VIDEO_DEVICE(pub i32);
pub const RTCVD_RECEIVE: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(0i32);
pub const RTCVD_PREVIEW: RTC_VIDEO_DEVICE = RTC_VIDEO_DEVICE(1i32);
impl ::core::convert::From<i32> for RTC_VIDEO_DEVICE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_VIDEO_DEVICE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_WATCHER_EVENT_TYPE(pub i32);
pub const RTCWET_WATCHER_ADD: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(0i32);
pub const RTCWET_WATCHER_REMOVE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(1i32);
pub const RTCWET_WATCHER_UPDATE: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(2i32);
pub const RTCWET_WATCHER_OFFERING: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(3i32);
pub const RTCWET_WATCHER_ROAMED: RTC_WATCHER_EVENT_TYPE = RTC_WATCHER_EVENT_TYPE(4i32);
impl ::core::convert::From<i32> for RTC_WATCHER_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_WATCHER_EVENT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_WATCHER_MATCH_MODE(pub i32);
pub const RTCWMM_EXACT_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(0i32);
pub const RTCWMM_BEST_ACE_MATCH: RTC_WATCHER_MATCH_MODE = RTC_WATCHER_MATCH_MODE(1i32);
impl ::core::convert::From<i32> for RTC_WATCHER_MATCH_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_WATCHER_MATCH_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTC_WATCHER_STATE(pub i32);
pub const RTCWS_UNKNOWN: RTC_WATCHER_STATE = RTC_WATCHER_STATE(0i32);
pub const RTCWS_OFFERING: RTC_WATCHER_STATE = RTC_WATCHER_STATE(1i32);
pub const RTCWS_ALLOWED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(2i32);
pub const RTCWS_BLOCKED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(3i32);
pub const RTCWS_DENIED: RTC_WATCHER_STATE = RTC_WATCHER_STATE(4i32);
pub const RTCWS_PROMPT: RTC_WATCHER_STATE = RTC_WATCHER_STATE(5i32);
impl ::core::convert::From<i32> for RTC_WATCHER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RTC_WATCHER_STATE {
    type Abi = Self;
}
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct TRANSPORT_SETTING {
    pub SettingId: super::super::Networking::WinSock::TRANSPORT_SETTING_ID,
    pub Length: *mut u32,
    pub Value: *mut u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl TRANSPORT_SETTING {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for TRANSPORT_SETTING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TRANSPORT_SETTING").field("SettingId", &self.SettingId).field("Length", &self.Length).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for TRANSPORT_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.SettingId == other.SettingId && self.Length == other.Length && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for TRANSPORT_SETTING {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for TRANSPORT_SETTING {
    type Abi = Self;
}
