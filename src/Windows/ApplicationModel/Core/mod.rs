#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppListEntry(pub ::windows::core::IInspectable);
impl AppListEntry {
    pub fn DisplayInfo(&self) -> ::windows::core::Result<super::AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AppDisplayInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppListEntry2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn LaunchForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(&self, user: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IAppListEntry3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo> {
        let this = &::windows::core::Interface::cast::<IAppListEntry4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AppInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppListEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.AppListEntry;{ef00f07f-2108-490a-877a-8a9f17c25fad})");
}
unsafe impl ::windows::core::Interface for AppListEntry {
    type Vtable = IAppListEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef00f07f_2108_490a_877a_8a9f17c25fad);
}
impl ::windows::core::RuntimeName for AppListEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Core.AppListEntry";
}
impl ::core::convert::From<AppListEntry> for ::windows::core::IUnknown {
    fn from(value: AppListEntry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppListEntry> for ::windows::core::IUnknown {
    fn from(value: &AppListEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppListEntry> for ::windows::core::IInspectable {
    fn from(value: AppListEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppListEntry> for ::windows::core::IInspectable {
    fn from(value: &AppListEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppListEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppListEntry {}
unsafe impl ::core::marker::Sync for AppListEntry {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppRestartFailureReason(pub i32);
impl AppRestartFailureReason {
    pub const RestartPending: AppRestartFailureReason = AppRestartFailureReason(0i32);
    pub const NotInForeground: AppRestartFailureReason = AppRestartFailureReason(1i32);
    pub const InvalidUser: AppRestartFailureReason = AppRestartFailureReason(2i32);
    pub const Other: AppRestartFailureReason = AppRestartFailureReason(3i32);
}
impl ::core::convert::From<i32> for AppRestartFailureReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppRestartFailureReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppRestartFailureReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Core.AppRestartFailureReason;i4)");
}
impl ::windows::core::DefaultType for AppRestartFailureReason {
    type DefaultType = Self;
}
pub struct CoreApplication {}
impl CoreApplication {
    pub fn Id() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Suspending<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::SuspendingEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuspending<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn Resuming<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveResuming<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties() -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        })
    }
    pub fn GetCurrentView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn Run<'a, Param0: ::windows::core::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), viewsource.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RunWithActivationFactories<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IGetActivationFactory>>(activationfactorycallback: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication(|this| unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), activationfactorycallback.into_param().abi()).ok() })
    }
    pub fn Exit() -> ::windows::core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn Exiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationExit(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveExiting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplicationExit(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn UnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplicationUnhandledError(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplicationUnhandledError(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IncrementApplicationUseCount() -> ::windows::core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn DecrementApplicationUseCount() -> ::windows::core::Result<()> {
        Self::ICoreApplicationUseCount(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Views() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<CoreApplicationView>>(result__)
        })
    }
    pub fn CreateNewView<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(runtimetype: Param0, entrypoint: Param1) -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), runtimetype.into_param().abi(), entrypoint.into_param().abi(), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn MainView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn CreateNewViewFromMainView() -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    pub fn CreateNewViewWithViewSource<'a, Param0: ::windows::core::IntoParam<'a, IFrameworkViewSource>>(viewsource: Param0) -> ::windows::core::Result<CoreApplicationView> {
        Self::ICoreImmersiveApplication3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), viewsource.into_param().abi(), &mut result__).from_abi::<CoreApplicationView>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn LeavingBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLeavingBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn EnteredBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ICoreApplication2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnteredBackground<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> ::windows::core::Result<()> {
        Self::ICoreApplication2(|this| unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestRestartAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(launcharguments: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), launcharguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn RequestRestartForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, launcharguments: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>> {
        Self::ICoreApplication3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), launcharguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>(result__)
        })
    }
    pub fn ICoreApplication<R, F: FnOnce(&ICoreApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationExit<R, F: FnOnce(&ICoreApplicationExit) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationExit> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationUnhandledError<R, F: FnOnce(&ICoreApplicationUnhandledError) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationUnhandledError> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplicationUseCount<R, F: FnOnce(&ICoreApplicationUseCount) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplicationUseCount> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication<R, F: FnOnce(&ICoreImmersiveApplication) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication2<R, F: FnOnce(&ICoreImmersiveApplication2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreImmersiveApplication3<R, F: FnOnce(&ICoreImmersiveApplication3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreImmersiveApplication3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplication2<R, F: FnOnce(&ICoreApplication2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICoreApplication3<R, F: FnOnce(&ICoreApplication3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreApplication, ICoreApplication3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CoreApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplication";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreApplicationView(pub ::windows::core::IInspectable);
impl CoreApplicationView {
    #[cfg(feature = "UI_Core")]
    pub fn CoreWindow(&self) -> ::windows::core::Result<super::super::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Core::CoreWindow>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IsMain(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsHosted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::core::Result<super::super::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Core::CoreDispatcher>(result__)
        }
    }
    pub fn IsComponent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn TitleBar(&self) -> ::windows::core::Result<CoreApplicationViewTitleBar> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CoreApplicationViewTitleBar>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HostedViewClosing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveHostedViewClosing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<ICoreApplicationView6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreApplicationView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationView;{638bb2db-451d-4661-b099-414f34ffb9f1})");
}
unsafe impl ::windows::core::Interface for CoreApplicationView {
    type Vtable = ICoreApplicationView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x638bb2db_451d_4661_b099_414f34ffb9f1);
}
impl ::windows::core::RuntimeName for CoreApplicationView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationView";
}
impl ::core::convert::From<CoreApplicationView> for ::windows::core::IUnknown {
    fn from(value: CoreApplicationView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreApplicationView> for ::windows::core::IUnknown {
    fn from(value: &CoreApplicationView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreApplicationView> for ::windows::core::IInspectable {
    fn from(value: CoreApplicationView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreApplicationView> for ::windows::core::IInspectable {
    fn from(value: &CoreApplicationView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CoreApplicationViewTitleBar(pub ::windows::core::IInspectable);
impl CoreApplicationViewTitleBar {
    pub fn SetExtendViewIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ExtendViewIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SystemOverlayLeftInset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SystemOverlayRightInset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LayoutMetricsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLayoutMetricsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsVisibleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsVisibleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CoreApplicationViewTitleBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.CoreApplicationViewTitleBar;{006d35e3-e1f1-431b-9508-29b96926ac53})");
}
unsafe impl ::windows::core::Interface for CoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x006d35e3_e1f1_431b_9508_29b96926ac53);
}
impl ::windows::core::RuntimeName for CoreApplicationViewTitleBar {
    const NAME: &'static str = "Windows.ApplicationModel.Core.CoreApplicationViewTitleBar";
}
impl ::core::convert::From<CoreApplicationViewTitleBar> for ::windows::core::IUnknown {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CoreApplicationViewTitleBar> for ::windows::core::IUnknown {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CoreApplicationViewTitleBar> for ::windows::core::IInspectable {
    fn from(value: CoreApplicationViewTitleBar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CoreApplicationViewTitleBar> for ::windows::core::IInspectable {
    fn from(value: &CoreApplicationViewTitleBar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CoreApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HostedViewClosingEventArgs(pub ::windows::core::IInspectable);
impl HostedViewClosingEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for HostedViewClosingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.HostedViewClosingEventArgs;{d238943c-b24e-4790-acb5-3e4243c4ff87})");
}
unsafe impl ::windows::core::Interface for HostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd238943c_b24e_4790_acb5_3e4243c4ff87);
}
impl ::windows::core::RuntimeName for HostedViewClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.HostedViewClosingEventArgs";
}
impl ::core::convert::From<HostedViewClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HostedViewClosingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HostedViewClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: HostedViewClosingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HostedViewClosingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HostedViewClosingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HostedViewClosingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HostedViewClosingEventArgs {}
unsafe impl ::core::marker::Sync for HostedViewClosingEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppListEntry {
    type Vtable = IAppListEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef00f07f_2108_490a_877a_8a9f17c25fad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppListEntry2 {
    type Vtable = IAppListEntry2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0a618ad_bf35_42ac_ac06_86eeeb41d04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppListEntry3 {
    type Vtable = IAppListEntry3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6099f28d_fc32_470a_bc69_4b061a76ef2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppListEntry4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppListEntry4 {
    type Vtable = IAppListEntry4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a131ed2_56f5_487c_8697_5166f3b33da0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppListEntry4_abi(
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
pub struct ICoreApplication(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplication {
    type Vtable = ICoreApplication_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aacf7a4_5e1d_49df_8034_fb6a68bc5ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activationfactorycallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplication2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplication2 {
    type Vtable = ICoreApplication2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x998681fb_1ab6_4b7f_be4a_9a0645224c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplication3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplication3 {
    type Vtable = ICoreApplication3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeec0d39_598b_4507_8a67_772632580a57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplication3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationExit(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationExit {
    type Vtable = ICoreApplicationExit_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf86461d_261e_4b72_9acd_44ed2ace6a29);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationExit_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICoreApplicationUnhandledError(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationUnhandledError {
    type Vtable = ICoreApplicationUnhandledError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0e24ab0_dd09_42e1_b0bc_e0e131f78d7e);
}
impl ICoreApplicationUnhandledError {
    #[cfg(feature = "Foundation")]
    pub fn UnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnhandledErrorDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ICoreApplicationUnhandledError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f0e24ab0-dd09-42e1-b0bc-e0e131f78d7e}");
}
impl ::core::convert::From<ICoreApplicationUnhandledError> for ::windows::core::IUnknown {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICoreApplicationUnhandledError> for ::windows::core::IUnknown {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICoreApplicationUnhandledError> for ::windows::core::IInspectable {
    fn from(value: ICoreApplicationUnhandledError) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICoreApplicationUnhandledError> for ::windows::core::IInspectable {
    fn from(value: &ICoreApplicationUnhandledError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICoreApplicationUnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUnhandledError_abi(
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
pub struct ICoreApplicationUseCount(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationUseCount {
    type Vtable = ICoreApplicationUseCount_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x518dc408_c077_475b_809e_0bc0c57e4b74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationUseCount_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationView {
    type Vtable = ICoreApplicationView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x638bb2db_451d_4661_b099_414f34ffb9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationView2 {
    type Vtable = ICoreApplicationView2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68eb7adf_917f_48eb_9aeb_7de53e086ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationView3 {
    type Vtable = ICoreApplicationView3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07ebe1b3_a4cf_4550_ab70_b07e85330bc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationView5 {
    type Vtable = ICoreApplicationView5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc095a8_8ef0_446d_9e60_3a3e0428c671);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationView6(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationView6 {
    type Vtable = ICoreApplicationView6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc119d49a_0679_49ba_803f_b79c5cf34cca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationView6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreApplicationViewTitleBar(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreApplicationViewTitleBar {
    type Vtable = ICoreApplicationViewTitleBar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x006d35e3_e1f1_431b_9508_29b96926ac53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreApplicationViewTitleBar_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication {
    type Vtable = ICoreImmersiveApplication_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ada0e3e_e4a2_4123_b451_dc96bf800419);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, runtimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication2 {
    type Vtable = ICoreImmersiveApplication2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x828e1e36_e9e3_4cfc_9b66_48b78ea9bb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication2_abi(
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
pub struct ICoreImmersiveApplication3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICoreImmersiveApplication3 {
    type Vtable = ICoreImmersiveApplication3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a05b2f_ee0d_41e5_8314_cf10c91bf0af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreImmersiveApplication3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFrameworkView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFrameworkView {
    type Vtable = IFrameworkView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaab5cd0_8924_45ac_ad0f_a08fae5d0324);
}
impl IFrameworkView {
    pub fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, CoreApplicationView>>(&self, applicationview: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), applicationview.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn SetWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Core::CoreWindow>>(&self, window: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), window.into_param().abi()).ok() }
    }
    pub fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, entrypoint: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), entrypoint.into_param().abi()).ok() }
    }
    pub fn Run(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Uninitialize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IFrameworkView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{faab5cd0-8924-45ac-ad0f-a08fae5d0324}");
}
impl ::core::convert::From<IFrameworkView> for ::windows::core::IUnknown {
    fn from(value: IFrameworkView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFrameworkView> for ::windows::core::IUnknown {
    fn from(value: &IFrameworkView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFrameworkView> for ::windows::core::IInspectable {
    fn from(value: IFrameworkView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFrameworkView> for ::windows::core::IInspectable {
    fn from(value: &IFrameworkView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFrameworkView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, applicationview: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFrameworkViewSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFrameworkViewSource {
    type Vtable = IFrameworkViewSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd770614_65c4_426c_9494_34fc43554862);
}
impl IFrameworkViewSource {
    pub fn CreateView(&self) -> ::windows::core::Result<IFrameworkView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IFrameworkView>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFrameworkViewSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cd770614-65c4-426c-9494-34fc43554862}");
}
impl ::core::convert::From<IFrameworkViewSource> for ::windows::core::IUnknown {
    fn from(value: IFrameworkViewSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFrameworkViewSource> for ::windows::core::IUnknown {
    fn from(value: &IFrameworkViewSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFrameworkViewSource> for ::windows::core::IInspectable {
    fn from(value: IFrameworkViewSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFrameworkViewSource> for ::windows::core::IInspectable {
    fn from(value: &IFrameworkViewSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFrameworkViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkViewSource_abi(
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
pub struct IHostedViewClosingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHostedViewClosingEventArgs {
    type Vtable = IHostedViewClosingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd238943c_b24e_4790_acb5_3e4243c4ff87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostedViewClosingEventArgs_abi(
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
pub struct IUnhandledError(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUnhandledError {
    type Vtable = IUnhandledError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9459b726_53b5_4686_9eaf_fa8162dc3980);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledError_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnhandledErrorDetectedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x679ab78b_b336_4822_ac40_0d750f0b7a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledErrorDetectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UnhandledError(pub ::windows::core::IInspectable);
impl UnhandledError {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Propagate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UnhandledError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledError;{9459b726-53b5-4686-9eaf-fa8162dc3980})");
}
unsafe impl ::windows::core::Interface for UnhandledError {
    type Vtable = IUnhandledError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9459b726_53b5_4686_9eaf_fa8162dc3980);
}
impl ::windows::core::RuntimeName for UnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledError";
}
impl ::core::convert::From<UnhandledError> for ::windows::core::IUnknown {
    fn from(value: UnhandledError) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UnhandledError> for ::windows::core::IUnknown {
    fn from(value: &UnhandledError) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UnhandledError> for ::windows::core::IInspectable {
    fn from(value: UnhandledError) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UnhandledError> for ::windows::core::IInspectable {
    fn from(value: &UnhandledError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UnhandledError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UnhandledError {}
unsafe impl ::core::marker::Sync for UnhandledError {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UnhandledErrorDetectedEventArgs(pub ::windows::core::IInspectable);
impl UnhandledErrorDetectedEventArgs {
    pub fn UnhandledError(&self) -> ::windows::core::Result<UnhandledError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnhandledError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UnhandledErrorDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs;{679ab78b-b336-4822-ac40-0d750f0b7a2b})");
}
unsafe impl ::windows::core::Interface for UnhandledErrorDetectedEventArgs {
    type Vtable = IUnhandledErrorDetectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x679ab78b_b336_4822_ac40_0d750f0b7a2b);
}
impl ::windows::core::RuntimeName for UnhandledErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.UnhandledErrorDetectedEventArgs";
}
impl ::core::convert::From<UnhandledErrorDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UnhandledErrorDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UnhandledErrorDetectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UnhandledErrorDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UnhandledErrorDetectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UnhandledErrorDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UnhandledErrorDetectedEventArgs {}
unsafe impl ::core::marker::Sync for UnhandledErrorDetectedEventArgs {}
