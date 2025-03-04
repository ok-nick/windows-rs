#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AdvancedCapturedPhoto(pub ::windows::core::IInspectable);
impl AdvancedCapturedPhoto {
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn Mode(&self) -> ::windows::core::Result<super::Devices::AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__: super::Devices::AdvancedPhotoMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::AdvancedPhotoMode>(result__)
        }
    }
    pub fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FrameBoundsRelativeToReferencePhoto(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>> {
        let this = &::windows::core::Interface::cast::<IAdvancedCapturedPhoto2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Rect>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedCapturedPhoto {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedCapturedPhoto;{f072728b-b292-4491-9d41-99807a550bbf})");
}
unsafe impl ::windows::core::Interface for AdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf072728b_b292_4491_9d41_99807a550bbf);
}
impl ::windows::core::RuntimeName for AdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedCapturedPhoto";
}
impl ::core::convert::From<AdvancedCapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: AdvancedCapturedPhoto) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AdvancedCapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: &AdvancedCapturedPhoto) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AdvancedCapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: AdvancedCapturedPhoto) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AdvancedCapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: &AdvancedCapturedPhoto) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AdvancedCapturedPhoto {}
unsafe impl ::core::marker::Sync for AdvancedCapturedPhoto {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AdvancedPhotoCapture(pub ::windows::core::IInspectable);
impl AdvancedPhotoCapture {
    #[cfg(feature = "Foundation")]
    pub fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CaptureWithContextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, context: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OptionalReferencePhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionalReferencePhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AllPhotosCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAllPhotosCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedPhotoCapture;{83ffaafa-6667-44dc-973c-a6bce596aa0f})");
}
unsafe impl ::windows::core::Interface for AdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83ffaafa_6667_44dc_973c_a6bce596aa0f);
}
impl ::windows::core::RuntimeName for AdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedPhotoCapture";
}
impl ::core::convert::From<AdvancedPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: AdvancedPhotoCapture) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AdvancedPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoCapture) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AdvancedPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: AdvancedPhotoCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AdvancedPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AdvancedPhotoCapture {}
unsafe impl ::core::marker::Sync for AdvancedPhotoCapture {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastBackgroundService(pub ::windows::core::IInspectable);
impl AppBroadcastBackgroundService {
    pub fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    pub fn SetSignInInfo<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastBackgroundServiceSignInInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SignInInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceSignInInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundServiceSignInInfo>(result__)
        }
    }
    pub fn SetStreamInfo<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastBackgroundServiceStreamInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StreamInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceStreamInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundServiceStreamInfo>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetViewerCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ViewerCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), reason, providerspecificreason).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeartbeatRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeartbeatRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn BroadcastChannel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastChannel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BroadcastTitleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastTitleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BroadcastLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BroadcastChannelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastChannelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastBackgroundService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundService;{bad1e72a-fa94-46f9-95fc-d71511cda70b})");
}
unsafe impl ::windows::core::Interface for AppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad1e72a_fa94_46f9_95fc_d71511cda70b);
}
impl ::windows::core::RuntimeName for AppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundService";
}
impl ::core::convert::From<AppBroadcastBackgroundService> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastBackgroundService) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundService> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastBackgroundService) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastBackgroundService> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastBackgroundService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundService> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastBackgroundService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastBackgroundServiceSignInInfo(pub ::windows::core::IInspectable);
impl AppBroadcastBackgroundServiceSignInInfo {
    pub fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetOAuthRequestUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetOAuthCallbackUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    pub fn SetUserName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SignInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSignInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn UserNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastBackgroundServiceSignInInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo;{5e735275-88c8-4eca-89ba-4825985db880})");
}
unsafe impl ::windows::core::Interface for AppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e735275_88c8_4eca_89ba_4825985db880);
}
impl ::windows::core::RuntimeName for AppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo";
}
impl ::core::convert::From<AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastBackgroundServiceSignInInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastBackgroundServiceSignInInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastBackgroundServiceSignInInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastBackgroundServiceSignInInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastBackgroundServiceStreamInfo(pub ::windows::core::IInspectable);
impl AppBroadcastBackgroundServiceStreamInfo {
    pub fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    pub fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredVideoEncodingBitrate(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SetBandwidthTestBitrate(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BandwidthTestBitrate(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SetAudioCodec<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AudioCodec(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn BroadcastStreamReader(&self) -> ::windows::core::Result<AppBroadcastStreamReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VideoEncodingResolutionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoEncodingResolutionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VideoEncodingBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoEncodingBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ReportProblemWithStream(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundServiceStreamInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastBackgroundServiceStreamInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo;{31dc02bc-990a-4904-aa96-fe364381f136})");
}
unsafe impl ::windows::core::Interface for AppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31dc02bc_990a_4904_aa96_fe364381f136);
}
impl ::windows::core::RuntimeName for AppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo";
}
impl ::core::convert::From<AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastBackgroundServiceStreamInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastBackgroundServiceStreamInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastBackgroundServiceStreamInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastBackgroundServiceStreamInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(0i32);
    pub const Started: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(1i32);
    pub const Failed: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(2i32);
}
impl ::core::convert::From<i32> for AppBroadcastCameraCaptureState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastCameraCaptureState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraCaptureState;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastCameraCaptureState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastCameraCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs;{1e334cd0-b882-4b88-8692-05999aceb70f})");
}
unsafe impl ::windows::core::Interface for AppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e334cd0_b882_4b88_8692_05999aceb70f);
}
impl ::windows::core::RuntimeName for AppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastCameraCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastCameraCaptureStateChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(0i32);
    pub const TopCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(1i32);
    pub const TopRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(2i32);
    pub const MiddleLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(3i32);
    pub const MiddleCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(4i32);
    pub const MiddleRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(5i32);
    pub const BottomLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(6i32);
    pub const BottomCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(7i32);
    pub const BottomRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(8i32);
}
impl ::core::convert::From<i32> for AppBroadcastCameraOverlayLocation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastCameraOverlayLocation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraOverlayLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlayLocation;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastCameraOverlayLocation {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(0i32);
    pub const Medium: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(1i32);
    pub const Large: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(2i32);
}
impl ::core::convert::From<i32> for AppBroadcastCameraOverlaySize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastCameraOverlaySize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraOverlaySize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlaySize;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastCameraOverlaySize {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: AppBroadcastCaptureTargetType = AppBroadcastCaptureTargetType(0i32);
    pub const EntireDisplay: AppBroadcastCaptureTargetType = AppBroadcastCaptureTargetType(1i32);
}
impl ::core::convert::From<i32> for AppBroadcastCaptureTargetType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastCaptureTargetType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCaptureTargetType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCaptureTargetType;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastCaptureTargetType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(0i32);
    pub const UserCanceled: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(1i32);
    pub const AuthorizationFail: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(2i32);
    pub const ForegroundAppActivated: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(3i32);
}
impl ::core::convert::From<i32> for AppBroadcastExitBroadcastModeReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastExitBroadcastModeReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastExitBroadcastModeReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastExitBroadcastModeReason;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastExitBroadcastModeReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastGlobalSettings(pub ::windows::core::IInspectable);
impl AppBroadcastGlobalSettings {
    pub fn IsBroadcastEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCameraCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectedCameraId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedCameraId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CameraOverlayLocation(&self) -> ::windows::core::Result<AppBroadcastCameraOverlayLocation> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraOverlayLocation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraOverlayLocation>(result__)
        }
    }
    pub fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CameraOverlaySize(&self) -> ::windows::core::Result<AppBroadcastCameraOverlaySize> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraOverlaySize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraOverlaySize>(result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastGlobalSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastGlobalSettings;{b2cb27a5-70fc-4e17-80bd-6ba0fd3ff3a0})");
}
unsafe impl ::windows::core::Interface for AppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2cb27a5_70fc_4e17_80bd_6ba0fd3ff3a0);
}
impl ::windows::core::RuntimeName for AppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastGlobalSettings";
}
impl ::core::convert::From<AppBroadcastGlobalSettings> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastGlobalSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastGlobalSettings> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastGlobalSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastGlobalSettings> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastGlobalSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastGlobalSettings> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastGlobalSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastHeartbeatRequestedEventArgs {
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastHeartbeatRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs;{cea54283-ee51-4dbf-9472-79a9ed4e2165})");
}
unsafe impl ::windows::core::Interface for AppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcea54283_ee51_4dbf_9472_79a9ed4e2165);
}
impl ::windows::core::RuntimeName for AppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs";
}
impl ::core::convert::From<AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
pub struct AppBroadcastManager {}
impl AppBroadcastManager {
    pub fn GetGlobalSettings() -> ::windows::core::Result<AppBroadcastGlobalSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastGlobalSettings>(result__)
        })
    }
    pub fn ApplyGlobalSettings<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastGlobalSettings>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn GetProviderSettings() -> ::windows::core::Result<AppBroadcastProviderSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastProviderSettings>(result__)
        })
    }
    pub fn ApplyProviderSettings<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastProviderSettings>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn IAppBroadcastManagerStatics<R, F: FnOnce(&IAppBroadcastManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppBroadcastManager, IAppBroadcastManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppBroadcastManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastManager";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(0i32);
    pub const Started: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(1i32);
    pub const Failed: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(2i32);
}
impl ::core::convert::From<i32> for AppBroadcastMicrophoneCaptureState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastMicrophoneCaptureState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastMicrophoneCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastMicrophoneCaptureState;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastMicrophoneCaptureState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs;{a86ad5e9-9440-4908-9d09-65b7e315d795})");
}
unsafe impl ::windows::core::Interface for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa86ad5e9_9440_4908_9d09_65b7e315d795);
}
impl ::windows::core::RuntimeName for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPlugIn(pub ::windows::core::IInspectable);
impl AppBroadcastPlugIn {
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ProviderSettings(&self) -> ::windows::core::Result<AppBroadcastProviderSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastProviderSettings>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugIn {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugIn;{520c1e66-6513-4574-ac54-23b79729615b})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520c1e66_6513_4574_ac54_23b79729615b);
}
impl ::windows::core::RuntimeName for AppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugIn";
}
impl ::core::convert::From<AppBroadcastPlugIn> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPlugIn) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPlugIn> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPlugIn) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPlugIn> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPlugIn) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPlugIn> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPlugIn) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugIn {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugIn {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPlugInManager(pub ::windows::core::IInspectable);
impl AppBroadcastPlugInManager {
    pub fn IsBroadcastProviderAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PlugInList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>>(result__)
        }
    }
    pub fn DefaultPlugIn(&self) -> ::windows::core::Result<AppBroadcastPlugIn> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugIn>(result__)
        }
    }
    pub fn SetDefaultPlugIn<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastPlugIn>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    pub fn IAppBroadcastPlugInManagerStatics<R, F: FnOnce(&IAppBroadcastPlugInManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppBroadcastPlugInManager, IAppBroadcastPlugInManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugInManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInManager;{e550d979-27a1-49a7-bbf4-d7a9e9d07668})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe550d979_27a1_49a7_bbf4_d7a9e9d07668);
}
impl ::windows::core::RuntimeName for AppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInManager";
}
impl ::core::convert::From<AppBroadcastPlugInManager> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPlugInManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPlugInManager> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPlugInManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPlugInManager> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPlugInManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPlugInManager> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPlugInManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugInManager {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastPlugInState(pub i32);
impl AppBroadcastPlugInState {
    pub const Unknown: AppBroadcastPlugInState = AppBroadcastPlugInState(0i32);
    pub const Initialized: AppBroadcastPlugInState = AppBroadcastPlugInState(1i32);
    pub const MicrosoftSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(2i32);
    pub const OAuthSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(3i32);
    pub const ProviderSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(4i32);
    pub const InBandwidthTest: AppBroadcastPlugInState = AppBroadcastPlugInState(5i32);
    pub const ReadyToBroadcast: AppBroadcastPlugInState = AppBroadcastPlugInState(6i32);
}
impl ::core::convert::From<i32> for AppBroadcastPlugInState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastPlugInState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugInState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPlugInState;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastPlugInState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPlugInStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastPlugInStateChangedEventArgs {
    pub fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugInStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs;{4881d0f2-abc5-4fc6-84b0-89370bb47212})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4881d0f2_abc5_4fc6_84b0_89370bb47212);
}
impl ::windows::core::RuntimeName for AppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPlugInStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPlugInStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPlugInStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPlugInStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugInStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPreview(pub ::windows::core::IInspectable);
impl AppBroadcastPreview {
    pub fn StopPreview(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPreviewState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PreviewStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePreviewStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn PreviewStreamReader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamReader>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreview;{14b60f5a-6e4a-4b80-a14f-67ee77d153e7})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14b60f5a_6e4a_4b80_a14f_67ee77d153e7);
}
impl ::windows::core::RuntimeName for AppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreview";
}
impl ::core::convert::From<AppBroadcastPreview> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPreview> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPreview> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPreview> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreview {}
unsafe impl ::core::marker::Sync for AppBroadcastPreview {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: AppBroadcastPreviewState = AppBroadcastPreviewState(0i32);
    pub const Stopped: AppBroadcastPreviewState = AppBroadcastPreviewState(1i32);
    pub const Failed: AppBroadcastPreviewState = AppBroadcastPreviewState(2i32);
}
impl ::core::convert::From<i32> for AppBroadcastPreviewState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastPreviewState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPreviewState;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastPreviewState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPreviewStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastPreviewStateChangedEventArgs {
    pub fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPreviewState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs;{5a57f2de-8dea-4e86-90ad-03fc26b9653c})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a57f2de_8dea_4e86_90ad_03fc26b9653c);
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPreviewStreamReader(pub ::windows::core::IInspectable);
impl AppBroadcastPreviewStreamReader {
    pub fn VideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn VideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn VideoStride(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamVideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStreamReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamReader;{92228d50-db3f-40a8-8cd4-f4e371ddab37})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92228d50_db3f_40a8_8cd4_f4e371ddab37);
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamReader";
}
impl ::core::convert::From<AppBroadcastPreviewStreamReader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamReader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamReader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamReader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamReader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamReader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPreviewStreamVideoFrame(pub ::windows::core::IInspectable);
impl AppBroadcastPreviewStreamVideoFrame {
    pub fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamVideoHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStreamVideoFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame;{010fbea1-94fe-4499-b8c0-8d244279fb12})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x010fbea1_94fe_4499_b8c0_8d244279fb12);
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame";
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamVideoFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamVideoFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamVideoFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamVideoFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoFrame {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastPreviewStreamVideoHeader(pub ::windows::core::IInspectable);
impl AppBroadcastPreviewStreamVideoHeader {
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStreamVideoHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader;{8bef6113-da84-4499-a7ab-87118cb4a157})");
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bef6113_da84_4499_a7ab_87118cb4a157);
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader";
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamVideoHeader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamVideoHeader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamVideoHeader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamVideoHeader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoHeader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoHeader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastProviderSettings(pub ::windows::core::IInspectable);
impl AppBroadcastProviderSettings {
    pub fn SetDefaultBroadcastTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DefaultBroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastVideoEncodingBitrateMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastVideoEncodingBitrateMode>(result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastVideoEncodingResolutionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastVideoEncodingResolutionMode>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastProviderSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastProviderSettings;{c30bdf62-9948-458f-ad50-aa06ec03da08})");
}
unsafe impl ::windows::core::Interface for AppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc30bdf62_9948_458f_ad50_aa06ec03da08);
}
impl ::windows::core::RuntimeName for AppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastProviderSettings";
}
impl ::core::convert::From<AppBroadcastProviderSettings> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastProviderSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastProviderSettings> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastProviderSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastProviderSettings> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastProviderSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastProviderSettings> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastProviderSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastServices(pub ::windows::core::IInspectable);
impl AppBroadcastServices {
    pub fn CaptureTargetType(&self) -> ::windows::core::Result<AppBroadcastCaptureTargetType> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCaptureTargetType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCaptureTargetType>(result__)
        }
    }
    pub fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetBroadcastLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CanCapture(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnterBroadcastModeAsync<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastPlugIn>>(&self, plugin: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), plugin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    pub fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), reason).ok() }
    }
    pub fn StartBroadcast(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn PauseBroadcast(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ResumeBroadcast(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPreview<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, desiredsize: Param0) -> ::windows::core::Result<AppBroadcastPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), desiredsize.into_param().abi(), &mut result__).from_abi::<AppBroadcastPreview>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<AppBroadcastState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastServices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastServices;{8660b4d6-969b-4e3c-ac3a-8b042ee4ee63})");
}
unsafe impl ::windows::core::Interface for AppBroadcastServices {
    type Vtable = IAppBroadcastServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8660b4d6_969b_4e3c_ac3a_8b042ee4ee63);
}
impl ::windows::core::RuntimeName for AppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastServices";
}
impl ::core::convert::From<AppBroadcastServices> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastServices) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastServices> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastServices) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastServices> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastServices> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastServices {}
unsafe impl ::core::marker::Sync for AppBroadcastServices {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: AppBroadcastSignInResult = AppBroadcastSignInResult(0i32);
    pub const AuthenticationFailed: AppBroadcastSignInResult = AppBroadcastSignInResult(1i32);
    pub const Unauthorized: AppBroadcastSignInResult = AppBroadcastSignInResult(2i32);
    pub const ServiceUnavailable: AppBroadcastSignInResult = AppBroadcastSignInResult(3i32);
    pub const Unknown: AppBroadcastSignInResult = AppBroadcastSignInResult(4i32);
}
impl ::core::convert::From<i32> for AppBroadcastSignInResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastSignInResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastSignInResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInResult;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastSignInResult {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: AppBroadcastSignInState = AppBroadcastSignInState(0i32);
    pub const MicrosoftSignInInProgress: AppBroadcastSignInState = AppBroadcastSignInState(1i32);
    pub const MicrosoftSignInComplete: AppBroadcastSignInState = AppBroadcastSignInState(2i32);
    pub const OAuthSignInInProgress: AppBroadcastSignInState = AppBroadcastSignInState(3i32);
    pub const OAuthSignInComplete: AppBroadcastSignInState = AppBroadcastSignInState(4i32);
}
impl ::core::convert::From<i32> for AppBroadcastSignInState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastSignInState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastSignInState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInState;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastSignInState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastSignInStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastSignInStateChangedEventArgs {
    pub fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    pub fn Result(&self) -> ::windows::core::Result<AppBroadcastSignInResult> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInResult>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastSignInStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs;{02b692a4-5919-4a9e-8d5e-c9bb0dd3377a})");
}
unsafe impl ::windows::core::Interface for AppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b692a4_5919_4a9e_8d5e_c9bb0dd3377a);
}
impl ::windows::core::RuntimeName for AppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastSignInStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastSignInStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastSignInStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastSignInStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastState(pub ::windows::core::IInspectable);
impl AppBroadcastState {
    pub fn IsCaptureTargetRunning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ViewerCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ShouldCaptureCamera(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldCaptureCamera(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RestartCameraCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EncodedVideoSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn CameraCaptureState(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    pub fn CameraCaptureError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    pub fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn SetAuthenticationResult<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Authentication::Web::WebAuthenticationResult>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SetSignInState(&self, value: AppBroadcastSignInState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    pub fn TerminationReason(&self) -> ::windows::core::Result<AppBroadcastTerminationReason> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastTerminationReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastTerminationReason>(result__)
        }
    }
    pub fn TerminationReasonPlugInSpecific(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ViewerCountChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveViewerCountChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CameraCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PlugInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePlugInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastState;{ee08056d-8099-4ddd-922e-c56dac58abfb})");
}
unsafe impl ::windows::core::Interface for AppBroadcastState {
    type Vtable = IAppBroadcastState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee08056d_8099_4ddd_922e_c56dac58abfb);
}
impl ::windows::core::RuntimeName for AppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastState";
}
impl ::core::convert::From<AppBroadcastState> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastState> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastState> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastState> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastState {}
unsafe impl ::core::marker::Sync for AppBroadcastState {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastStreamAudioFrame(pub ::windows::core::IInspectable);
impl AppBroadcastStreamAudioFrame {
    pub fn AudioHeader(&self) -> ::windows::core::Result<AppBroadcastStreamAudioHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamAudioHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamAudioFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioFrame;{efab4ac8-21ba-453f-8bb7-5e938a2e9a74})");
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab4ac8_21ba_453f_8bb7_5e938a2e9a74);
}
impl ::windows::core::RuntimeName for AppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioFrame";
}
impl ::core::convert::From<AppBroadcastStreamAudioFrame> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamAudioFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioFrame> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamAudioFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastStreamAudioFrame> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamAudioFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioFrame> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamAudioFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastStreamAudioHeader(pub ::windows::core::IInspectable);
impl AppBroadcastStreamAudioHeader {
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamAudioHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioHeader;{bf21a570-6b78-4216-9f07-5aff5256f1b7})");
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf21a570_6b78_4216_9f07_5aff5256f1b7);
}
impl ::windows::core::RuntimeName for AppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioHeader";
}
impl ::core::convert::From<AppBroadcastStreamAudioHeader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamAudioHeader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioHeader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamAudioHeader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastStreamAudioHeader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamAudioHeader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioHeader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamAudioHeader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastStreamReader(pub ::windows::core::IInspectable);
impl AppBroadcastStreamReader {
    pub fn AudioChannels(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AudioSampleRate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioAacSequence(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn AudioBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TryGetNextAudioFrame(&self) -> ::windows::core::Result<AppBroadcastStreamAudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamAudioFrame>(result__)
        }
    }
    pub fn VideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn VideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn VideoBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamVideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AudioFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamReader;{b338bcf9-3364-4460-b5f1-3cc2796a8aa2})");
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb338bcf9_3364_4460_b5f1_3cc2796a8aa2);
}
impl ::windows::core::RuntimeName for AppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamReader";
}
impl ::core::convert::From<AppBroadcastStreamReader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastStreamReader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastStreamReader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastStreamReader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: AppBroadcastStreamState = AppBroadcastStreamState(0i32);
    pub const StreamReady: AppBroadcastStreamState = AppBroadcastStreamState(1i32);
    pub const Started: AppBroadcastStreamState = AppBroadcastStreamState(2i32);
    pub const Paused: AppBroadcastStreamState = AppBroadcastStreamState(3i32);
    pub const Terminated: AppBroadcastStreamState = AppBroadcastStreamState(4i32);
}
impl ::core::convert::From<i32> for AppBroadcastStreamState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastStreamState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastStreamState;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastStreamState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastStreamStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastStreamStateChangedEventArgs {
    pub fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs;{5108a733-d008-4a89-93be-58aed961374e})");
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5108a733_d008_4a89_93be_58aed961374e);
}
impl ::windows::core::RuntimeName for AppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastStreamVideoFrame(pub ::windows::core::IInspectable);
impl AppBroadcastStreamVideoFrame {
    pub fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamVideoHeader>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamVideoFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoFrame;{0f97cf2b-c9e4-4e88-8194-d814cbd585d8})");
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f97cf2b_c9e4_4e88_8194_d814cbd585d8);
}
impl ::windows::core::RuntimeName for AppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoFrame";
}
impl ::core::convert::From<AppBroadcastStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamVideoFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamVideoFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamVideoFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamVideoFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastStreamVideoHeader(pub ::windows::core::IInspectable);
impl AppBroadcastStreamVideoHeader {
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn IsKeyFrame(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasDiscontinuity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn FrameId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamVideoHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoHeader;{0b9ebece-7e32-432d-8ca2-36bf10b9f462})");
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b9ebece_7e32_432d_8ca2_36bf10b9f462);
}
impl ::windows::core::RuntimeName for AppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoHeader";
}
impl ::core::convert::From<AppBroadcastStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamVideoHeader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamVideoHeader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamVideoHeader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamVideoHeader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastTerminationReason(pub i32);
impl AppBroadcastTerminationReason {
    pub const NormalTermination: AppBroadcastTerminationReason = AppBroadcastTerminationReason(0i32);
    pub const LostConnectionToService: AppBroadcastTerminationReason = AppBroadcastTerminationReason(1i32);
    pub const NoNetworkConnectivity: AppBroadcastTerminationReason = AppBroadcastTerminationReason(2i32);
    pub const ServiceAbort: AppBroadcastTerminationReason = AppBroadcastTerminationReason(3i32);
    pub const ServiceError: AppBroadcastTerminationReason = AppBroadcastTerminationReason(4i32);
    pub const ServiceUnavailable: AppBroadcastTerminationReason = AppBroadcastTerminationReason(5i32);
    pub const InternalError: AppBroadcastTerminationReason = AppBroadcastTerminationReason(6i32);
    pub const UnsupportedFormat: AppBroadcastTerminationReason = AppBroadcastTerminationReason(7i32);
    pub const BackgroundTaskTerminated: AppBroadcastTerminationReason = AppBroadcastTerminationReason(8i32);
    pub const BackgroundTaskUnresponsive: AppBroadcastTerminationReason = AppBroadcastTerminationReason(9i32);
}
impl ::core::convert::From<i32> for AppBroadcastTerminationReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastTerminationReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastTerminationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastTerminationReason;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastTerminationReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastTriggerDetails(pub ::windows::core::IInspectable);
impl AppBroadcastTriggerDetails {
    pub fn BackgroundService(&self) -> ::windows::core::Result<AppBroadcastBackgroundService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundService>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastTriggerDetails;{deebab35-ec5e-4d8f-b1c0-5da6e8c75638})");
}
unsafe impl ::windows::core::Interface for AppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeebab35_ec5e_4d8f_b1c0_5da6e8c75638);
}
impl ::windows::core::RuntimeName for AppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastTriggerDetails";
}
impl ::core::convert::From<AppBroadcastTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: AppBroadcastVideoEncodingBitrateMode = AppBroadcastVideoEncodingBitrateMode(0i32);
    pub const Auto: AppBroadcastVideoEncodingBitrateMode = AppBroadcastVideoEncodingBitrateMode(1i32);
}
impl ::core::convert::From<i32> for AppBroadcastVideoEncodingBitrateMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastVideoEncodingBitrateMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastVideoEncodingBitrateMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingBitrateMode;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastVideoEncodingBitrateMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: AppBroadcastVideoEncodingResolutionMode = AppBroadcastVideoEncodingResolutionMode(0i32);
    pub const Auto: AppBroadcastVideoEncodingResolutionMode = AppBroadcastVideoEncodingResolutionMode(1i32);
}
impl ::core::convert::From<i32> for AppBroadcastVideoEncodingResolutionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppBroadcastVideoEncodingResolutionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastVideoEncodingResolutionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingResolutionMode;i4)");
}
impl ::windows::core::DefaultType for AppBroadcastVideoEncodingResolutionMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppBroadcastViewerCountChangedEventArgs(pub ::windows::core::IInspectable);
impl AppBroadcastViewerCountChangedEventArgs {
    pub fn ViewerCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastViewerCountChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs;{e6e11825-5401-4ade-8bd2-c14ecee6807d})");
}
unsafe impl ::windows::core::Interface for AppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6e11825_5401_4ade_8bd2_c14ecee6807d);
}
impl ::windows::core::RuntimeName for AppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastViewerCountChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastViewerCountChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastViewerCountChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastViewerCountChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppBroadcastViewerCountChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastViewerCountChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCapture(pub ::windows::core::IInspectable);
impl AppCapture {
    pub fn IsCapturingAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsCapturingVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CapturingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCapturingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<AppCapture> {
        Self::IAppCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCapture>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetAllowedAsync(allowed: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppCaptureStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), allowed, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IAppCaptureStatics<R, F: FnOnce(&IAppCaptureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCapture, IAppCaptureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAppCaptureStatics2<R, F: FnOnce(&IAppCaptureStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCapture, IAppCaptureStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCapture;{9749d453-a29a-45ed-8f29-22d09942cff7})");
}
unsafe impl ::windows::core::Interface for AppCapture {
    type Vtable = IAppCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9749d453_a29a_45ed_8f29_22d09942cff7);
}
impl ::windows::core::RuntimeName for AppCapture {
    const NAME: &'static str = "Windows.Media.Capture.AppCapture";
}
impl ::core::convert::From<AppCapture> for ::windows::core::IUnknown {
    fn from(value: AppCapture) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCapture> for ::windows::core::IUnknown {
    fn from(value: &AppCapture) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCapture> for ::windows::core::IInspectable {
    fn from(value: AppCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCapture> for ::windows::core::IInspectable {
    fn from(value: &AppCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureAlternateShortcutKeys(pub ::windows::core::IInspectable);
impl AppCaptureAlternateShortcutKeys {
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureAlternateShortcutKeys {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureAlternateShortcutKeys;{19e8e0ef-236c-40f9-b38f-9b7dd65d1ccc})");
}
unsafe impl ::windows::core::Interface for AppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19e8e0ef_236c_40f9_b38f_9b7dd65d1ccc);
}
impl ::windows::core::RuntimeName for AppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureAlternateShortcutKeys";
}
impl ::core::convert::From<AppCaptureAlternateShortcutKeys> for ::windows::core::IUnknown {
    fn from(value: AppCaptureAlternateShortcutKeys) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureAlternateShortcutKeys> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureAlternateShortcutKeys) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureAlternateShortcutKeys> for ::windows::core::IInspectable {
    fn from(value: AppCaptureAlternateShortcutKeys) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureAlternateShortcutKeys> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureAlternateShortcutKeys) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureDurationGeneratedEventArgs(pub ::windows::core::IInspectable);
impl AppCaptureDurationGeneratedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureDurationGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs;{c1f5563b-ffa1-44c9-975f-27fbeb553b35})");
}
unsafe impl ::windows::core::Interface for AppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1f5563b_ffa1_44c9_975f_27fbeb553b35);
}
impl ::windows::core::RuntimeName for AppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs";
}
impl ::core::convert::From<AppCaptureDurationGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureDurationGeneratedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureDurationGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureDurationGeneratedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureDurationGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureDurationGeneratedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureDurationGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureDurationGeneratedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCaptureDurationGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureDurationGeneratedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureFileGeneratedEventArgs(pub ::windows::core::IInspectable);
impl AppCaptureFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureFileGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureFileGeneratedEventArgs;{4189fbf4-465e-45bf-907f-165b3fb23758})");
}
unsafe impl ::windows::core::Interface for AppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4189fbf4_465e_45bf_907f_165b3fb23758);
}
impl ::windows::core::RuntimeName for AppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureFileGeneratedEventArgs";
}
impl ::core::convert::From<AppCaptureFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureFileGeneratedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureFileGeneratedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureFileGeneratedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureFileGeneratedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCaptureFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureFileGeneratedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: AppCaptureHistoricalBufferLengthUnit = AppCaptureHistoricalBufferLengthUnit(0i32);
    pub const Seconds: AppCaptureHistoricalBufferLengthUnit = AppCaptureHistoricalBufferLengthUnit(1i32);
}
impl ::core::convert::From<i32> for AppCaptureHistoricalBufferLengthUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCaptureHistoricalBufferLengthUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCaptureHistoricalBufferLengthUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureHistoricalBufferLengthUnit;i4)");
}
impl ::windows::core::DefaultType for AppCaptureHistoricalBufferLengthUnit {
    type DefaultType = Self;
}
pub struct AppCaptureManager {}
impl AppCaptureManager {
    pub fn GetCurrentSettings() -> ::windows::core::Result<AppCaptureSettings> {
        Self::IAppCaptureManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureSettings>(result__)
        })
    }
    pub fn ApplySettings<'a, Param0: ::windows::core::IntoParam<'a, AppCaptureSettings>>(appcapturesettings: Param0) -> ::windows::core::Result<()> {
        Self::IAppCaptureManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), appcapturesettings.into_param().abi()).ok() })
    }
    pub fn IAppCaptureManagerStatics<R, F: FnOnce(&IAppCaptureManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCaptureManager, IAppCaptureManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppCaptureManager {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureManager";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: AppCaptureMetadataPriority = AppCaptureMetadataPriority(0i32);
    pub const Important: AppCaptureMetadataPriority = AppCaptureMetadataPriority(1i32);
}
impl ::core::convert::From<i32> for AppCaptureMetadataPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCaptureMetadataPriority {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCaptureMetadataPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMetadataPriority;i4)");
}
impl ::windows::core::DefaultType for AppCaptureMetadataPriority {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureMetadataWriter(pub ::windows::core::IInspectable);
impl AppCaptureMetadataWriter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCaptureMetadataWriter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AddStringEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    pub fn AddInt32Event<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn AddDoubleEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn StartStringState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    pub fn StartInt32State<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn StartDoubleState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    pub fn StopState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    pub fn StopAllStates(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn RemainingStorageBytesAvailable(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MetadataPurged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMetadataPurged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureMetadataWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMetadataWriter;{e0ce4877-9aaf-46b4-ad31-6a60b441c780})");
}
unsafe impl ::windows::core::Interface for AppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0ce4877_9aaf_46b4_ad31_6a60b441c780);
}
impl ::windows::core::RuntimeName for AppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMetadataWriter";
}
impl ::core::convert::From<AppCaptureMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: AppCaptureMetadataWriter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureMetadataWriter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureMetadataWriter> for ::windows::core::IInspectable {
    fn from(value: AppCaptureMetadataWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureMetadataWriter> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureMetadataWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AppCaptureMetadataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AppCaptureMetadataWriter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AppCaptureMetadataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppCaptureMetadataWriter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppCaptureMetadataWriter {}
unsafe impl ::core::marker::Sync for AppCaptureMetadataWriter {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(0i32);
    pub const Started: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(1i32);
    pub const Failed: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(2i32);
}
impl ::core::convert::From<i32> for AppCaptureMicrophoneCaptureState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCaptureMicrophoneCaptureState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCaptureMicrophoneCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMicrophoneCaptureState;i4)");
}
impl ::windows::core::DefaultType for AppCaptureMicrophoneCaptureState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppCaptureMicrophoneCaptureStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs;{324d249e-45bc-4c35-bc35-e469fc7a69e0})");
}
unsafe impl ::windows::core::Interface for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x324d249e_45bc_4c35_bc35_e469fc7a69e0);
}
impl ::windows::core::RuntimeName for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureRecordOperation(pub ::windows::core::IInspectable);
impl AppCaptureRecordOperation {
    pub fn StopRecording(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureRecordingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsFileTruncated(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DurationGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDurationGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureRecordOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordOperation;{c66020a9-1538-495c-9bbb-2ba870ec5861})");
}
unsafe impl ::windows::core::Interface for AppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc66020a9_1538_495c_9bbb_2ba870ec5861);
}
impl ::windows::core::RuntimeName for AppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordOperation";
}
impl ::core::convert::From<AppCaptureRecordOperation> for ::windows::core::IUnknown {
    fn from(value: AppCaptureRecordOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureRecordOperation> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureRecordOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureRecordOperation> for ::windows::core::IInspectable {
    fn from(value: AppCaptureRecordOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureRecordOperation> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureRecordOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCaptureRecordOperation {}
unsafe impl ::core::marker::Sync for AppCaptureRecordOperation {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: AppCaptureRecordingState = AppCaptureRecordingState(0i32);
    pub const Completed: AppCaptureRecordingState = AppCaptureRecordingState(1i32);
    pub const Failed: AppCaptureRecordingState = AppCaptureRecordingState(2i32);
}
impl ::core::convert::From<i32> for AppCaptureRecordingState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCaptureRecordingState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCaptureRecordingState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureRecordingState;i4)");
}
impl ::windows::core::DefaultType for AppCaptureRecordingState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureRecordingStateChangedEventArgs(pub ::windows::core::IInspectable);
impl AppCaptureRecordingStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureRecordingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureRecordingStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs;{24fc8712-e305-490d-b415-6b1c9049736b})");
}
unsafe impl ::windows::core::Interface for AppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24fc8712_e305_490d_b415_6b1c9049736b);
}
impl ::windows::core::RuntimeName for AppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs";
}
impl ::core::convert::From<AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureRecordingStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureRecordingStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureRecordingStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureRecordingStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCaptureRecordingStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureRecordingStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureServices(pub ::windows::core::IInspectable);
impl AppCaptureServices {
    pub fn Record(&self) -> ::windows::core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RecordTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, starttime: Param0, duration: Param1) -> ::windows::core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    pub fn CanCapture(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<AppCaptureState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureServices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureServices;{44fec0b5-34f5-4f18-ae8c-b9123abbfc0d})");
}
unsafe impl ::windows::core::Interface for AppCaptureServices {
    type Vtable = IAppCaptureServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44fec0b5_34f5_4f18_ae8c_b9123abbfc0d);
}
impl ::windows::core::RuntimeName for AppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureServices";
}
impl ::core::convert::From<AppCaptureServices> for ::windows::core::IUnknown {
    fn from(value: AppCaptureServices) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureServices> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureServices) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureServices> for ::windows::core::IInspectable {
    fn from(value: AppCaptureServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureServices> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCaptureServices {}
unsafe impl ::core::marker::Sync for AppCaptureServices {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureSettings(pub ::windows::core::IInspectable);
impl AppCaptureSettings {
    #[cfg(feature = "Storage")]
    pub fn SetAppCaptureDestinationFolder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFolder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn AppCaptureDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetHistoricalBufferLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HistoricalBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HistoricalBufferLengthUnit(&self) -> ::windows::core::Result<AppCaptureHistoricalBufferLengthUnit> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureHistoricalBufferLengthUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureHistoricalBufferLengthUnit>(result__)
        }
    }
    pub fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnBatteryAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetMaximumRecordLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn MaximumRecordLength(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SetScreenshotDestinationFolder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFolder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn ScreenshotDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    pub fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureVideoEncodingBitrateMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingBitrateMode>(result__)
        }
    }
    pub fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureVideoEncodingResolutionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingResolutionMode>(result__)
        }
    }
    pub fn SetIsAppCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsAppCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsCpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsMemoryConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AlternateShortcutKeys(&self) -> ::windows::core::Result<AppCaptureAlternateShortcutKeys> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureAlternateShortcutKeys>(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SystemAudioGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MicrophoneGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn VideoEncodingFrameRateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingFrameRateMode> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: AppCaptureVideoEncodingFrameRateMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingFrameRateMode>(result__)
        }
    }
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureSettings;{14683a86-8807-48d3-883a-970ee4532a39})");
}
unsafe impl ::windows::core::Interface for AppCaptureSettings {
    type Vtable = IAppCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14683a86_8807_48d3_883a_970ee4532a39);
}
impl ::windows::core::RuntimeName for AppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureSettings";
}
impl ::core::convert::From<AppCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: AppCaptureSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: AppCaptureSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCaptureState(pub ::windows::core::IInspectable);
impl AppCaptureState {
    pub fn IsTargetRunning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    pub fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureState;{73134372-d4eb-44ce-9538-465f506ac4ea})");
}
unsafe impl ::windows::core::Interface for AppCaptureState {
    type Vtable = IAppCaptureState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73134372_d4eb_44ce_9538_465f506ac4ea);
}
impl ::windows::core::RuntimeName for AppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureState";
}
impl ::core::convert::From<AppCaptureState> for ::windows::core::IUnknown {
    fn from(value: AppCaptureState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCaptureState> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCaptureState> for ::windows::core::IInspectable {
    fn from(value: AppCaptureState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCaptureState> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCaptureState {}
unsafe impl ::core::marker::Sync for AppCaptureState {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(0i32);
    pub const High: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(1i32);
    pub const Standard: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(2i32);
}
impl ::core::convert::From<i32> for AppCaptureVideoEncodingBitrateMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCaptureVideoEncodingBitrateMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCaptureVideoEncodingBitrateMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingBitrateMode;i4)");
}
impl ::windows::core::DefaultType for AppCaptureVideoEncodingBitrateMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: AppCaptureVideoEncodingFrameRateMode = AppCaptureVideoEncodingFrameRateMode(0i32);
    pub const High: AppCaptureVideoEncodingFrameRateMode = AppCaptureVideoEncodingFrameRateMode(1i32);
}
impl ::core::convert::From<i32> for AppCaptureVideoEncodingFrameRateMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCaptureVideoEncodingFrameRateMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCaptureVideoEncodingFrameRateMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingFrameRateMode;i4)");
}
impl ::windows::core::DefaultType for AppCaptureVideoEncodingFrameRateMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(0i32);
    pub const High: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(1i32);
    pub const Standard: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(2i32);
}
impl ::core::convert::From<i32> for AppCaptureVideoEncodingResolutionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCaptureVideoEncodingResolutionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCaptureVideoEncodingResolutionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingResolutionMode;i4)");
}
impl ::windows::core::DefaultType for AppCaptureVideoEncodingResolutionMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraCaptureUI(pub ::windows::core::IInspectable);
impl CameraCaptureUI {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CameraCaptureUI, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PhotoSettings(&self) -> ::windows::core::Result<CameraCaptureUIPhotoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIPhotoCaptureSettings>(result__)
        }
    }
    pub fn VideoSettings(&self) -> ::windows::core::Result<CameraCaptureUIVideoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIVideoCaptureSettings>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUI;{48587540-6f93-4bb4-b8f3-e89e48948c91})");
}
unsafe impl ::windows::core::Interface for CameraCaptureUI {
    type Vtable = ICameraCaptureUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48587540_6f93_4bb4_b8f3_e89e48948c91);
}
impl ::windows::core::RuntimeName for CameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUI";
}
impl ::core::convert::From<CameraCaptureUI> for ::windows::core::IUnknown {
    fn from(value: CameraCaptureUI) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraCaptureUI> for ::windows::core::IUnknown {
    fn from(value: &CameraCaptureUI) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraCaptureUI> for ::windows::core::IInspectable {
    fn from(value: CameraCaptureUI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraCaptureUI> for ::windows::core::IInspectable {
    fn from(value: &CameraCaptureUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(0i32);
    pub const VerySmallQvga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(1i32);
    pub const SmallVga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(2i32);
    pub const MediumXga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(3i32);
    pub const Large3M: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(4i32);
    pub const VeryLarge5M: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(5i32);
}
impl ::core::convert::From<i32> for CameraCaptureUIMaxPhotoResolution {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CameraCaptureUIMaxPhotoResolution {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIMaxPhotoResolution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxPhotoResolution;i4)");
}
impl ::windows::core::DefaultType for CameraCaptureUIMaxPhotoResolution {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(0i32);
    pub const LowDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(1i32);
    pub const StandardDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(2i32);
    pub const HighDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(3i32);
}
impl ::core::convert::From<i32> for CameraCaptureUIMaxVideoResolution {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CameraCaptureUIMaxVideoResolution {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIMaxVideoResolution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxVideoResolution;i4)");
}
impl ::windows::core::DefaultType for CameraCaptureUIMaxVideoResolution {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: CameraCaptureUIMode = CameraCaptureUIMode(0i32);
    pub const Photo: CameraCaptureUIMode = CameraCaptureUIMode(1i32);
    pub const Video: CameraCaptureUIMode = CameraCaptureUIMode(2i32);
}
impl ::core::convert::From<i32> for CameraCaptureUIMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CameraCaptureUIMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMode;i4)");
}
impl ::windows::core::DefaultType for CameraCaptureUIMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraCaptureUIPhotoCaptureSettings(pub ::windows::core::IInspectable);
impl CameraCaptureUIPhotoCaptureSettings {
    pub fn Format(&self) -> ::windows::core::Result<CameraCaptureUIPhotoFormat> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIPhotoFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIPhotoFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxPhotoResolution> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIMaxPhotoResolution = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIMaxPhotoResolution>(result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CroppedSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetCroppedSizeInPixels<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CroppedAspectRatio(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetCroppedAspectRatio<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AllowCropping(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCropping(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIPhotoCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings;{b9f5be97-3472-46a8-8a9e-04ce42ccc97d})");
}
unsafe impl ::windows::core::Interface for CameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f5be97_3472_46a8_8a9e_04ce42ccc97d);
}
impl ::windows::core::RuntimeName for CameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings";
}
impl ::core::convert::From<CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: CameraCaptureUIPhotoCaptureSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &CameraCaptureUIPhotoCaptureSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: CameraCaptureUIPhotoCaptureSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &CameraCaptureUIPhotoCaptureSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CameraCaptureUIPhotoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIPhotoCaptureSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(0i32);
    pub const Png: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(1i32);
    pub const JpegXR: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(2i32);
}
impl ::core::convert::From<i32> for CameraCaptureUIPhotoFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CameraCaptureUIPhotoFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIPhotoFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIPhotoFormat;i4)");
}
impl ::windows::core::DefaultType for CameraCaptureUIPhotoFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraCaptureUIVideoCaptureSettings(pub ::windows::core::IInspectable);
impl CameraCaptureUIVideoCaptureSettings {
    pub fn Format(&self) -> ::windows::core::Result<CameraCaptureUIVideoFormat> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIVideoFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIVideoFormat>(result__)
        }
    }
    pub fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxVideoResolution> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIMaxVideoResolution = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIMaxVideoResolution>(result__)
        }
    }
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MaxDurationInSeconds(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetMaxDurationInSeconds(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowTrimming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowTrimming(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIVideoCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings;{64e92d1f-a28d-425a-b84f-e568335ff24e})");
}
unsafe impl ::windows::core::Interface for CameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e92d1f_a28d_425a_b84f_e568335ff24e);
}
impl ::windows::core::RuntimeName for CameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings";
}
impl ::core::convert::From<CameraCaptureUIVideoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: CameraCaptureUIVideoCaptureSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraCaptureUIVideoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &CameraCaptureUIVideoCaptureSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraCaptureUIVideoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: CameraCaptureUIVideoCaptureSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraCaptureUIVideoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &CameraCaptureUIVideoCaptureSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CameraCaptureUIVideoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIVideoCaptureSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: CameraCaptureUIVideoFormat = CameraCaptureUIVideoFormat(0i32);
    pub const Wmv: CameraCaptureUIVideoFormat = CameraCaptureUIVideoFormat(1i32);
}
impl ::core::convert::From<i32> for CameraCaptureUIVideoFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CameraCaptureUIVideoFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIVideoFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIVideoFormat;i4)");
}
impl ::windows::core::DefaultType for CameraCaptureUIVideoFormat {
    type DefaultType = Self;
}
pub struct CameraOptionsUI {}
impl CameraOptionsUI {
    pub fn Show<'a, Param0: ::windows::core::IntoParam<'a, MediaCapture>>(mediacapture: Param0) -> ::windows::core::Result<()> {
        Self::ICameraOptionsUIStatics(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediacapture.into_param().abi()).ok() })
    }
    pub fn ICameraOptionsUIStatics<R, F: FnOnce(&ICameraOptionsUIStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CameraOptionsUI, ICameraOptionsUIStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CameraOptionsUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraOptionsUI";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CapturedFrame(pub ::windows::core::IInspectable);
impl CapturedFrame {
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ControlValues(&self) -> ::windows::core::Result<CapturedFrameControlValues> {
        let this = &::windows::core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrameControlValues>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn BitmapProperties(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPropertySet> {
        let this = &::windows::core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CapturedFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrame;{1dd2de1f-571b-44d8-8e80-a08a1578766e})");
}
unsafe impl ::windows::core::Interface for CapturedFrame {
    type Vtable = ICapturedFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dd2de1f_571b_44d8_8e80_a08a1578766e);
}
impl ::windows::core::RuntimeName for CapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrame";
}
impl ::core::convert::From<CapturedFrame> for ::windows::core::IUnknown {
    fn from(value: CapturedFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CapturedFrame> for ::windows::core::IUnknown {
    fn from(value: &CapturedFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CapturedFrame> for ::windows::core::IInspectable {
    fn from(value: CapturedFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CapturedFrame> for ::windows::core::IInspectable {
    fn from(value: &CapturedFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IContentTypeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CapturedFrame {}
unsafe impl ::core::marker::Sync for CapturedFrame {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CapturedFrameControlValues(pub ::windows::core::IInspectable);
impl CapturedFrameControlValues {
    #[cfg(feature = "Foundation")]
    pub fn Exposure(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExposureCompensation(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsoSpeed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Focus(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn SceneMode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Flashed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlashPowerPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WhiteBalance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ZoomFactor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn FocusState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsoDigitalGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsoAnalogGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SensorFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn WhiteBalanceGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<WhiteBalanceGain>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<WhiteBalanceGain>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CapturedFrameControlValues {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrameControlValues;{90c65b7f-4e0d-4ca4-882d-7a144fed0a90})");
}
unsafe impl ::windows::core::Interface for CapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90c65b7f_4e0d_4ca4_882d_7a144fed0a90);
}
impl ::windows::core::RuntimeName for CapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrameControlValues";
}
impl ::core::convert::From<CapturedFrameControlValues> for ::windows::core::IUnknown {
    fn from(value: CapturedFrameControlValues) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CapturedFrameControlValues> for ::windows::core::IUnknown {
    fn from(value: &CapturedFrameControlValues) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CapturedFrameControlValues> for ::windows::core::IInspectable {
    fn from(value: CapturedFrameControlValues) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CapturedFrameControlValues> for ::windows::core::IInspectable {
    fn from(value: &CapturedFrameControlValues) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CapturedFrameControlValues {}
unsafe impl ::core::marker::Sync for CapturedFrameControlValues {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CapturedPhoto(pub ::windows::core::IInspectable);
impl CapturedPhoto {
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CapturedPhoto {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedPhoto;{b0ce7e5a-cfcc-4d6c-8ad1-0869208aca16})");
}
unsafe impl ::windows::core::Interface for CapturedPhoto {
    type Vtable = ICapturedPhoto_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ce7e5a_cfcc_4d6c_8ad1_0869208aca16);
}
impl ::windows::core::RuntimeName for CapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.CapturedPhoto";
}
impl ::core::convert::From<CapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: CapturedPhoto) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: &CapturedPhoto) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: CapturedPhoto) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: &CapturedPhoto) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CapturedPhoto {}
unsafe impl ::core::marker::Sync for CapturedPhoto {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: ForegroundActivationArgument = ForegroundActivationArgument(0i32);
    pub const MoreSettings: ForegroundActivationArgument = ForegroundActivationArgument(1i32);
}
impl ::core::convert::From<i32> for ForegroundActivationArgument {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ForegroundActivationArgument {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ForegroundActivationArgument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.ForegroundActivationArgument;i4)");
}
impl ::windows::core::DefaultType for ForegroundActivationArgument {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: GameBarCommand = GameBarCommand(0i32);
    pub const RecordHistoricalBuffer: GameBarCommand = GameBarCommand(1i32);
    pub const ToggleStartStopRecord: GameBarCommand = GameBarCommand(2i32);
    pub const StartRecord: GameBarCommand = GameBarCommand(3i32);
    pub const StopRecord: GameBarCommand = GameBarCommand(4i32);
    pub const TakeScreenshot: GameBarCommand = GameBarCommand(5i32);
    pub const StartBroadcast: GameBarCommand = GameBarCommand(6i32);
    pub const StopBroadcast: GameBarCommand = GameBarCommand(7i32);
    pub const PauseBroadcast: GameBarCommand = GameBarCommand(8i32);
    pub const ResumeBroadcast: GameBarCommand = GameBarCommand(9i32);
    pub const ToggleStartStopBroadcast: GameBarCommand = GameBarCommand(10i32);
    pub const ToggleMicrophoneCapture: GameBarCommand = GameBarCommand(11i32);
    pub const ToggleCameraCapture: GameBarCommand = GameBarCommand(12i32);
    pub const ToggleRecordingIndicator: GameBarCommand = GameBarCommand(13i32);
}
impl ::core::convert::From<i32> for GameBarCommand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameBarCommand {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameBarCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommand;i4)");
}
impl ::windows::core::DefaultType for GameBarCommand {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: GameBarCommandOrigin = GameBarCommandOrigin(0i32);
    pub const Cortana: GameBarCommandOrigin = GameBarCommandOrigin(1i32);
    pub const AppCommand: GameBarCommandOrigin = GameBarCommandOrigin(2i32);
}
impl ::core::convert::From<i32> for GameBarCommandOrigin {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameBarCommandOrigin {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameBarCommandOrigin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommandOrigin;i4)");
}
impl ::windows::core::DefaultType for GameBarCommandOrigin {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameBarServices(pub ::windows::core::IInspectable);
impl GameBarServices {
    pub fn TargetCapturePolicy(&self) -> ::windows::core::Result<GameBarTargetCapturePolicy> {
        let this = self;
        unsafe {
            let mut result__: GameBarTargetCapturePolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarTargetCapturePolicy>(result__)
        }
    }
    pub fn EnableCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn DisableCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn TargetInfo(&self) -> ::windows::core::Result<GameBarServicesTargetInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesTargetInfo>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AppBroadcastServices(&self) -> ::windows::core::Result<AppBroadcastServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastServices>(result__)
        }
    }
    pub fn AppCaptureServices(&self) -> ::windows::core::Result<AppCaptureServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureServices>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CommandReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCommandReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarServices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServices;{2dbead57-50a6-499e-8c6c-d330a7311796})");
}
unsafe impl ::windows::core::Interface for GameBarServices {
    type Vtable = IGameBarServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dbead57_50a6_499e_8c6c_d330a7311796);
}
impl ::windows::core::RuntimeName for GameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServices";
}
impl ::core::convert::From<GameBarServices> for ::windows::core::IUnknown {
    fn from(value: GameBarServices) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameBarServices> for ::windows::core::IUnknown {
    fn from(value: &GameBarServices) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameBarServices> for ::windows::core::IInspectable {
    fn from(value: GameBarServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameBarServices> for ::windows::core::IInspectable {
    fn from(value: &GameBarServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameBarServices {}
unsafe impl ::core::marker::Sync for GameBarServices {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameBarServicesCommandEventArgs(pub ::windows::core::IInspectable);
impl GameBarServicesCommandEventArgs {
    pub fn Command(&self) -> ::windows::core::Result<GameBarCommand> {
        let this = self;
        unsafe {
            let mut result__: GameBarCommand = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarCommand>(result__)
        }
    }
    pub fn Origin(&self) -> ::windows::core::Result<GameBarCommandOrigin> {
        let this = self;
        unsafe {
            let mut result__: GameBarCommandOrigin = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarCommandOrigin>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarServicesCommandEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesCommandEventArgs;{a74226b2-f176-4fcf-8fbb-cf698b2eb8e0})");
}
unsafe impl ::windows::core::Interface for GameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74226b2_f176_4fcf_8fbb_cf698b2eb8e0);
}
impl ::windows::core::RuntimeName for GameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesCommandEventArgs";
}
impl ::core::convert::From<GameBarServicesCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesCommandEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameBarServicesCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesCommandEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameBarServicesCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesCommandEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameBarServicesCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesCommandEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameBarServicesCommandEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesCommandEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: GameBarServicesDisplayMode = GameBarServicesDisplayMode(0i32);
    pub const FullScreenExclusive: GameBarServicesDisplayMode = GameBarServicesDisplayMode(1i32);
}
impl ::core::convert::From<i32> for GameBarServicesDisplayMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameBarServicesDisplayMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameBarServicesDisplayMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarServicesDisplayMode;i4)");
}
impl ::windows::core::DefaultType for GameBarServicesDisplayMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameBarServicesManager(pub ::windows::core::IInspectable);
impl GameBarServicesManager {
    #[cfg(feature = "Foundation")]
    pub fn GameBarServicesCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameBarServicesCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<GameBarServicesManager> {
        Self::IGameBarServicesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesManager>(result__)
        })
    }
    pub fn IGameBarServicesManagerStatics<R, F: FnOnce(&IGameBarServicesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameBarServicesManager, IGameBarServicesManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarServicesManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManager;{3a4b9cfa-7f8b-4c60-9dbb-0bcd262dffc6})");
}
unsafe impl ::windows::core::Interface for GameBarServicesManager {
    type Vtable = IGameBarServicesManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a4b9cfa_7f8b_4c60_9dbb_0bcd262dffc6);
}
impl ::windows::core::RuntimeName for GameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManager";
}
impl ::core::convert::From<GameBarServicesManager> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameBarServicesManager> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameBarServicesManager> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameBarServicesManager> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameBarServicesManager {}
unsafe impl ::core::marker::Sync for GameBarServicesManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(pub ::windows::core::IInspectable);
impl GameBarServicesManagerGameBarServicesCreatedEventArgs {
    pub fn GameBarServices(&self) -> ::windows::core::Result<GameBarServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServices>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs;{ededbd9c-143e-49a3-a5ea-0b1995c8d46e})");
}
unsafe impl ::windows::core::Interface for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xededbd9c_143e_49a3_a5ea_0b1995c8d46e);
}
impl ::windows::core::RuntimeName for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs";
}
impl ::core::convert::From<GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GameBarServicesTargetInfo(pub ::windows::core::IInspectable);
impl GameBarServicesTargetInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayMode(&self) -> ::windows::core::Result<GameBarServicesDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: GameBarServicesDisplayMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesDisplayMode>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarServicesTargetInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesTargetInfo;{b4202f92-1611-4e05-b6ef-dfd737ae33b0})");
}
unsafe impl ::windows::core::Interface for GameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4202f92_1611_4e05_b6ef_dfd737ae33b0);
}
impl ::windows::core::RuntimeName for GameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesTargetInfo";
}
impl ::core::convert::From<GameBarServicesTargetInfo> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesTargetInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GameBarServicesTargetInfo> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesTargetInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GameBarServicesTargetInfo> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesTargetInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GameBarServicesTargetInfo> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesTargetInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GameBarServicesTargetInfo {}
unsafe impl ::core::marker::Sync for GameBarServicesTargetInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(0i32);
    pub const EnabledByUser: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(1i32);
    pub const NotEnabled: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(2i32);
    pub const ProhibitedBySystem: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(3i32);
    pub const ProhibitedByPublisher: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(4i32);
}
impl ::core::convert::From<i32> for GameBarTargetCapturePolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GameBarTargetCapturePolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GameBarTargetCapturePolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarTargetCapturePolicy;i4)");
}
impl ::windows::core::DefaultType for GameBarTargetCapturePolicy {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf072728b_b292_4491_9d41_99807a550bbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedCapturedPhoto2 {
    type Vtable = IAdvancedCapturedPhoto2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18cf6cd8_cffe_42d8_8104_017bb318f4a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdvancedPhotoCapture(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83ffaafa_6667_44dc_973c_a6bce596aa0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad1e72a_fa94_46f9_95fc_d71511cda70b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppBroadcastPlugInState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundService2 {
    type Vtable = IAppBroadcastBackgroundService2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc8ccbbf_5549_4b87_959f_23ca401fd473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e735275_88c8_4eca_89ba_4825985db880);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceSignInInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9104285c_62cf_4a3c_a7ee_aeb507404645);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31dc02bc_990a_4904_aa96_fe364381f136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceStreamInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1e9f6d_94dc_4fce_9541_a9f129596334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e334cd0_b882_4b88_8692_05999aceb70f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastGlobalSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2cb27a5_70fc_4e17_80bd_6ba0fd3ff3a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastGlobalSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcea54283_ee51_4dbf_9472_79a9ed4e2165);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastManagerStatics {
    type Vtable = IAppBroadcastManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364e018b_1e4e_411f_ab3e_92959844c156);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa86ad5e9_9440_4908_9d09_65b7e315d795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugIn(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520c1e66_6513_4574_ac54_23b79729615b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugIn_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe550d979_27a1_49a7_bbf4_d7a9e9d07668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugInManagerStatics {
    type Vtable = IAppBroadcastPlugInManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2645c20_5c76_4cdc_9364_82fe9eb6534d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4881d0f2_abc5_4fc6_84b0_89370bb47212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14b60f5a_6e4a_4b80_a14f_67ee77d153e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a57f2de_8dea_4e86_90ad_03fc26b9653c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92228d50_db3f_40a8_8cd4_f4e371ddab37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x010fbea1_94fe_4499_b8c0_8d244279fb12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoHeader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bef6113_da84_4499_a7ab_87118cb4a157);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastProviderSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc30bdf62_9948_458f_ad50_aa06ec03da08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastProviderSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastServices(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastServices {
    type Vtable = IAppBroadcastServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8660b4d6_969b_4e3c_ac3a_8b042ee4ee63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastServices_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plugin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, desiredsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastSignInStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b692a4_5919_4a9e_8d5e_c9bb0dd3377a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastSignInStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastSignInResult) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastState {
    type Vtable = IAppBroadcastState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee08056d_8099_4ddd_922e_c56dac58abfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppBroadcastSignInState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastTerminationReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab4ac8_21ba_453f_8bb7_5e938a2e9a74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioHeader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf21a570_6b78_4216_9f07_5aff5256f1b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb338bcf9_3364_4460_b5f1_3cc2796a8aa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5108a733_d008_4a89_93be_58aed961374e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f97cf2b_c9e4_4e88_8194_d814cbd585d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoHeader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b9ebece_7e32_432d_8ca2_36bf10b9f462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeebab35_ec5e_4d8f_b1c0_5da6e8c75638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastViewerCountChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6e11825_5401_4ade_8bd2_c14ecee6807d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastViewerCountChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCapture(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCapture {
    type Vtable = IAppCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9749d453_a29a_45ed_8f29_22d09942cff7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19e8e0ef_236c_40f9_b38f_9b7dd65d1ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureAlternateShortcutKeys2 {
    type Vtable = IAppCaptureAlternateShortcutKeys2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3669090_dd17_47f0_95e5_ce42286cf338);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureAlternateShortcutKeys3 {
    type Vtable = IAppCaptureAlternateShortcutKeys3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b81448c_418e_469c_a49a_45b597c826b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureDurationGeneratedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1f5563b_ffa1_44c9_975f_27fbeb553b35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureDurationGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureFileGeneratedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4189fbf4_465e_45bf_907f_165b3fb23758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureFileGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureManagerStatics {
    type Vtable = IAppCaptureManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9e3ea7_6282_4735_8d4e_aa45f90f6723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appcapturesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureMetadataWriter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0ce4877_9aaf_46b4_ad31_6a60b441c780);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMetadataWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x324d249e_45bc_4c35_bc35_e469fc7a69e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureRecordOperation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc66020a9_1538_495c_9bbb_2ba870ec5861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureRecordingStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24fc8712_e305_490d_b415_6b1c9049736b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordingStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureServices(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureServices {
    type Vtable = IAppCaptureServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44fec0b5_34f5_4f18_ae8c_b9123abbfc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureServices_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureSettings {
    type Vtable = IAppCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14683a86_8807_48d3_883a_970ee4532a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureSettings2 {
    type Vtable = IAppCaptureSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcb8cee7_e26b_476f_9b1a_ec342d2a8fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureSettings3 {
    type Vtable = IAppCaptureSettings3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa93502fe_88c2_42d6_aaaa_40feffd75aec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureSettings4 {
    type Vtable = IAppCaptureSettings4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07c2774c_1a81_482f_a244_049d95f25b0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureSettings5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureSettings5 {
    type Vtable = IAppCaptureSettings5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18894522_b0e8_4ba0_8f13_3eaa5fa4013b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureState {
    type Vtable = IAppCaptureState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73134372_d4eb_44ce_9538_465f506ac4ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureStatics {
    type Vtable = IAppCaptureStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf922dd6c_0a7e_4e74_8b20_9c1f902d08a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCaptureStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCaptureStatics2 {
    type Vtable = IAppCaptureStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2d881d4_836c_4da4_afd7_facc041e1cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, allowed: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraCaptureUI(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraCaptureUI {
    type Vtable = ICameraCaptureUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48587540_6f93_4bb4_b8f3_e89e48948c91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: CameraCaptureUIMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraCaptureUIPhotoCaptureSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f5be97_3472_46a8_8a9e_04ce42ccc97d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIPhotoCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraCaptureUIVideoCaptureSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e92d1f_a28d_425a_b84f_e568335ff24e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIVideoCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICameraOptionsUIStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraOptionsUIStatics {
    type Vtable = ICameraOptionsUIStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0d5e34_3906_4b7d_946c_7bde844499ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOptionsUIStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediacapture: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICapturedFrame {
    type Vtable = ICapturedFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dd2de1f_571b_44d8_8e80_a08a1578766e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrame2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICapturedFrame2 {
    type Vtable = ICapturedFrame2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543fa6d1_bd78_4866_adda_24314bc65dea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90c65b7f_4e0d_4ca4_882d_7a144fed0a90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICapturedFrameControlValues2 {
    type Vtable = ICapturedFrameControlValues2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x500b2b88_06d2_4aa7_a7db_d37af73321d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedFrameWithSoftwareBitmap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICapturedFrameWithSoftwareBitmap {
    type Vtable = ICapturedFrameWithSoftwareBitmap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb58e8b6e_8503_49b5_9e86_897d26a3ff3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameWithSoftwareBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICapturedPhoto(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICapturedPhoto {
    type Vtable = ICapturedPhoto_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ce7e5a_cfcc_4d6c_8ad1_0869208aca16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedPhoto_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServices(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameBarServices {
    type Vtable = IGameBarServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dbead57_50a6_499e_8c6c_d330a7311796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServices_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameBarTargetCapturePolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesCommandEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74226b2_f176_4fcf_8fbb_cf698b2eb8e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesCommandEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameBarCommand) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameBarCommandOrigin) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameBarServicesManager {
    type Vtable = IGameBarServicesManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a4b9cfa_7f8b_4c60_9dbb_0bcd262dffc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xededbd9c_143e_49a3_a5ea_0b1995c8d46e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameBarServicesManagerStatics {
    type Vtable = IGameBarServicesManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34c1b616_ff25_4792_98f2_d3753f15ac13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameBarServicesTargetInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4202f92_1611_4e05_b6ef_dfd737ae33b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesTargetInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GameBarServicesDisplayMode) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagMediaRecording(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41c8baf7_ff3f_49f0_a477_f195e3ce5108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagMediaRecording2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLagMediaRecording2 {
    type Vtable = ILowLagMediaRecording2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6369c758_5644_41e2_97af_8ef56a25e225);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagMediaRecording3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLagMediaRecording3 {
    type Vtable = ILowLagMediaRecording3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c33ab12_48f7_47da_b41e_90880a5fe0ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagPhotoCapture(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa37251b7_6b44_473d_8f24_f703d6c0ec44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceCapture(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cc346bb_b9a9_4c91_8ffa_287e9c668669);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapture {
    type Vtable = IMediaCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61afbb4_fb10_4a34_ac18_ca80d9c8e7ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediacaptureinitializationsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, effectactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, propertyvalue: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, erroreventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, recordlimitationexceededeventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VideoRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VideoRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VideoRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VideoRotation) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapture2 {
    type Vtable = IMediaCapture2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cc68260_7da1_4043_b652_21b8878daff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, encoderproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapture3 {
    type Vtable = IMediaCapture3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4136f30_1564_466e_bc0a_af94e02ab016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapture4 {
    type Vtable = IMediaCapture4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbacd6fd6_fb08_4947_aea2_ce14eff0ce13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, definition: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, definition: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Devices::CameraStreamState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCaptureThermalStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapture5 {
    type Vtable = IMediaCapture5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda787c22_3a9b_4720_a71e_97900a316e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outputsize: super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapture6 {
    type Vtable = IMediaCapture6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x228948bd_4b20_4bb1_9fd6_a583212a1012);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputsources: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapture7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapture7 {
    type Vtable = IMediaCapture7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9169f102_8888_541a_95bc_24e4d462542a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture7_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capturemode: StreamingCaptureMode, displayregion: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d2f920d_a588_43c6_89d6_5ad322af006a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureFailedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80fde3f4_54c4_42c0_8d19_cea1a87ca18b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureFocusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e1bc7f_2277_493e_abee_d3f44ff98c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFocusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Devices")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9782ba70_ea65_4900_9356_8ca887726884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: StreamingCaptureMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhotoCaptureSource) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings2 {
    type Vtable = IMediaCaptureInitializationSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x404e0626_c9dc_43e9_aee4_e6bf1b57b44c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MediaCategory) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCategory) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::AudioProcessing) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings3 {
    type Vtable = IMediaCaptureInitializationSettings3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4160519d_be48_4730_8104_0cf6e9e97948);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings4 {
    type Vtable = IMediaCaptureInitializationSettings4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf502a537_4cb7_4d28_95ed_4f9f012e0518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings5 {
    type Vtable = IMediaCaptureInitializationSettings5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5a2e3b8_2626_4e94_b7b3_5308a0f64b1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture_Frames")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))] usize,
    #[cfg(feature = "Media_Capture_Frames")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCaptureSharingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MediaCaptureSharingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCaptureMemoryPreference) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MediaCaptureMemoryPreference) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings6 {
    type Vtable = IMediaCaptureInitializationSettings6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2e26b47_3db1_4d33_ab63_0ffa09056585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings7(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings7 {
    type Vtable = IMediaCaptureInitializationSettings7_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41546967_f58a_5d82_9ef4_ed572fb5e34e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings7_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCapturePauseResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec47ca3_4477_4b04_a06f_2c1c5182fe9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapturePauseResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureRelativePanelWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d896566_04be_5b89_b30e_bd34a9f12db0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureRelativePanelWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d83aafe_6d45_4477_8dc4_ac5bc01c4091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VideoDeviceCharacteristic) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureSettings2 {
    type Vtable = IMediaCaptureSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f9e7cfb_fa9f_4b13_9cbe_5ab94f1f3493);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaCategory) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureSettings3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureSettings3 {
    type Vtable = IMediaCaptureSettings3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x303c67c2_8058_4b1b_b877_8c2ef3528440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureStatics {
    type Vtable = IMediaCaptureStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacef81ff_99ed_4645_965e_1925cfc63834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: KnownVideoProfile, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureStopResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9db6a2a_a092_4ad1_97d4_f201f9d082db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStopResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoPreview {
    type Vtable = IMediaCaptureVideoPreview_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27727073_549e_447f_a20a_4f03c479d8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21a073bf_a3ee_4ecf_9ef6_50b0bc4e1305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfile2 {
    type Vtable = IMediaCaptureVideoProfile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97ddc95f_94ce_468f_9316_fc5bc2638f6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8012afef_b691_49ff_83f2_c1e76eaaea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfileMediaDescription2 {
    type Vtable = IMediaCaptureVideoProfileMediaDescription2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a6ef13_322d_413a_b85a_68a88e02f4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOptionalReferencePhotoCapturedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x470f88b3_1e6d_4051_9c8b_f1d85af047b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOptionalReferencePhotoCapturedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoCapturedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x373bfbc1_984e_4ff0_bf85_1c00aabc5a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoCapturedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoConfirmationCapturedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab473672_c28a_4827_8f8d_3636d3beb51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationCapturedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScreenCapture(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScreenCapture {
    type Vtable = IScreenCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89179ef7_cd12_4e0e_a6d4_5b3de98b2e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScreenCaptureStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScreenCaptureStatics {
    type Vtable = IScreenCaptureStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc898c3b0_c8a5_11e2_8b8b_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCaptureStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISourceSuspensionChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ece7b5e_d49b_4394_bc32_f97d6cedec1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceSuspensionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVideoStreamConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8770a6f_4390_4b5e_ad3e_0f8af0963490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct KnownVideoProfile(pub i32);
impl KnownVideoProfile {
    pub const VideoRecording: KnownVideoProfile = KnownVideoProfile(0i32);
    pub const HighQualityPhoto: KnownVideoProfile = KnownVideoProfile(1i32);
    pub const BalancedVideoAndPhoto: KnownVideoProfile = KnownVideoProfile(2i32);
    pub const VideoConferencing: KnownVideoProfile = KnownVideoProfile(3i32);
    pub const PhotoSequence: KnownVideoProfile = KnownVideoProfile(4i32);
    pub const HighFrameRate: KnownVideoProfile = KnownVideoProfile(5i32);
    pub const VariablePhotoSequence: KnownVideoProfile = KnownVideoProfile(6i32);
    pub const HdrWithWcgVideo: KnownVideoProfile = KnownVideoProfile(7i32);
    pub const HdrWithWcgPhoto: KnownVideoProfile = KnownVideoProfile(8i32);
    pub const VideoHdr8: KnownVideoProfile = KnownVideoProfile(9i32);
    pub const CompressedCamera: KnownVideoProfile = KnownVideoProfile(10i32);
}
impl ::core::convert::From<i32> for KnownVideoProfile {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for KnownVideoProfile {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for KnownVideoProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.KnownVideoProfile;i4)");
}
impl ::windows::core::DefaultType for KnownVideoProfile {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LowLagMediaRecording(pub ::windows::core::IInspectable);
impl LowLagMediaRecording {
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LowLagMediaRecording {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagMediaRecording;{41c8baf7-ff3f-49f0-a477-f195e3ce5108})");
}
unsafe impl ::windows::core::Interface for LowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41c8baf7_ff3f_49f0_a477_f195e3ce5108);
}
impl ::windows::core::RuntimeName for LowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.LowLagMediaRecording";
}
impl ::core::convert::From<LowLagMediaRecording> for ::windows::core::IUnknown {
    fn from(value: LowLagMediaRecording) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LowLagMediaRecording> for ::windows::core::IUnknown {
    fn from(value: &LowLagMediaRecording) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LowLagMediaRecording> for ::windows::core::IInspectable {
    fn from(value: LowLagMediaRecording) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LowLagMediaRecording> for ::windows::core::IInspectable {
    fn from(value: &LowLagMediaRecording) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LowLagPhotoCapture(pub ::windows::core::IInspectable);
impl LowLagPhotoCapture {
    #[cfg(feature = "Foundation")]
    pub fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CapturedPhoto>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LowLagPhotoCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoCapture;{a37251b7-6b44-473d-8f24-f703d6c0ec44})");
}
unsafe impl ::windows::core::Interface for LowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa37251b7_6b44_473d_8f24_f703d6c0ec44);
}
impl ::windows::core::RuntimeName for LowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoCapture";
}
impl ::core::convert::From<LowLagPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoCapture) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LowLagPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoCapture) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LowLagPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LowLagPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LowLagPhotoSequenceCapture(pub ::windows::core::IInspectable);
impl LowLagPhotoSequenceCapture {
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LowLagPhotoSequenceCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoSequenceCapture;{7cc346bb-b9a9-4c91-8ffa-287e9c668669})");
}
unsafe impl ::windows::core::Interface for LowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cc346bb_b9a9_4c91_8ffa_287e9c668669);
}
impl ::windows::core::RuntimeName for LowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoSequenceCapture";
}
impl ::core::convert::From<LowLagPhotoSequenceCapture> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoSequenceCapture) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceCapture> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoSequenceCapture) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LowLagPhotoSequenceCapture> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoSequenceCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceCapture> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoSequenceCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCapture(pub ::windows::core::IInspectable);
impl MediaCapture {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaCapture, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn InitializeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InitializeWithSettingsAsync<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureInitializationSettings>>(&self, mediacaptureinitializationsettings: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mediacaptureinitializationsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn StartRecordToStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn StartRecordToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CapturePhotoToStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, r#type: Param0, file: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), r#type.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn CapturePhotoToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, r#type: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), r#type.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AddEffectAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, mediastreamtype: MediaStreamType, effectactivationid: Param1, effectsettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), mediastreamtype, effectactivationid.into_param().abi(), effectsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetEncoderProperty<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1, propertyvalue: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), mediastreamtype, propertyid.into_param().abi(), propertyvalue.into_param().abi()).ok() }
    }
    pub fn GetEncoderProperty<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), mediastreamtype, propertyid.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Failed<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureFailedEventHandler>>(&self, erroreventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), erroreventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RecordLimitationExceeded<'a, Param0: ::windows::core::IntoParam<'a, RecordLimitationExceededEventHandler>>(&self, recordlimitationexceededeventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), recordlimitationexceededeventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecordLimitationExceeded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn MediaCaptureSettings(&self) -> ::windows::core::Result<MediaCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureSettings>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> ::windows::core::Result<super::Devices::AudioDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::AudioDeviceController>(result__)
        }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::VideoDeviceController>(result__)
        }
    }
    pub fn SetPreviewMirroring(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetPreviewMirroring(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPreviewRotation(&self, value: VideoRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetPreviewRotation(&self) -> ::windows::core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__: VideoRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoRotation>(result__)
        }
    }
    pub fn SetRecordRotation(&self, value: VideoRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetRecordRotation(&self) -> ::windows::core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__: VideoRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoRotation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareLowLagRecordToStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareLowLagRecordToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagPhotoCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagPhotoSequenceCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SetEncodingPropertiesAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>, Param2: ::windows::core::IntoParam<'a, super::MediaProperties::MediaPropertySet>>(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: Param1, encoderproperties: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), mediastreamtype, mediaencodingproperties.into_param().abi(), encoderproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub fn PrepareVariablePhotoSequenceCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FocusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFocusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PhotoConfirmationCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoConfirmationCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub fn AddAudioEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), definition.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub fn AddVideoEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition>>(&self, definition: Param0, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), definition.into_param().abi(), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ResumeRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CameraStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Devices")]
    pub fn CameraStreamState(&self) -> ::windows::core::Result<super::Devices::CameraStreamState> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::Devices::CameraStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::CameraStreamState>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPreviewFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPreviewFrameCopyAsync<'a, Param0: ::windows::core::IntoParam<'a, super::VideoFrame>>(&self, destination: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ThermalStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveThermalStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn ThermalStatus(&self) -> ::windows::core::Result<MediaCaptureThermalStatus> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: MediaCaptureThermalStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureThermalStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareAdvancedPhotoCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn IsVideoProfileSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0) -> ::windows::core::Result<bool> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllVideoProfiles<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConcurrentProfiles<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindKnownVideoProfiles<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0, name: KnownVideoProfile) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), name, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, effect: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StopRecordWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, Frames::MediaFrameSource>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, Frames::MediaFrameSource>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderAsync<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSource>>(&self, inputsource: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), inputsource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAsync<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, inputsource: Param0, outputsubtype: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAndSizeAsync<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, inputsource: Param0, outputsubtype: Param1, outputsize: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), outputsize.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn CreateMultiSourceFrameReaderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>>(&self, inputsources: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), inputsources.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>(result__)
        }
    }
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateRelativePanelWatcher<'a, Param1: ::windows::core::IntoParam<'a, super::super::UI::WindowManagement::DisplayRegion>>(&self, capturemode: StreamingCaptureMode, displayregion: Param1) -> ::windows::core::Result<MediaCaptureRelativePanelWatcher> {
        let this = &::windows::core::Interface::cast::<IMediaCapture7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capturemode, displayregion.into_param().abi(), &mut result__).from_abi::<MediaCaptureRelativePanelWatcher>(result__)
        }
    }
    pub fn IMediaCaptureStatics<R, F: FnOnce(&IMediaCaptureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaCapture, IMediaCaptureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapture;{c61afbb4-fb10-4a34-ac18-ca80d9c8e7ee})");
}
unsafe impl ::windows::core::Interface for MediaCapture {
    type Vtable = IMediaCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61afbb4_fb10_4a34_ac18_ca80d9c8e7ee);
}
impl ::windows::core::RuntimeName for MediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapture";
}
impl ::core::convert::From<MediaCapture> for ::windows::core::IUnknown {
    fn from(value: MediaCapture) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCapture> for ::windows::core::IUnknown {
    fn from(value: &MediaCapture) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCapture> for ::windows::core::IInspectable {
    fn from(value: MediaCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCapture> for ::windows::core::IInspectable {
    fn from(value: &MediaCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCapture> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCapture) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCapture> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCapture) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: MediaCaptureDeviceExclusiveControlStatus = MediaCaptureDeviceExclusiveControlStatus(0i32);
    pub const SharedReadOnlyAvailable: MediaCaptureDeviceExclusiveControlStatus = MediaCaptureDeviceExclusiveControlStatus(1i32);
}
impl ::core::convert::From<i32> for MediaCaptureDeviceExclusiveControlStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureDeviceExclusiveControlStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureDeviceExclusiveControlStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatus;i4)");
}
impl ::windows::core::DefaultType for MediaCaptureDeviceExclusiveControlStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(pub ::windows::core::IInspectable);
impl MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<MediaCaptureDeviceExclusiveControlStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaCaptureDeviceExclusiveControlStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureDeviceExclusiveControlStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs;{9d2f920d-a588-43c6-89d6-5ad322af006a})");
}
unsafe impl ::windows::core::Interface for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d2f920d_a588_43c6_89d6_5ad322af006a);
}
impl ::windows::core::RuntimeName for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
impl ::core::convert::From<MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureFailedEventArgs(pub ::windows::core::IInspectable);
impl MediaCaptureFailedEventArgs {
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Code(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFailedEventArgs;{80fde3f4-54c4-42c0-8d19-cea1a87ca18b})");
}
unsafe impl ::windows::core::Interface for MediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80fde3f4_54c4_42c0_8d19_cea1a87ca18b);
}
impl ::windows::core::RuntimeName for MediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFailedEventArgs";
}
impl ::core::convert::From<MediaCaptureFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureFailedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureFailedEventHandler(::windows::core::IUnknown);
impl MediaCaptureFailedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = MediaCaptureFailedEventHandler_box::<F> {
            vtable: &MediaCaptureFailedEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, MediaCapture>, Param1: ::windows::core::IntoParam<'a, MediaCaptureFailedEventArgs>>(&self, sender: Param0, erroreventargs: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), erroreventargs.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureFailedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({2014effb-5cd8-4f08-a314-0d360da59f14})");
}
unsafe impl ::windows::core::Interface for MediaCaptureFailedEventHandler {
    type Vtable = MediaCaptureFailedEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2014effb_5cd8_4f08_a314_0d360da59f14);
}
#[repr(C)]
#[doc(hidden)]
pub struct MediaCaptureFailedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, erroreventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct MediaCaptureFailedEventHandler_box<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const MediaCaptureFailedEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::core::Result<()> + 'static> MediaCaptureFailedEventHandler_box<F> {
    const VTABLE: MediaCaptureFailedEventHandler_abi = MediaCaptureFailedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MediaCaptureFailedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, erroreventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <MediaCapture as ::windows::core::Abi>::Abi as *const <MediaCapture as ::windows::core::DefaultType>::DefaultType),
            &*(&erroreventargs as *const <MediaCaptureFailedEventArgs as ::windows::core::Abi>::Abi as *const <MediaCaptureFailedEventArgs as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureFocusChangedEventArgs(pub ::windows::core::IInspectable);
impl MediaCaptureFocusChangedEventArgs {
    #[cfg(feature = "Media_Devices")]
    pub fn FocusState(&self) -> ::windows::core::Result<super::Devices::MediaCaptureFocusState> {
        let this = self;
        unsafe {
            let mut result__: super::Devices::MediaCaptureFocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::MediaCaptureFocusState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureFocusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFocusChangedEventArgs;{81e1bc7f-2277-493e-abee-d3f44ff98c04})");
}
unsafe impl ::windows::core::Interface for MediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e1bc7f_2277_493e_abee_d3f44ff98c04);
}
impl ::windows::core::RuntimeName for MediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFocusChangedEventArgs";
}
impl ::core::convert::From<MediaCaptureFocusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureFocusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureFocusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureFocusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureFocusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureFocusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureFocusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureFocusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureFocusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureFocusChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureInitializationSettings(pub ::windows::core::IInspectable);
impl MediaCaptureInitializationSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaCaptureInitializationSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetAudioDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetVideoDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__: StreamingCaptureMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    pub fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__: PhotoCaptureSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    pub fn SetMediaCategory(&self, value: MediaCategory) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__: MediaCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCategory>(result__)
        }
    }
    pub fn SetAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetAudioSource<'a, Param0: ::windows::core::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn SetVideoSource<'a, Param0: ::windows::core::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    pub fn VideoProfile(&self) -> ::windows::core::Result<MediaCaptureVideoProfile> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfile>(result__)
        }
    }
    pub fn SetVideoProfile<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PreviewMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    pub fn SetPreviewMediaDescription<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RecordMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    pub fn SetRecordMediaDescription<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PhotoMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    pub fn SetPhotoMediaDescription<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SourceGroup(&self) -> ::windows::core::Result<Frames::MediaFrameSourceGroup> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Frames::MediaFrameSourceGroup>(result__)
        }
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SetSourceGroup<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSourceGroup>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<MediaCaptureSharingMode> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: MediaCaptureSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureSharingMode>(result__)
        }
    }
    pub fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MemoryPreference(&self) -> ::windows::core::Result<MediaCaptureMemoryPreference> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: MediaCaptureMemoryPreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureMemoryPreference>(result__)
        }
    }
    pub fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlwaysPlaySystemShutterSound(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn DeviceUriPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetDeviceUriPasswordCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeviceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDeviceUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureInitializationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureInitializationSettings;{9782ba70-ea65-4900-9356-8ca887726884})");
}
unsafe impl ::windows::core::Interface for MediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9782ba70_ea65_4900_9356_8ca887726884);
}
impl ::windows::core::RuntimeName for MediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureInitializationSettings";
}
impl ::core::convert::From<MediaCaptureInitializationSettings> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureInitializationSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureInitializationSettings> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureInitializationSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureInitializationSettings> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureInitializationSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureInitializationSettings> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureInitializationSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureInitializationSettings {}
unsafe impl ::core::marker::Sync for MediaCaptureInitializationSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: MediaCaptureMemoryPreference = MediaCaptureMemoryPreference(0i32);
    pub const Cpu: MediaCaptureMemoryPreference = MediaCaptureMemoryPreference(1i32);
}
impl ::core::convert::From<i32> for MediaCaptureMemoryPreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureMemoryPreference {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureMemoryPreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureMemoryPreference;i4)");
}
impl ::windows::core::DefaultType for MediaCaptureMemoryPreference {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCapturePauseResult(pub ::windows::core::IInspectable);
impl MediaCapturePauseResult {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCapturePauseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapturePauseResult;{aec47ca3-4477-4b04-a06f-2c1c5182fe9d})");
}
unsafe impl ::windows::core::Interface for MediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec47ca3_4477_4b04_a06f_2c1c5182fe9d);
}
impl ::windows::core::RuntimeName for MediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapturePauseResult";
}
impl ::core::convert::From<MediaCapturePauseResult> for ::windows::core::IUnknown {
    fn from(value: MediaCapturePauseResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCapturePauseResult> for ::windows::core::IUnknown {
    fn from(value: &MediaCapturePauseResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCapturePauseResult> for ::windows::core::IInspectable {
    fn from(value: MediaCapturePauseResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCapturePauseResult> for ::windows::core::IInspectable {
    fn from(value: &MediaCapturePauseResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCapturePauseResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCapturePauseResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCapturePauseResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCapturePauseResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureRelativePanelWatcher(pub ::windows::core::IInspectable);
impl MediaCaptureRelativePanelWatcher {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RelativePanel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Enumeration::Panel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::Panel>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureRelativePanelWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureRelativePanelWatcher;{7d896566-04be-5b89-b30e-bd34a9f12db0})");
}
unsafe impl ::windows::core::Interface for MediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d896566_04be_5b89_b30e_bd34a9f12db0);
}
impl ::windows::core::RuntimeName for MediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureRelativePanelWatcher";
}
impl ::core::convert::From<MediaCaptureRelativePanelWatcher> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureRelativePanelWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureRelativePanelWatcher> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureRelativePanelWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureRelativePanelWatcher> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureRelativePanelWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureRelativePanelWatcher> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureRelativePanelWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCaptureRelativePanelWatcher> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCaptureRelativePanelWatcher) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCaptureRelativePanelWatcher> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCaptureRelativePanelWatcher) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureRelativePanelWatcher {}
unsafe impl ::core::marker::Sync for MediaCaptureRelativePanelWatcher {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureSettings(pub ::windows::core::IInspectable);
impl MediaCaptureSettings {
    pub fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__: StreamingCaptureMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    pub fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__: PhotoCaptureSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    pub fn VideoDeviceCharacteristic(&self) -> ::windows::core::Result<VideoDeviceCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: VideoDeviceCharacteristic = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoDeviceCharacteristic>(result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CameraSoundRequiredForRegion(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Horizontal35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PitchOffsetDegrees(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Vertical35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    pub fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: MediaCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCategory>(result__)
        }
    }
    pub fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureSettings;{1d83aafe-6d45-4477-8dc4-ac5bc01c4091})");
}
unsafe impl ::windows::core::Interface for MediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d83aafe_6d45_4477_8dc4_ac5bc01c4091);
}
impl ::windows::core::RuntimeName for MediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureSettings";
}
impl ::core::convert::From<MediaCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: MediaCaptureSharingMode = MediaCaptureSharingMode(0i32);
    pub const SharedReadOnly: MediaCaptureSharingMode = MediaCaptureSharingMode(1i32);
}
impl ::core::convert::From<i32> for MediaCaptureSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureSharingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureSharingMode;i4)");
}
impl ::windows::core::DefaultType for MediaCaptureSharingMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureStopResult(pub ::windows::core::IInspectable);
impl MediaCaptureStopResult {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureStopResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureStopResult;{f9db6a2a-a092-4ad1-97d4-f201f9d082db})");
}
unsafe impl ::windows::core::Interface for MediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9db6a2a_a092_4ad1_97d4_f201f9d082db);
}
impl ::windows::core::RuntimeName for MediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureStopResult";
}
impl ::core::convert::From<MediaCaptureStopResult> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureStopResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureStopResult> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureStopResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureStopResult> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureStopResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureStopResult> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureStopResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCaptureStopResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCaptureStopResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCaptureStopResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCaptureStopResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: MediaCaptureThermalStatus = MediaCaptureThermalStatus(0i32);
    pub const Overheated: MediaCaptureThermalStatus = MediaCaptureThermalStatus(1i32);
}
impl ::core::convert::From<i32> for MediaCaptureThermalStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCaptureThermalStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureThermalStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureThermalStatus;i4)");
}
impl ::windows::core::DefaultType for MediaCaptureThermalStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureVideoProfile(pub ::windows::core::IInspectable);
impl MediaCaptureVideoProfile {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPreviewMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedRecordMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPhotoMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConcurrency(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSourceInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureVideoProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfile;{21a073bf-a3ee-4ecf-9ef6-50b0bc4e1305})");
}
unsafe impl ::windows::core::Interface for MediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21a073bf_a3ee_4ecf_9ef6_50b0bc4e1305);
}
impl ::windows::core::RuntimeName for MediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfile";
}
impl ::core::convert::From<MediaCaptureVideoProfile> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureVideoProfile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfile> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureVideoProfile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureVideoProfile> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureVideoProfile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfile> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureVideoProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureVideoProfile {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfile {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaCaptureVideoProfileMediaDescription(pub ::windows::core::IInspectable);
impl MediaCaptureVideoProfileMediaDescription {
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn FrameRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsVariablePhotoSequenceSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn IsHdrVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureVideoProfileMediaDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription;{8012afef-b691-49ff-83f2-c1e76eaaea1b})");
}
unsafe impl ::windows::core::Interface for MediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8012afef_b691_49ff_83f2_c1e76eaaea1b);
}
impl ::windows::core::RuntimeName for MediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription";
}
impl ::core::convert::From<MediaCaptureVideoProfileMediaDescription> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureVideoProfileMediaDescription) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfileMediaDescription> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureVideoProfileMediaDescription) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaCaptureVideoProfileMediaDescription> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureVideoProfileMediaDescription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfileMediaDescription> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureVideoProfileMediaDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureVideoProfileMediaDescription {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfileMediaDescription {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: MediaCategory = MediaCategory(0i32);
    pub const Communications: MediaCategory = MediaCategory(1i32);
    pub const Media: MediaCategory = MediaCategory(2i32);
    pub const GameChat: MediaCategory = MediaCategory(3i32);
    pub const Speech: MediaCategory = MediaCategory(4i32);
    pub const FarFieldSpeech: MediaCategory = MediaCategory(5i32);
    pub const UniformSpeech: MediaCategory = MediaCategory(6i32);
    pub const VoiceTyping: MediaCategory = MediaCategory(7i32);
}
impl ::core::convert::From<i32> for MediaCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaCategory {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCategory;i4)");
}
impl ::windows::core::DefaultType for MediaCategory {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: MediaStreamType = MediaStreamType(0i32);
    pub const VideoRecord: MediaStreamType = MediaStreamType(1i32);
    pub const Audio: MediaStreamType = MediaStreamType(2i32);
    pub const Photo: MediaStreamType = MediaStreamType(3i32);
    pub const Metadata: MediaStreamType = MediaStreamType(4i32);
}
impl ::core::convert::From<i32> for MediaStreamType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaStreamType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaStreamType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaStreamType;i4)");
}
impl ::windows::core::DefaultType for MediaStreamType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OptionalReferencePhotoCapturedEventArgs(pub ::windows::core::IInspectable);
impl OptionalReferencePhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OptionalReferencePhotoCapturedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs;{470f88b3-1e6d-4051-9c8b-f1d85af047b7})");
}
unsafe impl ::windows::core::Interface for OptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x470f88b3_1e6d_4051_9c8b_f1d85af047b7);
}
impl ::windows::core::RuntimeName for OptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs";
}
impl ::core::convert::From<OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: OptionalReferencePhotoCapturedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &OptionalReferencePhotoCapturedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: OptionalReferencePhotoCapturedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &OptionalReferencePhotoCapturedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OptionalReferencePhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for OptionalReferencePhotoCapturedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: PhotoCaptureSource = PhotoCaptureSource(0i32);
    pub const VideoPreview: PhotoCaptureSource = PhotoCaptureSource(1i32);
    pub const Photo: PhotoCaptureSource = PhotoCaptureSource(2i32);
}
impl ::core::convert::From<i32> for PhotoCaptureSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoCaptureSource {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoCaptureSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PhotoCaptureSource;i4)");
}
impl ::windows::core::DefaultType for PhotoCaptureSource {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoCapturedEventArgs(pub ::windows::core::IInspectable);
impl PhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    pub fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoCapturedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoCapturedEventArgs;{373bfbc1-984e-4ff0-bf85-1c00aabc5a45})");
}
unsafe impl ::windows::core::Interface for PhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x373bfbc1_984e_4ff0_bf85_1c00aabc5a45);
}
impl ::windows::core::RuntimeName for PhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoCapturedEventArgs";
}
impl ::core::convert::From<PhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoCapturedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoCapturedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoCapturedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoCapturedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoCapturedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoConfirmationCapturedEventArgs(pub ::windows::core::IInspectable);
impl PhotoConfirmationCapturedEventArgs {
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoConfirmationCapturedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoConfirmationCapturedEventArgs;{ab473672-c28a-4827-8f8d-3636d3beb51e})");
}
unsafe impl ::windows::core::Interface for PhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab473672_c28a_4827_8f8d_3636d3beb51e);
}
impl ::windows::core::RuntimeName for PhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoConfirmationCapturedEventArgs";
}
impl ::core::convert::From<PhotoConfirmationCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoConfirmationCapturedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoConfirmationCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoConfirmationCapturedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoConfirmationCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoConfirmationCapturedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoConfirmationCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoConfirmationCapturedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoConfirmationCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoConfirmationCapturedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: PowerlineFrequency = PowerlineFrequency(0i32);
    pub const FiftyHertz: PowerlineFrequency = PowerlineFrequency(1i32);
    pub const SixtyHertz: PowerlineFrequency = PowerlineFrequency(2i32);
    pub const Auto: PowerlineFrequency = PowerlineFrequency(3i32);
}
impl ::core::convert::From<i32> for PowerlineFrequency {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PowerlineFrequency {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PowerlineFrequency {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PowerlineFrequency;i4)");
}
impl ::windows::core::DefaultType for PowerlineFrequency {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RecordLimitationExceededEventHandler(::windows::core::IUnknown);
impl RecordLimitationExceededEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = RecordLimitationExceededEventHandler_box::<F> {
            vtable: &RecordLimitationExceededEventHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, MediaCapture>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for RecordLimitationExceededEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({3fae8f2e-4fe1-4ffd-aaba-e1f1337d4e53})");
}
unsafe impl ::windows::core::Interface for RecordLimitationExceededEventHandler {
    type Vtable = RecordLimitationExceededEventHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fae8f2e_4fe1_4ffd_aaba_e1f1337d4e53);
}
#[repr(C)]
#[doc(hidden)]
pub struct RecordLimitationExceededEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct RecordLimitationExceededEventHandler_box<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const RecordLimitationExceededEventHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows::core::Result<()> + 'static> RecordLimitationExceededEventHandler_box<F> {
    const VTABLE: RecordLimitationExceededEventHandler_abi = RecordLimitationExceededEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RecordLimitationExceededEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <MediaCapture as ::windows::core::Abi>::Abi as *const <MediaCapture as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ScreenCapture(pub ::windows::core::IInspectable);
impl ScreenCapture {
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    pub fn IsAudioSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SourceSuspensionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceSuspensionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<ScreenCapture> {
        Self::IScreenCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ScreenCapture>(result__)
        })
    }
    pub fn IScreenCaptureStatics<R, F: FnOnce(&IScreenCaptureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScreenCapture, IScreenCaptureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ScreenCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.ScreenCapture;{89179ef7-cd12-4e0e-a6d4-5b3de98b2e9b})");
}
unsafe impl ::windows::core::Interface for ScreenCapture {
    type Vtable = IScreenCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89179ef7_cd12_4e0e_a6d4_5b3de98b2e9b);
}
impl ::windows::core::RuntimeName for ScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.ScreenCapture";
}
impl ::core::convert::From<ScreenCapture> for ::windows::core::IUnknown {
    fn from(value: ScreenCapture) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScreenCapture> for ::windows::core::IUnknown {
    fn from(value: &ScreenCapture) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScreenCapture> for ::windows::core::IInspectable {
    fn from(value: ScreenCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScreenCapture> for ::windows::core::IInspectable {
    fn from(value: &ScreenCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScreenCapture {}
unsafe impl ::core::marker::Sync for ScreenCapture {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SourceSuspensionChangedEventArgs(pub ::windows::core::IInspectable);
impl SourceSuspensionChangedEventArgs {
    pub fn IsAudioSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsVideoSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SourceSuspensionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.SourceSuspensionChangedEventArgs;{2ece7b5e-d49b-4394-bc32-f97d6cedec1c})");
}
unsafe impl ::windows::core::Interface for SourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ece7b5e_d49b_4394_bc32_f97d6cedec1c);
}
impl ::windows::core::RuntimeName for SourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.SourceSuspensionChangedEventArgs";
}
impl ::core::convert::From<SourceSuspensionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SourceSuspensionChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SourceSuspensionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SourceSuspensionChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SourceSuspensionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SourceSuspensionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SourceSuspensionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SourceSuspensionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SourceSuspensionChangedEventArgs {}
unsafe impl ::core::marker::Sync for SourceSuspensionChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: StreamingCaptureMode = StreamingCaptureMode(0i32);
    pub const Audio: StreamingCaptureMode = StreamingCaptureMode(1i32);
    pub const Video: StreamingCaptureMode = StreamingCaptureMode(2i32);
}
impl ::core::convert::From<i32> for StreamingCaptureMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for StreamingCaptureMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for StreamingCaptureMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.StreamingCaptureMode;i4)");
}
impl ::windows::core::DefaultType for StreamingCaptureMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: VideoDeviceCharacteristic = VideoDeviceCharacteristic(0i32);
    pub const PreviewRecordStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(1i32);
    pub const PreviewPhotoStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(2i32);
    pub const RecordPhotoStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(3i32);
    pub const AllStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(4i32);
}
impl ::core::convert::From<i32> for VideoDeviceCharacteristic {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VideoDeviceCharacteristic {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceCharacteristic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoDeviceCharacteristic;i4)");
}
impl ::windows::core::DefaultType for VideoDeviceCharacteristic {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: VideoRotation = VideoRotation(0i32);
    pub const Clockwise90Degrees: VideoRotation = VideoRotation(1i32);
    pub const Clockwise180Degrees: VideoRotation = VideoRotation(2i32);
    pub const Clockwise270Degrees: VideoRotation = VideoRotation(3i32);
}
impl ::core::convert::From<i32> for VideoRotation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VideoRotation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VideoRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoRotation;i4)");
}
impl ::windows::core::DefaultType for VideoRotation {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VideoStreamConfiguration(pub ::windows::core::IInspectable);
impl VideoStreamConfiguration {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn InputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn OutputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VideoStreamConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.VideoStreamConfiguration;{d8770a6f-4390-4b5e-ad3e-0f8af0963490})");
}
unsafe impl ::windows::core::Interface for VideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8770a6f_4390_4b5e_ad3e_0f8af0963490);
}
impl ::windows::core::RuntimeName for VideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.VideoStreamConfiguration";
}
impl ::core::convert::From<VideoStreamConfiguration> for ::windows::core::IUnknown {
    fn from(value: VideoStreamConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VideoStreamConfiguration> for ::windows::core::IUnknown {
    fn from(value: &VideoStreamConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VideoStreamConfiguration> for ::windows::core::IInspectable {
    fn from(value: VideoStreamConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VideoStreamConfiguration> for ::windows::core::IInspectable {
    fn from(value: &VideoStreamConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VideoStreamConfiguration {}
unsafe impl ::core::marker::Sync for VideoStreamConfiguration {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WhiteBalanceGain {
    pub R: f64,
    pub G: f64,
    pub B: f64,
}
impl WhiteBalanceGain {}
impl ::core::default::Default for WhiteBalanceGain {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WhiteBalanceGain {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WhiteBalanceGain").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::core::cmp::PartialEq for WhiteBalanceGain {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for WhiteBalanceGain {}
unsafe impl ::windows::core::Abi for WhiteBalanceGain {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WhiteBalanceGain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Capture.WhiteBalanceGain;f8;f8;f8)");
}
impl ::windows::core::DefaultType for WhiteBalanceGain {
    type DefaultType = Self;
}
